use crate::db::v1::models::*;
use rocket::form::Form;
use crate::db::v1::DBQueryable;
use rocket::http::Status;

#[put("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/umfrage", data = "<data>") ]
pub fn umfrage(data: Form<Umfrage>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/uantwort", data = "<data>") ]
pub fn uantwort(data: Form<UAntwort>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<UmfrageBenutzer>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/ufrage", data = "<data>") ]
pub fn ufrage(data: Form<UFrage>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[put("/medien", data = "<data>") ]
pub fn medien(data: Form<Medien>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/artikel", data = "<data>") ]
pub fn artikel(data: Form<Artikel>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[put("/benutzer", data = "<data>") ]
pub fn benutzer(data: Form<Benutzer>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[put("/template", data = "<data>") ]
pub fn template(data: Form<Template>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/tparameter", data = "<data>") ]
pub fn tparameter(data: Form<TParameter>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::v1::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}