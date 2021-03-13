CREATE TABLE IF NOT EXISTS websites_en (
    site_id INT NOT NULL,
    url TEXT NOT NULL,
    date_added DATE NOT NULL,
    last_modified DATE,
    site_text TEXT NOT NULL,
    --     pointing_urls text[], -- List of all urls, that a given website contains
    --     pointing_urls text[], -- Can alternatively be done as references to the second db site_id below

    tokenized TSVECTOR
);

-- table that can be used to check whether we've been to that website
--  WARNING: this is not very effective when inserting a lot of values in our table, but ok for the first times

CREATE TABLE IF NOT EXISTS url_list_idx (
    site_id INT PRIMARY Key,
    url TEXT NOT NULL,
    date_added DATE NOT NULL,
    last_modified DATE,
);

-- creating non-clustered index on url:
CREATE INDEX url ON url_list_idx(url ASC);