use crate::db::models::*;
use rocket::form::Form;
use rocket::http::Status;

use crate::db::DBInsertable;
use crate::utils::cookies::{NewPermission};

#[post("/umfrageantwort", data = "<data>") ]
pub fn umfrageantwort(data: Form<NewUmfrageantwort>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/umfrage", data = "<data>") ]
pub fn umfrage(data: Form<NewUmfrage>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/umfragebenutzer", data = "<data>") ]
pub fn umfragebenutzer(data: Form<NewUmfrageBenutzer>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/umfrageufrage", data = "<data>") ]
pub fn umfrageufrage(data: Form<NewUmfrageUFrage>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/ufrage", data = "<data>") ]
pub fn ufrage(data: Form<NewUFrage>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/ufrageuantwort", data = "<data>") ]
pub fn ufrageuantwort(data: Form<NewUFrageUAntwort>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/uantwort", data = "<data>") ]
pub fn uantwort(data: Form<NewUAntwort>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}


///////////////////////////////////////////////////////////////


#[post("/medien", data = "<data>") ]
pub fn medien(data: Form<NewMedien>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/artikelmedien", data = "<data>") ]
pub fn artikelmedien(data: Form<NewArtikelMedien>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/artikel", data = "<data>") ]
pub fn artikel(data: Form<NewArtikel>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/artikelautor", data = "<data>") ]
pub fn artikelautor(data: Form<NewArtikelAutor>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/template", data = "<data>") ]
pub fn template(data: Form<NewTemplate>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/templatetparameter", data = "<data>") ]
pub fn templatetparameter(data: Form<NewTemplateTParameter>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/tparameter", data = "<data>") ]
pub fn tparameter(data: Form<NewTParameter>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}


/////////////////////////////////////////////////////////////////


#[post("/benutzer", data = "<data>") ]
pub fn benutzer(data: Form<NewBenutzer>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/benutzerberechtigung", data = "<data>") ]
pub fn benutzerberechtigung(data: Form<NewBenutzerBerechtigung>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/berechtigung", data = "<data>") ]
pub fn berechtigung(data: Form<NewBerechtigung>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/apikey", data = "<data>") ]
pub fn apikey(data: Form<NewApiKey>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}


///////////////////////////////////////////////////////////////////


#[post("/sspiel", data = "<data>") ]
pub fn sspiel(data: Form<NewSSpiel>, new: NewPermission) -> Result<Status, Status> {
	let mut conn = crate::db::establish_connection();
	let mut data2 = data.clone();
	data2.apikeyid = crate::utils::apikey::generate();
	match data.into_inner().new(&mut conn) {
		Ok(_) => Ok(Status::Ok),
		Err(e) => {
			println!("{:?}", e);
			println!("{:?}", data2);
			Err(Status::InternalServerError)
		}
	}
}

#[post("/sspieler", data = "<data>") ]
pub fn sspieler(data: Form<NewSSpieler>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}


////////////////////////////////////////////////////////////////


#[post("/mspiel", data = "<data>") ]
pub fn mspiel(data: Form<NewMSpiel>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/mspieler", data = "<data>") ]
pub fn mspieler(data: Form<NewMSpieler>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/team", data = "<data>") ]
pub fn team(data: Form<NewTeam>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}

#[post("/benutzerteam", data = "<data>") ]
pub fn benutzerteam(data: Form<NewBenutzerTeam>, new: NewPermission) -> Result<Status, Status> {
	match data.into_inner().new(&mut crate::db::establish_connection()) {
		Ok(_) => Ok(Status::Ok),
		Err(_) => Err(Status::InternalServerError)
	}
}
