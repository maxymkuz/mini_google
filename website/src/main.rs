#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use std::collections::HashMap;

use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, CookieJar};
use rocket::form::Form;

use rocket_contrib::templates::Template;

#[derive(FromForm)]
struct Login {
    username: String,
    password: String
}

#[derive(Debug)]
struct User(usize);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}

#[post("/login", data = "<login>")]
fn login(cookies: &CookieJar<'_>, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    if login.username == "Sergio" && login.password == "password" {
        cookies.add_private(Cookie::new("user_id", 1.to_string()));
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username/password."))
    }
}

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[get("/login")]
fn login_user(_user: User) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage<'_>>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
        if msg.name() == "error" {
            context.insert("flash_type", "Error");
        }
    }

    Template::render("login", &context)
}

#[get("/")]
fn user_index(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("user_id", user.0);
    Template::render("index", &context)
}

#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to(uri!(login_page))
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, user_index, login, logout, login_user, login_page])
}
