This is a WIP database directory. We are figuring out how to transfer from a PostgreSQL database
to an Elasticsearch one.

This Rust module should currently be able of populating the database with crawler's
results and giving back backend's search queries.

**Left to figure out:**
* Connect this to the lang detect backend
* Connect this to the crawler
* Connect this to the search backend
* Figure out all the Docker stuff

## Instructions for local Docker-less deployment

```
# Install elasticsearch using your system's package manager
pacman -S elastic search

# Create keystore
elasticsearch-keystore create

# Start the service each time you want to work with elasticsearch or enable
# it if you want it to start at boot
systemctl start elasticsearch.service
systemctl enable elasticsearch.service

# Check if it works
curl http://127.0.0.1:9200
```

You can also read [on this relationship between SQL concepts and
Elasticsearch respective ones](https://www.elastic.co/blog/what-is-an-elasticsearch-index).
