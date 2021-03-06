import time

from sqlalchemy import create_engine

# TODO я потім красиво перепишу і фіч допишу, але поки хай робе як робе
db_name = 'database'
db_user = 'admin'
db_pass = 'postgres'
db_host = 'db'
db_port = '5432'

# Connection to the database
db_string = 'postgres://{}:{}@{}:{}/{}'.format(db_user, db_pass, db_host,
                                               db_port, db_name)
db = create_engine(db_string)


def insert_row(row, tokenize=True):
    """
    Inserts a row in the end of db. Be careful to meet db requirements
    :param tokenize: bool, whether we want to immediately tokenize text
    :param row: a tuple of the following structure:
    (url:string, current_date:string(YYYY-MM-DD), web_text:string,
    last_modified:string(YYYY-MM-DD))
    Note that last modified parameter is optional
    :return:
    """
    if len(row) not in {3, 4}:
        raise ValueError("wrong arguments passed to insert a row")
    # TODO Потім це красивіше перепишу
    if tokenize:
        if len(row) == 3:
            db.execute(f"INSERT INTO websites_en (url, date_added, site_text, tokenized) "
                       f"VALUES ('{row[0]}', '{row[1]}', '{row[2]}', to_tsvector('{row[2]}'));")
        elif len(row) == 4:  # with last modified column
            db.execute(f"INSERT INTO websites_en (url, date_added, site_text, "
                       f"last_modified, tokenized) VALUES ('{row[0]}', '{row[1]}', '{row[2]}'"
                       f",'{row[3]}', to_tsvector('{row[2]}'));")
    else:
        if len(row) == 3:
            db.execute(f"INSERT INTO websites_en (url, date_added, site_text) "
                       f"VALUES ('{row[0]}', '{row[1]}', '{row[2]}');")
        elif len(row) == 4:  # with last modified column
            db.execute(f"INSERT INTO websites_en (url, date_added, site_text, "
                       f"last_modified) VALUES ('{row[0]}', '{row[1]}', '{row[2]}'"
                       f",'{row[3]}');")


def delete_row(url):
    """
    Deletes a row by url. Does nothing if url not in db
    :param url: string
    :return:
    """
    db.execute(f"DELETE FROM websites_en"
               f"WHERE url={url};")


def get_all_rows():
    # Retrieve all rows from db
    query = "SELECT * FROM websites_en"
    result_set = db.execute(query)
    return [r for r in result_set]


def get_row_by_url(url1):
    query = f"SELECT * FROM websites_en " \
            f"WHERE url='{url1}';"
    result_set = db.execute(query)
    return [r for r in result_set]


def delete_all_entries():
    # simply deletes all entries in db, lol
    db.execute("DELETE FROM websites_en;")
    print("Successfully deleted all entries from websites_en")


def get_ranked_rows_by_query(input_query, num_responses):
    """
    Runs through all tokenized rows, and returns the list of {num_responses} elements
    each of which is in the form of (website url, site_text), that are ranked in the descending
    order by how much it matches the query
    :param input_query: string            Example: 'Data | scrapping | (computer & program)'
    :param num_responses: int
    :return: list
    """
    db_query = f"""
    SELECT url, ts_rank_cd(tokenized, to_tsquery('{input_query}')), site_text AS rank
    FROM websites_en, to_tsquery('{input_query}') query
    WHERE query @@ tokenized
    ORDER BY rank DESC
    LIMIT {num_responses};
    """
    result_set = db.execute(db_query)
    return [r for r in result_set]


if __name__ == '__main__':
    # От приклади
    print('Application started')
    delete_all_entries()
    # while True:
    insert_row(('url1', '1977-07-23', 'some text'))
    insert_row(('url2', '1977-07-23', 'some text 2 text', '2007-07-23'), tokenize=True)
    insert_row(('url2', '1977-07-23', 'some text 2 text', '2007-07-23'), tokenize=False)
    insert_row(('url2', '1977-07-23', 'some text 2 text', '2007-07-23'))
    insert_row(('url1', '1977-07-23', 'some text'), tokenize=False)

    print("BY URL", get_row_by_url('url1'))
    print("ALLL   ", get_all_rows())
    time.sleep(3)
    print('_'*20)
    print(get_ranked_rows_by_query("some & 2", 2))
