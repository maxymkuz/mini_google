#![warn(missing_docs)]
//! This crate is an implementation of multi-threaded asynchronous Rust crawler.
//!
//! It is one of two versions of such a crawler (the other being
//! developed in Python at https://github.com/maxymkuz/mini_google )
use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{collections::BTreeMap, time::Instant};

mod scrape;
mod thread_pool;
use thread_pool::{Result, ScrapeData, ScrapeParam, ThreadPool, WorkerResult};

// TODO: Write a couple of tests :)

struct Arguments {
    input_file: String,
    output_file: String,
    user_agent: String,
    threads_num: u64,
    webpage_limit: u64,
    repeat_limit: u8,
}

/// Parses command line arguments, returns a tuple with them
fn arg_parser() -> Arguments {
    // Matching our command-line arguments
    // Clap also creates a nice help page for our program
    let matches = App::new("Rust Structured Data Crawler")
        .version("0.1.0")
        .about("Crawls all links from a given website inside of its high-level domain,  collecting structured data into a file")
        .arg(Arg::with_name("input_file")
                .short("if")
                .long("inp_file")
                .takes_value(true)
                .help("Input file with website URLs"))
        .arg(Arg::with_name("output_file")
                .short("of")
                .long("out_file")
                .takes_value(true)
                .help("Output file for collected structured data"))
        .arg(Arg::with_name("threads_num")
                .short("t")
                .long("threads")
                .takes_value(true)
                .help("Number of threads (workers) in the thread pool"))
        .arg(Arg::with_name("page_limit")
                .short("l")
                .long("limit")
                .takes_value(true)
                .help("Number of pages to crawl until stopping (Default is 1024)"))
        .arg(Arg::with_name("repeat_limit")
                .short("r")
                .long("replimit")
                .takes_value(true)
                .help("Number of times to retry crawling a page if it errors out (Default is 3)"))
        .get_matches();

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

    let repeat_limit: u8 = matches
        .value_of("replimit")
        .unwrap_or("3")
        .parse()
        .expect("Provide a valid repeat limit!");

    // This is used to signal webpages that we are the same client sending requests to them
    let user_agent: String = "rust-crawler-mini-google-ucu".to_string();

    // Returns the parsed arguments
    Arguments {
        input_file,
        output_file,
        user_agent,
        threads_num,
        webpage_limit,
        repeat_limit,
    }
}

/// Enum representing the state the URL is in.
///
/// I'm trying to be smart and keep its size to a minimum since every URL has one of these
/// in the BTree. So instead of operating with nanosecs and global time or something,
/// I am saving time in seconds that passed since the beginning of the program in u16
/// (which should leave about 18 hours of work for us to work with)
///
/// So the possible paths in this state machine for every URL are:
///
///```text
///                 ErrorFinal
///                      ^
/// VisitedFinal         |
///       ^          ErrorAttempt(x, y) <-
///        \             ^          \    |
///        Sent----------|           \---/
///          ^
///          |
///       NotSent
///```
///
/// I am not really using VisitedFinal at the moment, since I am just removing the URL from the
/// map as soon as it's done. I also should get rid of ErrorFinal I think. Maybe just keep them for
/// illustrative purposes or something? But I am just essentially wasting CPU time on updating them
/// instead of just removing the URLs.
#[derive(Debug)]
enum UrlState {
    /// Not sent and not visited - should be picked up by the thread and sent to workers
    NotSent,
    ///  * Sent - so we are waiting on the response and are not sending it to anyone else
    Sent,
    ///  * Tried to visit, but errorred (how_many_attempts: u8, time_of_the_last_attempt: u16) - to be
    ///    sent to the queue once again once enough time passes for a new attempt
    ErrorAttempt(u8, u16),
    ///  * Errored - we went through several attempts, got nothing, so forget about this URL
    _ErrorFinal,
    ///  * Visited - everything is awesome
    _VisitedFinal,
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
    let args = arg_parser();

    // Create vectors to save webpages we have to crawl and structured data on them
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
    let input = File::open(args.input_file)?;
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

    // TODO: Switch to using SOLR or ElasticSearch or whatever we decide upon
    // currently all of this is commented since I'd have to rework it anyway
    //
    // Establishing the database connection pool

    // Spawn the database connector in a separate async task

    // TODO: Ensure that we've connected to the database before starting the application,
    // retry if not

    // Creating a thread pool with asynchronous scrapper workers to send URLs to
    let pool = ThreadPool::new(args.threads_num, args.user_agent, high_level_domain);

    // Opening an output file
    let mut output = File::create(args.output_file)?;

