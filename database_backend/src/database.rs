use std::collections::HashMap;

pub use elasticsearch::{
    http::{response::Response, transport::Transport, StatusCode},
    Elasticsearch, IndexParts, SearchParts,
};
use serde_json::Value;

/// Establishes a database connection. Should be called by a thread before doing anything else
/// with the database.
pub fn establish_database_connection() -> Elasticsearch {
    // TODO: Implement a smarter retry system
    loop {
        match Transport::single_node("http://elasticsearch:9200") {
            Ok(transport) => {
                let client = Elasticsearch::new(transport);
                println!("Successfuly connected to the database, yay!");
                return client;
            }
            Err(_) => {
                println!("Failed to connect to the database, retrying in 500 msec");
                std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }
        }
    }
    //let client = Elasticsearch::default();
    //client
}

/// Create an index. Returns 200 if it did create an index and 409 if the index was already
/// there.
pub async fn send_scrapped_website(
    client: &Elasticsearch,
    body: Value,
) -> Result<Response, Box<dyn std::error::Error>> {
    let response = client
        .index(IndexParts::IndexId("english", "1"))
        .body(body)
        .send()
        .await?;
    Ok(response)
}

/// Searches the index and returns parsed top 10 results starting from the 0th one.
/// Change this for your backend queries if you need to.
/// TODO: Also take starting elements and size of the list as parameters, we can probably implement
/// web paging this way.
pub async fn get_search(
    client: &Elasticsearch,
    body: Value,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    // Sends a search request to elasticsearch and gets top 10 values.
    // TODO: Start taking from and size parameters
    // TODO: Figure out whether we need to sort this, for pagination see:
    // https://www.elastic.co/guide/en/elasticsearch/reference/current/paginate-search-results.html#search-after
    let response = client
        .search(SearchParts::Index(&["english"]))
        .from(0)
        .size(10)
        .body(body)
        .send()
        .await?;
    let mut response = response.json::<Value>().await?;

    // Parses out the json we receive from Elasticsearch. Will definitely change in the future
    let hits: Vec<HashMap<String, String>> = response["hits"]["hits"]
        .as_array_mut()
        .unwrap()
        .drain(..)
        .map(|x| {
            let mut a = HashMap::new();
            a.insert(
                "description".to_string(),
                x["_source"]["full_text"]
                    .as_str()
                    .unwrap_or("")
                    .chars()
                    .take(215)
                    .collect(),
            );
            a.insert(
                "url".to_string(),
                x["_source"]["url"].as_str().unwrap_or("").to_string(),
            );
            a.insert(
                "title".to_string(),
                x["_source"]["title"].as_str().unwrap_or("").to_string(),
            );
            a
        })
        .collect();

    Ok(hits)
}
