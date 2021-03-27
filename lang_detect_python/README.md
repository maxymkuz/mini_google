A little set up of a language detection server in Python with the 
usage of it in a Rust program.

Since we plan to launch the language detection server on its own
as a separate service in a layer between Rust crawler and Rust backend
and the database, this is a rough prototype of that you can test locally.

To test it on your own:
```bash
# To launch python language detection server
pip install -r requirements.txt
python3 lang_detect.py
```
