#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket::request::Request;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
// use rocket::response::content::Json;

// Homepage of the website
#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", String::from("Jane"));

    Template::render("home", &context)
}

// Page, generated after user searches for something
// TODO: create a template for a case, when there are no search results
#[get("/search?<user_search>")]
fn search_page(user_search: &RawStr) -> Template {
    let mut context = HashMap::new();
    context.insert("title", String::from(user_search.url_decode().unwrap()));

    Template::render("home", &context)
}

// Catching some errors that might occur
// TODO: 1. create an html template for errors (with cat, of course)
//       2. serve cats on our server
#[catch(404)]
fn not_found(_req: &Request) -> Template {
    let mut context = HashMap::new();
    context.insert("error", String::from("Oh no! This is not a valid path ;=("));
    Template::render("error", &context)
}

#[catch(400)]
fn bad_request(_req: &Request) -> Template {
    let mut context = HashMap::new();
    context.insert("error", String::from("Oh no! A bad request was caught ;=("));
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
