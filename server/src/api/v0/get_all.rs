use crate::db::models::*;
use crate::db::*;

//use rocket::form::Form;
//use rocket::response::content;
//use rocket::http::{ContentType, Status};
use rocket::http::{Status};
use crate::utils::cookies::{ReadPermission};


#[get("/umfrageantwort") ]
pub fn umfrageantwort(_read: ReadPermission) -> impl Responder {
	match Umfrageantwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/umfrage") ]
pub fn umfrage(_read: ReadPermission) -> impl Responder {
	match Umfrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/umfragebenutzer") ]
pub fn umfragebenutzer(_read: ReadPermission) -> impl Responder {
	match UmfrageBenutzer::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/umfrageufrage") ]
pub fn umfrageufrage(_read: ReadPermission) -> impl Responder {
	match UmfrageUFrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/ufrage") ]
pub fn ufrage(_read: ReadPermission) -> impl Responder {
	match UFrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/ufrageuantwort") ]
pub fn ufrageuantwort(_read: ReadPermission) -> impl Responder {
	match UFrageUAntwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/uantwort") ]
pub fn uantwort(_read: ReadPermission) -> impl Responder {
	match UAntwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


////////////////////////////////////////////////////////////


#[get("/medien") ]
pub fn medien(_read: ReadPermission) -> impl Responder {
	match Medien::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/artikelmedien") ]
pub fn artikelmedien(_read: ReadPermission) -> impl Responder {
	match ArtikelMedien::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/artikel") ]
pub fn artikel(_read: ReadPermission) -> impl Responder {
	match Artikel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/artikelautor") ]
pub fn artikelautor(_read: ReadPermission) -> impl Responder {
	match ArtikelAutor::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/template") ]
pub fn template(_read: ReadPermission) -> impl Responder {
	match Template::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/templatetparameter") ]
pub fn templatetparameter(_read: ReadPermission) -> impl Responder {
	match TemplateTParameter::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/tparameter") ]
pub fn tparameter(_read: ReadPermission) -> impl Responder {
	match TParameter::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


/////////////////////////////////////////////////////////////////


#[get("/benutzer") ]
pub fn benutzer(_read: ReadPermission) -> impl Responder {
	match Benutzer::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/benutzerberechtigung") ]
pub fn benutzerberechtigung(_read: ReadPermission) -> impl Responder {
	match BenutzerBerechtigung::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/berechtigung") ]
pub fn berechtigung(_read: ReadPermission) -> impl Responder {
	match Berechtigung::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/apikey") ]
pub fn apikey(_read: ReadPermission) -> impl Responder {
	match ApiKey::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


////////////////////////////////////////////////////////////////


#[get("/sspiel") ]
pub fn sspiel(_read: ReadPermission) -> impl Responder {
	match SSpiel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/sspieler") ]
pub fn sspieler(_read: ReadPermission) -> impl Responder {
	match SSpieler::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


/////////////////////////////////////////////////////////////////


#[get("/mspiel") ]
pub fn mspiel(_read: ReadPermission) -> impl Responder {
	match MSpiel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/mspieler") ]
pub fn mspieler(_read: ReadPermission) -> impl Responder {
	match MSpieler::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/team") ]
pub fn team(_read: ReadPermission) -> impl Responder {
	match Team::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/benutzerteam") ]
pub fn benutzerteam(_read: ReadPermission) -> impl Responder {
	match BenutzerTeam::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
