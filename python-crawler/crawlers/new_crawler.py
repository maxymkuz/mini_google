from bs4 import BeautifulSoup
import requests
import json
import asyncio
import multiprocessing as mp
import time

def send_data(data):
    print(data)


def process_links(all_links, curr_link):
    """
    """
    links = {"internal": [], "external": []}
    for url in all_links:
        if url[:2] == "//":
            links["internal"].append("http:" + url)

        elif url[0] == "/":
            links["internal"].append(curr_link + url)

        elif url[0] == "#" or url == curr_link:
            continue
    return links



async def scrape(url):
    """
    Returns a parser for the given page
    or None if can't access it
    :param url: str
    :return: BeautifulSoup
    """
    try:
        return BeautifulSoup(requests.get(url).text, 'html.parser'), url
    except requests.exceptions.RequestException as e:
        print(f"Error while scraping: {url}")
        print(e)
        return None

async def parse(parser, curr_link, depth, max_depth=2):
    structured_data = None
    elem = parser.find("script", {"type": "application/ld+json"})
    if elem is not None:
        structured_data = json.loads(elem.contents[0])

    text = parser.get_text()

    links = {"internal": [], "external": []}
    if depth < max_depth:
        all_links = []
        for element in parser.find_all("a"):
            link = element.get("href")
            if link is None:
                continue
            all_links.append(link)
        links = process_links(all_links, curr_link)

    send_data(({"text": text, "structured_data": structured_data}, links["external"]))
    return links["internal"]



async def concurrent_processing(links_queue, parser_queue, max_depth, bad_links_queue):
    scraping_task = None
    if not links_queue.empty():
        link, depth = links_queue.get()
        scraping_task = asyncio.create_task(scrape(link))

    parsing_task = None
    if not parser_queue.empty():
        parser, parser_link, parser_depth = parser_queue.get()
        parsing_task = asyncio.create_task(parse(parser,
                                                 parser_link,
                                                 parser_depth,
                                                 max_depth=max_depth))

    empty = True
    if scraping_task is not None:
        result = await scraping_task
        print(2)
        if result is None:
            bad_links_queue.put(link)
        else:
            print(3)
            parser_queue.put((result[0], result[1], depth))
            print(4)
        empty = False

    if parsing_task is not None:
        print(6)
        new_links = await parsing_task
        for item in new_links:
            links_queue.put((item, parser_depth+1))
        empty = False

    return empty

def one_cycle(links_queue, parser_queue, max_depth, bad_links_queue):
    return asyncio.run(concurrent_processing(
                links_queue,
                parser_queue,
                max_depth,
                bad_links_queue
        )
    )



async def parse_all(links_queue, parser_queue, max_depth, bad_links_queue):
    prev_error = False
    while True:
        print(1)
        empty = await concurrent_processing(links_queue, parser_queue, max_depth, bad_links_queue)
        if empty:
            if not prev_error:
                time.sleep(5)
                prev_error = True
            else:
                return
        else:
            prev_error = False


async def main():
    links_queue = mp.Queue()
    parser_queue = mp.Queue()
    bad_links_queue = mp.Queue()
    links_queue.put(("https://en.wikipedia.org/wiki/Main_Page", 1))
    await concurrent_processing(links_queue, parser_queue, 2, bad_links_queue)


asyncio.run(main())