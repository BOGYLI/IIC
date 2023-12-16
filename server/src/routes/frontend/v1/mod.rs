use std::collections::HashMap;
use actix_web::{get, web, error, Responder, Error};
use actix_web_lab::respond::Html;

mod games;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/v1")
            .service(index)
            .service(idlescreen)
            .service(games_index)
            .configure(games::config)
            .service(karte)
            .service(geburtstag)
            .service(news)
            .service(wordpress)
            .service(wordpress_wrapper)
            .service(umfragen)



            .service(test)
            .service(test2)
    );
}


#[get("/")]
async fn index(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("init", "/f/v1/idlescreen");
    Ok(Html(tmpl.render("frontend/v1/index.html", &ctx)
    .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?))
}


#[get("/idlescreen")]
async fn idlescreen(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let uri = dotenvy::var("WORDPRESS_URL").expect("wp-lib requires WORDPRESS_URL - api");
    let posts = wp_lib::Post::get_from_uri_limited(&uri, 3).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("birthday", &false);
    ctx.insert("articles", &posts);
    match tmpl.render("frontend/v1/idlescreen.html", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}



#[get("/games")]
async fn games_index(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    match tmpl.render("frontend/v1/feedback/runde1/games.html.tera", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}



#[get("/karte")]
async fn karte(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    match tmpl.render("frontend/v1/feedback/runde1/karte.html.tera", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}



#[get("/geburtstag")]
async fn geburtstag(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("birthday", &false);
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("geburtstagsgruss_upload", "/api/v1/new/geburtstagsgruss");
    match tmpl.render("frontend/v1/feedback/runde1/geburtstag.html.tera", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}



#[get("/news")]
async fn news(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {
    let uri = dotenvy::var("WORDPRESS_URL").expect("wp-lib requires WORDPRESS_URL - api");
    let posts = wp_lib::Post::get_from_uri_limited(&uri, 16).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("list", &posts);
    match tmpl.render("frontend/v1/feedback/runde1/news.html.tera", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}



#[get("/wordpress/{count}/{index}")]
async fn wordpress(
    tmpl: web::Data<tera::Tera>,
    path: web::Path<(i64, usize)>

) -> Result<impl Responder, Error> {
    let (count, index_) = path.into_inner();
    let uri = dotenvy::var("WORDPRESS_URL").expect("wp-lib requires WORDPRESS_URL - api");
    let posts = wp_lib::Post::get_from_uri_limited(&uri, count).unwrap();

    if posts[index_].content.rendered.contains("https://bodensee-gymnasium.de/wp-content/uploads") && (posts[index_].content.rendered.len() < 500) {
        // Probably pdf file
        if let Ok(mut resp) = reqwest::get(&posts[index_].content.rendered).await {
            
        }
    }

    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("title", &posts[index_].title.rendered);
    ctx.insert("content", &posts[index_].content.rendered);
    match tmpl.render("frontend/v1/feedback/runde1/wordpress_post.html.tera", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}



#[get("/wordpress_wrapper/{count}/{index}")]
async fn wordpress_wrapper(
    tmpl: web::Data<tera::Tera>,
    path: web::Path<(i64, usize)>

) -> Result<impl Responder, Error> {
    let (count, index_) = path.into_inner();
    let uri = dotenvy::var("WORDPRESS_URL").expect("wp-lib requires WORDPRESS_URL - api");
    let posts = wp_lib::Post::get_from_uri_limited(&uri, count).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("title", &posts[index_].title.rendered);
    ctx.insert("index", &index_);
    ctx.insert("count", &count);
    match tmpl.render("frontend/v1/feedback/runde1/wordpress_post_wrapper.html.tera", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}



#[get("/umfragen")]
async fn umfragen(
    tmpl: web::Data<tera::Tera>,
) -> Result<impl Responder, Error> {

    let mut ctx = tera::Context::new();
    ctx.insert("idlescreen_url", "/f/v1/idlescreen");
    ctx.insert("umfragen", &Vec::<String>::new());
    match tmpl.render("frontend/v1/feedback/runde1/umfragen.html.tera", &ctx) {
        Ok(body) => Ok(actix_web::HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        },
    }
}









































#[actix_web::get("/test/{count}/{index}")]
async fn test2(
    tmpl: web::Data<tera::Tera>,
    path: web::Path<(i64, usize)>

) -> Result<impl actix_web::Responder, actix_web::Error> {
    let (count, index_) = path.into_inner();
    Ok(format!("{}, {}", count, index_))
}








#[get("/test")]
async fn test(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, Error> {
    let s = if let Some(name) = query.get("name") {
        // submitted form
        let mut ctx = tera::Context::new();
        ctx.insert("name", name);
        ctx.insert("text", "Welcome!");
        tmpl.render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };

    Ok(Html(s))
}