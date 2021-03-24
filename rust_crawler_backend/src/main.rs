use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;

// Module that currently is supposed to read data from file, and push it to database somehow (NOT IMPLEMENTED YET)
// Later, this will be fully-functional backend for crawlers to identify the language and talk to db

mod lib; // MODULE THAT TALKS TO PYTHON LANGDETECT

// Struct to better represent a single Website as json. It has a primitive for now.
// FOR FUTURE: add vector of websites, vector of different languages?, and their probabilities
#[derive(Serialize, Deserialize)]
struct Website {
    url: String,
    text: String,
    language: String,
}


fn main() {
    file_to_db();
}

// Function that parses file line by line, and adds url and text to database(DB NOT IMPLEMENTED YET)
#[tokio::main]
async fn file_to_db() {

    // Reading from pre-made file with data line by line
    if let Ok(lines) = read_lines("./data/100_lines_collected_data.txt") {
        let mut website: Website = Website{
            url: String::from("sample data"),
            text: String::from("sample data"),
            language: String::from("sample data"),
        };

        for (index, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if index % 3 == 0 { // Parsing a vector of links, divided by space
                    // First link=this page url. All subsequent
                    let link_vector: Vec<&str> = ip.split(' ').collect::<Vec<&str>>(); // we don't need any memory efficiency here, it is all temporary

                    // Getting only the link of the page, for now
                    website.url = link_vector[0].to_owned(); // cloning data?
                }
                if index % 3 == 1 { // if it is a website text
                    website.text = ip;

                    let languages :Vec<(String, f64)> = lib::send_lang_detection_query(&website.text).await.unwrap();
                    // saving only a dominant language(with the highest probability) to struct
                    website.language = languages[0].0.to_owned();

                    // Creating a json to push to database
                    let website_json = json!({
                        "url": website.url,
                        "text": website.text,
                        "language": website.language
                    });

                    println!("{}", website_json.to_string());

                    // HERE, WE ARE READY TO SEND THIS JSON TO DB
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
