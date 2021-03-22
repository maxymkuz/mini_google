#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

// use rocket::Request;
use rocket::config::{Config, Environment};
use rocket::http::{RawStr, Status};
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::content::Json;
use rocket::Outcome;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
// use rocket::request::FromParam;
// use rocket::http::{Cookie, CookieJar};
// use async_trait::async_trait;
// use serde::Serialize;

#[derive(Debug)]
struct UserSearch(String);

// /// Returns true if `key` is a valid API key string.
// fn is_valid(key: &str) -> bool {
//     key == "valid_api_key"
// }
//
#[derive(Debug)]
enum UserSearchError {
    No,
}

// A.S.: Why do we need to implement all of this stuff (which also does not compile)
// if user's search is only a String, which has a default FromRequest implementation?
// Are we planning to make UserSearch something other than a String?
impl<'a, 'r> FromRequest<'a, 'r> for UserSearch {
    type Error = UserSearchError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // A.S.: .get(value) returns an iterator over this header's values, not a string
        let result: String = request
            .headers()
            .get("user_search")
            .nth(0)
            .unwrap()
            .to_string();
        println!("Hello, {:?}!", result);
        // request.cookies()
        //         .get_private("user_id")
        //         .and_then(|cookie| cookie.value().parse().ok())
        //         .and_then(|id| db.get_user(id).ok())
        Outcome::Success(UserSearch("yes".to_string()))
    }
}

// #[derive(FromForm, Debug)]
// struct Book {
//   title: String,
//   author: String,
//   isbn: String
// }

// #[derive(FromForm, Debug)]
// struct SearchRequest {
//     title: String,
//     // url: String,
//     // description: String
// }

// #[derive(Debug)]
// struct SearchRequest(String);
//
// // #[rocket::async_trait]
// impl<'a,'r> FromRequest<'a,'r> for SearchRequest {
//     type Error = std::convert::Infallible;
//
//     //TODO: async
//     fn from_request(request: &'r Request<'_>) -> request::Outcome<SearchRequest, Self::Error> {
//         request.cookies()
//             .get_private("user_search")
//             .and_then(|cookie| cookie.value().parse().ok())
//             .map(|id| SearchRequest(id))
//             .or_forward(())
//     }
// }

// A.S. It's really weird that you take UserSearch as a necessary argument on the homepage.
// Isn't it supposed to be an invitation for the user to input the query?
// I've removed it temporarily, this way you can actually access the homepage
#[get("/")]
//fn index(user_search: UserSearch) -> Template {
fn index() -> Template {
    //println!("Hello, {:?}!", user_search);
    let mut context = HashMap::new();
    context.insert("title", String::from("Jane"));

    Template::render("home", &context)
}

// A.S.: I don't really understand how this is supposed to work. From what I've read, name here
// acts as a Request Guard, not as a normal query. Maybe you actually want this:
// https://rocket.rs/v0.4/guide/requests/#query-strings ?
#[get("/hello")]
fn hello(name: UserSearch) -> String {
    format!("Hello, {:?}!", name)
    //   Json("{
    //   'status': 'success',
    //   'message': 'Hello API!'
    // }")
}

#[catch(404)]
fn not_found(_req: &Request) -> String {
    format!("Oh no! This is not a valid path ;=(")
}

#[catch(400)]
fn bad_request(_req: &Request) -> String {
    format!("Oh no! A bad request was caught ;=(")
}

// #[post("/book", data = "<book_form>")]
// fn new_book(book_form: Form<Book>) -> String {
//   let book: Book = book_form.into_inner();
//   let mut dummy_db: Vec<Book> = Vec::new();
//   dummy_db.push(book);
//
//   format!("Book added successfully: {:?}", dummy_db)
// }

fn main() {
    let config = Config::build(Environment::Staging)
    .address("0.0.0.0")
    .port(5000)
    .finalize()?;

    let app = rocket::custom(config);

    app.register(catchers![not_found])
        .mount("/", routes![index])
        .mount("/api", routes![hello])
        .attach(Template::fairing())
        .launch();
}
