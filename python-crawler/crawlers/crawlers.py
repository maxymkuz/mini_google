import abc
from bs4 import BeautifulSoup
import requests
import threading
import json
from datetime import date


class AbstractCrawler(metaclass=abc.ABCMeta):
    """
    Abstract class for creating different versions
    of crawlers
    """
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
        """
        Converts the link found on the page
        so that it could be accessed on next
        iterations
        :param link: str
        :param prev_link: str, previously
        visited link from which we got to the current one
        :return: str
        """
        if link[:2] == "//":
            return "http:" + link

        if link[0] == "/":
            return prev_link + link

        if link[0] == "#":
            return None

        return link


class BSCrawler(AbstractCrawler):
    def __init__(self, cursor):
        self.cursor = cursor

    def scrape(self, url):
        """
        Returns a parser for the given page
        or None if can't access it
        :param url: str
        :return: BeautifulSoup
        """
        try:
            return BeautifulSoup(requests.get(url).text, 'html.parser')
        except requests.exceptions.RequestException as e:
            print(f"Error while scraping: {url}")
            print(e)
            return None

    def get_text(self, parser):
        """
        Gets all text from the given BeautifulSoup object
        :param parser: BeautifulSoup
        :return: str
        """
        return parser.get_text()

    def get_links(self, parser, prev_link):
        """
        Gets list of all links from the given BeautifulSoup object
        :param parser: BeautifulSoup
        :param prev_link: str
        :return: list
        """
        links = []
        for element in parser.find_all("a"):
            link = element.get("href")
            if link is None:
                continue
            links.append(self.process_link(link, prev_link))

        return links

    def get_structured_data(self, parser):
        """
        Gets dictionary representing website structured data
        from the given BeautifulSoup object
        :param parser: BeautifulSoup
        :return: dict
        """
        elem = parser.find("script", {"type": "application/ld+json"})
        if elem is not None:
            return json.loads(elem.contents[0])

    def crawl(self, link, add_links=True):
        """
        Gets text, structured data, links of the given page
        :param link: str
        :param add_links: bool (Decides whether links need to be returned)
        """
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
    """
    Class that contains many crawlers that
    can be launched on multiple threads
    """

    # code that shows us that the crawler finished its work
    FINISH_CODE = -1

    def __init__(self, crawlers, max_depth):
        """
        Initializes CrawleManager with given list of crawles,
        maximal depth of going into links
        :param crawlers: list
        :param max_depth: int
        :return:
        """
        self.crawlers = crawlers
        self.max_depth = max_depth
        self.websites = []
        self.mutex = threading.Lock()

    def add_websites(self, links, depth=1):
        """
        Adds websites with given depth to the queue of
        websites
        :param links:
        :param int:
        :return:
        """
        self.websites.extend([(link, depth) for link in links])

    def crawl_next(self, i):
        """
        Crawls next website from the queue using
        crawler with index i
        :param i: int
        :return: None if everything is ok and FinishCode in
        other cases
        """
        self.mutex.acquire()
        if len(self.websites) > 0:
            link, depth = self.websites.pop(0)
        else:
            return CrawlersManager.FINISH_CODE
        self.mutex.release()

        # already processed or bad url
        if link is None:
            return

        rows = self.crawlers[i].cursor.get_row_by_url(link)
        date_added = None

        # if link is already in database
        if rows:
            date_added = rows[0][2]
            if date_added == date.today():
                return

        data = self.crawlers[i].crawl(link, depth < self.max_depth)
        if data is None:
            return CrawlersManager.FINISH_CODE

        try:
            new_modification_date = data[0]["structured_data"]["dateModified"]
            new_modification_date = new_modification_date[:10]
            new_modification_stamp = date.fromisoformat(new_modification_date)
        except (KeyError, TypeError):
            return

        # checks if the data is new
        if (date_added is None or new_modification_stamp > date_added):
            self.crawlers[i].cursor.insert_row(
                [
                    link, date.today().strftime("%Y-%d-%m"),
                    data[0]["text"],
                    new_modification_date
                ],
                tokenize=False
            )
            self.mutex.acquire()
            self.add_websites(data[1],
                              depth=depth + 1)
            self.mutex.release()
