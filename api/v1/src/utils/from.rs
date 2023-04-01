use super::DBQueryableUtils;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;


impl DBQueryableUtils<models::Umfrage, schema::umfrage::SqlType> for models::Umfrage {
    fn new_by_id(id: i32) -> models::Umfrage {
        models::Umfrage {
            id: id,
            titel: String::new(),
        }
    }
}

impl DBQueryableUtils<models::Medien, schema::medien::SqlType> for models::Medien {
    fn new_by_id(id: i32) -> models::Medien {
        models::Medien {
            id: id,
            typ: String::new(),
            pfad: String::new(),
            erstelldatum: String::new(),
        }
    }
}

impl DBQueryableUtils<models::Template, schema::template::SqlType> for models::Template {
    fn new_by_id(id: i32) -> models::Template {
        models::Template {
            id: id,
            pfad: String::new(),
        }
    }
}

impl DBQueryableUtils<models::TParameter, schema::tparameter::SqlType> for models::TParameter {
    fn new_by_id(id: i32) -> models::TParameter {
        models::TParameter {
            id: id,
            typ: String::new(),
            name: String::new(),
        }
    }
}

/*impl DBQueryableUtils<models::TemplateTParameter, schema::templatetparameter::SqlType> for models::TemplateTParameter {
    fn new_by_id(id: i32) -> models::TemplateTParameter {
        models::TemplateTParameter {
            id: id,
            titel: String::new(),
        }
    }
}*/

impl DBQueryableUtils<models::Benutzer, schema::benutzer::SqlType> for models::Benutzer {
    fn new_by_id(id: i32) -> models::Benutzer {
        models::Benutzer {
            id: id,
            name: String::new(),
            mebistoken: String::new(),
            passwort: String::new(),
        }
    }
}

impl DBQueryableUtils<models::UFrage, schema::ufrage::SqlType> for models::UFrage {
    fn new_by_id(id: i32) -> models::UFrage {
        models::UFrage {
            id: id,
            inhalt: String::new(),
        }
    }
}

impl DBQueryableUtils<models::UAntwort, schema::uantwort::SqlType> for models::UAntwort {
    fn new_by_id(id: i32) -> models::UAntwort {
        models::UAntwort {
            id: id,
            inhalt: String::new(),
            typ: String::new(),
        }
    }
}

/*impl DBQueryableUtils<models::UFrageUAntwort, schema::ufrageuantwort::SqlType> for models::UFrageUAntwort {
    fn new_by_id(id: i32) -> models::UFrageUAntwort {
        models::UFrageUAntwort {
            id: id,
            titel: String::new(),
        }
    }
}*/

/*impl DBQueryableUtils<models::UmfrageBenutzer, schema::umfragebenutzer::SqlType> for models::UmfrageBenutzer {
    fn new_by_id(id: i32) -> models::UmfrageBenutzer {
        models::UmfrageBenutzer {
            id: id,
            titel: String::new(),
        }
    }
}*/

/*impl DBQueryableUtils<models::Umfrageantwort, schema::umfrageantwort::SqlType> for models::Umfrageantwort {
    fn new_by_id(id: i32) -> models::Umfrageantwort {
        models::Umfrageantwort {
            id: id,
            titel: String::new(),
        }
    }
}*/

impl DBQueryableUtils<models::Artikel, schema::artikel::SqlType> for models::Artikel {
    fn new_by_id(id: i32) -> models::Artikel {
        models::Artikel {
            id: id,
            pfad: String::new(),
            erstelldatum: String::new(),
            status: String::new(),
            templateid: 0,
        }
    }
}

/*impl DBQueryableUtils<models::ArtikelAutor, schema::artikelautor::SqlType> for models::ArtikelAutor {
    fn new_by_id(id: i32) -> models::ArtikelAutor {
        models::ArtikelAutor {
            id: id,
            titel: String::new(),
        }
    }
}*/




impl DBQueryableUtils<models::SSpiel, schema::sspiel::SqlType> for models::SSpiel {
    fn new_by_id(id: i32) -> models::SSpiel {
        models::SSpiel {
            id: id,
            name: String::new(),
            url: String::new(),
            apikeyid: 0,
            highscore: None,
            best: None,
        }
    }
}

impl DBQueryableUtils<models::MSpiel, schema::mspiel::SqlType> for models::MSpiel {
    fn new_by_id(id: i32) -> models::MSpiel {
        models::MSpiel {
            id: id,
            name: String::new(),
            url: String::new(),
            apikeyid: 0,
            highscore: None,
            best: None,
        }
    }
}
/*
impl DBQueryableUtils<models::SSpieler, schema::artikel::SqlType> for models::Artikel {
    fn new_by_id(id: i32) -> models::Artikel {
        models::Artikel {
            matchid: id,
            team1id: 0,
            team2id: 0,
            spielid: 0,
            level: 0,
            score1: 0,
            score2: 0,
            einstellungen1: String::new(),
            einstellungen2: String::new(),
        }
    }
}

impl DBQueryableUtils<models::MSpieler, schema::artikel::SqlType> for models::Artikel {
    fn new_by_id(id: i32) -> models::Artikel {
        models::Artikel {
            id: id,
            pfad: String::new(),
            erstelldatum: String::new(),
            status: String::new(),
            templateid: 0,
        }
    }
}
*/
impl DBQueryableUtils<models::Team, schema::team::SqlType> for models::Team {
    fn new_by_id(id: i32) -> models::Team {
        models::Team {
            id: id,
            name: String::new(),
            apikeyid: 0,
            overallscore: 0,
        }
    }
}
/*
impl DBQueryableUtils<models::BenutzerTeam, schema::artikel::SqlType> for models::Artikel {
    fn new_by_id(id: i32) -> models::Artikel {
        models::Artikel {
            id: id,
            pfad: String::new(),
            erstelldatum: String::new(),
            status: String::new(),
            templateid: 0,
        }
    }
}*/