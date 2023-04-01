use crate::db::models::*;
use crate::db::*;

use rocket::form::Form;
use rocket::response::content;
use rocket::http::{ContentType, Status};
use crate::utils::cookies::{ApiKey};


#[get("/umfrageantwort") ]
pub fn umfrageantwort() -> Result<rocket::serde::json::Json<Vec<Umfrageantwort>>, Status> {
	match Umfrageantwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfrage") ]
pub fn umfrage() -> Result<rocket::serde::json::Json<Vec<Umfrage>>, Status> {
	match Umfrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/uantwort") ]
pub fn uantwort() -> Result<rocket::serde::json::Json<Vec<UAntwort>>, Status> {
	match UAntwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfragebenutzer") ]
pub fn umfragebenutzer() -> Result<rocket::serde::json::Json<Vec<UmfrageBenutzer>>, Status> {
	match UmfrageBenutzer::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/ufrage") ]
pub fn ufrage() -> Result<rocket::serde::json::Json<Vec<UFrage>>, Status> {
	match UFrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[get("/medien") ]
pub fn medien() -> Result<rocket::serde::json::Json<Vec<Medien>>, Status> {
	match Medien::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikel") ]
pub fn artikel() -> Result<rocket::serde::json::Json<Vec<Artikel>>, Status> {
	match Artikel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikelautor") ]
pub fn artikelautor() -> Result<rocket::serde::json::Json<Vec<ArtikelAutor>>, Status> {
	match ArtikelAutor::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[get("/benutzer") ]
pub fn benutzer() -> Result<rocket::serde::json::Json<Vec<Benutzer>>, Status> {
	match Benutzer::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[get("/template") ]
pub fn template() -> Result<rocket::serde::json::Json<Vec<Template>>, Status> {
	match Template::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/templatetparameter") ]
pub fn templatetparameter() -> Result<rocket::serde::json::Json<Vec<TemplateTParameter>>, Status> {
	match TemplateTParameter::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/tparameter") ]
pub fn tparameter() -> Result<rocket::serde::json::Json<Vec<TParameter>>, Status> {
	match TParameter::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}




#[get("/sspiel") ]
pub fn sspiel(apikey: ApiKey) -> Result<rocket::serde::json::Json<Vec<SSpiel>>, Status> {
	match SSpiel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/mspiel") ]
pub fn mspiel(apikey: ApiKey) -> Result<rocket::serde::json::Json<Vec<MSpiel>>, Status> {
	match MSpiel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/sspieler") ]
pub fn sspieler(apikey: ApiKey) -> Result<rocket::serde::json::Json<Vec<SSpieler>>, Status> {
	match SSpieler::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/mspieler") ]
pub fn mspieler(apikey: ApiKey) -> Result<rocket::serde::json::Json<Vec<MSpieler>>, Status> {
	match MSpieler::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/team") ]
pub fn team() -> Result<rocket::serde::json::Json<Vec<Team>>, Status> {
	match Team::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/benutzerteam") ]
pub fn benutzerteam() -> Result<rocket::serde::json::Json<Vec<BenutzerTeam>>, Status> {
	match BenutzerTeam::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
