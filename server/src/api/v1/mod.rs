mod delete;
mod update;
mod get_all;
mod get;
mod insert;


pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/v1")
            .configure(delete)
            .configure(update)
            .configure(get_all)
            .configure(get)
            .configure(new)
    );
}

fn delete(cfg: &mut actix_web::web::ServiceConfig) {
    use delete::*;
    cfg.service(
        actix_web::web::scope("/delete")
            // artikel
            .service(delete_artikel)
            .service(delete_artikelbenutzer)
            .service(delete_artikelmedien)
            .service(delete_medien)
            .service(delete_template)
            .service(delete_parameter)
            .service(delete_templateparameter)

            // berechtigung
            .service(delete_benutzer)
            .service(delete_benutzerberechtigung)
            .service(delete_berechtigung)
            .service(delete_apikey)

            // mspiel
            .service(delete_mspiel)
            .service(delete_matchmspiel)
            .service(delete_team)
            .service(delete_benutzerteam)

            // sspiel
            .service(delete_sspiel)
            .service(delete_benutzersspiel)

            // umfrage
            .service(delete_umfrage)
            .service(delete_frage)
            .service(delete_antwort)
            .service(delete_frageantwort)
            .service(delete_umfragebenutzer)
            .service(delete_umfragebenutzerfrage)
            .service(delete_umfragefrage)

            //.route(actix_web::web::head().to(actix_web::HttpResponse::MethodNotAllowed)),
    );
}


fn update(cfg: &mut actix_web::web::ServiceConfig) {
    use update::*;
    cfg.service(
        actix_web::web::scope("/update")
        // artikel
        .service(update_artikel)
        .service(update_artikelbenutzer)
        .service(update_artikelmedien)
        .service(update_medien)
        .service(update_template)
        .service(update_parameter)
        .service(update_templateparameter)

        // berechtigung
        .service(update_benutzer)
        .service(update_benutzerberechtigung)
        .service(update_berechtigung)
        .service(update_apikey)

        // mspiel
        .service(update_mspiel)
        .service(update_matchmspiel)
        .service(update_team)
        .service(update_benutzerteam)

        // sspiel
        .service(update_sspiel)
        .service(update_benutzersspiel)

        // umfrage
        .service(update_umfrage)
        .service(update_frage)
        .service(update_antwort)
        .service(update_frageantwort)
        .service(update_umfragebenutzer)
        .service(update_umfragebenutzerfrage)
        .service(update_umfragefrage)

        // extras
        .service(update_revealjs)
        
            //.route(actix_web::web::head().to(actix_web::HttpResponse::MethodNotAllowed)),
    );
}

fn get_all(cfg: &mut actix_web::web::ServiceConfig) {
    use get_all::*;
    cfg.service(
        actix_web::web::scope("/get_all")
        // artikel
        .service(get_all_artikel)
        .service(get_all_artikelbenutzer)
        .service(get_all_artikelmedien)
        .service(get_all_medien)
        .service(get_all_template)
        .service(get_all_parameter)
        .service(get_all_templateparameter)

        // berechtigung
        .service(get_all_benutzer)
        .service(get_all_benutzerberechtigung)
        .service(get_all_berechtigung)
        .service(get_all_apikey)

        // mspiel
        .service(get_all_mspiel)
        .service(get_all_matchmspiel)
        .service(get_all_team)
        .service(get_all_benutzerteam)

        // sspiel
        .service(get_all_sspiel)
        .service(get_all_benutzersspiel)

        // umfrage
        .service(get_all_umfrage)
        .service(get_all_frage)
        .service(get_all_antwort)
        .service(get_all_frageantwort)
        .service(get_all_umfragebenutzer)
        .service(get_all_umfragebenutzerfrage)
        .service(get_all_umfragefrage)
        
            //.route(actix_web::web::head().to(actix_web::HttpResponse::MethodNotAllowed)),
    );
}

fn get(cfg: &mut actix_web::web::ServiceConfig) {
    use get::*;
    cfg.service(
        actix_web::web::scope("/get")
        // artikel
        .service(get_artikel)
        .service(get_artikelbenutzer)
        .service(get_artikelmedien)
        .service(get_medien)
        .service(get_template)
        .service(get_parameter)
        .service(get_templateparameter)

        // berechtigung
        .service(get_benutzer)
        .service(get_benutzerberechtigung)
        .service(get_berechtigung)
        .service(get_apikey)

        // mspiel
        .service(get_mspiel)
        .service(get_matchmspiel)
        .service(get_team)
        .service(get_benutzerteam)

        // sspiel
        .service(get_sspiel)
        .service(get_benutzersspiel)

        // umfrage
        .service(get_umfrage)
        .service(get_frage)
        .service(get_antwort)
        .service(get_frageantwort)
        .service(get_umfragebenutzer)
        .service(get_umfragebenutzerfrage)
        .service(get_umfragefrage)
        
            //.route(actix_web::web::head().to(actix_web::HttpResponse::MethodNotAllowed)),
    );
}

fn new(cfg: &mut actix_web::web::ServiceConfig) {
    use insert::*;
    cfg.service(
        actix_web::web::scope("/new")
        // artikel
        .service(new_artikel)
        .service(new_artikelbenutzer)
        .service(new_artikelmedien)
        .service(new_medien)
        .service(new_template)
        .service(new_parameter)
        .service(new_templateparameter)

        // berechtigung
        .service(new_benutzer)
        .service(new_benutzerberechtigung)
        .service(new_berechtigung)
        .service(new_apikey)

        // mspiel
        .service(new_mspiel)
        .service(new_matchmspiel)
        .service(new_team)
        .service(new_benutzerteam)

        // sspiel
        .service(new_sspiel)
        .service(new_benutzersspiel)

        // umfrage
        .service(new_umfrage)
        .service(new_frage)
        .service(new_antwort)
        .service(new_frageantwort)
        .service(new_umfragebenutzer)
        .service(new_umfragebenutzerfrage)
        .service(new_umfragefrage)
        
           //.route(actix_web::web::head().to(actix_web::HttpResponse::MethodNotAllowed)),
    );
}