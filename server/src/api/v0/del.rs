use crate::db::models::*;
use actix_web::web::Form;
use actix_web::web;
use rocket::http::Status;

//use crate::utils::from::*;
use crate::utils::DBQueryableUtils;
use crate::db::DBQueryable;
use crate::utils::cookies::{DeletePermission};


//
#[get("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[get("/umfrage/<id>") ]
pub fn umfrage(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match Umfrage::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<UmfrageBenutzer>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

//
#[get("/umfrageufrage", data = "<data>") ]
pub fn umfrageufrage(data: Form<UmfrageUFrage>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[get("/ufrage/<id>") ]
pub fn ufrage(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match UFrage::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/ufrageuantwort", data = "<data>") ]
pub fn ufrageuantwort(data: Form<UFrageUAntwort>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[get("/uantwort/<id>") ]
pub fn uantwort(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match UAntwort::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


//////////////////////////////////////////////////////////////////////


#[get("/medien/<id>") ]
pub fn medien(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match Medien::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/artikelmedien", data = "<data>") ]
pub fn artikelmedien(data: Form<ArtikelMedien>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[get("/artikel/<id>") ]
pub fn artikel(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	match Artikel::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[get("/template/<id>") ]
pub fn template(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match Template::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[get("/tparameter/<id>") ]
pub fn tparameter(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match TParameter::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


///////////////////////////////////////////////////////////////


#[get("/benutzer/<id>") ]
pub fn benutzer(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match Benutzer::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/benutzerberechtigung", data = "<data>") ]
pub fn benutzerberechtigung(data: Form<BenutzerBerechtigung>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[get("/berechtigung/<id>") ]
pub fn berechtigung(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match Berechtigung::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[get("/apikey/<id>") ]
pub fn apikey(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match ApiKey::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


//////////////////////////////////////////////////////////////


#[get("/sspiel/<id>") ]
pub fn sspiel(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match SSpiel::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/sspieler", data = "<data>") ]
pub fn sspieler(data: Form<SSpieler>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//


////////////////////////////////////////////////////////////


#[get("/mspiel/<id>") ]
pub fn mspiel(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match MSpiel::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/mspieler", data = "<data>") ]
pub fn mspieler(data: Form<MSpieler>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[get("/team/<id>") ]
pub fn team(path: web::Path<i32>, _del: DeletePermission) -> impl Responder {
	let id = path.into_inner();
	match Team::new_by_id(id).delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

//
#[get("/benutzerteam", data = "<data>") ]
pub fn benutzerteam(data: Form<BenutzerTeam>, _del: DeletePermission) -> impl Responder {
	match data.into_inner().delete(&mut crate::db::establish_connection()) {
		Ok(data) => Ok(web::Json(data)),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//
