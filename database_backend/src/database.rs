use std::time::Duration;

pub use elasticsearch::{
    http::{response::Response, StatusCode},
    Elasticsearch, IndexParts, SearchParts,
};
use serde_json::{json, Value};

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

/// Searches the index for a "text" string. You can also search in "url" columns etc.
/// This returns top 10 results starting from the 0th one. Change this for your backend queries
/// if you need to.
pub async fn get_search(
    client: &Elasticsearch,
    body: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let response = client
        .search(SearchParts::Index(&["english"]))
        .from(0)
        .size(10)
        .body(body)
        .send()
        .await?;
    let response = response.json::<Value>().await?;
    Ok(response)
}

pub fn parse_search(response_body: &Value) -> Vec<&Value> {
    let hits: Vec<&Value> = response_body["hits"]["hits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| &x["_source"])
        .collect();
    hits
}

pub fn establish_database_connection() -> Elasticsearch {
    let client = Elasticsearch::default();
    client
}

#[tokio::main]
async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a stupid elasticsearch client and putting a json in there. Not sure if works 100%
    // It's not that elasticsearch is stupid it's just that we should create an asynchronous
    // connection pool instead
    let client = Elasticsearch::default();

    // Send along a scrapped website result in a json
    let body = json!({"url": "site.com", "fulltext": "exampletext with a mention of ukraine", "urls": ["url1", "url2"]});
    let res = send_scrapped_website(&client, body).await?;

    // See if it was successfully sent, if not we should probably retry?
    match res.status_code() {
        StatusCode::OK => {
            println!("Everything went fine");
        }
        _ => panic!("Something went wrong idk"),
    }

    // Sleep for a while because elasticsearch spends some time indexing the insertions and
    // we want to see a successful search later
    std::thread::sleep(Duration::from_millis(100));

    // Form a search query and send it along
    let search_query = json!({
        "query": {
            "match": {
                "fulltext": "ukraine"
            }
        }
    });
    let search_result = get_search(&client, search_query).await?;

    // Prints out the resulting search hits as an array like this:
    // [Object({"fulltext": String("exampletext with a mention of ukraine"),
    // "url": String("site.com"), "urls": Array([String("url1"), String("url2")])})]
    println!("{:?}", parse_search(&search_result));

    Ok(())
}
