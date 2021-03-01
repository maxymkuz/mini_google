**Parallel crawler for structured data in Rust**

Docker image is just 14mb thanks to the static compilation and scratch image

## Usage

Just run it and it will print out structured data found on the page:

```bash
# A quick test build of the rust executable
# The CLI executable displays its progress
cargo run -- -i file.txt -o out.txt -t 8

# Choose a maximum number of webpages to crawl (default is 1024)
cargo run -- -i file.txt -o out.txt -t 8 -l 1024

# A help page with CLI parameters' descriptions
cargo run -- --help

# Build and run a Docker image
# This build is relatively slow and produces a static executable
# The progress is not displayed correctly in the docker container
./build.sh
docker run --rm lastgenius/rust-crawler -i file.txt -o out.txt -t 8
```

## Architecture

This is a rough outline of how this crawler works. I will try to update it in the
case of any major changes, but it's always better to check out the module's documentation
itself in case you need to understand everything better and on a deeper level. 

You can build the documentation locally with `cargo doc --open`, and choosing the
crate from the menu on the left.

## Resources to quickly pick up what's going on here

Some useful resources on Rust in general, as well as on concurrency and web:
* A pretty useful tutorial: https://rolisz.ro/2020/03/01/web-crawler-in-rust/
* Rust Cookbook's examples:
    * [Concurrency](https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html)
    * [Database](https://rust-lang-nursery.github.io/rust-cookbook/database.html)
    * [Networking](https://rust-lang-nursery.github.io/rust-cookbook/net.html)
    * [Web](https://rust-lang-nursery.github.io/rust-cookbook/web.html)
* [Tokio tutorial](https://tokio.rs/tokio/tutorial/)
* [std;;thread documentation](https://doc.rust-lang.org/std/thread/index.html)
* [std::sync:mpsc documentation](https://doc.rust-lang.org/std/sync/mpsc/index.html)
* [Async book](https://rust-lang.github.io/async-book/)

I'm using these libraries (Rust calls them crates):
* [select.rs](https://github.com/utkarshkukreti/select.rs)
* [reqwest](https://github.com/seanmonstar/reqwest)
