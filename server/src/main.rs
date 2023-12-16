/*#![feature(trace_macros)]
#![feature(log_syntax)]
trace_macros!(true);*/

use std::collections::BTreeMap;

use actix_web::{web, App, HttpResponse, HttpServer, FromRequest};

mod db;
mod utils;

// web
mod api;
mod stati;
mod routes;


#[derive(serde_derive::Serialize, serde_derive::Deserialize, Clone, Debug, actix_jwt_auth_middleware::FromRequest)]
struct User {
    id: u32,
}
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key, get, middleware::Logger, post, HttpMessage, HttpRequest, Responder,
};

use actix_web::{dev::ServiceRequest, Error};
use actix_web_grants::GrantsMiddleware;

// You can use custom type instead of String
async fn extract(req: &mut ServiceRequest) -> Result<Vec<String>, Error> {
    //let requ: HttpRequest = req.request();
    let (req, pay) = req.parts_mut();
    let user = Identity::from_request(req, pay).await;
    match user {
        Ok(_) => Ok(vec!["ADMIN".to_string()]),
        Err(_) => Ok(vec![])
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8089");

    let secret_key = Key::generate();
    use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("certs/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("certs/cert.pem").unwrap();


    HttpServer::new(move || {
        let mut tera = tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/templates/**/*")).unwrap();
        tera.register_function("media_url", utils::tera_fcts::media_url(BTreeMap::new()));
        tera.register_function("format_date", utils::tera_fcts::format_date(BTreeMap::new()));

        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                // disable secure cookie for local testing
                .cookie_secure(false)
                .build();

        
        
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(GrantsMiddleware::with_extractor(extract))
            // Install the identity framework first.
            .wrap(IdentityMiddleware::default())
            // The identity system is built on top of sessions. You must install the session
            // middleware to leverage `actix-identity`. The session middleware must be mounted
            // AFTER the identity middleware: `actix-web` invokes middleware in the OPPOSITE
            // order of registration when it receives an incoming request.
            .wrap(session_mw)
            .wrap(actix_web::middleware::Logger::default())

            .configure(api::config)
            .configure(stati::config)   // static html
            .configure(routes::frontend::config)

            .configure(routes::system::config)
            //.use_jwt(authority, web::scope("").configure(routes::backend::config))
            .configure(routes::backend::config)
            .service(reveal)
            .service(index)
            .service(login)
            .service(logout)

            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8087))?
    .bind_openssl("127.0.0.1:8089", builder)?
    .run()
    .await
}

#[actix_web::get("/revealjs/{presentationid}")]
async fn reveal(
    path: web::Path<i32> ,
) -> Result<impl actix_web::Responder, actix_web::Error> {
    use crate::db::models::*;
    use crate::db;
    use diesel_ext_traits::*;
    use actix_files::*;
    let mut presentation = Revealjs::default();
    presentation.id = path.into_inner();
    presentation=presentation.get(&mut db::establish_connection()).unwrap();
    log::info!("{}", &presentation.datei);
    Ok(NamedFile::open(presentation.datei))
}


#[actix_web::get("/hello")]
async fn index(user: Option<Identity>) -> impl actix_web::Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[actix_web::get("/login")]
async fn login(request: actix_web::HttpRequest) -> impl actix_web::Responder {
    // Some kind of authentication should happen here -
    // e.g. password-based, biometric, etc.
    // [...]

    // Attached a verified user identity to the active
    // session.
    let x = Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

#[actix_web::get("/logout")]
async fn logout(user: Identity) -> impl actix_web::Responder {
    user.logout();
    HttpResponse::NoContent()
}