//use crate::db::DBQueryable;

//use diesel::pg::PgConnection;
use diesel::prelude::*;

//pub mod from;
//pub mod cookies;
pub mod apikey;
//pub mod api_permissions;

pub mod tera_fcts;

pub mod date;


pub trait DBQueryableUtils<T: Queryable<S, diesel::pg::Pg>, S> {
    fn new_by_id(id: i32) -> Self where Self: Sized;
}



pub mod files {
    use std::io::Write;

    use actix_multipart::Multipart;
    use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
    use futures::{StreamExt, TryStreamExt};

    pub async fn save_file(mut payload: Multipart, folder_path: String) -> Option<String> {
        let mut filepath_final = String::new();
        // iterate over multipart stream
        while let Ok(Some(mut field)) = payload.try_next().await {
            let content_type = field.content_disposition();
            let filename = content_type.get_filename().unwrap();
            let filepath = format!("./{}/{}", folder_path, filename);
            if std::path::Path::new(&filepath).exists() {
                return None;
            }
            filepath_final = filepath.clone();

            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap()
                .unwrap();

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .unwrap();
            }
        }

        Some(filepath_final)
    }
}