pub use elasticsearch::{
    http::{response::Response, StatusCode},
    Elasticsearch, IndexParts, SearchParts,
};
use serde_json::Value;

/// Establishes a database connection. Should be called by a thread before doing anything else
/// with the database.
pub fn establish_database_connection() -> Elasticsearch {
    let client = Elasticsearch::default();
    client
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
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let response = client
        .search(SearchParts::Index(&["english"]))
        .from(0)
        .size(10)
        .body(body)
        .send()
        .await?;
    let mut response = response.json::<Value>().await?;

    let hits: Vec<Value> = response["hits"]["hits"]
        .as_array_mut()
        .unwrap()
        .drain(..)
        .map(|x| x["_source"].clone())
        .collect();
    Ok(hits)
}
