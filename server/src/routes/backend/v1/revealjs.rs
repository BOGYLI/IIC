use actix_web::{get, web::{self, Path}, error, Responder, Error};

use actix_web_lab::respond::Html;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/revealjs")
            .service(download)
            .service(play)
            .service(preview)
            .service(view)
            .service(delete)
            .service(upload)
    );
}




#[get("/download/{id}")]
async fn download (
    tmpl: web::Data<tera::Tera>,
    path: Path<i32>
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/b/v1/revealjs");
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/tictactoeContainer.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

#[get("/play/{id}")]
async fn play (
    tmpl: web::Data<tera::Tera>,
    path: Path<i32>
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let ctx = tera::Context::new();
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/TicTacToe.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

#[get("/preview/{id}")]
async fn preview (
    tmpl: web::Data<tera::Tera>,
    path: Path<i32>
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("exit", "/b/v1/home/revealjs");
    ctx.insert("reveal", &id);
    Ok(Html(tmpl.render("backend/v1/revealjs/preview.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

#[get("/view/{id}")]
async fn view (
    tmpl: web::Data<tera::Tera>,
    path: Path<i32>
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("exit", "/b/v1/home/revealjs");
    ctx.insert("reveal", &id);
    Ok(actix_files::NamedFile::open("static/favicon.ico")?)
}


#[get("/delete")]
async fn delete (
    tmpl: web::Data<tera::Tera>,
    path: Path<i32>
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/simonsays.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}



#[get("/upload")]
async fn upload (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("edit_api_key", "/f/v1/idlescreen");
    ctx.insert("read_api_key", "/f/v1/idlescreen");
    Ok(Html(tmpl.render("frontend/v1/feedback/runde1/Clickthebutton.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}