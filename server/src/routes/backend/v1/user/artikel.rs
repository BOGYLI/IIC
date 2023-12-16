use actix_web::{get, web::{self, Path, Redirect}, error, Responder, Error, post};

use actix_web_lab::respond::Html;

use crate::{db::{models::{Artikel, Template, NewTemplate, Parameter, NewParameter, NewTemplateParameter, TemplateParameter, NewArtikelParameter, NewArtikel, ArtikelParameter}, self}, utils};
use diesel_ext_traits::{DBQueryable, DBInsertable};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/artikel")
            .service(new)
            .service(new_post)
            .service(edit)
            .service(edit_post)
            .service(view)
            .service(delete_post)
            .service(status)

            .service(index_templates)
    );
}


#[get("/new/{templateid}")]
async fn new (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let templateId = path.into_inner();
    let mut template = Template::default();
    template.id = templateId;
    if let Ok(template) = template.get(&mut db::establish_connection()) {
        if let Ok(parameters) = Parameter::get_connected_with_template(template.id, &mut db::establish_connection()) {
            let mut ctx = tera::Context::new();
            ctx.insert("idlescreen_url", "/b/v1/user");
            ctx.insert("template", &template);
            ctx.insert("parameters", &parameters);
            Ok(Html(tmpl.render("backend/v1/user/artikel/new.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template Error {:?}", e)))?))
        } else {
            return Err(error::ErrorInternalServerError("Database Error"));
        }
    } else {
        return Err(error::ErrorInternalServerError("Database Error"));
    }
}

#[post("/new/{templateid}")]
async fn new_post (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
    data: web::Form<std::collections::HashMap<i32,String> >
) -> Result<impl Responder, Error> {
    let templateId = path.into_inner();
    let mut template = Template::default();
    template.id = templateId;
    let mut artikel = NewArtikel {
        erstelldatum: utils::date::formatted_time(),
        status: String::from("fehler"),
        templateid: template.id
    }.new(&mut db::establish_connection()).unwrap();    // TODO! no unwrap
    if let Ok(template) = template.get(&mut db::establish_connection()) {
        if let Ok(parameters) = Parameter::get_connected_with_template(template.id, &mut db::establish_connection()) {
            //let mut values: Vec<(Parameter, String)> = Vec::new();
            for parameter in parameters {
                let val = if let Some(val) = data.get(&parameter.id) {
                    val.to_owned()
                } else {String::new()};

                //values.push((parameter, val));

                let con = NewArtikelParameter{
                    artikelid: artikel.id,
                    parameterid: parameter.id,
                    wert: val
                }.new(&mut db::establish_connection()).unwrap();    // TODO! no unwrap()
            }

            //artikel.status = String::from("entwurf");
            artikel.update(&mut db::establish_connection()).unwrap();   // TODO! no unwrap()
            

            Ok(Redirect::to(format!("/b/v1/user/artikel/edit/{}", artikel.id)).see_other())
        } else {
            return Err(error::ErrorInternalServerError("Database Error"));
        }
    } else {
        return Err(error::ErrorInternalServerError("Database Error"));
    }
}

