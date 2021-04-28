
CREATE TABLE name_idx
(
	website_id SERIAL UNIQUE,
	website_str TEXT NOT NULL UNIQUE
);

CREATE TABLE pagerank
(
	website_id INTEGER NOT NULL UNIQUE,
	rank DOUBLE PRECISION NOT NULL,
	incoming_links INTEGER[]
);

CREATE UNIQUE INDEX website_string_idx ON name_idx (website_str);
CREATE UNIQUE INDEX website_id_idx ON name_idx (website_id);
