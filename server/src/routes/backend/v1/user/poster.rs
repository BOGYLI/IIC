use actix_multipart::Multipart;
use actix_web::{get, web::{self, Path, Redirect}, error, Responder, Error, post};

use actix_web_lab::respond::Html;

use crate::{db::{models::{Artikel, Template, NewTemplate, Parameter, NewParameter, NewTemplateParameter, TemplateParameter, NewArtikelParameter, NewArtikel, ArtikelParameter, NewPoster}, self}, utils::{self, files}};
use diesel_ext_traits::{DBQueryable, DBInsertable};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/poster")
            .service(new_post)
    );
}


#[post("/new")]
async fn new_post (
    tmpl: web::Data<tera::Tera>,
    payload: Multipart
) -> Result<impl Responder, Error> {
    let upload_status = files::save_file(payload, "frontend/poster".to_string()).await;

    match upload_status {
        Some(path) => {
            let n =NewPoster {
                datei: path,
                erstelldatum: utils::date::formatted_time(),
                status: String::from("entwurf"),
                oeffentlich: false,
            };
            let poster = n.new(&mut db::establish_connection()).unwrap();    // TODO: no unwrap()
            Ok(Redirect::to("/b/v1/user/poster").see_other())
        },
        _ => Err(error::ErrorInternalServerError("Couldn´t save file"))
    }
}