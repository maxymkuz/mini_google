use select::document::Document;
use select::predicate::Name;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let webpage = if args.len() == 1 {
        "https://en.wikipedia.org/wiki/Buffy_the_Vampire_Slayer"
    } else {
        args[1].as_str()
    };
    // Creating a blocking Client to send requests with
    let client = reqwest::blocking::Client::new();

    // Sending a blocking get request, unwrapping the Result we get
    let mut res = client.get(webpage).send().unwrap();
    println!("Status for {}: {}", webpage, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    // Searching for structured data on the page.
    // We are looking for <script type="application/ld+json"> and we need all of its contents
    Document::from(body.as_str())
        .find(Name("script"))
        .filter_map(|n| {
            if (n.attr("type") == Some("application/ld+json")) {
                Some(n.text())
            } else {
                None
            }
        })
        .for_each(|x| println!("{}", x));
}
