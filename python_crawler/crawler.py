import asyncio
import multiprocessing
from datetime import datetime, timedelta
from urllib.parse import urlparse, urljoin
import requests
import json
from bs4 import BeautifulSoup
import sys
import queue
from database.elastic_manipulator import ElasticSearchDB


def get_many(mp_queue, queue, n):
    """
    Safely gets n items from the queue
    and inserts them into another one
    (if there are less items takes all of them)
    (if there are no items returns False)
    """
    if mp_queue.empty():
        return False
    for i in range(n):
        if mp_queue.empty():
            break
        item = mp_queue.get()
        queue.put_nowait(item)
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

    def __init__(self, queue, db, max_depth=2,
                 concurrent_tasks=4, max_queue_size=32,
                 max_cycle=3, delay=2):
        self.delay = delay
        self.db = db
        self.max_depth = max_depth
        self.max_queue_size = max_queue_size
        self.concurrent_tasks = concurrent_tasks
        self.max_cycle = max_cycle
        self.global_queue = queue

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

    def parse(self, response):
        """
        Parses website data to get text, structured data and links
        """
        parser = BeautifulSoup(response.text, 'html.parser')

        text = parser.get_text()

        structured_data = None
        elem = parser.find("script", {"type": "application/ld+json"})
        if elem is not None:
            structured_data = json.loads(elem.contents[0])
        
        title = parser.title
        if title is not None:
            title = title.contents[0]

        links = [element.get("href") for element in parser.find_all("a")]

        sorted_links = self.sort_links(response.url, links)

        return {
            'url': response.url,
            'text': text,
            'structured_data': structured_data,
            'title': title,
            'external_links': sorted_links["external_links"],
            'internal_links': sorted_links["internal_links"]
            }

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
        data = self.parse(response)

        self.send_data(data, item.depth + 1)
        print(1)

    def back_to_queue(self, item):
        """
        Returns item to the queue if it hasn't returned
        url content
        """
        if item.cycle > item.max_cycle:
            return
        
        # time when we will be able to process this item
        item.time = datetime.now() + timedelta(
            seconds=self.delay ** item.cycle
            )
        item.cycle += 1
        self.global_queue.put(item)

    def send_data(self, data, depth):
        """
        Sends data somewhere
        !! TO DO: connect to something
        """
        if depth <= self.max_depth:
            for link in data["internal_links"]:
                self.global_queue.put(Item(link, datetime.now(), depth))
        self.db.add_data(data, data["url"])
        print(data)

    async def process_all(self):
        """
        Processes all items in local the queue
        """
        while True:
            try:
                item = self.global_queue.get(True, 2)
                self.process_item(item)
            except queue.Empty:
                break

    async def crawl_global_queue(self):
        """
        Asynchronously processes all items in the local queue
        """
        tasks = []

        for i in range(self.concurrent_tasks):
            task = asyncio.create_task(self.process_all())
            tasks.append(task)

        await asyncio.gather(*tasks, return_exceptions=True)


def multiprocessing_crawl(queue, db, max_depth=2, processes=4,
                          concurrent_tasks=4, max_queue_size=32,
                          max_cycle=3, delay=2):
    """
    Processes all items in the queue using multiple sub-processes
    """
    crawlers = [Crawler(
        queue, db, max_depth=max_depth, concurrent_tasks=concurrent_tasks,
        max_queue_size=max_queue_size, max_cycle=max_cycle,
        delay=delay
    ) for i in range(processes - 1)]

    processes = [
        multiprocessing.Process(
            target=asyncio.run, args=(item.crawl_global_queue(),)
            )
        for item in crawlers]

    for item in processes:
        item.start()

    asyncio.run(Crawler(
        queue, db, max_depth=max_depth, concurrent_tasks=concurrent_tasks,
        max_queue_size=max_queue_size, max_cycle=max_cycle,
        delay=delay
    ).crawl_global_queue())

    for item in processes:
        item.join()


def main():
    args = sys.argv
    filename = args[1]
    db = ElasticSearchDB()
    if not db.connect("http://localhost", 9200):
        print("Can't connect to elastic")
        return -1
    print("Connected to elastic")

    queue = multiprocessing.Queue()
    with open(filename) as f:
        for line in f.readlines():
            queue.put(Item(line.strip(), datetime.now()))
            print(line.strip())

    multiprocessing_crawl(queue, db, *map(int, args[2:]))
    print("Finished")


if __name__ == "__main__":
    main()
