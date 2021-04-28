import psycopg2
from psycopg2.extras import execute_values


class PageRankDb:
    def __init__(self, connection):
        self.connection = connection
        self.cursor = connection.cursor()

    def add_url(self, url):
        self.cursor.execute(
            """
            INSERT INTO name_idx (website_str)
            VALUES (%s)
            ON CONFLICT DO NOTHING;
            """,
            (url,)
        )

        self.cursor.execute(
            """
            SELECT website_id FROM name_idx 
            WHERE website_str = %s;
            """,
            (url,)
        )
        self.connection.commit()
        return self.cursor.fetchone()

    def add_init_urls(self, sorted_urls):
        """
        :param url_id: list
        :return:
        """
        execute_values(self.cursor,
                       """
                        INSERT INTO name_idx (website_str)
                        VALUES %s
                        ON CONFLICT DO NOTHING;
                        """,
                       [(url,) for url in sorted_urls])
        self.connection.commit()

    def add_init_inlinks(self, id_incoming):
        """

        :param id_incoming: dict
        :return:
        """
        execute_values(self.cursor,
                       """
                        INSERT INTO pagerank (website_id, rank, incoming_links)
                        VALUES %s
                        ON CONFLICT DO NOTHING;
                        """,
                       [(id, 0, list(inlinks)) for id, inlinks in id_incoming.items()])
        self.connection.commit()

    def add_init_file(self, filename):
        url_id = {}
        id_incoming = {}
        with open(filename) as f:
            for num, line in enumerate(f, start=0):
                if num % 3 != 0:
                    continue
                new_urls = line.strip().split()
                for url in new_urls:
                    url_id[url] = url_id.get(url, len(url_id))

                for url in new_urls[1:]:
                    id_incoming[url_id[url]] = id_incoming.get(url_id[url], set())
                    id_incoming[url_id[url]].add(url_id[new_urls[0]])

        self.add_init_urls(sorted(list(url_id), key=lambda url: url_id[url]))
        self.add_init_inlinks(id_incoming)








#print(parse_file("example.txt"))

conn = psycopg2.connect(dbname="test_db", user="postgres", port=5433, password="postgres")

x = PageRankDb(conn)


x.add_init_file("example.txt")


conn.close()


