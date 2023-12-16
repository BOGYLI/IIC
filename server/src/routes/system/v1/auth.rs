
use actix_identity::Identity;
use actix_web::{get, web::{self, Form}, error, Responder, Error, post, HttpResponse, HttpMessage, dev::HttpServiceFactory};

use actix_web_lab::respond::Html;
use diesel_ext_traits::DBInsertable;
use serde_derive::{Serialize, Deserialize};

use crate::db::models::{NewBenutzer, Benutzer};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/auth")
            .service(login)
            .service(post_login)
            .service(logout)

            .service(register)
            .service(post_register)
    );
}

#[get("/login")]
async fn login(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let ctx = tera::Context::new();
    Ok(Html(tmpl.render("system/v1/login.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    pub name: String,
    pub password: String,
    pub token: Option<String>,
}


#[post("/login")]
async fn post_login(request: actix_web::HttpRequest, cred: Form<Credentials>) -> Result<impl Responder, Error> {
    let cred = cred.into_inner();
    match Benutzer::authenticate(&cred.name, &cred.password, &mut crate::db::establish_connection()) {
        Some(b) => {
            Identity::login(&request.extensions(), b.id.to_string().into()).unwrap();
            Ok(HttpResponse::Ok().body("Du bist eingeloggt"))
        },
        None => {
            Ok(HttpResponse::NotAcceptable().body("Falsche Zugangsdaten"))
        }
    }
    
}


#[get("/logout")]
async fn logout(
    tmpl: web::Data<tera::Tera>,
    user: Identity
) -> Result<impl Responder, Error> {
    user.logout();
    Ok(HttpResponse::NoContent())
}





// TODO: remove in production
#[get("/register")]
async fn register(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let ctx = tera::Context::new();
    Ok(Html(tmpl.render("system/v1/dev/register.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}



#[post("/register")]
async fn post_register(request: actix_web::HttpRequest, benutzer: Form<NewBenutzer>) -> Result<impl Responder, Error> {
    let benutzer = benutzer.into_inner();
    benutzer.new(&mut crate::db::establish_connection()).unwrap();
    
    Ok(HttpResponse::Ok().body("Du bist registriert"))
}