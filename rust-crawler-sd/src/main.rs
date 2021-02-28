use clap::{App, Arg};
use error_chain::error_chain;
use reqwest::Url;
use select::document::Document;
use select::predicate::{Name, Predicate};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::thread::JoinHandle;

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

struct ScrapeRes {
    all_links: HashSet<String>,
    structured_data: String,
    webpage: String,
}

fn get_links_from_html(html: &str, base_url: &str, high_level_domain: &str) -> HashSet<String> {
    // Looks for all elements in the html body that are valid
    // links, saving unique ones in the HashSet
    Document::from(html)
        .find(Name("a").or(Name("link")))
        .filter_map(|n| n.attr("href"))
        .filter_map(|x| normalize_url(x, base_url, high_level_domain))
        .collect::<HashSet<String>>()
}

fn normalize_url(url: &str, base_url: &str, high_level_domain: &str) -> Option<String> {
    // If the URL was valid, we check whether it has a host and whether
    // this host is the high level domain we accept
    // If the URL was relative, reqwest also returns an Err variant
    let base = Url::parse(base_url).ok().expect("Invalid page URL");
    let joined = base.join(url).expect("Invalid page URL");
    if joined.has_host() && joined.host_str().unwrap() == high_level_domain {
        Some(joined.to_string())
    } else {
        None
    }
}

fn scrape(webpage_url: &str, user_agent: &str, high_level_domain: &str) -> Result<ScrapeRes> {
    // Creating a blocking Client to send requests with
    // TODO: Maybe use an asynchronous client instead of a blocking one?
    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap();

    // Sending a blocking get request, unwrapping the Result we get
    println!("{}", webpage_url);
    let res = client.get(webpage_url).send().unwrap().text().unwrap();

    // Finding all links on the page
    let all_links = get_links_from_html(&res, &webpage_url, &high_level_domain);

    // Searching for structured data on the page.
    // We are looking for <script type="application/ld+json"> and we need all of its contents
    let structured_data: String = Document::from(res.as_str())
        .find(Name("script"))
        .filter(|n| n.attr("type") == Some("application/ld+json"))
        .map(|x| x.text())
        .nth(0)
        .ok_or_else(|| "The page didn't have structured data")?;
    let webpage = webpage_url.to_string();

    Ok(ScrapeRes {
        all_links,
        structured_data,
        webpage,
    })
}

fn main() -> Result<()> {
    // Matching our command-line arguments
    // Clap also creates a nice help page for our program
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

    let user_agent: &'static str = "rust-crawler-mini-google-ucu";

    // Create vectors to save webpages we have to crawl and structured data on them
    let mut webpages: Vec<String> = vec![];
    let mut visited_webpages: HashSet<String> = HashSet::new();
    let mut structured_data: HashMap<String, String> = HashMap::new();

    // Reading the input file with URLs
    let input = File::open(input_file)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        webpages.push(line.expect("Provide a valid input in the input file"));
    }

    // Currently we have one high-level domain for all the app.
    // TODO: Transfer this high-level domain parsing into each worker, or delete it altogether
    let mut high_level_domain = webpages[0].clone().to_string();
    high_level_domain = Url::parse(&high_level_domain)
        .unwrap()
        .host_str()
        .unwrap()
        .to_string();

    // Crawling through all webpages
    while webpages.len() > 0 && visited_webpages.len() < 1000 {
        let mut workers: Vec<JoinHandle<_>> = vec![];
        let mut new_webpages: Vec<String> = vec![];

        // Iterating over all currently picked up URLs
        // THIS IS THE WORST WAY EVER DON'T EVEN THINK ABOUT IT
        // TODO: Implement something okayish
        let limit = if webpages.len() < 200 {
            webpages.len()
        } else {
            200
        };
        for webpage in &webpages[..limit] {
            // This is required for the webpage to last long enough
            // for the thread after the loop
            let webpage = webpage.to_string();
            let high_level_domain = high_level_domain.to_string();

            if visited_webpages.contains(&webpage) {
                continue;
            };

            // Creating a new worker for each webpage
            // TODO: Maybe give them a list of URLs instead of a single URL?
            // TODO: Probably better to give them one URL, and let them continue
            // crawling on whatever they find on this page?
            workers.push(thread::spawn(move || {
                scrape(&webpage, &user_agent, &high_level_domain)
            }));
        }

        for worker in workers {
            // Joining each worker and adding the data collected by them to the global containers
            match worker.join() {
                Ok(worker_res) => match worker_res {
                    Ok(scrape_res) => {
                        // Add newly collected links
                        for new_webpage in scrape_res.all_links {
                            if !visited_webpages.contains(&new_webpage) {
                                new_webpages.push(new_webpage);
                            };
                        }
                        // Add collected structured data
                        // TODO: Add the whole scrapped text and possibly headers as a separate
                        // entity
                        structured_data
                            .insert(scrape_res.webpage.clone(), scrape_res.structured_data);

                        // Remember that we've visited this webpage
                        visited_webpages.insert(scrape_res.webpage);
                    }
                    Err(_) => (),
                },
                Err(_) => (),
            }
        }

        webpages.extend(new_webpages);
        println!("{}", visited_webpages.len());
    }

    // Opening an output file
    let mut output = File::create(output_file)?;

    // Writing collected structured data to the file
    for sd in structured_data {
        write!(output, "{:?}\n", sd)?;
    }

    // We have to return Ok with an empty () inside of it at the end of main since it returns a
    // Result for us to be able to use the question mark operator
    Ok(())
}
