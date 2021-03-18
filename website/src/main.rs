#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

// use rocket::Request;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::http::RawStr;
// use rocket::request::FromParam;
// use rocket::http::{Cookie, CookieJar};
// use async_trait::async_trait;

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

impl<'a, 'r> FromRequest<'a, 'r> for UserSearch {
    type Error = UserSearchError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let result: String = request.headers().get("user_search");
        println!("Hello, {:?}!", result);
        // request.cookies()
        //         .get_private("user_id")
        //         .and_then(|cookie| cookie.value().parse().ok())
        //         .and_then(|id| db.get_user(id).ok())
        Outcome::Success(UserSearch("yes".to_string()))
    }
}

// use serde::Serialize;

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


#[get("/")]
fn index(user_search: UserSearch) -> Template {
    println!("Hello, {:?}!", user_search);
    let mut context = HashMap::new();
    context.insert("title", String::from("Jane"));

    Template::render("home", &context)
}

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
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index])
        .mount("/api", routes![hello])
        .attach(Template::fairing())
        .launch();
}
