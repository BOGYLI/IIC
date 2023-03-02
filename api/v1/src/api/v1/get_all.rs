use crate::db::v1::models::*;
use crate::db::v1::*;

use rocket::form::Form;
use rocket::response::content;
use rocket::http::{ContentType, Status};


#[get("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>) -> Result<rocket::serde::json::Json<Vec<Umfrageantwort>>, Status> {
	match Umfrageantwort::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfrage", data = "<data>") ]
pub fn umfrage(data: Form<Umfrage>) -> Result<rocket::serde::json::Json<Vec<Umfrage>>, Status> {
	match Umfrage::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/uantwort", data = "<data>") ]
pub fn uantwort(data: Form<UAntwort>) -> Result<rocket::serde::json::Json<Vec<UAntwort>>, Status> {
	match UAntwort::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<UmfrageBenutzer>) -> Result<rocket::serde::json::Json<Vec<UmfrageBenutzer>>, Status> {
	match UmfrageBenutzer::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/ufrage", data = "<data>") ]
pub fn ufrage(data: Form<UFrage>) -> Result<rocket::serde::json::Json<Vec<UFrage>>, Status> {
	match UFrage::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[get("/medien", data = "<data>") ]
pub fn medien(data: Form<Medien>) -> Result<rocket::serde::json::Json<Vec<Medien>>, Status> {
	match Medien::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikel", data = "<data>") ]
pub fn artikel(data: Form<Artikel>) -> Result<rocket::serde::json::Json<Vec<Artikel>>, Status> {
	match Artikel::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>) -> Result<rocket::serde::json::Json<Vec<ArtikelAutor>>, Status> {
	match ArtikelAutor::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[get("/benutzer", data = "<data>") ]
pub fn benutzer(data: Form<Benutzer>) -> Result<rocket::serde::json::Json<Vec<Benutzer>>, Status> {
	match Benutzer::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[get("/template", data = "<data>") ]
pub fn template(data: Form<Template>) -> Result<rocket::serde::json::Json<Vec<Template>>, Status> {
	match Template::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>) -> Result<rocket::serde::json::Json<Vec<TemplateTParameter>>, Status> {
	match TemplateTParameter::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/tparameter", data = "<data>") ]
pub fn tparameter(data: Form<TParameter>) -> Result<rocket::serde::json::Json<Vec<TParameter>>, Status> {
	match TParameter::get_all(&data.into_inner(), &mut crate::db::v1::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}