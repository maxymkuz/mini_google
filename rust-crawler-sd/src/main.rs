#![warn(missing_docs)]
//! This crate is an implementation of multi-threaded asynchronous Rust crawler.
//!
//! It is one of two versions of such a crawler (the other being
//! developed in Python at https://github.com/maxymkuz/mini_google )
use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

mod scrape;
mod thread_pool;
use thread_pool::{Result, ScrapeData, ScrapeRes, ThreadPool};

// TODO: Write a couple of tests

/// The main thread function that parses command line arguments, reads webpage links from
/// the input file and launches the thread pool, waits for it to finish and writes collected
/// data back on disk.
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
        .arg(
            Arg::with_name("threads_num")
                .short("t")
                .long("threads")
                .takes_value(true)
                .help("Number of threads (workers) in the thread pool"),
        )
        .arg(
            Arg::with_name("page_limit")
                .short("l")
                .long("limit")
                .takes_value(true)
                .help("Number of pages to crawl until stopping"),
        ).get_matches();

    let input_file = matches
        .value_of("input_file")
        .expect("Provide an input file!");

    let output_file = matches
        .value_of("output_file")
        .expect("Provide an output file!");

    let threads_num: usize = matches
        .value_of("threads_num")
        .expect("Provide a valid number of threads!")
        .parse()
        .expect("Provide a valid number of threads!");

    let webpage_limit: usize = matches
        .value_of("page_limit")
        .expect("Provide a valid number of threads!")
        .parse()
        .unwrap_or(1024);

    let user_agent: String = "rust-crawler-mini-google-ucu".to_string();

    // Create vectors to save webpages we have to crawl and structured data on them
    let mut webpages: Vec<String> = vec![];
    let mut visited_webpages: HashSet<String> = HashSet::new();
    let mut total_pages_sent: usize = 0;
    let mut structured_data: HashMap<String, String> = HashMap::new();

    // Reading the input file with URLs
    let input = File::open(input_file)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        webpages.push(line.expect("Provide a valid input in the input file"));
    }

    // Currently we have one high-level domain for all the app (I am crawling Wikipedia)
    // TODO: Transfer this high-level domain parsing into each worker, or delete it altogether
    let mut high_level_domain = webpages[0].clone().to_string();
    high_level_domain = Url::parse(&high_level_domain)
        .unwrap()
        .host_str()
        .unwrap()
        .to_string();

    // Creating a thread pool with asynchronous scrapper workers to send URLs to
    let pool = ThreadPool::new(threads_num, user_agent, high_level_domain);

    // A nice TUI debug interface
    // TODO: Add a nice way to see what each thread is doing right now
    let pb = ProgressBar::new(webpage_limit as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:40.cyan/blue} {pos:>5}/{len:5} {msg}")
            .progress_chars("##-"),
    );
    pb.set_message("Collecting data from webpages...");

    // Crawling through all webpages
    while visited_webpages.len() < webpage_limit {
        // Send new urls to the scrapper
        let mut sent_webpages: Vec<String> = vec![];

        // TODO: Think of something more flexible and nice
        if webpages.len() > 20 && total_pages_sent < webpage_limit * 2 {
            while sent_webpages.len() < 20 {
                match webpages.pop() {
                    Some(webpage) => {
                        if !visited_webpages.contains(&webpage) {
                            sent_webpages.push(webpage);
                        }
                    }
                    None => (),
                }
            }
            total_pages_sent += sent_webpages.len();
            pool.url_sender.send(sent_webpages).unwrap();
        }

        // Try to receive structured data from our end of the channel
        match pool.sd_receiver.try_recv() {
            Ok((url, sd)) => {
                structured_data.insert(url.clone(), sd);
                visited_webpages.insert(url);
                pb.set_position(visited_webpages.len() as u64);
            }
            Err(_) => (),
        };

        // Try to receive newly collected links from our end of the channel
        match pool.new_url_receiver.try_recv() {
            Ok(new_urls) => {
                //println!("Received {} new URLs", new_urls.len());
                webpages.extend(new_urls);
            }
            Err(_) => (),
        };
    }

    pb.finish_with_message("Finished collecting data");

    println!("We have visited {} webpages", visited_webpages.len());

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
