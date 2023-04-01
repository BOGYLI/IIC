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


use rocket::fs::TempFile;

pub mod db;
pub mod api;
pub mod utils;
pub mod schema;

use utils::v1::cookies::*;

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



// PLACEHOLDERS
#[get("/login")]
async fn login_page() -> Template {
    Template::render("placeholders/login", context! {
    })
}
#[get("/loginfail", rank= 2)]
async fn login_fail() -> Template {
    Template::render("placeholders/loginfail", context! {
    })
}

use crate::db::models::SSpiel;
#[get("/sspiel")]
async fn sspiel() -> Result<Template, Status> {
    match SSpiel::get_all(&mut crate::db::v1::establish_connection()) {
        Ok(games) => {
            Ok(Template::render("placeholders/games", context! {
                games: games
            }))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

use crate::utils::v1::DBQueryableUtils;
use crate::db::v1::DBQueryable;
use rocket::serde::json::Json;

#[derive(Deserialize, FromForm)]
struct Login<'r> {
    vorname: &'r str,
    nachname: &'r str,
    passwort: &'r str,
}
#[post("/login", data = "<data>")]
async fn login(data: Form<Login<'_>>, cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    match Benutzer::authenticate(data.vorname, data.nachname, data.passwort, &mut crate::db::v1::establish_connection()) {
        Some(benutzer) => {
            cookies.remove_private(Cookie::named("user_id"));
            cookies.add_private(Cookie::new("user_id", benutzer.id.to_string()));
            Ok(Template::render("placeholders/loggedin", context! {
            }))
        }
        None => {
            Err(Redirect::to(uri!(login_fail)))
        }
    }
}

use crate::db::models::Benutzer;
#[get("/whoami")]
async fn whoami(user: Benutzer) -> Template {
    Template::render("placeholders/whoami", context! {
        vorname: user.vorname,
        nachname: user.nachname,
        klasse: user.klasse,
        rolle: user.rolle
    })
}
#[get("/whoami", rank = 2)]
async fn whoami2() -> Redirect {
    Redirect::to(uri!(login_page))
}

#[get("/wordpress/<index>")]
async fn wordpress_post(index: usize) -> Template {
    let posts = wp_lib::Post::get_from_uri("https://bodensee-gymnasium.de").unwrap();
    Template::render("placeholders/wordpress", context! {
        title: posts[index].title.rendered.clone(),
        content: posts[index].content.rendered.clone()
    })
}

#[get("/placeholders/<file>")]
async fn placeholders(file: String) -> Template {
    Template::render(format!("placeholders/{}",file), context!{})
}


#[get("/register")]
async fn register() -> Template {
    Template::render("placeholders/register", context! {
        klassen: vec!["5a", "Q11"],
        rollen: vec!["Schueler", "Lehrer"]
    })
}


#[get("/clickthebutton")]
async fn feat_v1_clickthebutton() -> Template {
    Template::render("features/v1/clickthebutton", context! {
        //alltimecount: ClickTheButton:get_all()
    })
}


#[get("/lehrer")]
async fn lehrer(rolle: Lehrer) -> Template {
    Template::render("placeholders/lehrer", context! {
    })
}

#[get("/lehrer", rank = 2)]
async fn keinlehrer(user: Benutzer) -> Template {
    Template::render("placeholders/wrong_role", context! {
        rolle: user.rolle
    })
}

#[get("/schueler", rank = 2)]
async fn schueler(user: Benutzer) -> Template {
    Template::render("placeholders/wrong_role", context! {
        rolle: user.rolle
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

use std::env;
#[launch]
fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "full");
    dotenvy::var("USERNAME").expect("mebis-lib requires USERNAME - credential");
    dotenvy::var("PASSWORD").expect("mebis-lib requires PASSWORD - credential");
    dotenvy::var("URL").expect("mebis-lib requires URL - api");
    
    use rocket::config::{Config, TlsConfig, CipherSuite};
    let tls_config = TlsConfig::from_paths("certs/cert.pem", "certs/key.pem")
        .with_ciphers(CipherSuite::TLS_V13_SET)
        .with_preferred_server_cipher_order(true);

    let config = Config {
        tls: Some(tls_config),
        ..Default::default()
    };
    
    rocket::custom(config)
        .mount("/", routes![index, favicon, api_v1, home, wordpress_post, login_page, login_fail, login, register, whoami, whoami2, placeholders, sspiel, lehrer, keinlehrer, schueler])
        .mount("/features/v1", routes![feat_v1_news, feat_v1_navi, feat_v1_spiele, feat_v1_umfragen, feat_v1_geburtstag])
        .mount("/api/v1/delete", routes![api::v1::del::umfrage, api::v1::del::medien, api::v1::del::template, api::v1::del::tparameter, api::v1::del::benutzer, api::v1::del::ufrage, api::v1::del::uantwort, api::v1::del::artikel, api::v1::del::sspiel, api::v1::del::mspiel, api::v1::del::team])
        .mount("/api/v1/edit", routes![api::v1::edit::umfrageantwort, api::v1::edit::umfrage, api::v1::edit::uantwort, api::v1::edit::umfragebenutzer, api::v1::edit::ufrage, api::v1::edit::medien, api::v1::edit::artikel, api::v1::edit::artikelautor, api::v1::edit::benutzer, api::v1::edit::template, api::v1::edit::templatetparameter, api::v1::edit::tparameter, api::v1::edit::sspiel, api::v1::edit::mspiel, api::v1::edit::sspieler, api::v1::edit::mspieler, api::v1::edit::team, api::v1::edit::benutzerteam])
        .mount("/api/v1/get_all", routes![api::v1::get_all::umfrageantwort, api::v1::get_all::umfrage, api::v1::get_all::uantwort, api::v1::get_all::umfragebenutzer, api::v1::get_all::ufrage, api::v1::get_all::medien, api::v1::get_all::artikel, api::v1::get_all::artikelautor, api::v1::get_all::benutzer, api::v1::get_all::template, api::v1::get_all::templatetparameter, api::v1::get_all::tparameter, api::v1::get_all::sspiel, api::v1::get_all::mspiel, api::v1::get_all::sspieler, api::v1::get_all::mspieler, api::v1::get_all::team, api::v1::get_all::benutzerteam])
        .mount("/api/v1/get", routes![api::v1::get::umfrage, api::v1::get::medien, api::v1::get::template, api::v1::get::tparameter, api::v1::get::benutzer, api::v1::get::ufrage, api::v1::get::uantwort, api::v1::get::artikel, api::v1::del::sspiel, api::v1::del::mspiel, api::v1::del::team])
        .mount("/api/v1/new", routes![api::v1::new::umfrageantwort, api::v1::new::umfrage, api::v1::new::uantwort, api::v1::new::umfragebenutzer, api::v1::new::ufrage, api::v1::new::medien, api::v1::new::artikel, api::v1::new::artikelautor, api::v1::new::benutzer, api::v1::new::template, api::v1::new::templatetparameter, api::v1::new::tparameter, api::v1::new::sspiel, api::v1::new::mspiel, api::v1::new::sspieler, api::v1::new::mspieler, api::v1::new::team, api::v1::new::benutzerteam])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![not_found, internal])
        .attach(Template::fairing())
}
