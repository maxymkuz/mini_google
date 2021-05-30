# Mini Google
Course project for the Architecture of Computer Systems course.

## Overview:
![](images/recording.gif)

## Architecture:
![](images/structure.png)

We are working on multiple components of the web crawler at the same time:

* [Website backend](./website)
* [Elasticsearch database backend](./database_backend)
* Two crawlers (one in [Python](./python_crawler), and one in [Rust](./rust_crawler))
* Language detection backend in [Rust](./rust_lang_detection) and [Python](./lang_detect_python).

Each component is intended to run as a separate Docker container, for us
to be able to freely mix them in different amounts and on different computers/servers.

Progress can be tracked [over here](./PROGRESS.md).

## Usage:

Launch each container independently with instructions in respective directories,
or launch all of them together:
```
# Download the file with crawled websites, or crawl the websites on your own into
# the root of the project as out.txt: https://drive.google.com/file/d/1XsnWbmk4YzLmZqWjRaMXDzMC_-Rv0Zwm/view

docker-compose build

docker-compose up
```

## Prerequisites:

## Credits:
* [Vyacheslav Shevchuk](https://github.com/OldFrostDragon)
* [Andriy Sultanov](https://github.com/LastGenius-edu)
* [Maksym Kuzyshyn](https://github.com/maxymkuz)
* [Maksym Protsyk](https://github.com/maksprotsyk)
* [Daria Omelkina](https://github.com/dariaomelkina)

## License:
[MIT License](https://github.com/maxymkuz/mini_google/blob/main/LICENSE)
