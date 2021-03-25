use elasticsearch::Elasticsearch;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

// Module that currently is supposed to read data from file, and push it to database somehow (NOT IMPLEMENTED YET)
// Later, this will be fully-functional backend for crawlers to identify the language and talk to db

mod database; // MODULE THAT HANDLES THE ACTUAL DATABASE QUERIES
mod lang_detect; // MODULE THAT TALKS TO PYTHON LANGDETECT
mod web_listener;

// Struct to better represent a single Website as json. It's primitive for now
// FOR FUTURE: add vector of websites, vector of different languages?, and their probabilities
#[derive(Serialize, Deserialize)]
struct CrawledWebsite {
    url: String,
    full_text: String,
    language: String,
    // urls: Vec<String>, // uncomment later
}

// we will use this func later in crawlers, so it has to be separate. Possibly make lang detection here later
#[tokio::main]
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

//  Andrew Розкоментуй і пофікси лайфтайми
// // Func to get a response from db, according to user querry
// #[tokio::main]
// async fn get_response<'a>(query: &'a str, client: &'a Elasticsearch) -> Result<Vec<&'a Value>, Box<dyn std::error::Error>> { // Andriy глянь за лайфтайми
//     // Search example query
//     // We are looking in the column 'fulltext' for certain text pattern
//     let search_query = serde_json::json!({
//         "query": {
//             "match": {
//                 "language": query.to_string()
//             }
//         }
//     });
//     let search_result  = database::get_search(&client, search_query).await?;
//     let result = database::parse_search(&search_result);
//     println!("{:?}", result.len());
//     Ok(result.clone())
// }

// Function that parses file line by line, and inserts url, text and language to the database
// This is basically a mock for the request from a crawler until we figure that out
// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creating future json
    // let mut url = String::new();
    // let mut full_text = String::new();
    // let language = "en";
    // let mut urls: Vec<String> = Vec::new();
    let mut website = CrawledWebsite {
        url: "sample_url".to_string(),
        full_text: "sample_text".to_string(),
        language: "sample_lan".to_string(),
    };

    // Establish a database connection
    let client = database::establish_database_connection();

    // Reading from pre-made file with data line by line
    let input = File::open("./data/100_lines_collected_data.txt")?;
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
                struct_to_db(&website, &client)?; // Andriy handle
            }
        }
    }

    // Wait so that Elasticsearch will have time to index all of this. This is just so you'd be
    // able to get a valid result down below.
    //
    // I am not sure what the problem is, but it seems to be indexing the actual fulltext for a bit
    // longer........
    //std::thread::sleep(std::time::Duration::from_millis(10000));

    // get_response("en", &client);  //  Andrew Розкоментуй і пофікси лайфтайми
    // Search example query
    // We are looking in the column 'fulltext' for certain text pattern
    let search_query = serde_json::json!({
        "query": {
            "match": {
                "language": "english"
            }
        }
    });
    let search_result = database::get_search(&client, search_query).await?;
    let result = database::parse_search(&search_result);
    println!("{:?}", result.len());

    web_listener::launch_server();

    Ok(())
}
