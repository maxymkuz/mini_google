from flask import Flask, request

app = Flask(__name__)


@app.route('/', methods=['POST', 'GET'])
def login():
    if request.method == 'POST':
        search_string = request.form['user_search']
        return search_string
    else:
        search_string = request.args.get('user_search')
        return search_string


if __name__ == '__main__':
    app.run(port=5001, debug=True)
