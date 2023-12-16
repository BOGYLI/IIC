use actix_files as fs;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/static")
            // .service()
            .service(
                fs::Files::new("/", "./frontend/static/")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    );
}