**A Backend for the database that serves as an intermediate stage between the
crawler and the database and between the backend and the database, answering their
respective `insert` and `get` queries.**

Done in Rust using Elasticsearch as a database. Connect to the language detector
in Python on a different container, but this might be a subject to change.

**Left to figure out:**
* Connect this to the lang detect backend
* Connect this to the crawler
* Connect this to the search backend
* Figure out all the Docker stuff
