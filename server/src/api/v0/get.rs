use crate::db::models::*;
//use rocket::form::Form;
use rocket::http::Status;

//use crate::utils::from::*;
use crate::utils::DBQueryableUtils;
use crate::db::DBQueryable;
use crate::utils::cookies::{ReadPermission};



/*
#[post("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>) -> impl Responder {
	match data.into_inner() {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[post("/umfrage/<id>") ]
pub fn umfrage(id: i32, _read: ReadPermission) -> impl Responder {
	match Umfrage::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[post("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<UmfrageBenutzer>) -> impl Responder {
	match data.into_inner() {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

/*
#[post("/umfrageufrage", data = "<data>") ]
pub fn umfrageufrage(data: Form<UmfrageBenutzer>) -> impl Responder {
	match data.into_inner() {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[post("/ufrage/<id>") ]
pub fn ufrage(id: i32, _read: ReadPermission) -> impl Responder {
	match UFrage::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[post("/ufrageuantwort", data = "<data>") ]
pub fn ufrageuantwort(data: Form<UmfrageBenutzer>) -> impl Responder {
	match data.into_inner() {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[post("/uantwort/<id>") ]
pub fn uantwort(id: i32, _read: ReadPermission) -> impl Responder {
	match UAntwort::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


///////////////////////////////////////////////////////////////


#[post("/medien/<id>") ]
pub fn medien(id: i32, _read: ReadPermission) -> impl Responder {
	match Medien::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[post("/artikelmedien", data = "<data>") ]
pub fn artikelmedien(data: Form<UmfrageBenutzer>) -> impl Responder {
	match data.into_inner() {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[post("/artikel/<id>") ]
pub fn artikel(id: i32, _read: ReadPermission) -> impl Responder {
	match Artikel::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*#[post("/artikelautor/del", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>) -> impl Responder {
	match ArtikelAutor::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}*/

#[post("/template/<id>") ]
pub fn template(id: i32, _read: ReadPermission) -> impl Responder {
	match Template::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*#[post("/templatetparameter/del", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>) -> impl Responder {
	match TemplateTParameter::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}*/

#[post("/tparameter/<id>") ]
pub fn tparameter(id: i32, _read: ReadPermission) -> impl Responder {
	match TParameter::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


/////////////////////////////////////////////////////////////////


#[post("/benutzer/<id>") ]
pub fn benutzer(id: i32, _read: ReadPermission) -> impl Responder {
	match Benutzer::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*#[post("/benutzerberechtigung/<id>") ]
pub fn benutzerberechtigung(id: i32) -> impl Responder {
	match UmfrageBenutzer::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}*/

#[post("/berechtigung/<id>") ]
pub fn berechtigung(id: i32, _read: ReadPermission) -> impl Responder {
	match Berechtigung::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[post("/apikey/<id>") ]
pub fn apikey(id: i32, _read: ReadPermission) -> impl Responder {
	match ApiKey::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


////////////////////////////////////////////////////////////////


#[post("/sspiel/<id>") ]
pub fn sspiel(id: i32, _read: ReadPermission) -> impl Responder {
	match SSpiel::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*#[post("/sspieler/<id>") ]
pub fn sspieler(id: i32) -> Result<web::Json<SSpieler>, Status> {
	match SSpieler::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/


///////////////////////////////////////////////////////////////


#[post("/mspiel/<id>") ]
pub fn mspiel(id: i32, _read: ReadPermission) -> impl Responder {
	match MSpiel::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[post("/mspieler/<id>") ]
pub fn mspieler(id: i32) -> Result<web::Json<MSpieler>, Status> {
	match MSpieler::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}*/

#[post("/team/<id>") ]
pub fn team(id: i32, _read: ReadPermission) -> impl Responder {
	match Team::new_by_id(id).get(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*#[post("/benutzerteam/<id>") ]
pub fn benutzerteam(id: i32) -> Result<web::Json<BenutzerTeam>, Status> {
	match BenutzerTeam::new_by_id(id).get(&mut crate::db::establish_connection()()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}*/
