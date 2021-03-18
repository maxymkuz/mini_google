import requests
import time
time.sleep(1) # just in case, it's an example, not a production app


text_to_detect = "Ich heise Hello!, Dungeon Master is dominating"

res = requests.post('http://lang_detect_python:5001/detect', json={"text":text_to_detect}) 

if res.ok:
    print(res.json()["response"])
else: 
    # If the input is wrong, or Internal error has occured:
    print("Sth unexpected has happened, handle it")
    print(res.json()["response"])