    // A nice TUI debug interface with the current progress
    // TODO: Add a nice way to see what each thread is doing right now
    let prog_bar = ProgressBar::new(args.webpage_limit as u64);
    prog_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:40.cyan/blue} {pos:>5}/{len:5} {msg}")
            .progress_chars("##-"),
    );
    prog_bar.set_message("Collecting data from webpages...");

    // In order not to overwhelm the channel we have to send just enough urls through
    let mut needed_sends: u64 = args.threads_num + 10;
    let urls_per_send = 50;
    let program_start_time = Instant::now();

    // Calculating the time we should wait between attempts, with the formula being:
    // time_to_wait = START_WAIT_TIME + EXPONENT**(number_of_attempts)
    const START_WAIT_TIME: u16 = 2;
    const EXPONENT: u16 = 5;
    let attempt_time_wait: Vec<u16> = (0u32..args.repeat_limit as u32)
        .map(|i| START_WAIT_TIME + EXPONENT.pow(i))
        .collect();

    // TODO: Hey, I should definitely look into what might be the bottleneck here
    // (not really a bottleneck since it's crazily fast, but still) and if it's the queue,
    // consider switching to crossbeam's MCPC bounded queue. Will solve a few problems I have
    // here and get rid of weird hacks I put on top of std mpsc channel. Will have to see

    // TODO: Have no idea whether it's a valid concern, from what my perf runs and flamegraphs and
    // a little focused profiling told me the absolute majority of our CPU time is spent parsing
    // the HTML page, but: I might still have to figure stuff out with lifetimes here. We are
    // definitely copying stuff around unnecessarily but I am too lazy to sit down and improve this
    // code's speed by 5% by worsening its readability to infinity :)

    // Crawling through all webpages until we have enough unique pages crawled
    while visited_webpages < args.webpage_limit {
        // If we have webpages to send and the channel is not overwhelmed, we need to send new
        // sets of URLs, which are then processed in the workers asynchronously
        if webpages.len() > urls_per_send && needed_sends > 0 {
            // Creating the container to send URLs through the channel in and an iterator through
            // our webpages list
            let mut webpages_to_send: Vec<String> = vec![];
            let mut webpage_iter = webpages.iter_mut();

            // Look at what time it is right now to check whether we need
            // to repeat a crawl on some pages
            let now = Instant::now();

            while webpages_to_send.len() < urls_per_send {
                // TODO: As above, maybe switch to using BTreeMap.pop_first() once it's stable?
                // TODO: Check up with the database whether the URL has been checked. Or just not care

                // As long as there are pages in the list, send them along and update their state
                if let Some(webpage) = webpage_iter.next() {
                    match webpage.1 {
                        UrlState::NotSent => {
                            webpages_to_send.push(webpage.0.clone());
                            *webpage.1 = UrlState::Sent;
                        }
                        UrlState::ErrorAttempt(attempts, last_time) => {
                            // We still have to attempt to crawl the page
                            if *attempts < args.repeat_limit {
                                // Check whether enough time passed between now and last_time,
                                // depending on the number of attempts so far
                                if (now - program_start_time).as_secs() as u16
                                    > *last_time + attempt_time_wait[(*attempts as usize) - 1]
                                {
                                    // Try to crawl it once again
                                    webpages_to_send.push(webpage.0.clone());
                                }
                                // Otherwise, wait
                            }
                        }
                        _ => continue,
                    }
                } else {
                    break;
                }
            }
            pool.url_sender.send(webpages_to_send).unwrap();
            needed_sends -= 1;
        }

        // If there is data in the channel, we destructure it and change the state of the URL
        // accordingly, updating whether the scrape was successful or resulted in an error
        if let Ok(worker_result) = pool.new_data_receiver.try_recv() {
            match worker_result {
                WorkerResult::Done(url, title, _sd, new_urls, full_text) => {
                    // Writing collected structured data to the file
                    // Should probaly switch to this: https://docs.rs/async-std/1.9.0/async_std/fs/struct.File.html#impl-Write
                    // But this is more of a debug thing so who cares
                    write!(output, "{}\n{}\n{}\n", url, title, full_text);

                    // Adding newly collected links to the webpage list
                    new_urls.into_iter().for_each(|new_url| {
                        webpages.entry(new_url.clone()).or_insert(UrlState::NotSent);
                    });

                    // Switching the state of the url to visited!
                    webpages.remove(&url);

                    // Send the collected data into SQL database

                    // We've just received scrapped data, we need to send a new set of URLs back
                    // Updating the progress bar
                    needed_sends += 1;
                    visited_webpages += 1;
                    prog_bar.set_position(visited_webpages);
                }
                WorkerResult::Failed(url) => {
                    // URL failed. Update its state, incrementing fail attempts number and updating the last attempt time
                    let now = Instant::now();
                    let last_attempt_time = (now - program_start_time).as_secs() as u16;
                    let state = webpages
                        .entry(url.clone())
                        .or_insert(UrlState::ErrorAttempt(0, 0));

                    // If the URL has already failed too many times, discard it completely.
                    // If not, update its attempts number and time, give it one more chance
                    // If its URLs' first error, start its error count
                    match state {
                        UrlState::ErrorAttempt(num_attempts, _)
                            if (*num_attempts + 1) >= args.repeat_limit =>
                        {
                            //*state = UrlState::ErrorFinal;
                            webpages.remove(&url);
                        }
                        UrlState::ErrorAttempt(num_attempts, _) => {
                            *state = UrlState::ErrorAttempt(*num_attempts + 1, last_attempt_time)
                        }
                        UrlState::Sent => *state = UrlState::ErrorAttempt(1, last_attempt_time),
                        _ => (),
                    }
                }
            }
        }
    }

    prog_bar.finish_with_message("Finished collecting data");

    println!("We have visited {} webpages", visited_webpages);

    // Dropping the thread pool and joining all the threads
    // I had to do this explicitly since we wrote to the file after it, there is really no need to
    // do this still, but whatever, won't bother with it
    println!("Hold on while we close the thread pool...");
    drop(pool);

    // We have to return Ok with an empty () inside of it at the end of main since it returns a
    // Result for us to be able to use the question mark operator
    Ok(())
}
