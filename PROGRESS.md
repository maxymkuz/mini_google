#### Progress tracker for the project. Helps us understand where we are now and what we are still missing.

*To do sometime in the future:*
* Start implementing the actual PageRank algorithm (mathematically and programmatically)
* Start deploying this thing and exposing it to the outside world
* Think of security mechanisms that are going to have to be in place for us to deploy this safely
* Clean everything up, add some documentation

## Week 5

- [ ] Research Elasticsearch (huh) and figure out the schema, the queries we will
have to send both from the crawling and from the backend sides.

- [ ] Look into whether we can transfer HTML parsing to Elasticsearch from crawlers.

- [ ] Figure out a smart system of giving out the URLs to the crawling workers,
custom round-robin algorithms and error re-attempts.

- [ ] See what we can do right now to improve our later cloud deployment (devopsy things)

- [ ] Implement proper pagination for the backend.


## Week 4

- [x] Look into better full text parsing for Rust crawlers.

- [x] Implement the main functionality of the website backend in Rust (the main page, error handling, user query handling)

- [x] Transfer the python crawler to a multiprocessor architecture.

- [x] Start laying out the final architecture of the project - the way all these systems are going to interact with each other.

## Week 3

- [x] Implement repeated attempts policy for sites that error out when we try to scrape them. Number of 
attempts, time between them should be configurable.

- [x] Figure out whether we should put our database queries in transactions to maintain data integrity

- [x] Think of database schemas and implementation details for a functional PageRank algorithm

- [x] Develop a protocol for communication between the backend and the database, which endpoints 
it should query, whether it's going to be a GET/POST request, etc.

- [x] Pick a library for language detection (both for a user's request and webpage's content). We've decided to
make a separate module that is going to be used both for the backend and for the crawlers (since both work in
Rust and we've found a Rust library for language detection it should be fine!)

## Week 2

- [x] Implement the database with the Python and Rust backends for it.

- [x] Update crawlers to connect them to the database.

- [x] Connect all of the containers into a single docker-compose project.

- [x] Figure out how to work with queries in different languages, + Ukrainian language tokenization.


## Week 1

- [x] Simple (but parallel) crawlers for structured data. Taking an input file with links
and outputting collected structured data in another file. Taking a simple configuration
with where to look for the files, how many threads to use, how many websites to crawl etc.

- [x] Docker images for crawlers, with minimal dependencies and sizes. Also a simple docker-compose file. Commits above.

- [x] Figure out how to work with the database, does it support our languages (currently English and Ukrainian),
which tables and columns we need etc.

- [x] A simple frontend prototype. Documented query format from the frontend to the database 


