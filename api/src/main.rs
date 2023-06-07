#[macro_use] extern crate rocket;
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

pub mod db;
pub mod api;
pub mod utils;
pub mod schema;
pub mod catchers;
pub mod admin;
pub mod user;
pub mod tests;

pub mod feedback;

use utils::cookies::*;

#[get("/")]
async fn start(perm: HTMLPermission) -> Template {
    Template::render("iframe", context! {
        init: "/feedback/runde1/idlescreen"
    })
}

#[get("/index")]
async fn index(perm: HTMLPermission) -> Template {
    Template::render("index", context! {
    })
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/images/favicon.ico").await.ok()
}

#[derive(Deserialize, FromForm)]
pub struct HTMLActivation<'r> {
    pin: &'r str
}
use rocket::request::Outcome;
#[post("/htmlactivation", data = "<data>")]
pub async fn htmlactivation(data: Form<HTMLActivation<'_>>, cookies: &CookieJar<'_>) -> Result<Redirect, Status> {
    match dotenvy::var("SCREENPIN") {
        Ok(screenpin) => {
            if screenpin == data.pin {
                cookies.remove_private(Cookie::named("html_access"));
                cookies.add_private(Cookie::new("html_access", "1"));
                return Ok(Redirect::to(uri!("/")));
            }
        }
        Err(_) => {}
    }
    Err(Status::Locked)
}

use std::env;
#[launch]
fn rocket() -> _ {
    //env::set_var("RUST_BACKTRACE", "full");
    dotenvy::dotenv().ok();
    dotenvy::var("USERNAME").expect("mebis-lib requires USERNAME - credential");
    dotenvy::var("PASSWORD").expect("mebis-lib requires PASSWORD - credential");
    dotenvy::var("URL").expect("mebis-lib requires URL - api");
    dotenvy::var("SCREENPIN").expect("SCREENPIN required for graphical authentication of trusted device");
    
    use rocket::config::{Config, TlsConfig, CipherSuite};
    let tls_config = TlsConfig::from_paths("certs/cert.pem", "certs/key.pem")
        .with_ciphers(CipherSuite::TLS_V13_SET)
        .with_preferred_server_cipher_order(true);
        
    use rocket::config::SecretKey;
    use std::net::Ipv4Addr;
    let tmp = dotenvy::var("SECRET_KEY").expect("rocket requires SECRET_KEY - cookies");
    let secret_key = tmp.as_bytes();
    //println!("{:?}", secret_key);
    let config = Config {
        tls: Some(tls_config),
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        secret_key: SecretKey::from(secret_key),
        ..Default::default()
    };
    
    rocket::custom(config)
        .mount("/", routes![start, index, favicon, htmlactivation])

        .mount("/user", routes![user::register, user::login_page, user::login_fail, user::login_post, user::whoami, user::whoami_redirect])
        .mount("/admin", routes![admin::dashboard])

        .mount("/tests", routes![tests::wordpress_post])

        .mount("/api/v1/delete", routes![api::del::umfrage, api::del::medien, api::del::template, api::del::tparameter, api::del::benutzer, api::del::ufrage, api::del::uantwort, api::del::artikel, api::del::sspiel, api::del::mspiel, api::del::team])
        .mount("/api/v1/edit", routes![api::edit::umfrageantwort, api::edit::umfrage, api::edit::uantwort, /*api::edit::umfragebenutzer,*/ api::edit::ufrage, api::edit::medien, api::edit::artikel, /*api::edit::artikelautor,*/ api::edit::benutzer, api::edit::template, /*api::edit::templatetparameter,*/ api::edit::tparameter, api::edit::sspiel, api::edit::mspiel, api::edit::sspieler, api::edit::mspieler, api::edit::team, /*api::edit::benutzerteam*/])
        .mount("/api/v1/get_all", routes![api::get_all::umfrageantwort, api::get_all::umfrage, api::get_all::uantwort, api::get_all::umfragebenutzer, api::get_all::ufrage, api::get_all::medien, api::get_all::artikel, api::get_all::artikelautor, api::get_all::benutzer, api::get_all::template, api::get_all::templatetparameter, api::get_all::tparameter, api::get_all::sspiel, api::get_all::mspiel, api::get_all::sspieler, api::get_all::mspieler, api::get_all::team, api::get_all::benutzerteam])
        .mount("/api/v1/get", routes![api::get::umfrage, api::get::medien, api::get::template, api::get::tparameter, api::get::benutzer, api::get::ufrage, api::get::uantwort, api::get::artikel, api::del::sspiel, api::del::mspiel, api::del::team])
        .mount("/api/v1/new", routes![api::new::umfrageantwort, api::new::umfrage, api::new::uantwort, api::new::umfragebenutzer, api::new::ufrage, api::new::medien, api::new::artikel, api::new::artikelautor, api::new::benutzer, api::new::template, api::new::templatetparameter, api::new::tparameter, api::new::sspiel, api::new::mspiel, api::new::sspieler, api::new::mspieler, api::new::team, api::new::benutzerteam])


        .mount("/feedback/runde1", routes![feedback::runde1::idlescreen, feedback::runde1::clickthebutton, feedback::runde1::tictactoe, feedback::runde1::games, feedback::runde1::karte, feedback::runde1::birthday, feedback::runde1::birthdaydemo, feedback::runde1::news, feedback::runde1::umfragen, feedback::runde1::umfrage_create, feedback::runde1::umfrage_view, feedback::runde1::umfrage_result, feedback::runde1::refresh])

        .mount("/static", FileServer::from("static"))
        .register("/", catchers![catchers::not_authorized, catchers::locked, catchers::not_found, catchers::internal])
        .attach(Template::custom(|engines|{
            use std::collections::BTreeMap;
            let url = BTreeMap::new();
            engines.tera.register_function("media_url", api::wordpress::media_url(url.clone()));
            engines.tera.register_function("format_date", api::wordpress::format_date(url.clone()))
        }))
}
