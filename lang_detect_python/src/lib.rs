use reqwest::Client;
use serde::{Deserialize, Serialize};

/// A struct that holds the text query we send to the language detection server
#[derive(Serialize)]
struct Text<'a> {
    text: &'a str,
}

/// A struct that holds the response we get from the language detection server
#[derive(Deserialize, Debug)]
struct Response {
    response: Vec<(String, f64)>,
}

/// Send the text as a query to the language detection server, returns a response
/// from it if all went fine and a `reqwest::Error` otherwise
pub async fn send_lang_detection_query(text: &str) -> Result<Vec<(String, f64)>, ()> {
    let map = Text { text };

    // Send the request to the language detection server
    let client = Client::new();
    let res = client
        .post("http://0.0.0.0:5000/detect")
        .json(&map)
        .send()
        .await
        .unwrap();

    // If it was successful, parse it and return it
    if res.status().is_success() {
        let json_res: Response = res.json().await.unwrap();
        println!("{:?}", json_res.response);
        // {'response': [['nl', 0.7142824916142885], ['de', 0.28571299164119035]]}
        return Ok(json_res.response);
    };

    // If it was not successful, return an Err
    println!("Received response status: {:?}", res.status().as_u16());
    Err(())
}
