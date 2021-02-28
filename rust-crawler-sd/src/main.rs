use clap::{App, Arg};
use select::document::Document;
use select::predicate::Name;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    // Matching our command-line arguments
    let matches = App::new("Rust Structured Data Crawler")
        .version("0.1.0")
        .about("Crawls all links from a given website inside of its high-level domain,  collecting structured data into a file")
        .arg(
            Arg::with_name("input_file")
                .short("if")
                .long("inp_file")
                .takes_value(true)
                .help("Input file with website URLs"),
        )
        .arg(
            Arg::with_name("output_file")
                .short("of")
                .long("out_file")
                .takes_value(true)
                .help("Output file for collected structured data"),
        )
        .get_matches();

    let input_file = matches
        .value_of("input_file")
        .expect("Provide an input file!");

    let output_file = matches
        .value_of("output_file")
        .expect("Provide an output file!");

    // Reading an input file with links
    let input = File::open(input_file)?;
    let buffered = BufReader::new(input);

    let mut webpages: Vec<String> = vec![];
    let mut structured_data: Vec<String> = vec![];
    for line in buffered.lines() {
        webpages.push(line.expect("Provide a valid input in the input file"));
    }

    // Creating a blocking Client to send requests with
    let client = reqwest::blocking::Client::new();

    for webpage in webpages {
        // Sending a blocking get request, unwrapping the Result we get
        let mut res = client.get(&webpage).send().unwrap();
        println!("Status for {}: {}", webpage, res.status());

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        // Finding all links on the page
        let all_links = Document::from(body.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"));
        //.for_each(|x| println!("{}", x));

        // Searching for structured data on the page.
        // We are looking for <script type="application/ld+json"> and we need all of its contents
        structured_data.push(
            Document::from(body.as_str())
                .find(Name("script"))
                .filter(|n| n.attr("type") == Some("application/ld+json"))
                .map(|x| x.text())
                .collect(),
        );
    }

    // Opening an output file
    let mut output = File::create(output_file)?;

    for sd in structured_data {
        write!(output, "{:?}\n", sd)?;
    }

    Ok(())
}
