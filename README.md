# Mini Google
Course project for the Architecture of Computer Systems course.

## Overview:

We are working on multiple components of the web crawler at the same time:

* [Frontend in Rust](./frontend)
* Database in PostgreSQL
* Two crawlers (one in [Python](./python-crawler) and one in [Rust](./rust-crawler-sd) )

Each component is intended to run as a separate Docker container, for us
to be able to freely mix them in different amounts and on different computers/servers.

Example docker containers for [Rust](./rust-docker) and [Python](./python-docker) are also
temporarily available here.

Progress can be tracked [over here](./PROGRESS.md).

## Purpose and usage:

## Prerequisites:

## Credits:
* [Vyacheslav Shevchuk](https://github.com/OldFrostDragon)
* [Andriy Sultanov](https://github.com/LastGenius-edu)
* [Maksym Kuzyshyn](https://github.com/maxymkuz)
* [Maksym Protsyk](https://github.com/maksprotsyk)
* [Daria Omelkina](https://github.com/dariaomelkina)

## License:
[MIT License](https://github.com/maxymkuz/mini_google/blob/main/LICENSE)
