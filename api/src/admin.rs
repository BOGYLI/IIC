use rocket_dyn_templates::{Template, context};
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

use crate::db::models::*;
use crate::db::*;

//use crate::utils::cookies::Admin;
#[get("/dashboard")]
pub async fn dashboard(/*rolle: Admin*/) -> Template {
    let mut sspiel_data: Vec<SSpiel> = vec![];
    if let Ok(data) = SSpiel::get_all(&mut crate::db::establish_connection()) {
        sspiel_data = data;
    }
    Template::render("admin/dashboard", context! {
        sspiel_data: sspiel_data
    })
}