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
    Template::render("tests/feedback/runde1/wordpress_post", context! {
        title: posts[index].title.rendered.clone(),
        content: posts[index].content.rendered.clone()
    })
}


/*#[get("/wordpress/<id>/banner")]
pub async fn wordpress_banner(id: i64) -> Template {
    let posts = wp_lib::Media::from(id).unwrap();
    println!("{:?}", posts);
    Template::render("placeholders/wordpress", context! {
        title: posts[index].title.rendered.clone(),
        content: posts[index].content.rendered.clone()
    })
}*/

use std::collections::BTreeMap;
use rocket_dyn_templates::serde::json::Value;
use tera;
use tera::Function;
use std::collections::HashMap;
//use rocket_contrib::json::JsonValue;
use rocket::serde::json::from_value;
use rocket::serde::json::to_value;
use tera::Number;

pub fn media_url(_urls: BTreeMap<String, String>) -> impl Function {
    Box::new(move |args: &HashMap<std::string::String, Value>| -> Result<Value, tera::Error> {
        match args.get("id") {
            Some(val) => match from_value::<Number>(val.clone()) {
                Ok(v) =>  {
                    let num = from_value::<Number>(tera::Value::Number(v)).unwrap();
                    let uri = dotenvy::var("WORDPRESS_URL").expect("wp-lib requires WORDPRESS_URL - api");
                    let valu = to_value(wp_lib::Media::from(&uri, num.to_string()).unwrap().guid.rendered).unwrap();
                    Ok(valu)},
                Err(e) => Err(e.into()),
            },
            None => Err("oops".into()),
        }
    })
}

pub fn format_date(_urls: BTreeMap<String, String>) -> impl Function {
    Box::new(move |args: &HashMap<std::string::String, Value>| -> Result<Value, tera::Error> {
        match args.get("date") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) =>  {
                    let date = v.split('T').collect::<Vec<&str>>();
                    Ok(to_value(date[0]).unwrap())
                },
                Err(e) => Err(e.into()),
            },
            None => Err("oops".into()),
        }
    })
}

/*pub fn media_url(urls: BTreeMap<String, i64>) -> impl tera::Function {
    Box::new(move |args: BTreeMap<String, i64>| -> Result<Value, Value> {
        match args.get("id") {
            Some(val) => match from_value::<i64>(val.clone()) {
                Ok(v) =>  Ok(to_value(wp_lib::Media::from(val).unwrap().guid.rendered).unwrap()),
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}*/
