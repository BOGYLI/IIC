use crate::db::models::*;
use crate::db::*;

use rocket::form::Form;
use rocket::response::content;
use rocket::http::{ContentType, Status};
use crate::utils::cookies::{ReadPermission};


#[get("/umfrageantwort") ]
pub fn umfrageantwort(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Umfrageantwort>>, Status> {
	match Umfrageantwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfrage") ]
pub fn umfrage(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Umfrage>>, Status> {
	match Umfrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfragebenutzer") ]
pub fn umfragebenutzer(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UmfrageBenutzer>>, Status> {
	match UmfrageBenutzer::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfrageufrage") ]
pub fn umfrageufrage(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UmfrageUFrage>>, Status> {
	match UmfrageUFrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/ufrage") ]
pub fn ufrage(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UFrage>>, Status> {
	match UFrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/ufrageuantwort") ]
pub fn ufrageuantwort(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UFrageUAntwort>>, Status> {
	match UFrageUAntwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/uantwort") ]
pub fn uantwort(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UAntwort>>, Status> {
	match UAntwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


////////////////////////////////////////////////////////////


#[get("/medien") ]
pub fn medien(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Medien>>, Status> {
	match Medien::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikelmedien") ]
pub fn artikelmedien(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<ArtikelMedien>>, Status> {
	match ArtikelMedien::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikel") ]
pub fn artikel(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Artikel>>, Status> {
	match Artikel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikelautor") ]
pub fn artikelautor(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<ArtikelAutor>>, Status> {
	match ArtikelAutor::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/template") ]
pub fn template(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Template>>, Status> {
	match Template::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/templatetparameter") ]
pub fn templatetparameter(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<TemplateTParameter>>, Status> {
	match TemplateTParameter::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/tparameter") ]
pub fn tparameter(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<TParameter>>, Status> {
	match TParameter::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


/////////////////////////////////////////////////////////////////


#[get("/benutzer") ]
pub fn benutzer(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Benutzer>>, Status> {
	match Benutzer::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/benutzerberechtigung") ]
pub fn benutzerberechtigung(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<BenutzerBerechtigung>>, Status> {
	match BenutzerBerechtigung::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/berechtigung") ]
pub fn berechtigung(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Berechtigung>>, Status> {
	match Berechtigung::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/apikey") ]
pub fn apikey(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<ApiKey>>, Status> {
	match ApiKey::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


////////////////////////////////////////////////////////////////


#[get("/sspiel") ]
pub fn sspiel(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<SSpiel>>, Status> {
	match SSpiel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/sspieler") ]
pub fn sspieler(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<SSpieler>>, Status> {
	match SSpieler::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


/////////////////////////////////////////////////////////////////


#[get("/mspiel") ]
pub fn mspiel(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<MSpiel>>, Status> {
	match MSpiel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/mspieler") ]
pub fn mspieler(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<MSpieler>>, Status> {
	match MSpieler::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/team") ]
pub fn team(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Team>>, Status> {
	match Team::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/benutzerteam") ]
pub fn benutzerteam(read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<BenutzerTeam>>, Status> {
	match BenutzerTeam::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
