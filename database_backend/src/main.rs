//! Module that currently is supposed to read data from file, and push it to database.
//! Later, this will be fully-functional backend for crawlers to identify the language and talk to db
use elasticsearch::{http::request::JsonBody, BulkParts, Elasticsearch};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, io::BufRead};
use std::{fs::File, io::BufReader};

mod database; // MODULE THAT HANDLES THE ACTUAL DATABASE QUERIES
mod lang_detect; // MODULE THAT TALKS TO PYTHON LANGDETECT
mod web_listener; // MODULE THAT LAUNCHES THE WEB SERVER AND LISTENS TO WEB BACKEND AND CRAWLER QUERIES

// TODO: add vector of websites, vector of different languages?, and their probabilities
#[derive(Serialize, Deserialize)]
struct CrawledWebsite {
    url: String,
    title: String,
    full_text: String,
    language: String,
    // urls: Vec<String>, // TODO: uncomment later when we will implement it too
}

/// Sending the crawled websites to the database. Does this in bulk and is usable for large
/// files
async fn struct_to_db(client: &Elasticsearch, body: Vec<JsonBody<Value>>) -> Result<(), ()> {
    // TODO: Implement proper language detection, error handling here etc.
    //let languages: Vec<(String, f64)> =
    //lang_detect::send_lang_detection_query(&website.text)
    //.await
    //.unwrap();
    // saving only a dominant language(with the highest probability) to struct
    //language = languages[0].0.to_owned();

    let response = match client
        .bulk(BulkParts::Index("english"))
        .body(body)
        .send()
        .await
    {
        Ok(x) => x,
        _ => return Err(()),
    };

    // Creating a json and pushing to database:
    match response.status_code() {
        StatusCode::OK => return Ok(()),
        _ => return Err(()),
    }
}

// Function that parses file line by line, and inserts url, text and language to the database
// This is basically a mock for the request from a crawler until we figure that out
// It also launches the web listener that's handling crawlers and backend requests.
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Success!!");
    let filename = std::env::args()
        .nth(1)
        .unwrap_or("./src/collected.txt".to_string()); // ---------------------------
                                                      // .unwrap_or("./data/collected.txt".to_string());  // ---------------------------
                                                      // Creating future json
    let mut website = CrawledWebsite {
        url: "sample_url".to_string(),
        title: "sample_title".to_string(),
        full_text: "sample_text".to_string(),
        language: "sample_lan".to_string(),
    };

    // Establish a database connection
    let client = database::establish_database_connection();

    // Reading from pre-made file with data line by line
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    // Collecting requests to the Elasticsearch database
    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(10000);

    for (index, line) in buffered.lines().enumerate() {
        if let Ok(line) = line {
            if index % 3 == 0 {
                website.url = line;
            } else if index % 3 == 1 {
                website.title = line;
            } else if index % 3 == 2 {
                // Getting the full text of the website
                website.full_text = line.to_string();

                // Sending the data stored in struct website to database
                body.push(json!({"index": {"_id": (index/3).to_string()}}).into());
                body.push(json!(website).into());
            }
        }

        if body.len() == 5000 {
            println!("Pushed 5000 websites into Elastic");
            struct_to_db(&client, body)
                .await
                .expect("Couldn't bulk index the file");
            body = Vec::with_capacity(5000);
        }
    }

    println!("Put the file in the database!");

    // Just an example launch
    //let result = get_response(&client, "word").await?;
    //println!("{:?}", result.len());

    // Launching the web server that's going to listen to requests from the web backend and
    // crawlers. Currently only the backend queries are implemented.
    println!("Listening for queries and inserts!");
    web_listener::launch_server().await?;

    Ok(())
}

// Func to get a response from db, according to user query
async fn get_response<'a>(
    client: &'a Elasticsearch,
    query: &'a str,
) -> Result<Vec<HashMap<std::string::String, std::string::String>>, Box<dyn std::error::Error>> {
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
