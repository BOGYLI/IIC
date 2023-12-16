use actix_web::{get, web, error, Responder, Error};

use actix_web_lab::respond::Html;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/games")
            .service(tictactoe_container)
            .service(tictactoe_game)
            .service(simonsays_container)
            .service(clickthebutton_container)
            .service(clickthebutton_game)
    );
}




#[get("/tictactoe")]
async fn tictactoe_container (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/tictactoeContainer.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

#[get("/tictactoe_game")]
async fn tictactoe_game (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let ctx = tera::Context::new();
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/TicTacToe.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}



#[get("/simonsays")]
async fn simonsays_container (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/simonsays.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

#[get("/clickthebutton")]
async fn clickthebutton_container (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/clickthebuttonContainer.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}



#[get("/clickthebutton_game")]
async fn clickthebutton_game (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("edit_api_key", "/f/v1/idlescreen");
    ctx.insert("read_api_key", "/f/v1/idlescreen");
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/Clickthebutton.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}