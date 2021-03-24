use elasticsearch::{
    http::{headers::HeaderMap, request::JsonBody, Method, StatusCode},
    CreateParts, Elasticsearch, SearchParts,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body =
        JsonBody::new(json!({"url": "site.com", "text": "exampletext", "urls": ["url1", "url2"]}));

    // Creating a stupid elasticsearch client and putting a json in there. Not sure if works 100%
    // It's not that elasticsearch is stupid it's just that we should create an asynchronous
    // connection pool instead
    let client = Elasticsearch::default();

    // Create an index. Returns 200 if it did create an index and 409 if the index was already
    // there.
    let a = client
        .send(
            Method::Put,
            CreateParts::IndexId("english", "1").url().as_ref(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body),
            None,
        )
        .await?;

    match a.status_code() {
        StatusCode::CONFLICT | StatusCode::OK => (),
        _ => panic!("Something went wrong idk"),
    }

    // Searches the index for a "text" string. At least it should
    let response = client
        .search(SearchParts::Index(&["english"]))
        .from(0)
        .size(10)
        .body(json!({
            "query": {
                "match": {
                    "message": "text"
                }
            }
        }))
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    println!("{:?}", response_body);

    Ok(())
}
