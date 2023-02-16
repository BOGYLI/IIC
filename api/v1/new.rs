use db::v1::models::*;

#[post("/umfrageantwort/new", data = "<data>") ]
pub fn umfrageantwort_new(data: Form<Umfrageantwort>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/umfrage/new", data = "<data>") ]
pub fn umfrage_new(data: Form<Umfrage>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/uantwort/new", data = "<data>") ]
pub fn uantwort_new(data: Form<UAntwort>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/umfragebenutzer/new", data = "<data>") ]
pub fn umfragebenutzer_new(data: Form<Umfragebenutzer>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/umfrageufrage/new", data = "<data>") ]
pub fn umfrageufrage_new(data: Form<UmfrageUFrage>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/umfrageumfrageantwort/new", data = "<data>") ]
pub fn umfrageumfrageantwort_new(data: Form<UmfrageUmfrageAntwort>) -> Result<String, String> {
	Ok(String::from(""))
}



#[post("/medien/new", data = "<data>") ]
pub fn medien_new(data: Form<Medien>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/artikel/new", data = "<data>") ]
pub fn artikel_new(data: Form<Artikel>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/artikelautor/new", data = "<data>") ]
pub fn artikelautor_new(data: Form<ArtikelAutor>) -> Result<String, String> {
	Ok(String::from(""))
}



#[post("/benutzer/new", data = "<data>") ]
pub fn benutzer_new(data: Form<Benutzer>) -> Result<String, String> {
	Ok(String::from(""))
}



#[post("/template/new", data = "<data>") ]
pub fn template_new(data: Form<Template>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/templatetparameter/new", data = "<data>") ]
pub fn templatetparameter_new(data: Form<TemplateTParameter>) -> Result<String, String> {
	Ok(String::from(""))
}

#[post("/tparameter/new", data = "<data>") ]
pub fn tparameter_new(data: Form<TParameter>) -> Result<String, String> {
	Ok(String::from(""))
}