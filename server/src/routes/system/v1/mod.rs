
use actix_web::{get, web, error, Responder, Error, post, HttpResponse};

use actix_web_lab::respond::Html;

mod auth;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/v1")
            .configure(auth::config)
    );
}

#[get("/login")]
async fn get_login(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let ctx = tera::Context::new();
    Ok(Html(tmpl.render("system/v1/login.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

/*use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{AuthResult, Authority, FromRequest, TokenSigner};

use actix_web::{HttpServer};
use exonum_crypto::KeyPair;
use jwt_compact::alg::Ed25519;
use serde::{Deserialize, Serialize};
#[post("/login")]
async fn post_login(cookie_signer: web::Data<TokenSigner<crate::User,Ed25519> >) -> actix_jwt_auth_middleware::AuthResult<HttpResponse> {
    let user = crate::User { id: 1 };
    Ok(HttpResponse::Ok()
        .cookie(cookie_signer.create_access_cookie(&user)?)
        .cookie(cookie_signer.create_refresh_cookie(&user)?)
        .body("You are now logged in"))
}*/