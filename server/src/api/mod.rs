mod v1;
// mod v2;


pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api")
            .configure(v1::config)
            // .configure(v2::config)
    );
}