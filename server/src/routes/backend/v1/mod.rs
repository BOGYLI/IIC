use actix_web::{get, web, error, Responder, Error};

mod revealjs;
mod home;
mod user;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/v1")
            .configure(home::config)
            .service(revealjs_index)
            .configure(revealjs::config)
            .service(user_index)
            .configure(user::config)
    );
}

#[get("/revealjs")]
async fn revealjs_index(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/b/v1/homescreen");
    match tmpl.render("backend/v1/revealjs-explorer.html", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}


#[get("/user")]
async fn user_index(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/b/v1/homescreen");
    match tmpl.render("backend/v1/user-index.html", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}