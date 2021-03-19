**Parallel crawler for structured data in Rust**

Docker image is just 14mb thanks to the static compilation, multi-stage build and scratch image.

## Usage

```bash
# A quick test build of the rust executable
# The CLI executable displays its progress
# 'l' chooses a maximum number of webpages to crawl (default is 1024)
# 'r' chooses a maximum number of repeats for failed pages (default is 3)
cargo run --release -- -i file.txt -o out.txt -t 8 -l 300 -r 5

# A help page with CLI parameters' descriptions
cargo run -- --help

# Build and run a Docker image
# This build is relatively slow and produces a static executable
# The progress is not displayed correctly in the docker container
./build.sh
docker run --rm lastgenius/rust-crawler -i file.txt -o out.txt -t 8
```

## Examples

A few files that this monster has spent its time on:

* [A little snippet of shuffled links for you to have something to test the crawler with](./urls.txt)
* [2.5 Gb Full Text Data with URLs](https://drive.google.com/file/d/1OZK2P9GTj7EXZIXBv5jdhKP9Ed_kOlQ6/view?usp=sharing)
* [400 Mb of Full Text Data + New Links from URLs](https://drive.google.com/file/d/1KCW3m_wpx0qCVxZlu0XP30YiTzPZtett/view?usp=sharing)
* [900 Mb of unique shuffled links](https://drive.google.com/file/d/1qzVEbmsrvsqiHoHHExq4Hy0PAEguK9jq/view?usp=sharing)

You can play around with these files, better to use different CLI apps to keep yourself sane:

* Sort lines, keep unique ones: `sort file.txt -o out.txt -u`
* Shuffle lines `shuf file.txt -o out.txt` 

*And so on...*

## Architecture

This is a rough outline of how this crawler works. I will try to update it in the
case of any major changes, but it's always better to check out the module's documentation
itself in case you need to understand everything better and on a deeper level. 

You can build the documentation locally with `cargo doc --open`, and view it by choosing the
crate from the menu on the left.

Overall the program works like this, with a single main thread starting off, reading user input
from the command line, reading an input file with webpage links, and then creating a thread
pool of workers, establishing several MPSC (Multiple Producers Single Consumer) channels between
the main thread and threads in the pool:

* **URL channel** - through which the main thread sends URLs to crawl to the thread pool. Has to
be protected by a mutex (essentially is a queue with a lock).
* **New URL channel** - works as intended, with threads in the pool sending new URLs which they
acquire during the crawl.
* **Structured data channel** - works as intended, with threads in the pool sending structured
data acquired on a page, if any.

![](../images/rust-arch1.png)

Each worker is just an asynchronous single-thread `tokio` runtime that tries to get a vector
of new URLs to crawl through the URL channel, and then asynchonously shoots off requests and gets
data, which is then parsed and sent back to the main thread.

If URLs error out, the main thread is going to repeat this URL a few times,
waiting for exponentially more time between attempts, before discarding the URL altogether.

## Resources to quickly pick up what's going on here

Some useful resources on Rust in general, as well as on concurrency and web:
* [A pretty useful tutorial](https://rolisz.ro/2020/03/01/web-crawler-in-rust/)
* Rust Cookbook's examples:
    * [Concurrency](https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html)
    * [Database](https://rust-lang-nursery.github.io/rust-cookbook/database.html)
    * [Networking](https://rust-lang-nursery.github.io/rust-cookbook/net.html)
    * [Web](https://rust-lang-nursery.github.io/rust-cookbook/web.html)
* [Tokio tutorial](https://tokio.rs/tokio/tutorial/)
* [std::thread documentation](https://doc.rust-lang.org/std/thread/index.html)
* [std::sync::mpsc documentation](https://doc.rust-lang.org/std/sync/mpsc/index.html)
* [Async book](https://rust-lang.github.io/async-book/)

I'm using these libraries (Rust calls them crates):
* [select.rs](https://github.com/utkarshkukreti/select.rs)
* [reqwest](https://github.com/seanmonstar/reqwest)

And also these ones for nice debug print and argument parsing:
* [clap](https://docs.rs/clap/2.33.3/clap/)
* [indicatif](https://docs.rs/indicatif/0.15.0/indicatif/)
