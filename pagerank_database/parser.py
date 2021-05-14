import psycopg2
from psycopg2.extras import execute_values


class PageRankDb:
    def __init__(self, connection):
        self.connection = connection
        self.cursor = connection.cursor()

    def insert_urls(self, urls):
        execute_values(self.cursor,
                       """
                       INSERT INTO pagerank (website_str)
                       VALUES %s
                       ON CONFLICT (website_str) DO NOTHING
                       """,
                       [(url,) for url in urls]
                       )
        self.connection.commit()

    def insert_connections(self, pairs):
        execute_values(self.cursor,
                       """
                       INSERT INTO connections (in_website_id, out_website_id)
                       VALUES %s
                       ON CONFLICT DO NOTHING;
                       """,
                       pairs
                       )
        self.connection.commit()

    def build_id_dict(self, urls):
        self.cursor.execute(
            """
            SELECT website_id, website_str FROM pagerank;
            """
        )
        ids = {}
        while True:
            res = self.cursor.fetchone()
            if res is None:
                break
            # for every url from the set of new ones
            # we save its id
            if res[1] in urls:
                ids[res[1]] = res[0]
            print(res)
        self.connection.commit()
        return ids

    def add_file(self, filename):
        # set of links that will be saved to our database
        urls = set()

        # dictionary for containing ingoing links of every crawled page
        out_links = {}
        with open(filename) as f:
            for num, line in enumerate(f, start=0):
                if num % 3 != 0:
                    continue
                new_urls = line.strip().split()
                outgoing = set(new_urls[1:])

                urls.add(new_urls[0])
                out_links[new_urls[0]] = outgoing

        # we delete links from websites that were not crawled
        for url in out_links:
            out_links[url] &= urls

        # insert all urls to database
        self.insert_urls(urls)

        # get ids of every website which are stored in our database
        id_mapping = self.build_id_dict(urls)

        # pairs containing out_website_id, in_website_id for every link
        pairs = [(id_mapping[in_link], id_mapping[out_link])
                 for out_link, in_links in out_links.items()
                 for in_link in in_links]

        self.insert_connections(pairs)


conn = psycopg2.connect(user="postgres",
                        password="postgres",
                        host="127.0.0.1",
                        port="5432",
                        database="pagerank_db")

db = PageRankDb(conn)

db.add_file("collected_text_and_new_links.txt")

conn.close()
