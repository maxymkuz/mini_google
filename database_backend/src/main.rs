//! Module that currently is supposed to read data from file, and push it to database.
//! Later, this will be fully-functional backend for crawlers to identify the language and talk to db
use elasticsearch::Elasticsearch;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

mod database; // MODULE THAT HANDLES THE ACTUAL DATABASE QUERIES
mod lang_detect; // MODULE THAT TALKS TO PYTHON LANGDETECT
mod web_listener; // MODULE THAT LAUNCHES THE WEB SERVER AND LISTENS TO WEB BACKEND AND CRAWLER QUERIES

// TODO: add vector of websites, vector of different languages?, and their probabilities
#[derive(Serialize, Deserialize)]
struct CrawledWebsite {
    url: String,
    full_text: String,
    language: String,
    // urls: Vec<String>, // TODO: uncomment later when we will implement it too
}

// we will use this func later in crawlers, so it has to be separate. Possibly make lang detection here later
async fn struct_to_db(
    website: &CrawledWebsite,
    client: &Elasticsearch,
) -> Result<(), Box<dyn std::error::Error>> {
    // We do not care about language detection (for now)
    // TODO: Implement proper language detection, error handling here etc.
    //let languages: Vec<(String, f64)> =
    //lang_detect::send_lang_detection_query(&website.text)
    //.await
    //.unwrap();
    // saving only a dominant language(with the highest probability) to struct
    //language = languages[0].0.to_owned();

    // Sending the crawled website to the database
    // Retrying if something went wrong until we get it done
    loop {
        let response = database::send_scrapped_website(&client, serde_json::json!(website)).await?;

        // Creating a json and pushing to database:
        match response.status_code() {
            StatusCode::OK => break,
            _ => continue,
        }
    }
    Ok(())
}

// Func to get a response from db, according to user query
async fn get_response<'a>(
    client: &'a Elasticsearch,
    query: &'a str,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    // Search example query
    // We are looking in the column 'full_text' for certain text pattern
    let search_query = serde_json::json!({
        "query": {
            "match": {
                "full_text": query.to_string()
            }
        }
    });
    let search_result = database::get_search(&client, search_query).await?;
    Ok(search_result)
}

// Function that parses file line by line, and inserts url, text and language to the database
// This is basically a mock for the request from a crawler until we figure that out
// It also launches the web listener that's handling crawlers and backend requests.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or("./data/100_lines_collected_data.txt".to_string());
    // Creating future json
    let mut website = CrawledWebsite {
        url: "sample_url".to_string(),
        full_text: "sample_text".to_string(),
        language: "sample_lan".to_string(),
    };

    // Establish a database connection
    let client = database::establish_database_connection();

    // Wait so that Elasticsearch will have time to index all of this. This is just so you'd be
    // able to get a valid result down below.
    //
    // I am not sure what the problem is, but it seems to be indexing the actual fulltext for a bit
    // longer........
    std::thread::sleep(std::time::Duration::from_millis(5000));

    // Reading from pre-made file with data line by line
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    for (index, line) in buffered.lines().enumerate() {
        if let Ok(line) = line {
            if index % 3 == 0 {
                // Parsing a vector of links, divided by space
                // First link=this page url. All subsequent ones are urls this page links to
                let mut link_vector = line.split(' ');

                website.url = link_vector.next().unwrap().to_string();
                // urls = link_vector.map(|x| x.to_string()).collect();
            }
            if index % 3 == 1 {
                // Getting the full text of the website
                website.full_text = line.to_string();

                // Sending the data stored in struct website to database
                struct_to_db(&website, &client).await?;
            }
        }
    }

    println!("Put the file in the database");

    // Just an example launch
    //let result = get_response(&client, "word").await?;
    //println!("{:?}", result.len());

    // Launching the web server that's going to listen to requests from the web backend and
    // crawlers. Currently only the backend queries are implemented.
    web_listener::launch_server().unwrap();

    Ok(())
}
