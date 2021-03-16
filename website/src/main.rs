#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket::response::content::Json;
use rocket::request::{self, FromRequest, Request};
use rocket::request::Form;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::http::{Cookie, CookieJar};
// use async_trait::async_trait;

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

#[derive(Debug)]
struct SearchRequest(String);

// #[rocket::async_trait]
impl<'a,'r> FromRequest<'a,'r> for SearchRequest {
    type Error = std::convert::Infallible;

    //TODO: async
    fn from_request(request: &'r Request<'_>) -> request::Outcome<SearchRequest, Self::Error> {
        request.cookies()
            .get_private("user_search")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| SearchRequest(id))
            .or_forward(())
    }
}

#[get("/")]
fn index(search_request: SearchRequest) -> Template {
    let mut context = HashMap::new();
    context.insert("title", search_request.0);
    // #[derive(Serialize)]
    // struct Context {
    //   first_name: String,
    //   last_name: String
    // }
    //
    // let context = Context {
    //   first_name: String::from("Jane"),
    //   last_name: String::from("Doe")
    // };

    Template::render("home", &context)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[catch(404)]
fn not_found(_req: &Request) -> String {
    format!("Oh no! This is not a valid path ;=(")
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
