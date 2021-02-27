import abc
from bs4 import BeautifulSoup
import requests


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
        return parser.find("script", {"type": "application/ld+json"}).contents


class CrawlersManager:
    def __init__(self, crawler, max_depth):
        self.crawler = crawler
        self.data = {}
        self.max_depth = max_depth
        self.websites = []

    def add_websites(self, links, depth=1):
        self.websites.extend([(link, depth) for link in links])

    def crawl_next(self):
        link, depth = self.websites.pop(0)
        if link in self.data:
            return

        parser = self.crawler.scrape(link)
        if parser is None:
            return

        if depth < self.max_depth:
            self.add_websites(self.crawler.get_links(parser, link),
                              depth=depth+1)

        self.data[link] = self.crawler.get_text(parser)

    def crawl_all(self):
        while self.websites:
            self.crawl_next()
            print(self.websites)
            print(len(self.websites))
