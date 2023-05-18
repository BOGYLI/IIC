use rocket_dyn_templates::{Template, context};
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

use rocket::http::{Cookie, SameSite, CookieJar, Status};
use rocket::response::{Flash, Redirect};

use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::{self, Form,  Error};
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

use rocket::Request;
use rocket::response::Responder;


use rocket::fs::TempFile;

#[get("/idlescreen")]
pub async fn idlescreen() -> Template {
    let posts = wp_lib::Post::get_from_uri_limited("https://bodensee-gymnasium.de", 3).unwrap();
    //let media = wp_lib::Media::from(posts[0].featured_media).guid.rendered;
    //let val = wp_lib::Media::from("https://bodensee-gymnasium.de", posts[0].clone().featured_media.to_string()).unwrap();
    //println!("{:?}", val);

    //TODO Template correct built but not showed as html in browser
    // add caching
    Template::render("tests/feedback/runde1/Idlescreen", context! {
        articles: posts
    })
}

#[get("/clickthebutton")]
pub async fn clickthebutton() -> Template {
    Template::render("tests/feedback/runde1/Clickthebutton", context! {
    })
}

#[get("/tictactoe")]
pub async fn tictactoe() -> Template {
    Template::render("tests/feedback/runde1/TicTacToe", context! {
    //Template::render("games/tictactoe", context! {
    })
}


#[get("/games")]
pub async fn games() -> Template {
    Template::render("tests/feedback/runde1/games", context! {
    })
}

#[get("/karte")]
pub async fn karte() -> Template {
    Template::render("tests/feedback/runde1/karte", context! {
    })
}

#[get("/birthday")]
pub async fn birthday() -> Template {
    Template::render("tests/feedback/runde1/birthday", context! {
    })
}

#[derive(FromForm)]
pub struct Birthday {
    wish: String,
    autoren: String,
}

#[post("/birthday/demo", data = "<data>")]
pub async fn birthdaydemo(data: Form<Birthday>) -> Template {
    let d = data.into_inner();
    Template::render("tests/feedback/runde1/birthdaydemo", context! {
        wish: d.wish.clone(),
        autoren: d.autoren.clone()
    })
}

#[get("/news")]
pub async fn news() -> Template {
    Template::render("tests/feedback/runde1/news", context! {
    })
}

#[get("/umfrage")]
pub async fn umfrage() -> Template {
    Template::render("tests/feedback/runde1/umfrage", context! {
    })
}