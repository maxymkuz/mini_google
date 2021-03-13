#![warn(missing_docs)]
//! This crate is an implementation of multi-threaded asynchronous Rust crawler.
//!
//! It is one of two versions of such a crawler (the other being
//! developed in Python at https://github.com/maxymkuz/mini_google )
use chrono::{NaiveDate, Utc};
use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use tokio_postgres::NoTls;

mod scrape;
mod thread_pool;
use thread_pool::{Result, ScrapeData, ScrapeRes, ThreadPool};

// TODO: Write a couple of tests

/// Parses command line arguments, returns a tuple with them
fn arg_parser() -> (String, String, String, u64, u64) {
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
        .expect("Provide an input file!")
        .to_string();

    let output_file = matches
        .value_of("output_file")
        .expect("Provide an output file!")
        .to_string();

    let threads_num: u64 = matches
        .value_of("threads_num")
        .expect("Provide a valid number of threads!")
        .parse()
        .expect("Provide a valid number of threads!");

    let webpage_limit: u64 = matches
        .value_of("page_limit")
        .unwrap_or("1024")
        .parse()
        .expect("Provide a valid webpage limit!");

    // This is used to signal webpages that we are the same client sending requests to them
    let user_agent: String = "rust-crawler-mini-google-ucu".to_string();

    // Returns the parsed arguments
    (
        input_file,
        output_file,
        user_agent,
        threads_num,
        webpage_limit,
    )
}

/// Enum representing the state the URL is in.
enum UrlState {
    /// Not sent and not visited - should be picked up by the thread and sent to workers
    NotSent,
    Sent,
    ErrorAttempt(i8, i8),
    ErrorFinal,
    VisitedFinal,
}

