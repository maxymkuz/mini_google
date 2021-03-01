**A little parallel crawler for structured data in Rust**

Docker image is just 12.3mb thanks to the static compilation and scratch image

Just run it and it will print out structured data found on the page:

```bash
./build.sh

# A quick test build of the rust executable
cargo run -- -i file.txt -o out.txt -t 8
cargo run -- --help

# This build is relatively slow and produces a static executable
docker run --rm lastgenius/rust-crawler
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
