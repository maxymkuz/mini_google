#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
// extern crate serde_derive;
// extern crate serde;
// extern crate serde_json;

use reqwest::blocking::Client;
use rocket::{request::Request, State};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Data structure for json objects
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Result {
    title: String,
    url: String,
    description: String,
}

// Homepage of the website
#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("home", &context)
}

// Page, generated after user searches for something
// TODO: create a template for a case, when there are no search results
#[get("/search?<user_search>")]
fn search_page(client: State<Client>, user_search: String) -> Template {
    // TODO: here should be language detection
    let mut json_to_send = HashMap::new();
    // Create an empty context
    let mut context = HashMap::new();

    // Adding search string to the resulting page
    let mut new_str = String::new();
    new_str.clone_from(&user_search);
    let mut additional_data = HashMap::new();
    additional_data.insert(String::from("query"), new_str);
    let mut vector_additional_data = Vec::new();
    vector_additional_data.insert(0, additional_data);
    context.insert("additional_data", &vector_additional_data);

    json_to_send.insert("text", &user_search);

    // Send a request to the database backend (currently running locally)
    // If something goes wrong, we just return an empty template
    let results = match client
        .post("http://127.0.0.1:8080/search")
        .json(&json_to_send)
        .send()
    {
        Ok(x) => x,
        Err(_) => return Template::render("empty_search", &context),
    };

    let results: Vec<HashMap<String, String>> = match results.json() {
        Ok(x) => x,
        Err(_) => return Template::render("empty_search", &context),
    };

    // Don't mind this, this is for testing locally
    // let mut test1 = HashMap::new();
    // test1.insert(String::from("title"), String::from("AAAaaaaAAAAAA"));
    // test1.insert(String::from("description"), String::from("bla bla"));
    // test1.insert(String::from("url"), String::from("url ... bla bla"));
    // let mut test_results = Vec::new();
    // test_results.insert(0, test1);

    // TODO: add pagination
    // Displaying results on the website page
    context.insert("results", &results);
    Template::render("search", &context)
}

// Catching some errors that might occur
#[catch(404)]
fn not_found(_req: &Request) -> Template {
    let mut context = HashMap::new();
    context.insert("error", String::from("This is not a valid path ;=("));
    Template::render("error", &context)
}

#[catch(400)]
fn bad_request(_req: &Request) -> Template {
    let mut context = HashMap::new();
    context.insert("error", String::from("A bad request was caught ;=("));
    Template::render("error", &context)
}

fn main() {
    // Initializing a reqwest client for calls to the database backend
    let client = Client::new();

    rocket::ignite()
        .manage(client)
        .register(catchers![not_found, bad_request])
        .mount("/", routes![index, search_page])
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(Template::fairing())
        .launch();
}
