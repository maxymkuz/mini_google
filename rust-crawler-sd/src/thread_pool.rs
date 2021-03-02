//! `ThreadPool` module for multi processor Crawl Workers that work with a separate scrape function
//!
//! Contains `ThreadPool` itself and structs it requires scrape
//! functions to take as parameters and return: `ScrapeData` and `ScrapeRes`

use error_chain::error_chain;
use futures::stream::{self, StreamExt};
use std::collections::HashSet;
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

/// `ThreadPool` spawns and holds threads that crawl and scrape webpages.
///
/// Acquires webpages through the `url_sender` channel and gives back newly acquired URLs through
/// `new_url_receiver` channel, as well as structured data from the pages crawled
/// through `sd_receive`r channel
#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    pub url_sender: mpsc::Sender<Vec<String>>,
    pub new_url_receiver: mpsc::Receiver<HashSet<String>>,
    pub sd_receiver: mpsc::Receiver<(String, String)>,
}

impl ThreadPool {
    /// Create a new `ThreadPool` given the `size` - number of threads to spawn,
    /// HTTP `user_agent` name for crawling identification and the `high_level_domain` to crawl.
    pub fn new(size: usize, user_agent: String, high_level_domain: String) -> ThreadPool {
        assert!(size > 0);

        let (url_sender, url_receiver) = mpsc::channel();
        let (new_url_sender, new_url_receiver) = mpsc::channel();
        let (sd_sender, sd_receiver) = mpsc::channel();
        let url_receiver = Arc::new(Mutex::new(url_receiver));

        let page_data = PageData {
            url_receiver,
            new_url_sender,
            sd_sender,
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
            new_url_receiver,
            sd_receiver,
        }
    }
}

/// A helper struct that holds values `Worker` requires to be spawned
#[derive(Clone)]
struct PageData {
    url_receiver: Arc<Mutex<mpsc::Receiver<Vec<String>>>>,
    new_url_sender: mpsc::Sender<HashSet<String>>,
    sd_sender: mpsc::Sender<(String, String)>,
    user_agent: String,
    high_level_domain: String,
}

/// A struct that `scrape` function takes as a parameter, containing data given to it by the `Worker`
pub struct ScrapeData {
    pub webpage_url: String,
    pub user_agent: String,
    pub high_level_domain: String,
}

/// A struct that `scrape` function returns after scrapping a webpage
pub struct ScrapeRes {
    pub all_links: HashSet<String>,
    pub structured_data: String,
    pub webpage: String,
}

/// A single thread in the `ThreadPool`, working on a given `scrape` function and containing its
/// own `JoinHandle` and `id`
#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new `Worker` with the given `id` for debug identification and page_data of a
    /// webpage to scrape
    fn new(id: usize, page_data: PageData) -> Worker {
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
                            Ok(webpage_url) => webpage_url,
                            Err(RecvError) => {
                                break;
                            }
                        };
                        //println!("Worker {} got URLs {:?}", id, webpage_urls);
                        let webpage_urls = stream::iter(webpage_urls);

                        // TODO: Kill myself
                        webpage_urls
                            .for_each_concurrent(None, |webpage_url| {
                                let page_data = page_data.clone();
                                let webpage_url = webpage_url.clone();
                                let scrape_data = ScrapeData {
                                    webpage_url,
                                    user_agent: page_data.user_agent.clone(),
                                    high_level_domain: page_data.high_level_domain.clone(),
                                };
                                async move {
                                    //println!("Worker {} got URL {}", id, webpage_url);
                                    match scrape(scrape_data).await {
                                        Ok(scrape_res) => {
                                            // Send newly collected links
                                            page_data
                                                .new_url_sender
                                                .send(scrape_res.all_links)
                                                .expect("Other end of the channel closed");

                                            // Send collected structured data
                                            // TODO: Add the whole scrapped text and possibly headers as a separate entity
                                            page_data
                                                .sd_sender
                                                .send((
                                                    scrape_res.webpage,
                                                    scrape_res.structured_data,
                                                ))
                                                .expect("Other end of the channel closed");
                                        }
                                        Err(_) => (),
                                    };
                                    //println!("Worker {} finished URL {}", id, webpage_url);
                                }
                            })
                            .await;
                    }
                })
        });

        Worker { id, thread }
    }
}
