
CREATE TABLE IF NOT EXISTS pagerank
(
	website_id SERIAL,
	website_str TEXT NOT NULL UNIQUE,
	rank DOUBLE PRECISION NOT NULL DEFAULT 0.0,
	PRIMARY KEY (website_id)
);


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


CREATE MATERIALIZED VIEW IF NOT EXISTS outlinks AS
SELECT pagerank.website_id, COUNT(DISTINCT connections.in_website_id) AS outlinks FROM pagerank
LEFT JOIN connections ON pagerank.website_id = connections.out_website_id
GROUP BY pagerank.website_id;


CREATE MATERIALIZED VIEW IF NOT EXISTS inlinks AS
SELECT pagerank.website_id, COUNT(DISTINCT connections.out_website_id) AS inlinks FROM pagerank
LEFT JOIN connections ON pagerank.website_id = connections.in_website_id
GROUP BY pagerank.website_id;

CREATE MATERIALIZED VIEW IF NOT EXISTS previous_ranks AS
SELECT pagerank.website_id, pagerank.rank FROM pagerank;

CREATE UNIQUE INDEX website_string_idx ON pagerank (website_str);
CREATE UNIQUE INDEX website_id_idx ON pagerank (website_id);

CREATE INDEX conn_out_idx ON connections (out_website_id);
CREATE INDEX conn_in_idx ON connections (in_website_id);

CREATE INDEX outlink_idx ON outlinks (website_id);

CREATE INDEX inlink_idx ON inlinks (website_id);

CREATE INDEX rank_idx ON previous_ranks (website_id);
