use crate::db::models::*;
use rocket::form::Form;
use crate::db::DBQueryable;
use rocket::http::Status;
use crate::utils::cookies::{UpdatePermission};


//
#[put("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<Umfrageantwort>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
//

#[put("/umfrage", data = "<data>") ]
pub fn umfrage(data: Form<Umfrage>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[put("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<UmfrageBenutzer>, update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

/*
#[put("/umfrageufrage", data = "<data>") ]
pub fn umfrageufrage(data: Form<UmfrageUFrage>, update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[put("/uantwort", data = "<data>") ]
pub fn uantwort(data: Form<UAntwort>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[put("/ufrageuantwort", data = "<data>") ]
pub fn ufrageuantwort(data: Form<UFrageUAntwort>, update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[put("/ufrage", data = "<data>") ]
pub fn ufrage(data: Form<UFrage>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


//////////////////////////////////////////////////////////////


#[put("/medien", data = "<data>") ]
pub fn medien(data: Form<Medien>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[put("/artikelmedien", data = "<data>") ]
pub fn artikelmedien(data: Form<ArtikelAutor>, update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[put("/artikel", data = "<data>") ]
pub fn artikel(data: Form<Artikel>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[put("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<ArtikelAutor>, update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[put("/template", data = "<data>") ]
pub fn template(data: Form<Template>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[put("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<TemplateTParameter>, update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[put("/tparameter", data = "<data>") ]
pub fn tparameter(data: Form<TParameter>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


///////////////////////////////////////////////////////////////



#[put("/benutzer", data = "<data>") ]
pub fn benutzer(data: Form<Benutzer>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[put("/benutzerberechtigung", data = "<data>") ]
pub fn benutzer(data: Form<BenutzerBerechtigung>, update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/

#[put("/berechtigung", data = "<data>") ]
pub fn berechtigung(data: Form<Berechtigung>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


///////////////////////////////////////////////////////////////


#[put("/sspiel", data = "<data>") ]
pub fn sspiel(data: Form<SSpiel>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[put("/sspieler", data = "<data>") ]
pub fn sspieler(data: Form<SSpieler>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}


//////////////////////////////////////////////////////////////


#[put("/mspieler", data = "<data>") ]
pub fn mspieler(data: Form<MSpieler>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[put("/mspiel", data = "<data>") ]
pub fn mspiel(data: Form<MSpiel>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[put("/team", data = "<data>") ]
pub fn team(data: Form<Team>, _update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

/*
#[put("/benutzerteam", data = "<data>") ]
pub fn benutzerteam(data: Form<BenutzerTeam>, update: UpdatePermission) -> impl Responder {
	match data.into_inner().update(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}
*/
