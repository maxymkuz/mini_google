### Mini Google Website
Web application written in Rust.

## Details:
Currently backend works with json files of such structure:
```
{
  "website title": {
    "url": "website url",
    "description": "short description"
  }
}
```

Webapp uses Rust Rocket framework and Tera templates.

## Usage:

Without Docker:
```bash
# Set Rust toolchain in this project to nightly
rustup override set nightly

# Build and run the app
cargo run
```
With Docker:
```
coming soon
```

Here is how main page should look like:

![](../images/home_page.png)
