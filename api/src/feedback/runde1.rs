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

use crate::db::DBQueryable;
use crate::db::models::Umfrage;

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

#[get("/refresh")]
pub async fn refresh() -> Redirect {
    use cache_lib as cache;
    cache::post::refresh();
    Redirect::to(uri!("/feedback/runde1/idlescreen"))
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

#[get("/umfragen")]
pub async fn umfragen() -> Template {
    let umfragen: Vec<String> = vec![];
    Template::render("tests/feedback/runde1/umfragen", context! {
        umfragen: umfragen
    })
}

#[get("/umfrage/create")]
pub async fn umfrage_create() -> Template {
    match Umfrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => {
            Template::render("tests/feedback/runde1/umfrage/create", context! {
                umfragen: data
            })
        },
		Err(_) => {
            let data: Vec<Umfrage> = vec![];
            Template::render("tests/feedback/runde1/umfrage/create", context! {
                umfragen: data
            })
        }
	}
    
}

#[get("/umfrage/<id>")]
pub async fn umfrage_view(id: i64) -> Template {
    Template::render("tests/feedback/runde1/umfrage/view", context! {
    })
}

#[get("/umfrage/result/<id>")]
pub async fn umfrage_result(id: i64) -> Template {
    Template::render("tests/feedback/runde1/umfrage/result", context! {
    })
}