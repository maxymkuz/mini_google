**A Backend for the database that serves as an intermediate stage between the
crawler and the database and between the backend and the database, answering their
respective `insert` and `get` queries.**

Done in Rust using Elasticsearch as a database. Also connects to the language detector
in Python in a different container, but this might be a subject to change.

### Instructions

```
# Web backend
# You should probably query something like 127.0.0.1:8080 with the json that contains the query string.
# The up-to-date info is in src/web_listener.rs towards the bottom
# I am going to try to work out how this works with Docker tomorrow and will update this

# Crawlers
# (WIP)
```

### Left to figure out:
* Connect this to the lang detect backend
* Connect this to the crawler
* Connect this to the search backend
* Figure out all the Docker stuff
