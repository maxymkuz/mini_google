#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
// extern crate serde_derive;
// extern crate serde;
// extern crate serde_json;

use rocket::http::RawStr;
use rocket::request::Request;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::File;

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
    let context: HashMap<&str, String> = HashMap::new();
    Template::render("home", &context)
}

// Page, generated after user searches for something
// TODO: create a template for a case, when there are no search results
#[get("/search?<user_search>")]
fn search_page(user_search: &RawStr) -> Template {

    // TODO: here should be language detection
    // TODO: here should be database connection and request

    // Parsing json results
    let json_file_path = Path::new("test_data.json");
    let file = File::open(json_file_path).expect("file not found");
    let results: Vec<Result> = serde_json::from_reader(file)
        .expect("error while reading or parsing");

    let mut context = HashMap::new();
    if results.is_empty() {
       return Template::render("empty_search", &context)
    }

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
    rocket::ignite()
        .register(catchers![not_found, bad_request])
        .mount("/", routes![index, search_page])
        .mount("/public", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .attach(Template::fairing())
        .launch();
}
