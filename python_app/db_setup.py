import psycopg2
from psycopg2 import Error

# Simple python script to initialize the table if not exist, need to rewrite later to add new functionality.


def setup_table(table_name, cursor):
    """
    create a default table named table_name, if not exists
    :param table_name: string
    :return: None
    """
    cursor.execute(f"""CREATE TABLE IF NOT EXISTS {table_name} (
    site_id SERIAL NOT NULL,
    url TEXT NOT NULL,
    date_added DATE NOT NULL,
    last_modified DATE,
    site_text TEXT NOT NULL,
    --     pointing_urls text[], -- List of all urls, that a given website contains, there is no
    -- soft to add these currently, TODO later
    tokenized TSVECTOR
    );
    """)
    print("Now", table_name, "exists!")


try:
    # Connect to an existing database
    connection = psycopg2.connect(user="postgres",
                                  password="postgres",
                                  host="localhost",
                                  port="5432",
                                  database="main_fts")

    # Create a cursor to perform database operations
    cursor = connection.cursor()
    # Print PostgreSQL details
    print("PostgreSQL server information")
    print(connection.get_dsn_parameters(), "\n")
    # Executing a SQL query, we can delete it later
    cursor.execute("SELECT version();")
    # Fetch result
    record = cursor.fetchone()
    print("You are connected to - ", record, "\n")

    setup_table("websites_en", cursor)

    # Твій код сюда якщо ще щось треба

    # If we have finished talking to database, close the connection
    if (connection):
        cursor.close()
        connection.close()
        print("PostgreSQL connection is closed")

except (Exception, Error) as error:
    print("Error while connecting to PostgreSQL", error)


