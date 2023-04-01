use crate::db::v1::models::*;
use rocket::form::Form;
use rocket::http::Status;

use crate::utils::from::*;
use crate::utils::v1::DBQueryableUtils;
use crate::db::v1::DBQueryable;
use crate::utils::v1::cookies::{ApiKey};


#[get("/umfrage/<id>") ]
pub fn umfrage(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Umfrage::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/medien/<id>") ]
pub fn medien(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Medien::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/template/<id>") ]
pub fn template(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Template::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/tparameter/<id>") ]
pub fn tparameter(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match TParameter::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[get("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match TemplateTParameter::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

#[get("/benutzer/<id>") ]
pub fn benutzer(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Benutzer::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/ufrage/<id>") ]
pub fn ufrage(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UFrage::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/uantwort/<id>") ]
pub fn uantwort(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UAntwort::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[get("/ufrageuantwort/<id>") ]
pub fn ufrageuantwort(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UFrageUAntwort::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

/*#[get("/umfragebenutzer/<id>") ]
pub fn umfragebenutzer(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UmfrageBenutzer::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

/*#[get("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Umfrageantwort::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

#[get("/artikel/<id>") ]
pub fn artikel(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Artikel::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[get("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match ArtikelAutor::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/






#[get("/sspiel/<id>") ]
pub fn sspiel(id: i32, apikey: ApiKey) -> Result<rocket::serde::json::Json<usize>, Status> {
	match SSpiel::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/mspiel/<id>") ]
pub fn mspiel(id: i32, apikey: ApiKey) -> Result<rocket::serde::json::Json<usize>, Status> {
	match MSpiel::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
/*
#[get("/sspieler/<id>") ]
pub fn sspieler(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match SSpieler::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/mspieler/<id>") ]
pub fn mspieler(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match MSpieler::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
*/
#[get("/team/<id>") ]
pub fn team(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Team::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
/*
#[get("/benutzerteam/<id>") ]
pub fn benutzerteam(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match BenutzerTeam::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
*/
