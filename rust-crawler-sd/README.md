**Parallel crawler for structured data in Rust**

Docker image is just 14mb thanks to the static compilation and scratch image

Just run it and it will print out structured data found on the page:

```bash
# A quick test build of the rust executable
# The CLI executable displays its progress
cargo run -- -i file.txt -o out.txt -t 8

# A help page with CLI parameters' descriptions
cargo run -- --help

# Build and run a Docker image
# This build is relatively slow and produces a static executable
# The progress is not displayed correctly in the docker container
./build.sh
docker run --rm lastgenius/rust-crawler -i file.txt -o out.txt -t 8
```

Some useful resources on Rust in general, as well as on concurrency and web:
* A pretty useful tutorial: https://rolisz.ro/2020/03/01/web-crawler-in-rust/
* Rust Cookbook's examples:
    * [Concurrency](https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html)
    * [Database](https://rust-lang-nursery.github.io/rust-cookbook/database.html)
    * [Networking](https://rust-lang-nursery.github.io/rust-cookbook/net.html)
    * [Web](https://rust-lang-nursery.github.io/rust-cookbook/web.html)

I'm using these libraries (Rust calls them crates):
* [select.rs](https://github.com/utkarshkukreti/select.rs)
* [reqwest](https://github.com/seanmonstar/reqwest)
