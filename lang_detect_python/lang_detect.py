from flask import Flask
from flask import request, jsonify
import json
import os

# TODO use and train Facebook fasttext library, it should be more precise and FAST!
# Also, the potential problem is that this library is not deterministic!
from langdetect import detect_langs


app = Flask(__name__)

@app.route('/')
def example():
    return json.dumps({"response": "Just nothing"}), 200


@app.route('/detect', methods=['POST'])
def detect_language():
    """
    Returns a json in the form of:
    {'response': [['nl', 0.7142824916142885], ['de', 0.28571299164119035]]}
    """
    try:
        if not request.is_json:
            return json.dumps({"response": "Input is not a json"}), 400
        # Parsing input json
        content = request.get_json()  

        if "text" not in content:
            return json.dumps({"response": "No text field in json"}), 400
        # Retrieving text field from json
        text = content["text"]
        
        if not isinstance(text, str) or len(text) == 0:
            return json.dumps({"response": "Text field is of inapropriate type, or empty"}), 400
        # Detects language, returns a list of Language.langdetect objects
        langs = detect_langs(text)

        print(type(langs[0]))
        print(langs[0].prob, type(langs[0].prob))
        print(langs[0].lang, type(langs[0].lang))
        return json.dumps({"response": [[lang.lang, lang.prob] for lang in langs]}), 200

    except:
        return json.dumps({"response": "Unexpected internal server error! Save it, and handle."}), 500


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=os.getenv('PORT'))