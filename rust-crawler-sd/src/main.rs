use clap::{App, Arg};
use error_chain::error_chain;
use indicatif::ProgressBar;
use reqwest::Url;
use select::document::Document;
use select::predicate::{Name, Predicate};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// TODO: Add documentation and comments where needed
// TODO: Write a couple of tests
// TODO: Divide this file into several modules

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    url_sender: mpsc::Sender<String>,
    new_url_receiver: mpsc::Receiver<HashSet<String>>,
    sd_receiver: mpsc::Receiver<(String, String)>,
}

impl ThreadPool {
    pub fn new(size: usize, user_agent: String, high_level_domain: String) -> ThreadPool {
        assert!(size > 0);

        let (url_sender, url_receiver) = mpsc::channel();
        let (new_url_sender, new_url_receiver) = mpsc::channel();
        let (sd_sender, sd_receiver) = mpsc::channel();

        let url_receiver = Arc::new(Mutex::new(url_receiver));

        let mut workers = Vec::new();

        for id in 0..size {
            let new_url_sender = new_url_sender.clone();
            let sd_sender = sd_sender.clone();
            let user_agent = user_agent.clone();
            let high_level_domain = high_level_domain.clone();
            workers.push(Worker::new(
                id,
                Arc::clone(&url_receiver),
                new_url_sender,
                sd_sender,
                user_agent,
                high_level_domain,
            ));
        }

        ThreadPool {
            workers,
            url_sender,
            new_url_receiver,
            sd_receiver,
        }
    }

    pub fn send_url_job(&self, url: &str) {
        self.url_sender.send(url.to_string()).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(
        id: usize,
        url_receiver: Arc<Mutex<mpsc::Receiver<String>>>,
        new_url_sender: mpsc::Sender<HashSet<String>>,
        sd_sender: mpsc::Sender<(String, String)>,
        user_agent: String,
        high_level_domain: String,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let webpage_url = url_receiver
                .lock()
                .unwrap()
                .recv()
                .expect(&format!("Thread #{} didn't receive a valid url", id));

            //println!("Worker {} got a URL {}", id, webpage_url);

            match scrape(&webpage_url, &user_agent, &high_level_domain) {
                Ok(scrape_res) => {
                    // Send newly collected links
                    new_url_sender.send(scrape_res.all_links).unwrap();

                    // Send collected structured data
                    // TODO: Add the whole scrapped text and possibly headers as a separate entity
                    sd_sender
                        .send((scrape_res.webpage, scrape_res.structured_data))
                        .unwrap();
                }
                Err(_) => (),
            };
        });

        Worker { id, thread }
    }
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

struct ScrapeRes {
    all_links: HashSet<String>,
    structured_data: String,
    webpage: String,
}

fn scrape(webpage_url: &str, user_agent: &str, high_level_domain: &str) -> Result<ScrapeRes> {
    // Creating a blocking Client to send requests with
    // TODO: Maybe use an asynchronous client instead of a blocking one?
    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap();

    // Sending a blocking get request, unwrapping the Result we get
    // TODO: See if there is a point in making each thread asynchronous on its own?
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
        ).arg(
            Arg::with_name("threads_num")
                .short("t")
                .long("threads")
                .takes_value(true)
                .help("Number of threads (workers) in the thread pool"),
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

    let user_agent: String = "rust-crawler-mini-google-ucu".to_string();

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

    // Currently we have one high-level domain for all the app (I am crawling Wikipedia)
    // TODO: Transfer this high-level domain parsing into each worker, or delete it altogether
    let mut high_level_domain = webpages[0].clone().to_string();
    high_level_domain = Url::parse(&high_level_domain)
        .unwrap()
        .host_str()
        .unwrap()
        .to_string();

    // Creating a thread pool with scrapper workers to send URLs to
    let pool = ThreadPool::new(threads_num, user_agent, high_level_domain);

    // A nice TUI debug interface
    // TODO: Add a nice way to see what each thread is doing right now
    let webpage_limit: usize = 1024;
    let pb = ProgressBar::new(webpage_limit as u64);

    // Crawling through all webpages
    while visited_webpages.len() < webpage_limit {
        match webpages.pop() {
            Some(webpage) => {
                if !visited_webpages.contains(&webpage) {
                    pool.url_sender.send(webpage).unwrap();
                }
            }
            None => (),
        }

        match pool.sd_receiver.try_recv() {
            Ok((url, sd)) => {
                structured_data.insert(url.clone(), sd);
                visited_webpages.insert(url);
            }
            Err(_) => (),
        };

        match pool.new_url_receiver.try_recv() {
            Ok(new_urls) => {
                //println!("Received {} new URLs", new_urls.len());
                pb.inc(1);
                webpages.extend(new_urls);
            }
            Err(_) => (),
        };
    }

    for worker in pool.workers {
        worker.thread.join().unwrap();
    }

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
