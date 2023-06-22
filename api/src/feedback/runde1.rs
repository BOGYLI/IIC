use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use rocket::form::{Form};
use rocket::response::{Redirect};
/*use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

use rocket::http::{Cookie, SameSite, CookieJar, Status};
use rocket::response::{Flash, Redirect};

use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::{self, Form,  Error};
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

use rocket::Request;
use rocket::response::Responder;


use rocket::fs::TempFile;*/

use crate::db::DBQueryable;
use crate::db::models::{Umfrage, UAntwort, UFrage, ApiKey};
use crate::utils::DBQueryableUtils;
use crate::utils::cookies::HTMLPermission;

#[get("/idlescreen")]
pub async fn idlescreen(_perm: HTMLPermission) -> Template {
    let uri = dotenvy::var("WORDPRESS_URL").expect("wp-lib requires WORDPRESS_URL - api");
    let posts = wp_lib::Post::get_from_uri_limited(&uri, 3).unwrap();
    //let media = wp_lib::Media::from(posts[0].featured_media).guid.rendered;
    //let val = wp_lib::Media::from(&uri, posts[0].clone().featured_media.to_string()).unwrap();
    //println!("{:?}", val);

    //TODO Template correct built but not showed as html in browser
    // add caching
    Template::render("tests/feedback/runde1/Idlescreen", context! {
        articles: posts
    })
}

#[get("/refresh")]
pub async fn refresh(_perm: HTMLPermission) -> Redirect {
    use cache_lib as cache;
    cache::post::refresh();
    Redirect::to(uri!("/feedback/runde1/idlescreen"))
}

#[get("/games/clickthebutton")]
pub async fn clickthebutton(_perm: HTMLPermission) -> Template {
    Template::render("tests/feedback/runde1/Clickthebutton", context! {
    })
}

#[get("/games/tictactoe")]
pub async fn tictactoe(_perm: HTMLPermission) -> Template {
    Template::render("tests/feedback/runde1/TicTacToe", context! {
    //Template::render("games/tictactoe", context! {
    })
}


#[get("/games")]
pub async fn games(_perm: HTMLPermission) -> Template {
    Template::render("tests/feedback/runde1/games", context! {
    })
}

#[get("/karte")]
pub async fn karte(_perm: HTMLPermission) -> Template {
    Template::render("tests/feedback/runde1/karte", context! {
    })
}

#[get("/birthday")]
pub async fn birthday(_perm: HTMLPermission) -> Template {
    Template::render("tests/feedback/runde1/birthday", context! {
    })
}

#[derive(FromForm)]
pub struct Birthday {
    wish: String,
    autoren: String,
}

#[post("/birthday/demo", data = "<data>")]
pub async fn birthdaydemo(data: Form<Birthday>) -> Template {
    let d = data.into_inner();
    Template::render("tests/feedback/runde1/birthdaydemo", context! {
        wish: d.wish.clone(),
        autoren: d.autoren.clone()
    })
}

#[get("/news")]
pub async fn news(_perm: HTMLPermission) -> Template {
    Template::render("tests/feedback/runde1/news", context! {
    })
}

#[get("/umfragen")]
pub async fn umfragen(_perm: HTMLPermission) -> Template {
    match Umfrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => {
            Template::render("tests/feedback/runde1/umfragen", context! {
                umfragen: data,
            })
        }, Err(_) => {
            let data: Vec<Umfrage> = vec![];
            Template::render("tests/feedback/runde1/umfragen", context! {
                umfragen: data,
            })
        }
    }
}

#[get("/umfrage/create")]
pub async fn umfrage_create(_perm: HTMLPermission) -> Template {
    match Umfrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => {
            Template::render("tests/feedback/runde1/umfrage/create", context! {
                umfragen: data,
                api_new: ApiKey::api_new().expect("ApiKey muss existieren (normal durch migration)").wert,
            })
        },
		Err(_) => {
            let data: Vec<Umfrage> = vec![];
            Template::render("tests/feedback/runde1/umfrage/create", context! {
                umfragen: data,
                api_new: ApiKey::api_new().expect("ApiKey muss existieren (normal durch migration)").wert,
            })
        }
	}
    
}

