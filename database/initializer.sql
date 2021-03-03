CREATE TABLE IF NOT EXISTS websites_en (
    site_id SERIAL NOT NULL,
    url TEXT NOT NULL,
    date_added DATE NOT NULL,
    last_modified DATE,
    site_text TEXT NOT NULL,
    tokenized TSVECTOR
);