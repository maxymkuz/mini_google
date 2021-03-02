from crawlers.crawlers import BSCrawler, CrawlersManager
import json
import time
import threading
import sys


def crawl_thread(i, manager):
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
            print("Crawled", i)
            prev_error = False


def crawl_all(depth, threads_num, in_file, output):
    crawlers = [BSCrawler() for i in range(threads_num)]
    manager = CrawlersManager(crawlers, depth)
    with open(in_file) as f:
        manager.add_websites([line.strip() for line in f.readlines()])

    threads = []
    for i in range(threads_num):
        threads.append(threading.Thread(target=crawl_thread,
                                        args=(i, manager)))
        threads[i].start()

    for i in range(threads_num):
        threads[i].join()

    with open(output, 'w') as outfile:
        json.dump(manager.data, outfile, indent=4)


def main():
    args = sys.argv
    args[1] = int(args[1])
    args[2] = int(args[2])
    crawl_all(*args[1:])
    print("Finished")


if __name__ == "__main__":
    main()
