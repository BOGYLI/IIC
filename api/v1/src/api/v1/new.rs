use crate::db::v1::models::*;
use rocket::form::Form;
use rocket::http::Status;

use crate::db::v1::DBInsertable;

#[post("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<NewUmfrageantwort>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/umfrage", data = "<data>") ]
pub fn umfrage(data: Form<NewUmfrage>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/uantwort", data = "<data>") ]
pub fn uantwort(data: Form<NewUAntwort>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<NewUmfrageBenutzer>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/ufrage", data = "<data>") ]
pub fn ufrage(data: Form<NewUFrage>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[post("/medien", data = "<data>") ]
pub fn medien(data: Form<NewMedien>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/artikel", data = "<data>") ]
pub fn artikel(data: Form<NewArtikel>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<NewArtikelAutor>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[post("/benutzer", data = "<data>") ]
pub fn benutzer(data: Form<NewBenutzer>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[post("/template", data = "<data>") ]
pub fn template(data: Form<NewTemplate>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<NewTemplateTParameter>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/tparameter", data = "<data>") ]
pub fn tparameter(data: Form<NewTParameter>) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}