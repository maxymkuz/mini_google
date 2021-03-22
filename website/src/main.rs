#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::config::{Config, Environment};
use rocket::http::{RawStr, Status};
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::content::Json;
use rocket::Outcome;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

// A.S. It's really weird that you take UserSearch as a necessary argument on the homepage.
// Isn't it supposed to be an invitation for the user to input the query?
// I've removed it temporarily, this way you can actually access the homepage
#[get("/")]
fn index() -> Template {
    //println!("Hello, {:?}!", user_search);
    let mut context = HashMap::new();
    context.insert("title", String::from("Jane"));

    Template::render("home", &context)
}

#[get("/search?<user_search>")]
fn search_page(user_search: &RawStr) -> Template {
    let mut context = HashMap::new();
    context.insert("title", String::from(user_search.url_decode().unwrap()));

    Template::render("home", &context)
}

// A.S.: I don't really understand how this is supposed to work. From what I've read, name here
// acts as a Request Guard, not as a normal query. Maybe you actually want this:
// https://rocket.rs/v0.4/guide/requests/#query-strings ?
#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name)
}

#[catch(404)]
fn not_found(_req: &Request) -> String {
    format!("Oh no! This is not a valid path ;=(")
}

#[catch(400)]
fn bad_request(_req: &Request) -> String {
    format!("Oh no! A bad request was caught ;=(")
}


fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index, search_page])
        .mount("/api", routes![hello])
        .attach(Template::fairing())
        .launch();
}
