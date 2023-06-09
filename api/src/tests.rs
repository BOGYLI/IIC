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



#[get("/wordpress/<index>")]
pub async fn wordpress_post(index: usize) -> Template {
    let posts = wp_lib::Post::get_from_uri("https://bodensee-gymnasium.de").unwrap();
    println!("{:?}", posts);
    Template::render("placeholders/wordpress", context! {
        title: posts[index].title.rendered.clone(),
        content: posts[index].content.rendered.clone()
    })
}