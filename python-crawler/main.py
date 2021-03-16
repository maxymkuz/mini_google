from crawlers.crawlers import BSCrawler, CrawlersManager
from database.db_manipulator import DataBaseTable, DataBaseCursor
import time
import threading
import sys


def crawl_thread(i, manager):
    """
    Completely crawles all websites in the manager queue
    until there are no left using crawler at index i
    :param i: int
    :param manager: CrawlerManager
    """
    prev_error = False
    while True:
        res = manager.crawl_next(i)
        if res == CrawlersManager.FINISH_CODE:
            if not prev_error:
                time.sleep(2)
                prev_error = True
            else:
                return
        else:
            prev_error = False
        print(i)

def crawl_good_links(manager, threads_num):
    threads = []
    for i in range(threads_num):
        threads.append(threading.Thread(target=crawl_thread,
                                        args=(i, manager)))
        threads[i].start()

    for i in range(threads_num):
        threads[i].join()


def crawl_all(depth, threads_num, in_file, max_iteration=4):
    """
    Completely crawles all websites from the file
    with given number of threads and saves data
    into database
    :param depth: int
    :param threads_num: int
    :param in_file: str
    :return:
    """
    table = DataBaseTable(
            "database", "admin",
            "postgres", "db", 5432, "websites_en"
            )
    crawlers = [BSCrawler(DataBaseCursor(table)) for i in range(threads_num)]
    manager = CrawlersManager(crawlers, depth)
    print(1)
    with open(in_file) as f:
        manager.add_websites([line.strip() for line in f.readlines()])
    i = 0
    while True:
        crawl_good_links(manager, threads_num)
        i += 1
        if i > max_iteration:
            break
        if manager.bad_urls:
            time.sleep(manager.delay)
            manager.delay *= manager.coef
            manager.add_websites(manager.bad_urls)
            manager.bad_urls = []
        else:
            break



def main():
    args = sys.argv
    args[1] = int(args[1])
    args[2] = int(args[2])
    crawl_all(*args[1:])
    print("Finished")


if __name__ == "__main__":
    main()
