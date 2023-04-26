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

use utils::cookies::*;

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
async fn index(perm: HTMLPermission) -> Template {
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
    match SSpiel::get_all(&mut crate::db::establish_connection()) {
        Ok(games) => {
            Ok(Template::render("placeholders/games", context! {
                games: games
            }))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

use crate::utils::DBQueryableUtils;
use crate::db::DBQueryable;
use rocket::serde::json::Json;

#[derive(Deserialize, FromForm)]
struct Login<'r> {
    name: &'r str,
    passwort: Option<&'r str>,
    token: Option<&'r str>,
}
#[post("/login", data = "<data>")]
async fn login(data: Form<Login<'_>>, cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    match data.token {
        Some(token) => {
            match Benutzer::mebis(data.name.to_string(), token.to_string(), &mut crate::db::establish_connection()) {
                Some(benutzer) => {
                    cookies.remove_private(Cookie::named("user_id"));
                    cookies.add_private(Cookie::new("user_id", benutzer.id.to_string()));
                    return Ok(Template::render("placeholders/loggedin", context! {
                    }));
                }
                None => {}
            }
        },
        None => return Err(Redirect::to(uri!(login_fail)))
    }
    match data.passwort {
        Some(passwort) => {
            match Benutzer::authenticate(data.name.to_string(), passwort.to_string(), &mut crate::db::establish_connection()) {
                Some(benutzer) => {
                    cookies.remove_private(Cookie::named("user_id"));
                    cookies.add_private(Cookie::new("user_id", benutzer.id.to_string()));
                    return Ok(Template::render("placeholders/loggedin", context! {
                    }));
                }
                None => return Err(Redirect::to(uri!(login_fail)))
            }
        },
        None => {}
    }
    Err(Redirect::to(uri!(login_fail)))
}

#[derive(Deserialize, FromForm)]
struct HTMLActivation<'r> {
    pin: &'r str
}
use rocket::request::Outcome;
#[post("/htmlactivation", data = "<data>")]
async fn htmlactivation(data: Form<HTMLActivation<'_>>, cookies: &CookieJar<'_>) -> Result<Template, Status> {
    match dotenvy::var("SCREENPIN") {
        Ok(screenpin) => {
            if screenpin == data.pin {
                cookies.remove_private(Cookie::named("html_access"));
                cookies.add_private(Cookie::new("html_access", "1"));
                return Ok(Template::render("idle", context! {
                }));
            }
        }
        Err(_) => {}
    }
    Err(Status::Locked)
}

use crate::db::models::Benutzer;
#[get("/whoami")]
async fn whoami(user: Benutzer) -> Template {
    Template::render("placeholders/whoami", context! {
        name: user.name,
        mebistoken: user.mebistoken,
        passwort: user.passwort,
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
        //rolle: user.rolle
    })
}

#[get("/schueler", rank = 2)]
async fn schueler(user: Benutzer) -> Template {
    Template::render("placeholders/wrong_role", context! {
        //rolle: user.rolle
    })
}



use std::env;
#[launch]
fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "full");
    dotenvy::var("USERNAME").expect("mebis-lib requires USERNAME - credential");
    dotenvy::var("PASSWORD").expect("mebis-lib requires PASSWORD - credential");
    dotenvy::var("URL").expect("mebis-lib requires URL - api");
    dotenvy::var("SCREENPIN").expect("SCREENPIN required for graphical authentication of trusted device");
    
    use rocket::config::{Config, TlsConfig, CipherSuite};
    let tls_config = TlsConfig::from_paths("certs/cert.pem", "certs/key.pem")
        .with_ciphers(CipherSuite::TLS_V13_SET)
        .with_preferred_server_cipher_order(true);
        
    use std::net::Ipv4Addr;
    let config = Config {
        tls: Some(tls_config),
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Default::default()
    };
    
    rocket::custom(config)
        .mount("/", routes![index, favicon, api_v1, home, wordpress_post, login_page, login_fail, login, register, whoami, whoami2, placeholders, sspiel, lehrer, keinlehrer, schueler, htmlactivation])
        //.mount("/features/v1", routes![feat_v1_news, feat_v1_navi, feat_v1_spiele, feat_v1_umfragen, feat_v1_geburtstag])
        .mount("/api/v1/delete", routes![api::del::umfrage, api::del::medien, api::del::template, api::del::tparameter, api::del::benutzer, api::del::ufrage, api::del::uantwort, api::del::artikel, api::del::sspiel, api::del::mspiel, api::del::team])
        .mount("/api/v1/edit", routes![api::edit::umfrageantwort, api::edit::umfrage, api::edit::uantwort, /*api::edit::umfragebenutzer,*/ api::edit::ufrage, api::edit::medien, api::edit::artikel, /*api::edit::artikelautor,*/ api::edit::benutzer, api::edit::template, /*api::edit::templatetparameter,*/ api::edit::tparameter, api::edit::sspiel, api::edit::mspiel, api::edit::sspieler, api::edit::mspieler, api::edit::team, /*api::edit::benutzerteam*/])
        .mount("/api/v1/get_all", routes![api::get_all::umfrageantwort, api::get_all::umfrage, api::get_all::uantwort, api::get_all::umfragebenutzer, api::get_all::ufrage, api::get_all::medien, api::get_all::artikel, api::get_all::artikelautor, api::get_all::benutzer, api::get_all::template, api::get_all::templatetparameter, api::get_all::tparameter, api::get_all::sspiel, api::get_all::mspiel, api::get_all::sspieler, api::get_all::mspieler, api::get_all::team, api::get_all::benutzerteam])
        .mount("/api/v1/get", routes![api::get::umfrage, api::get::medien, api::get::template, api::get::tparameter, api::get::benutzer, api::get::ufrage, api::get::uantwort, api::get::artikel, api::del::sspiel, api::del::mspiel, api::del::team])
        .mount("/api/v1/new", routes![api::new::umfrageantwort, api::new::umfrage, api::new::uantwort, api::new::umfragebenutzer, api::new::ufrage, api::new::medien, api::new::artikel, api::new::artikelautor, api::new::benutzer, api::new::template, api::new::templatetparameter, api::new::tparameter, api::new::sspiel, api::new::mspiel, api::new::sspieler, api::new::mspieler, api::new::team, api::new::benutzerteam])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![catchers::not_authorized, catchers::locked, catchers::not_found, catchers::internal])
        .attach(Template::fairing())
}
