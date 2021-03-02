import abc
from bs4 import BeautifulSoup
import requests
import threading


class AbstractCrawler(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def scrape(self, url):
        pass

    @abc.abstractmethod
    def get_text(self, parser):
        pass

    @abc.abstractmethod
    def get_links(self, parser, prev_link):
        pass

    @abc.abstractmethod
    def get_structured_data(self, parser):
        pass

    def process_link(self, link, prev_link):
        if link[:2] == "//":
            return "http:" + link

        if link[0] == "/":
            return prev_link + link

        if link[0] == "#":
            return prev_link

        return link


class BSCrawler(AbstractCrawler):
    def __init__(self):
        self.mutex = threading.Lock()

    def scrape(self, url):
        try:
            return BeautifulSoup(requests.get(url).text, 'html.parser')
        except requests.exceptions.RequestException as e:
            print(f"Error while scraping: {url}")
            print(e)
            return None

    def get_text(self, parser):
        return parser.get_text()

    def get_links(self, parser, prev_link):
        links = []
        for element in parser.find_all("a"):
            link = element.get("href")
            if link is None or len(link) < 2:
                continue
            links.append(self.process_link(link, prev_link))

        return links

    def get_structured_data(self, parser):
        elem = parser.find("script", {"type": "application/ld+json"})
        if elem is not None:
            return elem.contents

    def crawl(self, link, add_links=True):

        parser = self.scrape(link)
        if parser is None:
            return

        if add_links:
            new_links = self.get_links(parser, link)
        else:
            new_links = []

        return ({"text": self.get_text(parser),
                "structured_data": self.get_structured_data(parser)},
                new_links)


class CrawlersManager:
    FINISH_CODE = -1

    def __init__(self, crawlers, max_depth):
        self.crawlers = crawlers
        self.data = {}
        self.max_depth = max_depth
        self.websites = []

    def add_websites(self, links, depth=1):
        self.websites.extend([(link, depth) for link in links])

    def crawl_next(self, i):
        link = None
        self.crawlers[i].mutex.acquire()
        if len(self.websites) > 0:
            link, depth = self.websites.pop(0)

        self.crawlers[i].mutex.release()
        if link is None:
            return CrawlersManager.FINISH_CODE

        if link in self.data:
            return

        try:
            self.data[link], new_links = self.crawlers[i].crawl(link, depth < self.max_depth)
        except TypeError:
            return CrawlersManager.FINISH_CODE

        self.add_websites(new_links,
                          depth=depth+1)
