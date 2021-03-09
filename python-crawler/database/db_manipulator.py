import time

from sqlalchemy import create_engine
import sqlalchemy


class DataBaseTable:
    INIT_DELAY = 10

    def __init__(self, name, user, password, host, port, table):
        """
        Creates a connection to the database with given
        credentials and saves the table which will be
        using in the future
        All parameters can be strings
        """
        # Connection to the database,
        # sleep and wait if database is not ready yet
        while True:
            try:
                login_string = 'postgres://{}:{}@{}:{}/{}'.format(
                    user, password, host,
                    port, name
                    )
                self.db = create_engine(login_string)
                break
            except Exception as e:
                print("Failed to connect to database, "
                      "reconnecting in 1 second")
                print(e)
                time.sleep(1)
        self.table = table
        time.sleep(DataBaseTable.INIT_DELAY)

    def insert_row(self, row, tokenize=True):
        """
        Inserts a row in the end of db. Be careful to meet db requirements
        :param tokenize: bool, whether we want to immediately tokenize text
        :param row: a tuple of the following structure:
        (url:string, current_date:string(YYYY-MM-DD), web_text:string,
        last_modified:string(YYYY-MM-DD))
        Note that last modified parameter is optional
        :return:
        """
        if tokenize:
            if len(row) == 3:
                self.db.execute(
                    sqlalchemy.text(
                        f"INSERT INTO {self.table} (url,"
                        f" date_added, site_text, tokenized) "
                        f"VALUES ('{row[0]}', '{row[1]}', "
                        f"'{row[2]}', to_tsvector('{row[2]}'));"
                    )
                )
            elif len(row) == 4:  # with last modified column
                self.db.execute(
                    sqlalchemy.text(
                        f"INSERT INTO {self.table} (url,"
                        f" date_added, site_text, "
                        f"last_modified, tokenized) VALUES "
                        f"('{row[0]}', '{row[1]}', '{row[2]}'"
                        f",'{row[3]}', to_tsvector('{row[2]}'));"
                        )
                )
        else:
            if len(row) == 3:
                self.db.execute(
                    sqlalchemy.text(
                        f"INSERT INTO {self.table}"
                        f" (url, date_added, site_text) "
                        f"VALUES ('{row[0]}', '{row[1]}', '{row[2]}');"
                        )
                )
            elif len(row) == 4:  # with last modified column
                self.db.execute(
                    sqlalchemy.text(
                        f"INSERT INTO {self.table} "
                        f"(url, date_added, site_text, "
                        f"last_modified) VALUES"
                        f" ('{row[0]}', '{row[1]}', '{row[2]}'"
                        f",'{row[3]}');"
                        )
                    )

    def delete_row(self, url):
        """
        Deletes a row by url. Does nothing if url not in db
        :param url: string
        :return:
        """
        self.db.execute(
            sqlalchemy.text(
                f"DELETE FROM {self.table}"
                f"WHERE url={url};"
                )
            )

    def get_all_rows(self):
        # Retrieve all rows from db
        query = sqlalchemy.text(f"SELECT * FROM {self.table}")
        return [r for r in self.db.execute(query)]

    def get_row_by_url(self, url):
        query = f"SELECT * FROM {self.table} " \
                f"WHERE url='{url}';"
        result_set = self.db.execute(sqlalchemy.text(query))
        return [r for r in result_set]

    def delete_all_entries(self):
        # simply deletes all entries in db, lol
        self.db.execute(sqlalchemy.text(f"DELETE FROM {self.table};"))
        print(f"Successfully deleted all entries from {self.table}")

    def get_ranked_rows_by_query(self, input_query, num_responses):
        """
        Runs through all tokenized rows, and
        returns the list of {num_responses} elements
        each of which is in the form of (website url, site_text),
        that are ranked in the descending
        order by how much it matches the query

        :param input_query: string
        (Example: 'Data | scrapping | (computer & program)')

        :param num_responses: int
        :return: list
        """
        db_query = f"""
        SELECT url, ts_rank_cd(tokenized, to_tsquery('{input_query}')), site_text AS rank
        FROM {self.table}, to_tsquery('{input_query}') query
        WHERE query @@ tokenized
        ORDER BY rank DESC
        LIMIT {num_responses};
        """
        result_set = self.db.execute(sqlalchemy.text(db_query))
        return [r for r in result_set]


# TODO я потім красиво перепишу і фіч допишу, але поки хай робе як робе

if __name__ == '__main__':
    # От приклади
    print(1)
    table = DataBaseTable(
        database", "admin",
        "postgres", "db", 5432, "websites_en"
     )
    print('Application started')

    table.delete_all_entries()
    # while True:
    table.insert_row(('url1', '1977-07-23', 'some text'))
    table.insert_row(
        ('url2', '1977-07-23', 'some text 2 text', '2007-07-23'),
        tokenize=True)
    table.insert_row(
        ('url2', '1977-07-23', 'some text 2 text', '2007-07-23'),
        tokenize=False)
    table.insert_row(('url2', '1977-07-23', 'some text 2 text', '2007-07-23'))
    table.insert_row(('url1', '1977-07-23', 'some text'), tokenize=False)

    print("BY URL", table.get_row_by_url('url1'))
    print("ALLL   ", table.get_all_rows())
    time.sleep(3)
    print('_'*20)
    print(table.get_ranked_rows_by_query("some & 2", 2))
