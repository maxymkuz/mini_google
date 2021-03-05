## Search Engine UI

Basic UI (to be) implemented using Rust.

### Prerequisites:

* Zola 

### Usage (without Docker):
```bash
cd zola-webpage
zola build
zola serve
```
### Usage (with Docker):
To build container:
```bash
./build.sh
```
To run container (site will be available at http://0.0.0.0:5000/):
```bash
./run.sh
```
To stop container:
```bash
./stop.sh
```
## Probable query format:
(Still needs more discussion.)

The search request will likely come in a string format, 
it will consist of some key-words/sentences, provided by the user 
to the input element of the website.

In order to display results of the search, website will need 
main information about them: title of the resulting webpage, its url and a short part
of the text (not more than 1--2 sentences), where searched words mostly appear on
that webpage.

Considering, that we use Zola to generate website, it is likely, that 
each search result will be saved as an md file, in order to display it on the page.
This can also be a way to cache search results for each user.