use reqwest::Client;
use reqwest::StatusCode;

let mut map = HashMap::new();
map.insert("text", "Ich heise Hello!, Dungeon Master is dominating");

let client = reqwest::Client::new();
let res = client.post("http://lang_detect_python:5001/detect")
    .json(&map)
    .send()
    .await?;

match res.status() {
    StatusCode:: OK => {
        println!("niceee");
        // parse json here
        // response.json().await?; // Якось так, перепиши як там правильно і розпарси його
        // прилітає така штука
        // {'response': [['nl', 0.7142824916142885], ['de', 0.28571299164119035]]}

    },
    s -> println!("Received response status: {:?}", s),
}