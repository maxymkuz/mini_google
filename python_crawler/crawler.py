import asyncio
import multiprocessing
from datetime import datetime, timedelta
from urllib.parse import urlparse, urljoin
import requests
import json
from bs4 import BeautifulSoup
import sys



class SafeQueue:
    """
    Queue class for multiprocess programs
    """
    def __init__(self):
        self.queue = asyncio.Queue()
        self.mutex = multiprocessing.Lock()

    def get(self):
        """
        Safely gets item from the queue
        """
        try:
            self.mutex.acquire()
            item = self.queue.get_nowait()
        except asyncio.QueueEmpty:
            return None
        finally:
            self.mutex.release()

        return item

    def put(self, item):
        """
        Safely puts item to the queue
        """
        try:
            self.mutex.acquire()
            self.queue.put_nowait(item)
        finally:
            self.mutex.release()

    def empty(self):
        """
        Checks if queue is empty
        """
        return self.queue.empty()

    def get_many(self, queue, n):
        """
        Safely gets n items from the queue
        and inserts them into another one
        (if there are less items takes all of them)
        (if there are no items returns False)
        """
        self.mutex.acquire()
        if self.empty():
            self.mutex.release()
            return False
        for i in range(n):
            if self.queue.empty():
                break
            item = self.queue.get_nowait()
            queue.put_nowait(item)
        self.mutex.release()
        return True

class Item:
    """
    Contains information about item that we won't to
    scrape
    """
    def __init__(self, url, time, depth=1):
        self.time = time
        self.cycle = 1
        self.url = url
        self.depth = depth


class Crawler:
    """
    Perfrom asynchronous crawling of given queue of items
    """
    def __init__(self, queue, max_depth=2, concurrent_tasks=4,  max_queue_size=32, max_cycle=3, delay=2):
        self.delay = delay
        self.max_depth = max_depth
        self.max_queue_size = max_queue_size
        self.concurrent_tasks = concurrent_tasks
        self.max_cycle = max_cycle
        self.global_queue = queue
        self.local_queue = None

    @staticmethod
    def sort_links(page_url, links):
        """
        Parses links and checks which are internal and external
        """
        domain = urlparse(page_url).netloc
        sorted_links = {"external_links": set(), "internal_links": set()}
        for url in links:
            if url is None or url == "":
                continue
            parsed = urlparse(urljoin(page_url, url))
            new_link = f"{parsed.scheme}://{parsed.netloc}{parsed.path}"
            parsed = urlparse(new_link)
            if parsed.netloc and parsed.scheme:
                if parsed.netloc == domain:
                    sorted_links["internal_links"].add(new_link)
                else:
                    sorted_links["external_links"].add(new_link)
        sorted_links["internal_links"] = list(sorted_links["internal_links"])
        sorted_links["external_links"] = list(sorted_links["external_links"])
        return sorted_links


    def get_response(self, url):
        return requests.get(url)

    def parse(self, response, depth):
        """
        Parses website data to get text, structured data and links
        """
        parser = BeautifulSoup(response.text, 'html.parser')

        text = parser.get_text

        structured_data = None
        elem = parser.find("script", {"type": "application/ld+json"})
        if elem is not None:
            structured_data = json.loads(elem.contents[0])

        if depth < self.max_depth:
            links = [element.get("href") for element in parser.find_all("a")]
        else:
            links = []

        sorted_links = self.sort_links(response.url, links)

        return {'url': response.url, 'text': text, 'structured_data': structured_data,
                'external_links': sorted_links["external_links"], 'internal_links': sorted_links["internal_links"]}


    def process_item(self, item):
        """
        Processes one item from the queue
        """
        if item.time > datetime.now():
            self.global_queue.put(item)
            return

        response = self.get_response(item.url)
        print(f"Responded {item.url}: {response.status_code}")
        if (response.status_code == 404):
            return
        elif (response.status_code != requests.codes.ok):
            self.back_to_queue(item)
            return
        data = self.parse(response, item.depth)

        self.send_data(data, item.depth+1)

    def back_to_queue(self, item):
        """
        Returns item to the queue if it is too early to scrape it
        """
        if item.cycle > item.max_cycle:
            return
        item.time = datetime.now() + timedelta(seconds=self.delay**item.cycle)
        item.cycle += 1
        self.global_queue.put(item)

    def send_data(self, data, depth):
        """
        Sends data somewhere
        !! TO DO: connect to something
        """
        for link in data["internal_links"]:
            self.global_queue.put(Item(link, datetime.now(), depth))

    async def process_all(self):
        """
        Processes all items in local the queue
        """
        while True:
            item = await self.local_queue.get()
            self.process_item(item)
            self.local_queue.task_done()

    async def crawl_local_queue(self):
        """
        Asynchronously processes all items in the local queue
        """
        tasks = []

        for i in range(self.concurrent_tasks):
            task = asyncio.create_task(self.process_all())
            tasks.append(task)

        await self.local_queue.join()
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)
        print("Crawled one queue")

    async def crawl_global_queue(self):
        """
        Asynchronously processes all items in the global queue
        """
        prev_empty = False
        while True:
            self.local_queue = asyncio.Queue(self.max_queue_size)
            not_empty = self.global_queue.get_many(self.local_queue, self.max_queue_size)
            if not not_empty:
                if prev_empty:
                    break
                else:
                    prev_empty = True
                    await asyncio.sleep(1)
            else:
                prev_empty = False
            await self.crawl_local_queue()



def multiprocessing_crawl(queue, max_depth=2, processes=4, concurrent_tasks=4,
                          max_queue_size=32, max_cycle=3, delay=2):
    """
    Processes all items in the queue using multiple sub-processes
    """
    crawlers = [Crawler(
        queue, max_depth=max_depth, concurrent_tasks=concurrent_tasks,
        max_queue_size=max_queue_size, max_cycle=max_cycle,
        delay=delay
    ) for i in range(processes-1)]
    processes = [multiprocessing.Process(target=asyncio.run, args=(item.crawl_global_queue(),)) for item in crawlers]
    for item in processes:
        item.start()


    asyncio.run(Crawler(
        queue, max_depth=max_depth, concurrent_tasks=concurrent_tasks,
        max_queue_size=max_queue_size, max_cycle=max_cycle,
        delay=delay
    ).crawl_global_queue())

    for item in processes:
        item.join()



def main():
    args = sys.argv
    filename = args[1]
    queue = SafeQueue()
    with open(filename) as f:
        for line in f.readlines():
            queue.put(Item(line.strip(), datetime.now()))
            print(line.strip())

    multiprocessing_crawl(queue, *map(int, args[2:]))
    print("Finished")


if __name__ == "__main__":
    main()