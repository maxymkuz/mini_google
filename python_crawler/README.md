## Python Crawler

Implementation of crawlers and their manager written on python
Crawlers can collect all text, structured data and links from the given
list of webpages

### Before

 Install python libraries with
 ```bash
 pip install --no-cache-dir -r requirements.txt
 ```


### Usage (without Docker):
```bash
cd python-crawler
python main.py "in file" "max depth" "number of threads" "concurrent_tasks" "max_queue_size" "max_cycles" "delay"
```
### Usage (with Docker):
To build container:
```bash
./build.sh
```