#[get("/edit/{artikelid}")]
async fn edit (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let artikelId = path.into_inner();
    let mut artikel = Artikel::default();//Artikel {id: artikelId}
    artikel.id = artikelId;
    if let Ok(artikel) = artikel.get(&mut db::establish_connection()) {
        if let Ok(parameters) = Parameter::get_connected_with_artikel(artikel.id, &mut db::establish_connection()) {
            let values = parameters.iter().map(|p| 
                {
                    let tmp = ArtikelParameter{artikelid: artikel.id, parameterid: p.id, wert: String::new()};
                    let tmp = tmp.get(&mut db::establish_connection()).unwrap();  // TODO no unwrap()
                    log::info!("{:?}", &tmp);
                    log::info!("{:?}", &(tmp.artikelid, p.to_owned(), &tmp.wert));
                    (tmp.artikelid, p.to_owned(), tmp.wert)
                }
            ).collect::<Vec<(i32, Parameter, String)>>();
            let mut ctx = tera::Context::new();
            ctx.insert("idlescreen_url", "/b/v1/user");
            ctx.insert("artikel", &artikel);
            ctx.insert("values", &values);
            Ok(Html(tmpl.render("backend/v1/user/artikel/edit.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
        } else {
            return Err(error::ErrorInternalServerError("Database Error"));
        }
    } else {
        return Err(error::ErrorInternalServerError("Database Error"));
    }
}

#[post("/edit/{artikelid}")]
async fn edit_post (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
    data: web::Form<std::collections::HashMap<i32,String> >
) -> Result<impl Responder, Error> {
    let artikelId = path.into_inner();
    let mut artikel = Artikel::default();
    log::info!("U P D A T E   UPDATE   U P D A T E");
    artikel.id = artikelId;
    if let Ok(mut artikel) = artikel.get(&mut db::establish_connection()) {
        if let Ok(parameters) = Parameter::get_connected_with_artikel(artikel.id, &mut db::establish_connection()) {
            //let mut values: Vec<(Parameter, String)> = Vec::new();
            for parameter in parameters {
                let val = if let Some(val) = data.get(&parameter.id) {
                    val.to_owned()
                } else {String::new()};

                //values.push((parameter, val));

                let con = ArtikelParameter{
                    artikelid: artikel.id,
                    parameterid: parameter.id,
                    wert: val
                };
                log::info!("{:?}", &con);
                con.update(&mut db::establish_connection()).unwrap();    // TODO! no unwrap()
            }

            artikel.status = String::from("entwurf");
            artikel.update(&mut db::establish_connection()).unwrap();   // TODO! no unwrap()
            

            Ok(Redirect::to(format!("/b/v1/user/artikel/view/{}", artikel.id)).see_other())
        } else {
            return Err(error::ErrorInternalServerError("Database Error"));
        }
    } else {
        return Err(error::ErrorInternalServerError("Database Error"));
    }
}


#[get("/status/{artikelid}")]
async fn status (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let artikelId = path.into_inner();
    let mut artikel = Artikel::default();//Artikel {id: artikelId}
    artikel.id = artikelId;
    let artikel = artikel.get(&mut db::establish_connection()).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/b/v1/user");
    ctx.insert("artikel", &artikel);
    Ok(Html(tmpl.render("backend/v1/user/artikel/status.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template Error"))?))
}

#[get("/view/{artikelid}")]
async fn view (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let artikelId = path.into_inner();
    let mut artikel = Artikel::default();//Artikel {id: artikelId}
    artikel.id = artikelId;
    if let Ok(artikel) = artikel.get(&mut db::establish_connection()) {
        let mut template = Template::default();
        template.id = artikel.templateid;
        let template = template.get(&mut db::establish_connection()).expect("Tempalteerror");  // TODO no unwrap()
        
        if let Ok(parameters) = Parameter::get_connected_with_artikel(artikel.id, &mut db::establish_connection()) {
            let mut ctx = tera::Context::new();
            let values = parameters.iter().map(|p| 
                {
                    let tmp = ArtikelParameter{artikelid: artikel.id, parameterid: p.id, wert: String::new()};
                    let tmp = tmp.get(&mut db::establish_connection()).unwrap();  // TODO no unwrap()
                    log::info!("{:?}", &tmp);
                    log::info!("{:?}", &(tmp.artikelid, p.to_owned(), &tmp.wert));
                    ctx.insert(&p.name, &tmp.wert.replace("\r\n", "<br>"));
                    (tmp.artikelid, p.to_owned(), tmp.wert)
                }
            ).collect::<Vec<(i32, Parameter, String)>>();
            ctx.insert("idlescreen_url", "/b/v1/user");
            let template_ = format!("artikel-templates/{}", template.pfad);
            log::info!("{}", &template_);
            Ok(Html(tmpl.render(&template_, &ctx)
            .map_err(|e| error::ErrorInternalServerError(format!("Template Error {:?}", e)))?))
        } else {
            return Err(error::ErrorInternalServerError("Database Error"));
        }
    } else {
        return Err(error::ErrorInternalServerError("Database Error"));
    }
}
#[post("/delete/{artikelid}")]
async fn delete_post (
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<impl Responder, Error> {
    let artikelId = path.into_inner();
    let mut artikel = Artikel::default();//Artikel {id: artikelId}
    artikel.id = artikelId;
    if let Ok(artikel) = artikel.get(&mut db::establish_connection()) {
        if let Ok(parameters) = Parameter::get_connected_with_artikel(artikel.id, &mut db::establish_connection()) {
            parameters.iter().for_each(|p| 
                {
                    let tmp = ArtikelParameter{artikelid: artikel.id, parameterid: p.id, wert: String::new()};
                    let mut conn = db::establish_connection();
                    let tmp = tmp.get(&mut conn).unwrap();  // TODO no unwrap()
                    tmp.delete(&mut conn).unwrap();
                    p.delete(&mut conn).unwrap();
                }
            );
            artikel.delete(&mut db::establish_connection()).unwrap();
            Ok(Redirect::to("/b/v1/user/artikel").see_other())
        } else {
            return Err(error::ErrorInternalServerError("Database Error"));
        }
    } else {
        return Err(error::ErrorInternalServerError("Database Error"));
    }
}



#[get("/index_templates")]      // TODO: use for revealjs too
async fn index_templates (
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    use std::fs::*;
    use std::path::PathBuf;
    use std::io::Read;
    let templates = read_dir("frontend/templates/artikel-templates")?.filter_map(|e| e.ok()).filter_map(|e| e.file_name().into_string().ok()).collect::<Vec<String>>();
    log::info!("{:?}", &templates);
    match Template::get_all(&mut db::establish_connection()) {
        Ok(db_templates) => {
            let new_templates = templates.iter().filter(|t| db_templates.iter().filter(|e| e.pfad.eq(*t)).collect::<Vec<&Template>>().len() == 0).map(|t| NewTemplate{pfad: t.clone()}.new(&mut db::establish_connection())).filter_map(|t| t.ok()).collect::<Vec<Template>>();
            for new_template in &new_templates {
                let mut file = File::open(PathBuf::from("frontend/templates/artikel-templates").join(&new_template.pfad))?;
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                let frags = buf.split("{{").map(|f| f.split("}}"));//TODO: map(s -> vec{}) !!.collect::<Vec<&str>>();
                let mut fragments: Vec<String> = Vec::new();
                for frag in frags {
                    for fragment in frag {
                        fragments.push(fragment.to_owned());
                    }
                }
                //let mut parameters: Vec<String> = Vec::new();
                debug_assert!(fragments.len()%2!=0);  // xx{{yy}}xx{{aa}}xx -> [xx, yy, xx, aa, xx] | xx{{yy}}{{aa}}xx -> [xx, yy, , aa, xx]
                for (i, parameter) in fragments.iter().enumerate() {
                    if i%2 != 0 {
                        let parameter = parameter.split('|').map(|s| s.trim()).collect::<Vec<&str>>()[0].to_owned();
                        //parameters.push(parameter.clone())
                        let n_parameter = NewParameter{name: parameter.clone(), typ: String::from("general")};
                        if let Ok(p) = n_parameter.new(&mut db::establish_connection()) {
                            let t_parameter = NewTemplateParameter{templateid: new_template.id, parameterid: p.id};
                            match t_parameter.new(&mut db::establish_connection()) {
                                Ok(_) => {
                                    
                                },
                                Err(_) => {return Err(error::ErrorInternalServerError("Database Error (Template <--> Parameter)-connection"));}     // TODO: ?
                            }
                        }
                        
                    }
                }
            }
            log::info!("{:?}", &new_templates);
            let ctx = tera::Context::new();
            Ok(Redirect::to("/b/v1/user/artikel").see_other())
        },
        Err(_) => Err(error::ErrorInternalServerError("Database Error"))
    }
}