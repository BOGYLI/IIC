use rocket_dyn_templates::{Template, context};
use std::path::{Path, PathBuf};
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


use rocket::fs::TempFile;

use crate::db;
use crate::api;
use crate::utils;
use crate::schema;
use crate::catchers;

use utils::cookies::*;

#[derive(Debug, Responder)]
pub enum ResponseError {
    #[response(status = 400)]
    BadRequest(Template),
    #[response(status = 401)]
    Unauthorized(Template),
    #[response(status = 404)]
    NotFound(Template),
    #[response(status = 423)]
    Locked(Template),
    #[response(status = 500)]
    InternalServerError(Template),
}

#[catch(401)]
pub fn not_authorized(req: &Request) -> ResponseError {
    ResponseError::Unauthorized(Template::render("errors/401", context! {
        uri: req.uri()
    }))
}
#[catch(404)]
pub fn not_found(req: &Request) -> ResponseError {
    ResponseError::NotFound(Template::render("errors/404", context! {
        request: req.to_string(),
        uri: req.uri()
    }))
}
#[catch(423)]
pub fn locked(req: &Request) -> ResponseError {
    ResponseError::Unauthorized(Template::render("errors/nopin", context! {
        
    }))
}
#[catch(500)]
pub fn internal(req: &Request) -> ResponseError {
    ResponseError::InternalServerError(Template::render("errors/500", context! {
        uri: req.uri()
    }))
}
