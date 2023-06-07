use crate::db::models::*;
use crate::db::*;

//use rocket::form::Form;
//use rocket::response::content;
//use rocket::http::{ContentType, Status};
use rocket::http::{Status};
use crate::utils::cookies::{ReadPermission};


#[get("/umfrageantwort") ]
pub fn umfrageantwort(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Umfrageantwort>>, Status> {
	match Umfrageantwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfrage") ]
pub fn umfrage(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Umfrage>>, Status> {
	match Umfrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfragebenutzer") ]
pub fn umfragebenutzer(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UmfrageBenutzer>>, Status> {
	match UmfrageBenutzer::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/umfrageufrage") ]
pub fn umfrageufrage(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UmfrageUFrage>>, Status> {
	match UmfrageUFrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/ufrage") ]
pub fn ufrage(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UFrage>>, Status> {
	match UFrage::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/ufrageuantwort") ]
pub fn ufrageuantwort(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UFrageUAntwort>>, Status> {
	match UFrageUAntwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/uantwort") ]
pub fn uantwort(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<UAntwort>>, Status> {
	match UAntwort::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


////////////////////////////////////////////////////////////


#[get("/medien") ]
pub fn medien(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Medien>>, Status> {
	match Medien::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikelmedien") ]
pub fn artikelmedien(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<ArtikelMedien>>, Status> {
	match ArtikelMedien::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikel") ]
pub fn artikel(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Artikel>>, Status> {
	match Artikel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/artikelautor") ]
pub fn artikelautor(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<ArtikelAutor>>, Status> {
	match ArtikelAutor::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/template") ]
pub fn template(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Template>>, Status> {
	match Template::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/templatetparameter") ]
pub fn templatetparameter(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<TemplateTParameter>>, Status> {
	match TemplateTParameter::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/tparameter") ]
pub fn tparameter(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<TParameter>>, Status> {
	match TParameter::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


/////////////////////////////////////////////////////////////////


#[get("/benutzer") ]
pub fn benutzer(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Benutzer>>, Status> {
	match Benutzer::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/benutzerberechtigung") ]
pub fn benutzerberechtigung(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<BenutzerBerechtigung>>, Status> {
	match BenutzerBerechtigung::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/berechtigung") ]
pub fn berechtigung(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Berechtigung>>, Status> {
	match Berechtigung::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/apikey") ]
pub fn apikey(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<ApiKey>>, Status> {
	match ApiKey::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


////////////////////////////////////////////////////////////////


#[get("/sspiel") ]
pub fn sspiel(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<SSpiel>>, Status> {
	match SSpiel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/sspieler") ]
pub fn sspieler(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<SSpieler>>, Status> {
	match SSpieler::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


/////////////////////////////////////////////////////////////////


#[get("/mspiel") ]
pub fn mspiel(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<MSpiel>>, Status> {
	match MSpiel::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/mspieler") ]
pub fn mspieler(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<MSpieler>>, Status> {
	match MSpieler::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/team") ]
pub fn team(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<Team>>, Status> {
	match Team::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/benutzerteam") ]
pub fn benutzerteam(_read: ReadPermission) -> Result<rocket::serde::json::Json<Vec<BenutzerTeam>>, Status> {
	match BenutzerTeam::get_all(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
