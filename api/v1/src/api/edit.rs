use crate::db::models::*;
use rocket::form::Form;
use crate::db::DBQueryable;
use rocket::http::Status;
use crate::utils::cookies::{ApiKey};

#[put("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/umfrage", data = "<data>") ]
pub fn umfrage(data: Form<Umfrage>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/uantwort", data = "<data>") ]
pub fn uantwort(data: Form<UAntwort>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<UmfrageBenutzer>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/ufrage", data = "<data>") ]
pub fn ufrage(data: Form<UFrage>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[put("/medien", data = "<data>") ]
pub fn medien(data: Form<Medien>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/artikel", data = "<data>") ]
pub fn artikel(data: Form<Artikel>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[put("/benutzer", data = "<data>") ]
pub fn benutzer(data: Form<Benutzer>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}



#[put("/template", data = "<data>") ]
pub fn template(data: Form<Template>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/tparameter", data = "<data>") ]
pub fn tparameter(data: Form<TParameter>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}




#[put("/sspiel", data = "<data>") ]
pub fn sspiel(data: Form<SSpiel>, apikey: ApiKey) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/mspiel", data = "<data>") ]
pub fn mspiel(data: Form<MSpiel>, apikey: ApiKey) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/sspieler", data = "<data>") ]
pub fn sspieler(data: Form<SSpieler>, apikey: ApiKey) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/mspieler", data = "<data>") ]
pub fn mspieler(data: Form<MSpieler>, apikey: ApiKey) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/team", data = "<data>") ]
pub fn team(data: Form<Team>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[put("/benutzerteam", data = "<data>") ]
pub fn benutzerteam(data: Form<BenutzerTeam>) -> Result<Status, Status> {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}
