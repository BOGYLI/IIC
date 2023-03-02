use crate::db::v1::models::*;
use rocket::form::Form;
use rocket::http::Status;

use crate::utils::from::*;
use crate::utils::v1::DBQueryableUtils;
use crate::db::v1::DBQueryable;


#[post("/umfrage/<id>") ]
pub fn umfrage(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Umfrage::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/medien/<id>") ]
pub fn medien(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Medien::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/template/<id>") ]
pub fn template(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Template::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/tparameter/<id>") ]
pub fn tparameter(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match TParameter::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[post("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match TemplateTParameter::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

#[post("/benutzer/<id>") ]
pub fn benutzer(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Benutzer::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/ufrage/<id>") ]
pub fn ufrage(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UFrage::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/uantwort/<id>") ]
pub fn uantwort(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UAntwort::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[post("/ufrageuantwort/<id>") ]
pub fn ufrageuantwort(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UFrageUAntwort::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

/*#[post("/umfragebenutzer/<id>") ]
pub fn umfragebenutzer(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UmfrageBenutzer::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

/*#[post("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Umfrageantwort::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/

#[post("/artikel/<id>") ]
pub fn artikel(id: i32) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Artikel::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

/*#[post("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>) -> Result<rocket::serde::json::Json<usize>, Status> {
	match ArtikelAutor::new_by_id(id).delete(&mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}*/






