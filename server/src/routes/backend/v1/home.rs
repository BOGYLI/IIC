use actix_web::{get, web, error, Responder, Error};

use actix_web_lab::respond::Html;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/home")
            .service(index)
    );
}

#[get("/")]
async fn index(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let ctx = tera::Context::new();
    Ok(Html(tmpl.render("backend/v1/index.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}