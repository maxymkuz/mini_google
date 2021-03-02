Progress tracker for the project. Helps us understand where we are now and
what we are still missing.

## Week 1

- [x] Simple (but parallel) crawlers for structured data. Taking an input file with links
and outputting collected structured data in another file. Taking a simple configuration
with where to look for the files, how many threads to use, how many websites to crawl etc.
Rust - ffc7d636f6a1e335b1b1fc61ddb666088197f11d . Python - 071e99e6cd0da1a94f30ae5c23557cc3c1a4fd68

- [x] Docker images for crawlers, with minimal dependencies and sizes. Also a simple docker-compose file. Commits above.

- [x] Figure out how to work with the database, does it support our languages (currently English and Ukrainian),
which tables and columns we need etc.

- [x] A simple frontend prototype. Documented query format from the frontend to the database - 921ccf5d55a7ce76c30bc2a0e4307ceec5b9a716