/// The main thread function that parses command line arguments, reads webpage links from
/// the input file and launches the thread pool of crawlers.
///
/// While they are working on the webpages the main thread sends them through the channel,
/// we receive parsed text and structured data back, sending it to the database and adding
/// newly collected links to the list of stuff to parse.
///
/// Overall, this works in several threads, and each thread performs its work asynchronously,
/// sending out several requests simultaneously, either to the crawled website or to the database.
#[tokio::main]
async fn main() -> Result<()> {
    // Parsing the command line arguments
    let (input_file, output_file, user_agent, threads_num, webpage_limit) = arg_parser();

    // Create vectors to save webpages we have to crawl and structured data on them
    //
    // Alright, gonna try to implement a smarter data structure here.
    // What we need is a BTreeMap, probably, holding both the url itself,
    // and the state of that url it's currently in, something like an enum state machine:
    //  * Unsent and unvisited - should be picked up by the thread and sent to workers
    //  * Sent - so we are waiting on the response and are not sending it to anyone else
    //  * Visited - everything is awesome
    //  * Tried to visit, but errorred (how_many_attempts: i8, time_of_the_last_attempt: smth) - to be
    //    sent to the queue once again once enough time passes for a new attempt
    //  * Errored - we went through several attempts, got nothing, so forget about this URL
    //
    //  This would make everything a lot more flexable and we wouldn't have to clone stuff around
    //  that much, I think. I should also probably figure stuff out with these lifetimes, so that
    //  one thing would actually own the String, and others would just get a &'a str or whatever.
    //  But the string should probably be Pin, right? Have no idea
    let mut webpages: BTreeMap<String, UrlState> = BTreeMap::new();

    // Trying to keep clones to a minimum here. I don't think we need to keep
    // a list of visited webpages, since we already hold that info in the map
    //let mut visited_webpages: HashSet<String> = HashSet::new();
    //let mut structured_data: HashMap<String, String> = HashMap::new();
    let mut visited_webpages: u64 = 0;

    // Reading the input file with URLs
    let input = File::open(input_file)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        webpages.insert(
            line.expect("Provide a valid input in the input file"),
            UrlState::NotSent,
        );
    }

    // Determining the high-level domain we crawl, which workers will later use to filter
    // out the links we are not interested in
    // Currently we have one high-level domain for all the app (I am crawling Wikipedia)
    // TODO: Transfer this high-level domain parsing into each worker, or delete it altogether
    // TODO: We could also switch to using BTreeMap.first() once it's stable, see https://github.com/rust-lang/rust/issues/62924
    // instead of creating an iterator and grabbing the first element out of it
    let mut high_level_domain = webpages.keys().next().unwrap().clone().to_string();
    high_level_domain = Url::parse(&high_level_domain)
        .unwrap()
        .host_str()
        .unwrap()
        .to_string();

    // Establishing the database connection pool
    // TODO: Switch to using SOLR or ElasticSearch or whatever we decide upon
    //let (client, connection) = tokio_postgres::connect(
    //"dbname=main_fts user=postgres password=postgres host=localhost port=5432",
    //NoTls,
    //)
    //.await?;

    // Spawn the database connector in a separate async task
    //tokio::spawn(async move {
    //if let Err(e) = connection.await {
    //eprintln!("connection error: {}", e);
    //}
    //});

    // Creating the database tables we need if they are not already created
    //client
    //.query(
    //"CREATE TABLE IF NOT EXISTS websites_en (
    //site_id SERIAL NOT NULL,
    //url TEXT NOT NULL,
    //date_added DATE NOT NULL,
    //last_modified DATE,
    //site_text TEXT NOT NULL,
    //tokenized TSVECTOR);",
    //&[],
    //)
    //.await?;

    // Creating a thread pool with asynchronous scrapper workers to send URLs to
    let pool = ThreadPool::new(threads_num, user_agent, high_level_domain);

    // Opening an output file
    let mut output = File::create(output_file)?;

    // A nice TUI debug interface with the current progress
    // TODO: Add a nice way to see what each thread is doing right now
    let prog_bar = ProgressBar::new(webpage_limit as u64);
    prog_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:40.cyan/blue} {pos:>5}/{len:5} {msg}")
            .progress_chars("##-"),
    );
    prog_bar.set_message("Collecting data from webpages...");

    // In order not to overwhelm the channel we have to send just enough urls through
    let mut needed_sends: u64 = threads_num + 10;
    let urls_per_send = 100;

    // Crawling through all webpages until we have enough unique pages crawled
    while visited_webpages < webpage_limit {
        // If we have webpages to send and the channel is not overwhelmed, we need to send new
        // sets of URLs, which are then processed in the workers asynchronously
        if webpages.len() > urls_per_send && needed_sends > 0 {
            // Creating the container to send URLs through the channel in and an iterator through
            // our webpages list
            let mut webpages_to_send: Vec<String> = vec![];
            let mut webpage_iter = webpages.iter();

            while webpages_to_send.len() < urls_per_send {
                // TODO: As above, maybe switch to using BTreeMap.pop_first() once it's stable?
                if let Some(webpage) = webpage_iter.next() {
                    // TODO: Instead check whether the database contains this url and whether we should update it
                    match webpage.1 {
                        // TODO: Set the state to Sent once it's actually sent. might need some
                        // interior mutability for the enum though, dk
                        UrlState::NotSent => webpages_to_send.push(webpage.0.clone()),
                        UrlState::ErrorAttempt(attempts, last_time) => {

                            // TODO: Check whether enough time passed between now and last_time,
                            // depending on the number of attempts so far for us to send it once
                            // again and try to crawl
                        }
                        _ => continue,
                    }
                }
            }
            pool.url_sender.send(webpages_to_send).unwrap();
            needed_sends -= 1;
        }

        // Try to receive structured data and newly collected links from our end of the channel
        if let Ok((url, sd, new_urls, full_text)) = pool.new_data_receiver.try_recv() {
            //println!("Received {} new URLs", new_urls.len());
            // Writing collected structured data to the file
            write!(output, "{}: \n{:?}\n{:?}", url, sd, full_text)?;

            //structured_data.insert(url.clone(), sd);
            //visited_webpages.insert(url.clone());

            // Adding newly collected links to the webpage list
            new_urls.into_iter().for_each(|url| {
                webpages.entry(url.clone()).or_insert(UrlState::NotSent);
            });

            // Updating the progress bar
            prog_bar.set_position(visited_webpages);

            // Send the collected data into SQL database
            //let now: NaiveDate = Utc::now().date().naive_utc();

            //client
            //.query(
            //"INSERT INTO websites_en (url, date_added, site_text, tokenized) \
            //VALUES ($1, $2, $3, to_tsvector($4));",
            //&[&url, &now, &full_text, &full_text],
            //)
            //.await?;

            // We've just received scrapped data, we need to send a new set of URLs back
            needed_sends += 1;
            visited_webpages += 1;
        };
    }

    prog_bar.finish_with_message("Finished collecting data");

    println!("We have visited {} webpages", visited_webpages);

    // Dropping the thread pool and joining all the threads
    println!("Hold on while we close the thread pool...");
    drop(pool);

    // We have to return Ok with an empty () inside of it at the end of main since it returns a
    // Result for us to be able to use the question mark operator
    Ok(())
}
