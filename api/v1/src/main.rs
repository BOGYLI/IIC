#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

use rocket::http::{Cookie, SameSite, CookieJar};
use rocket::response::{Flash, Redirect};

use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::{self, Form,  Error};
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

use rocket::Request;


use rocket::fs::TempFile;

pub mod db;
pub mod api;
pub mod utils;


/*#[get("/static/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static/").join(file);
    println!("{:?}", path.clone());
    NamedFile::open(path).await.ok()
}*/

#[get("/api/v1/<template>")]
async fn api_v1(template: String) -> Template {
    Template::render(format!("api/v1/{}", template), context! {
    })
}


#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {
    })
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/images/favicon.ico").await.ok()
}

#[get("/home")]
async fn home() -> Template {
    Template::render("home", context! {
    })
}

#[get("/news")]
async fn feat_v1_news() -> Template {
    Template::render("features/v1/news", context! {
    })
}

#[get("/navi")]
async fn feat_v1_navi() -> Template {
    Template::render("features/v1/navi", context! {
    })
}

#[get("/spiele")]
async fn feat_v1_spiele() -> Template {
    Template::render("features/v1/spiele", context! {
    })
}

#[get("/umfragen")]
async fn feat_v1_umfragen() -> Template {
    Template::render("features/v1/umfragen", context! {
    })
}

#[get("/geburtstag")]
async fn feat_v1_geburtstag() -> Template {
    Template::render("features/v1/geburtstag", context! {
    })
}


#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render("errors/404", context! {
        uri: req.uri()
    })
}
#[catch(500)]
fn internal(req: &Request) -> Template {
    Template::render("errors/500", context! {
        uri: req.uri()
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon, api_v1, home])
        .mount("/features/v1", routes![feat_v1_news, feat_v1_navi, feat_v1_spiele, feat_v1_umfragen, feat_v1_geburtstag])
        .mount("/api/v1/delete", routes![api::v1::del::umfrage, api::v1::del::medien, api::v1::del::template, api::v1::del::tparameter, api::v1::del::benutzer, api::v1::del::ufrage, api::v1::del::uantwort, api::v1::del::artikel])
        .mount("/api/v1/edit", routes![api::v1::edit::umfrageantwort, api::v1::edit::umfrage, api::v1::edit::uantwort, api::v1::edit::umfragebenutzer, api::v1::edit::ufrage, api::v1::edit::medien, api::v1::edit::artikel, api::v1::edit::artikelautor, api::v1::edit::benutzer, api::v1::edit::template, api::v1::edit::templatetparameter, api::v1::edit::tparameter])
        .mount("/api/v1/get_all", routes![api::v1::get_all::umfrageantwort, api::v1::get_all::umfrage, api::v1::get_all::uantwort, api::v1::get_all::umfragebenutzer, api::v1::get_all::ufrage, api::v1::get_all::medien, api::v1::get_all::artikel, api::v1::get_all::artikelautor, api::v1::get_all::benutzer, api::v1::get_all::template, api::v1::get_all::templatetparameter, api::v1::get_all::tparameter])
        .mount("/api/v1/get", routes![api::v1::get::umfrage, api::v1::get::medien, api::v1::get::template, api::v1::get::tparameter, api::v1::get::benutzer, api::v1::get::ufrage, api::v1::get::uantwort, api::v1::get::artikel])
        .mount("/api/v1/new", routes![api::v1::new::umfrageantwort, api::v1::new::umfrage, api::v1::new::uantwort, api::v1::new::umfragebenutzer, api::v1::new::ufrage, api::v1::new::medien, api::v1::new::artikel, api::v1::new::artikelautor, api::v1::new::benutzer, api::v1::new::template, api::v1::new::templatetparameter, api::v1::new::tparameter])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![not_found, internal])
        .attach(Template::fairing())
}
