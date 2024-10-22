//! `ThreadPool` module for multi processor Crawl Workers that work with a separate scrape function
//!
//! Contains `ThreadPool` itself and structs it requires scrape
//! functions to take as parameters and return: `ScrapeData` and `ScrapeRes`

use error_chain::error_chain;
use futures::stream::{self, StreamExt};
use std::collections::BTreeSet;
use std::sync::mpsc::RecvError;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::scrape::scrape;

// A quick way to make it so that a question mark operator will work in functions
// that can return several types of errors, automatically implements required
// From traits for the specified Error types.
error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

/// A result of the worker's scrape attempt.
/// If it has done everything, returns Done with all the parameters,
/// if it has encountered errors, sends back the URL that errored
pub enum WorkerResult {
    /// The URL was parsed successfully, return all of the scrapped data
    Done(String, String, Option<String>, BTreeSet<String>, String),
    /// Something went wrong during the processing
    Failed(String),
}

/// `ThreadPool` spawns and holds threads that crawl and scrape webpages.
///
/// Acquires webpages through the `url_sender` channel and gives back newly acquired URLs through
/// `new_url_receiver` channel, as well as structured data from the pages crawled
/// through `sd_receive`r channel
#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    pub url_sender: mpsc::Sender<Vec<String>>,
    pub new_data_receiver: mpsc::Receiver<WorkerResult>,
}

impl ThreadPool {
    /// Create a new `ThreadPool` given the `size` - number of threads to spawn,
    /// HTTP `user_agent` name for crawling identification and the `high_level_domain` to crawl.
    pub fn new(size: u64, user_agent: String, high_level_domain: String) -> ThreadPool {
        assert!(size > 0);

        let (url_sender, url_receiver) = mpsc::channel();
        let (new_data_sender, new_data_receiver) = mpsc::channel();
        let url_receiver = Arc::new(Mutex::new(url_receiver));

        let page_data = PageData {
            url_receiver,
            new_data_sender,
            user_agent,
            high_level_domain,
        };

        let mut workers = Vec::new();

        for id in 0..size {
            let page_data = page_data.clone();
            workers.push(Worker::new(id, page_data));
        }

        ThreadPool {
            workers,
            url_sender,
            new_data_receiver,
        }
    }
}

/// A helper struct that holds values `Worker` requires to be spawned
#[derive(Clone)]
struct PageData {
    url_receiver: Arc<Mutex<mpsc::Receiver<Vec<String>>>>,
    new_data_sender: mpsc::Sender<WorkerResult>,
    user_agent: String,
    high_level_domain: String,
}

/// A struct that `scrape` function takes as a parameter, containing data given to it by the `Worker`
pub struct ScrapeParam {
    pub webpage_url: String,
    pub user_agent: String,
    pub high_level_domain: String,
}

/// A struct that `scrape` function returns after scrapping a webpage
pub struct ScrapeData {
    pub webpage: String,
    pub page_title: String,
    pub all_links: BTreeSet<String>,
    pub structured_data: Option<String>,
    pub full_text: String,
}

/// A single thread in the `ThreadPool`, working on a given `scrape` function and containing its
/// own `JoinHandle` and `id`
#[allow(dead_code)]
struct Worker {
    id: u64,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new `Worker` with the given `id` for debug identification and page_data of a
    /// webpage to scrape
    fn new(id: u64, page_data: PageData) -> Worker {
        let thread = thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    loop {
                        let webpage_urls = page_data.url_receiver.lock().unwrap().recv();

                        // If the other end has disconnected, we should also quit the loop
                        let webpage_urls = match webpage_urls {
                            Ok(webpage_urls) => webpage_urls,
                            Err(RecvError) => {
                                break;
                            }
                        };

                        //println!("Worker {} got URLs {:?}", id, webpage_urls);
                        let webpage_urls = stream::iter(webpage_urls);

                        webpage_urls
                            .for_each_concurrent(None, |webpage_url| {
                                let page_data = page_data.clone();
                                let webpage_url = webpage_url.clone();
                                let scrape_data = ScrapeParam {
                                    webpage_url,
                                    user_agent: page_data.user_agent.clone(),
                                    high_level_domain: page_data.high_level_domain.clone(),
                                };
                                async move {
                                    match scrape(scrape_data).await {
                                        Ok(scrape_res) => {
                                            // Send newly collected links and structured data
                                            page_data
                                                .new_data_sender
                                                .send(WorkerResult::Done(
                                                    scrape_res.webpage,
                                                    scrape_res.page_title,
                                                    scrape_res.structured_data,
                                                    scrape_res.all_links,
                                                    scrape_res.full_text,
                                                ))
                                                .ok();
                                        }
                                        Err(url) => {
                                            page_data
                                                .new_data_sender
                                                .send(WorkerResult::Failed(url))
                                                .ok();
                                        }
                                    };
                                    //println!("Worker {} finished URL {}", id, webpage_url);
                                }
                            })
                            .await;
                    }
                })
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
