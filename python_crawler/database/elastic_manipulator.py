from elasticsearch import Elasticsearch, exceptions
import time


NAME = "database"


class ElasticSearchDB:
    def __init__(self):
        self.es = None

    def connect(self, host, port, wait=5):
        """
        Tries to connect to the given elasticsearch
        database
        Returns ElasticSearch object or None
        """
        start = time.time()
        while True:
            try:
                es = Elasticsearch(hosts=host, port=port)
                es.exists(index="test", id=1)
            except exceptions.ConnectionError:
                if time.time() - start > wait:
                    break
            else:
                self.es = es
                return True
        return False

    def add_data(self, json, id, index=NAME):
        """
        Adds given json data to the database index
        """
        self.es.index(index=index, body=json, id=id)

    def match_data(self, key, text, n, index=NAME):
        """
        Returns first n results in full-text search query
        (by given key)
        """
        response = self.es.search(
            index=index,
            body={
             "size": n,
             "query": {
                 "match": {
                     key: text
                  }
              }
            })
        return [hit["_source"] for hit in response["hits"]["hits"]]

    def search_by_key(self, key, value, index=NAME):
        # !TODO: change normalizer and implement this
        pass

    def get_all_data(self, index=NAME):
        """
        Returns all data from the database
        """
        response = self.es.search(
            index=index,
            body={
                "query":
                    {
                        "match_all": {}
                    }
                })
        return [hit["_source"] for hit in response["hits"]["hits"]]


if __name__ == "__main__":
    db = ElasticSearchDB()
    print(db.connect("http://localhost", 9200))
    # print(db.add_data({"test": "tkknesd"}, "test"))
    print([x["title"] for x in db.match_data("text", "crane", 10)])
