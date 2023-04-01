use crate::db::models::*;
use rocket::form::Form;
use rocket::http::Status;

use crate::utils::from::*;
use crate::utils::DBQueryableUtils;
use crate::db::DBQueryable;
use crate::utils::cookies::{ApiKey};


#[post("/umfrage/<id>") ]
pub fn umfrage(id: i32) -> Result<rocket::serde::json::Json<Umfrage>, Status> {
	match Umfrage::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/medien/<id>") ]
pub fn medien(id: i32) -> Result<rocket::serde::json::Json<Medien>, Status> {
	match Medien::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/template/<id>") ]
pub fn template(id: i32) -> Result<rocket::serde::json::Json<Template>, Status> {
	match Template::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/tparameter/<id>") ]
pub fn tparameter(id: i32) -> Result<rocket::serde::json::Json<TParameter>, Status> {
	match TParameter::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[post("/templatetparameter/del", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match TemplateTParameter::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

#[post("/benutzer/<id>") ]
pub fn benutzer(id: i32) -> Result<rocket::serde::json::Json<Benutzer>, Status> {
	match Benutzer::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/ufrage/<id>") ]
pub fn ufrage(id: i32) -> Result<rocket::serde::json::Json<UFrage>, Status> {
	match UFrage::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/uantwort/<id>") ]
pub fn uantwort(id: i32) -> Result<rocket::serde::json::Json<UAntwort>, Status> {
	match UAntwort::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[post("/ufrageuantwort/<id>") ]
pub fn ufrageuantwort(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UFrageUAntwort::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

/*#[post("/umfragebenutzer/<id>") ]
pub fn umfragebenutzer(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UmfrageBenutzer::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

/*#[post("/umfrageantwort/del", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Umfrageantwort::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

#[post("/artikel/<id>") ]
pub fn artikel(id: i32) -> Result<rocket::serde::json::Json<Artikel>, Status> {
	match Artikel::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[post("/artikelautor/del", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match ArtikelAutor::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/





#[post("/sspiel/<id>") ]
pub fn sspiel(id: i32, apikey: ApiKey) -> Result<rocket::serde::json::Json<SSpiel>, Status> {
	match SSpiel::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/mspiel/<id>") ]
pub fn mspiel(id: i32, apikey: ApiKey) -> Result<rocket::serde::json::Json<MSpiel>, Status> {
	match MSpiel::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[post("/sspieler/<id>") ]
pub fn sspieler(id: i32) -> Result<rocket::serde::json::Json<SSpieler>, Status> {
	match SSpieler::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/mspieler/<id>") ]
pub fn mspieler(id: i32) -> Result<rocket::serde::json::Json<MSpieler>, Status> {
	match MSpieler::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

#[post("/team/<id>") ]
pub fn team(id: i32) -> Result<rocket::serde::json::Json<Team>, Status> {
	match Team::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[post("/benutzerteam/<id>") ]
pub fn benutzerteam(id: i32) -> Result<rocket::serde::json::Json<BenutzerTeam>, Status> {
	match BenutzerTeam::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/


