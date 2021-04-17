### Mini Google Website
Web application written in Rust + using [Bulma](https://github.com/jgthms/bulma) CSS.

## Details:
Currently backend works with json files of such structure:
```
[
  {
    "title": "website title",
    "url": "website url",
    "description": "short description"
  },
  .
  .
  .
]
```

Webapp uses Rust Rocket framework and Tera templates.

## Usage:

Without Docker:
```bash
# Set Rust toolchain in this project to nightly
rustup install nightly-2021-02-13
cargo +nightly-2021-02-13 build --release

# Run project
cargo run
```
With Docker:
```
# Build docker container
./build.sh
# Run website on 0.0.0.0:5000
./run.sh

# When (if) you want to stop:
./stop.sh
```

## Customization:
For customization of the CSS Bulma is used.

To add your own changes, navigate to **mybulma/sass/** folder
and edit **mystyles.scss**, after that, in the same folder
execute such command:
```bash
npm run css-build
```
In **mybulma/css/** folder there will be css file, that can be used
for the project. (Simply replace previous css by that one in the **static/css/** folder)


Here is how main page looks like, when light theme customization is used:

![main page of the web app (has a cat)](../images/home_page.png)
![search page of the web app (has a cat)](../images/empty_page.png)
![error page of the web app (has a cat)](../images/error.png)
