use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{get, web::{self, Path, Redirect}, error, Responder, Error, post};

use actix_web_lab::respond::Html;

use crate::{db::{models::{Revealjs, Template, NewTemplate, Parameter, NewParameter, NewTemplateParameter, TemplateParameter, NewRevealjs, NewPoster}, self}, utils::{self, files}};
use diesel_ext_traits::{DBQueryable, DBInsertable};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/presentation")
            .service(new_post)
            .service(edit)
            .service(edit_post)
            .service(status)
            .service(view)
            .service(delete_post)

            .service(template_linux)
            .service(template_windows)
            .service(template_macos)
    );
}


#[post("/new")]
async fn new_post (
    tmpl: web::Data<tera::Tera>,
    payload: Multipart
) -> Result<impl Responder, Error> {
    let upload_status = files::save_file(payload, "frontend/revealjs".to_string()).await;

    match upload_status {
        Some(path) => {
            let n =NewRevealjs {
                datei: path,
                erstelldatum: utils::date::formatted_time(),
                status: String::from("entwurf"),
                oeffentlich: false,
            };
            let poster = n.new(&mut db::establish_connection()).expect("failed to insert");    // TODO: no unwrap()
            Ok(Redirect::to("/b/v1/user/presentation").see_other())
        },
        _ => Err(error::ErrorInternalServerError("Couldn´t save file"))
    }
}



#[get("/edit/{presentationid}")]
async fn edit (
    tmpl: web::Data<tera::Tera>,
    path: Path<i32>
) -> Result<impl Responder, Error> {
    let presentation_id = path.into_inner();
    let mut presentation = Revealjs::default();
    presentation.id = presentation_id;
    presentation = presentation.get(&mut db::establish_connection()).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/b/v1/user/presentation");
    ctx.insert("presentation", &presentation);
    Ok(Html(tmpl.render("backend/v1/user/presentation/edit.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}


#[post("/edit/{presentationid}")]
async fn edit_post (
    tmpl: web::Data<tera::Tera>,
    path: Path<i32>,
    payload: Multipart
) -> Result<impl Responder, Error> {
    let presentation_id = path.into_inner();
    let mut presentation = Revealjs::default();
    presentation.id = presentation_id;
    presentation = presentation.get(&mut db::establish_connection()).unwrap();
    std::fs::remove_file(presentation.datei).unwrap();
    let upload_status = files::save_file(payload, "frontend/revealjs".to_string()).await;

    match upload_status {
        Some(path) => {
            presentation.datei = path;
            let poster = presentation.update(&mut db::establish_connection()).expect("failed to insert");    // TODO: no unwrap()
            Ok(Redirect::to("/b/v1/user/presentation").see_other())
        },
        _ => Err(error::ErrorInternalServerError("Couldn't save file"))
    }
}




#[get("/status/{presentationid}")]
async fn status (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let presentationId = path.into_inner();
    let mut presentation = Revealjs::default();//Revealjs {id: presentationId}
    presentation.id = presentationId;
    let presentation = presentation.get(&mut db::establish_connection()).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/b/v1/user");
    ctx.insert("presentation", &presentation);
    Ok(Html(tmpl.render("backend/v1/user/presentation/status.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

#[get("/view/{presentationid}")]
async fn view (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let presentationId = path.into_inner();
    let mut presentation = Revealjs::default();//Revealjs {id: presentationId}
    presentation.id = presentationId;
    if let Ok(presentation) = presentation.get(&mut db::establish_connection()) {
        //let datei = presentation.datei;
        let mut ctx = tera::Context::new();
        ctx.insert("idlescreen_url", "/b/v1/user");
        ctx.insert("exit", "/b/v1/user/presentation");
        ctx.insert("presentation", &presentation);
        //let template_ = format!("presentation-templates/{}", datei);    // TODO
        //log::info!("{}", &template_);
        Ok(Html(tmpl.render("backend/v1/user/presentation/view.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template Error {:?}", e)))?))
    } else {
        return Err(error::ErrorInternalServerError("Database Error"));
    }
}
#[post("/delete/{presentationid}")]
async fn delete_post (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let presentationId = path.into_inner();
    let mut presentation = Revealjs::default();//Revealjs {id: presentationId}
    presentation.id = presentationId;
    if let Ok(presentation) = presentation.get(&mut db::establish_connection()) {
        presentation.delete(&mut db::establish_connection()).unwrap();
        Ok(Redirect::to("/b/v1/user/presentation").see_other())
    } else {
        return Err(error::ErrorInternalServerError("Database Error"));
    }
}



#[get("/template/linux")]
async fn template_linux () -> Result<impl Responder, std::io::Error> {
    NamedFile::open("./frontend/presentation-templates/template-Linux.zip")
}
#[get("/template/windows")]
async fn template_windows () -> Result<impl Responder, std::io::Error> {
    NamedFile::open("./frontend/presentation-templates/template-Windows.zip")
}
#[get("/template/macos")]
async fn template_macos () -> Result<impl Responder, std::io::Error> {
    NamedFile::open("./frontend/presentation-templates/template-MacOS.zip")
}