use db::v1::models::*;

#[post("/umfrageantwort/del", data = "<data>") ]
pub fn umfrageantwort_del(data: Form<UmfrageantwortDEL>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/umfrage/del/<id>") ]
pub fn umfrage_del(id: i64) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/uantwort/del", data = "<data>") ]
pub fn uantwort_del(data: Form<UmfrageUFrageDEL>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/umfragebenutzer/del/<id>") ]
pub fn umfragebenutzer_del(id: i64) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/umfrageufrage/del", data = "<data>") ]
pub fn umfrageufrage_del(data: Form<UmfrageUFrageDEL>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/umfrageumfrageantwort/del", data = "<data>") ]
pub fn umfrageumfrageantwort_del(data: Form<UmfrageUmfrageAntwortDEL>) -> Result<String, String> {
	Ok(String::from(""))
}



#[post("/medien/del/<id>") ]
pub fn medien_del(id: i64) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/artikel/del/<id>") ]
pub fn artikel_del(id: i64) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/artikelautor/del", data = "<data>") ]
pub fn artikelautor_del(data: Form<ArtikelAutorDEL>) -> Result<String, String> {
	Ok(String::from(""))
}



#[post("/benutzer/del/<id>") ]
pub fn benutzer_del(id: i64) -> Result<String, String> {
	Ok(String::from(""))
}



#[post("/template/del/<id>") ]
pub fn template_del(id: i64) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/templatetparameter/del", data = "<data>") ]
pub fn templatetparameter_del(data: Form<TemplateTParameterDEL>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/tparameter/del/<id>") ]
pub fn tparameter_del(id: i64) -> Result<String, String> {
	Ok(String::from(""))
}