#[get("/umfrage/edit/<id>")]
pub async fn umfrage_edit(id: i32, _perm: HTMLPermission) -> Template {
    match UAntwort::get_all(&mut crate::db::establish_connection()) {
		Ok(antworten) => {
            match UFrage::get_all(&mut crate::db::establish_connection()) {
                Ok(fragen) => {
                    match UFrage::get_by_umfrage(id/*, &mut crate::db::establish_connection() */) {
                        Ok(umfragefragen) => {
                            let mut antwmoegl: Vec<Vec<UAntwort>> = vec![];
                            for ufrage in &umfragefragen {
                                if let Ok(d) = ufrage.get_uantworten() {
                                    antwmoegl.push(d);
                                }
                            }
                            Template::render("tests/feedback/runde1/umfrage/edit", context! {
                                umfrage: id,
                                antworten,
                                fragen,
                                umfragefragen,
                                antwortmoeglichkeiten: antwmoegl,
                             })
                        },
                        Err(_) => {
                            let umfragefragen: Vec<UFrage> = vec![];
                            let antwmoegl: Vec<Vec<UAntwort>> = vec![];
                            Template::render("tests/feedback/runde1/umfrage/edit", context! {
                                umfrage: id,
                                antworten,
                                fragen,
                                umfragefragen,
                                antwmoegl,
                             })
                        }
                    }
                },
                Err(_) => {
                    let fragen: Vec<UFrage> = vec![];
                    let umfragefragen: Vec<UFrage> = vec![];
                    let antwmoegl: Vec<Vec<UAntwort>> = vec![];
                    Template::render("tests/feedback/runde1/umfrage/edit", context! {
                        umfrage: id,
                        antworten,
                        fragen,
                        umfragefragen,
                        antwmoegl,
                     })
                }
            }
        },
		Err(_) => {
            let antworten: Vec<UAntwort> = vec![];
            let fragen: Vec<UFrage> = vec![];
            let umfragefragen: Vec<UFrage> = vec![];
            let antwmoegl: Vec<Vec<UAntwort>> = vec![];
            Template::render("tests/feedback/runde1/umfrage/edit", context! {
                umfrage: id,
                antworten,
                fragen,
                umfragefragen,
                antwmoegl,
             })
        }
	}
    
}

#[get("/umfrage/<id>")]
pub async fn umfrage_view(id: i32, _perm: HTMLPermission) -> Result<Template, Status> {
    if let Ok(umfrage) = Umfrage::new_by_id(id).get(&mut crate::db::establish_connection()) {
        match UFrage::get_by_umfrage(id/*, &mut crate::db::establish_connection() */) {
            Ok(questions) => {
                let mut antwmoegl: Vec<Vec<UAntwort>> = vec![];
                for ufrage in &questions {
                    if let Ok(d) = ufrage.get_uantworten() {
                        antwmoegl.push(d);
                    }
                }
                Ok(Template::render("tests/feedback/runde1/umfrage/view", context! {
                    umfrage,
                    questions,
                    antwortmoeglichkeiten: antwmoegl,
                    api_new: ApiKey::api_new().expect("ApiKey muss existieren (normal durch migration)"),
                 }))
            },
            Err(_) => {
                let questions: Vec<UFrage> = vec![];
                let antwmoegl: Vec<Vec<UAntwort>> = vec![];
                Ok(Template::render("tests/feedback/runde1/umfrage/view", context! {
                    umfrage,
                    questions,
                    antwortmoeglichkeiten: antwmoegl,
                    api_new: ApiKey::api_new().expect("ApiKey muss existieren (normal durch migration)"),
                 }))
            }
        }
    } else {
        Err(Status::NotFound)
    }
}

#[get("/umfrage/result/<id>")]
pub async fn umfrage_result(id: i32, _perm: HTMLPermission) -> Result<Template, Status> {
    if let Ok(umfrage) = Umfrage::new_by_id(id).get(&mut crate::db::establish_connection()) {
        let result = umfrage.result();
        Ok(Template::render("tests/feedback/runde1/umfrage/result", context! {
            result
        }))
    } else {
        Err(Status::NotFound)
    }
}


#[get("/games/simonsays")]
pub async fn simonsays(_perm: HTMLPermission) -> Template {
    Template::render("tests/feedback/runde1/simonsays", context! {
    })
}


#[get("/wordpress_wrapper/<index>")]
pub async fn wordpress_post(index: usize) -> Template {
    let uri = dotenvy::var("WORDPRESS_URL").expect("wp-lib requires WORDPRESS_URL - api");
    let posts = wp_lib::Post::get_from_uri_limited(&uri, 3).unwrap();
    Template::render("tests/feedback/runde1/wordpress_post_wrapper", context! {
        title: posts[index].title.rendered.clone(),
        index
    })
}
