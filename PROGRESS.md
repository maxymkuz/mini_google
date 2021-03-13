Progress tracker for the project. Helps us understand where we are now and
what we are still missing.

## Week 3

- [x] Implement repeated attempts policy for sites that error out when we try to scrape them. Number of 
attempts, time between them should be configurable.

- [x] Figure out whether we should put our database queries in transactions to maintain data integrity

- [x] Think of database schemas and implementation details for a functional PageRank algorithm

- [ ] Develop a protocol for communication between the backend and the database, which endpoints 
it should query, whether it's going to be a GET/POST request, etc.

- [x] Pick a library for language detection (both for a user's request and webpage's content). We've decided to
make a separate module that is going to be used both for the backend and for the crawlers (since both work in
Rust and we've found a Rust library for language detection it should be fine!)

## Week 2

- [x] Data base with python backend for it.

- [x] Updated crawlers to connect them to data base.

- [x] Full docker compose file with data base, crawlers and frontend.

- [x] Figure out how to work with queries in different languages, + Ukrainian language tokenization.


## Week 1

- [x] Simple (but parallel) crawlers for structured data. Taking an input file with links
and outputting collected structured data in another file. Taking a simple configuration
with where to look for the files, how many threads to use, how many websites to crawl etc.

- [x] Docker images for crawlers, with minimal dependencies and sizes. Also a simple docker-compose file. Commits above.

- [x] Figure out how to work with the database, does it support our languages (currently English and Ukrainian),
which tables and columns we need etc.

- [x] A simple frontend prototype. Documented query format from the frontend to the database 


