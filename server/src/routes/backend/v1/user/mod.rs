use actix_web::{get, web::{self, Path}, error, Responder, Error};
use actix_web_lab::respond::Html;
use diesel_ext_traits::DBQueryable;

use crate::db::{self, models::{Revealjs, Poster}};

mod artikel;
mod presentation;
mod poster;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/user")
            .service(artikel_index)
            .configure(artikel::config)
            .service(presentation_index)
            .configure(presentation::config)
            .service(poster_index)
            .configure(poster::config)
    );
}





#[get("/artikel")]
async fn artikel_index (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    use diesel_ext_traits::DBQueryable;
    use crate::db::{self, models::{Template, Artikel}};

    if let Ok(templates) = Template::get_all(&mut db::establish_connection()) {
        if let Ok(artikel) = Artikel::get_all(&mut db::establish_connection()) {
            // TODO: filter by user
            // TODO: if elevated permissions add reviewables-list (by other authors)
            let entwuerfe = artikel.iter().filter(|a| a.status.eq("entwurf")).map(|a| a.clone()).collect::<Vec<Artikel>>();
            let reviews = artikel.iter().filter(|a| a.status.starts_with("review")).map(|a| a.clone()).collect::<Vec<Artikel>>();
            let publics = artikel.iter().filter(|a| a.status.eq("public")).map(|a| a.clone()).collect::<Vec<Artikel>>();
            let mut ctx = tera::Context::new();
            ctx.insert("idlescreen_url", "/b/v1/user");
            ctx.insert("templates", &templates);
            ctx.insert("entwuerfe", &entwuerfe);
            ctx.insert("reviews", &reviews);
            ctx.insert("publics", &publics);
            Ok(Html(tmpl.render("backend/v1/user/artikel.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template Error {:?}", e)))?))
        } else {
            Err(error::ErrorInternalServerError("Database Error"))
        }
    } else {
        Err(error::ErrorInternalServerError("Database Error"))
    }
}



/*#[get("/präsentation")]
async fn presentation_index (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    use diesel_ext_traits::DBQueryable;
    use crate::db::{self, models::{Template, Artikel}};

    if let Ok(templates) = Template::get_all(&mut db::establish_connection()) {
        if let Ok(artikel) = Artikel::get_all(&mut db::establish_connection()) {
            // TODO: filter by user
            // TODO: if elevated permissions add reviewables-list (by other authors)
            let entwuerfe = artikel.iter().filter(|a| a.status.eq("entwurf")).map(|a| a.clone()).collect::<Vec<Artikel>>();
            let reviews = artikel.iter().filter(|a| a.status.starts_with("review")).map(|a| a.clone()).collect::<Vec<Artikel>>();
            let publics = artikel.iter().filter(|a| a.status.eq("public")).map(|a| a.clone()).collect::<Vec<Artikel>>();
            let mut ctx = tera::Context::new();
            ctx.insert("idlescreen_url", "/b/v1/user");
            ctx.insert("templates", &templates);
            ctx.insert("entwuerfe", &entwuerfe);
            ctx.insert("reviews", &reviews);
            ctx.insert("publics", &publics);
            Ok(Html(tmpl.render("backend/v1/user/presentation.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template Error {:?}", e)))?))
        } else {
            Err(error::ErrorInternalServerError("Database Error"))
        }
    } else {
        Err(error::ErrorInternalServerError("Database Error"))
    }
}*/
#[get("/presentation")]
async fn presentation_index(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    if let Ok(presentations) = Revealjs::get_all(&mut db::establish_connection()) {
        let entwuerfe = presentations.iter().filter(|a| a.status.eq("entwurf")).map(|a| a.clone()).collect::<Vec<Revealjs>>();
        let reviews = presentations.iter().filter(|a| a.status.starts_with("review")).map(|a| a.clone()).collect::<Vec<Revealjs>>();
        let publics = presentations.iter().filter(|a| a.status.eq("public")).map(|a| a.clone()).collect::<Vec<Revealjs>>();

        let mut ctx = tera::Context::new();
        ctx.insert("idlescreen_url", "/b/v1/homescreen");
        ctx.insert("entwuerfe", &entwuerfe);
        ctx.insert("reviews", &reviews);
        ctx.insert("publics", &publics);
        match tmpl.render("backend/v1/user/presentation.html", &ctx) {
            Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
            Err(err) => {
                eprintln!("## Tera error: {}", err);
                Err(error::ErrorInternalServerError(err))
            },
        }
    } else {
        Err(error::ErrorInternalServerError("Database Error"))
    }    
}


/*#[get("/poster")]
async fn poster_index(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/b/v1/homescreen");
    match tmpl.render("backend/v1/user/poster.html", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}*/


#[get("/poster")]
async fn poster_index(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    if let Ok(posters) = Poster::get_all(&mut db::establish_connection()) {
        let entwuerfe = posters.iter().filter(|a| a.status.eq("entwurf")).map(|a| a.clone()).collect::<Vec<Poster>>();
        let reviews = posters.iter().filter(|a| a.status.starts_with("review")).map(|a| a.clone()).collect::<Vec<Poster>>();
        let publics = posters.iter().filter(|a| a.status.eq("public")).map(|a| a.clone()).collect::<Vec<Poster>>();

        let mut ctx = tera::Context::new();
        ctx.insert("idlescreen_url", "/b/v1/homescreen");
        ctx.insert("entwuerfe", &entwuerfe);
        ctx.insert("reviews", &reviews);
        ctx.insert("publics", &publics);
        match tmpl.render("backend/v1/user/poster.html", &ctx) {
            Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
            Err(err) => {
                eprintln!("## Tera error: {}", err);
                Err(error::ErrorInternalServerError(err))
            },
        }
    } else {
        Err(error::ErrorInternalServerError("Database Error"))
    }    
}