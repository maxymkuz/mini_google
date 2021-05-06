CREATE DATABASE pagerank_db
    WITH
    OWNER = postgres
    ENCODING = 'UTF8'
    LC_COLLATE = 'C'
    LC_CTYPE = 'C'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;


-- table that will be accessed by search engine after calculating pagerank

CREATE TABLE IF NOT EXISTS pagerank
(
	website_id SERIAL,
	website_str TEXT NOT NULL UNIQUE,
	rank DOUBLE PRECISION NOT NULL DEFAULT 0.0,
	PRIMARY KEY (website_id)
);

-- index for queries of search engine
CREATE UNIQUE INDEX website_string_idx ON pagerank (website_str);

-- index for queries while calculating pagerank
CREATE UNIQUE INDEX website_id_idx ON pagerank (website_id);


-- teleportation matrix
CREATE TABLE IF NOT EXISTS connections
(
	out_website_id INT,
	in_website_id INT,
	CONSTRAINT fk_out_website
		FOREIGN KEY(out_website_id)
		REFERENCES pagerank(website_id)
		ON DELETE CASCADE,
	CONSTRAINT fk_in_website
		FOREIGN KEY(in_website_id)
		REFERENCES pagerank(website_id)
		ON DELETE CASCADE,
	PRIMARY KEY (out_website_id, in_website_id)
);


CREATE INDEX conn_out_idx ON connections (out_website_id);
CREATE INDEX conn_in_idx ON connections (in_website_id);


-- last values of calculated transition probabilities
CREATE MATERIALIZED VIEW IF NOT EXISTS counts AS
SELECT * FROM
(
    SELECT pagerank.website_id, COUNT(DISTINCT connections.in_website_id) AS outlinks FROM pagerank
    LEFT JOIN connections ON pagerank.website_id = connections.out_website_id
    GROUP BY pagerank.website_id
) AS inlinks
JOIN
(
    SELECT pagerank.website_id, COUNT(DISTINCT connections.out_website_id) AS inlinks FROM pagerank
    LEFT JOIN connections ON pagerank.website_id = connections.in_website_id
    GROUP BY pagerank.website_id
) AS outlinks
USING (website_id);

CREATE UNIQUE INDEX count_idx ON counts (website_id);

-- last values of calculated node weights
CREATE MATERIALIZED VIEW IF NOT EXISTS weight AS
SELECT
	connections.out_website_id,
	connections.in_website_id,
	CAST (inlinks AS DOUBLE PRECISION)/in_sum AS in_weight,
	CAST (outlinks AS DOUBLE PRECISION)/out_sum AS out_weight
FROM connections
JOIN
(
	SELECT connections.out_website_id, SUM(inlinks) AS in_sum, SUM(outlinks) AS out_sum FROM connections
	JOIN counts ON counts.website_id = connections.in_website_id
	GROUP BY connections.out_website_id
) AS link_sums
USING (out_website_id)
JOIN counts ON counts.website_id = connections.in_website_id;

CREATE INDEX weight_in_idx ON weight (in_website_id);


-- copy of pagerank values (because when we calculate pagerank,
-- we need to access previously calculated values and also save new ones)
CREATE MATERIALIZED VIEW IF NOT EXISTS previous_ranks AS
SELECT pagerank.website_id, pagerank.rank FROM pagerank;

CREATE INDEX rank_idx ON previous_ranks (website_id);