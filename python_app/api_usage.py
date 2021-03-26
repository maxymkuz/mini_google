import time
import requests

print("hey there 1\n\n\n")

time.sleep(10)  # just in case, it's an example, not a production app


print("hey there2\n\n\n")
r = requests.post("http://elasticsearch:9200", data={"text": "query"})
print(r)



#
# text_to_detect = "Ich heise Hello!, Dungeon Master is dominating"
#
# res = requests.post('http://lang_detect_python:5001/detect', json={"text":text_to_detect})
#
# if res.ok:
#     print(res.json()["response"])
# else:
#     # If the input is wrong, or Internal error has occured:
#     print("Sth unexpected has happened, handle it")
#     print(res.json()["response"])