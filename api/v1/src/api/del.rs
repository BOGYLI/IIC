use crate::db::models::*;
use rocket::form::Form;
use rocket::http::Status;

use crate::utils::from::*;
use crate::utils::DBQueryableUtils;
use crate::db::DBQueryable;
use crate::utils::cookies::{DeletePermission};


//
#[get("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

#[get("/umfrage/<id>") ]
pub fn umfrage(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Umfrage::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<UmfrageBenutzer>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

//
#[get("/umfrageufrage", data = "<data>") ]
pub fn umfrageufrage(data: Form<UmfrageUFrage>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

#[get("/ufrage/<id>") ]
pub fn ufrage(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UFrage::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/ufrageuantwort", data = "<data>") ]
pub fn ufrageuantwort(data: Form<UFrageUAntwort>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

#[get("/uantwort/<id>") ]
pub fn uantwort(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match UAntwort::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


//////////////////////////////////////////////////////////////////////


#[get("/medien/<id>") ]
pub fn medien(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Medien::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/artikelmedien", data = "<data>") ]
pub fn artikelmedien(data: Form<ArtikelMedien>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

#[get("/artikel/<id>") ]
pub fn artikel(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Artikel::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

#[get("/template/<id>") ]
pub fn template(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Template::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

#[get("/tparameter/<id>") ]
pub fn tparameter(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match TParameter::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


///////////////////////////////////////////////////////////////


#[get("/benutzer/<id>") ]
pub fn benutzer(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Benutzer::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/benutzerberechtigung", data = "<data>") ]
pub fn benutzerberechtigung(data: Form<BenutzerBerechtigung>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

#[get("/berechtigung/<id>") ]
pub fn berechtigung(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Berechtigung::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[get("/apikey/<id>") ]
pub fn apikey(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match ApiKey::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}


//////////////////////////////////////////////////////////////


#[get("/sspiel/<id>") ]
pub fn sspiel(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match SSpiel::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/sspieler", data = "<data>") ]
pub fn sspieler(data: Form<SSpieler>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//


////////////////////////////////////////////////////////////


#[get("/mspiel/<id>") ]
pub fn mspiel(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match MSpiel::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/mspieler", data = "<data>") ]
pub fn mspieler(data: Form<MSpieler>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//

#[get("/team/<id>") ]
pub fn team(id: i32, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match Team::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}

//
#[get("/benutzerteam", data = "<data>") ]
pub fn benutzerteam(data: Form<BenutzerTeam>, del: DeletePermission) -> Result<rocket::serde::json::Json<usize>, Status> {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(rocket::serde::json::Json(data)),
		Err(_) => Err(Status::InternalServerError)
	}
}
//
