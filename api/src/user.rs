use rocket_dyn_templates::{Template, context};
use rocket::response::{Redirect};
use rocket::serde::{Deserialize};
use rocket::form::{Form};
use rocket::http::{Cookie, CookieJar};
/*use std::path::{Path, PathBuf};
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
use rocket::http::uri::Absolute;

use rocket::fs::TempFile;*/

use crate::utils::cookies::Lehrer;


#[get("/register")]
pub async fn register(_rolle: Lehrer) -> Template {
    Template::render("user/register", context! {
        klassen: vec!["5a", "Q11"],
        rollen: vec!["Schueler", "Lehrer"]
    })
}

/*use crate::utils::DBQueryableUtils;
use crate::db::DBQueryable;
use rocket::serde::json::Json;*/

#[get("/login")]
pub async fn login_page() -> Template {
    Template::render("user/login", context! {
    })
}
#[get("/loginfail", rank= 2)]
pub async fn login_fail() -> Template {
    Template::render("user/loginfail", context! {
    })
}

#[derive(Deserialize, FromForm)]
pub struct Login<'r> {
    name: &'r str,
    passwort: Option<&'r str>,
    token: Option<&'r str>,
}
#[post("/login", data = "<data>")]
pub async fn login_post(data: Form<Login<'_>>, cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    match data.token {
        Some(token) => {
            /*match Benutzer::mebis(data.name.to_string(), token.to_string(), &mut crate::db::establish_connection()) {
                Some(benutzer) => {
                    cookies.remove_private(Cookie::named("user_id"));
                    cookies.add_private(Cookie::new("user_id", benutzer.id.to_string()));
                    return Ok(Template::render("user/loggedin", context! {
                    }));
                }
                None => {}
            }*/
            if let Some(benutzer) = Benutzer::mebis(data.name.to_string(), token.to_string(), &mut crate::db::establish_connection()) {
                cookies.remove_private(Cookie::named("user_id"));
                cookies.add_private(Cookie::new("user_id", benutzer.id.to_string()));
                return Ok(Template::render("user/loggedin", context! {
                }));
            }
        },
        None => return Err(Redirect::to(uri!("/user/loginfail")))
    }
    /*match data.passwort {
        Some(passwort) => {
            match Benutzer::authenticate(data.name.to_string(), passwort.to_string(), &mut crate::db::establish_connection()) {
                Some(benutzer) => {
                    cookies.remove_private(Cookie::named("user_id"));
                    cookies.add_private(Cookie::new("user_id", benutzer.id.to_string()));
                    return Ok(Template::render("user/loggedin", context! {
                    }));
                }
                None => return Err(Redirect::to(uri!("/user/loginfail")))
            }
        },
        None => {}
    }*/
    if let Some(passwort) = data.passwort {
        match Benutzer::authenticate(data.name.to_string(), passwort.to_string(), &mut crate::db::establish_connection()) {
            Some(benutzer) => {
                cookies.remove_private(Cookie::named("user_id"));
                cookies.add_private(Cookie::new("user_id", benutzer.id.to_string()));
                return Ok(Template::render("user/loggedin", context! {
                }));
            }
            None => return Err(Redirect::to(uri!("/user/loginfail")))
        }
    }
    Err(Redirect::to(uri!("/user/loginfail")))
}

use crate::db::models::Benutzer;
#[get("/whoami")]
pub async fn whoami(user: Benutzer) -> Template {
    Template::render("user/whoami", context! {
        name: user.name,
        mebistoken: user.mebistoken,
        passwort: user.passwort,
    })
}
#[get("/whoami", rank = 2)]
pub async fn whoami_redirect() -> Redirect {
    Redirect::to(uri!("/user/login"))
}

/*#[get("/lehrer")]
async fn lehrer(rolle: Lehrer) -> Template {
    Template::render("placeholders/lehrer", context! {
    })
}*/