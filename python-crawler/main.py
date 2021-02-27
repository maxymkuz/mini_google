from crawlers import BSCrawler, CrawlersManager
import json


def main():
    crawler = BSCrawler()
    manager = CrawlersManager(crawler, 2)

    manager.add_websites(["https://www.wikipedia.org/"])

    manager.crawl_all()
    with open('data.txt', 'w') as outfile:
        json.dump(manager.data, outfile, indent=4)


if __name__ == "__main__":
    main()
