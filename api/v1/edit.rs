use db::v1::models::*;

#[put("/umfrageantwort/edit", data = "<data>") ]
pub fn umfrageantwort_edit(data: Form<Umfrageantwort>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/umfrage/edit", data = "<data>") ]
pub fn umfrage_edit(data: Form<Umfrage>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/uantwort/edit", data = "<data>") ]
pub fn uantwort_edit(data: Form<UmfrageUFrage>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/umfragebenutzer/edit", data = "<data>") ]
pub fn umfragebenutzer_edit(data: Form<Umfragebenutzer>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/umfrageufrage/edit", data = "<data>") ]
pub fn umfrageufrage_edit(data: Form<UmfrageUFrage>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/umfrageumfrageantwort/edit", data = "<data>") ]
pub fn umfrageumfrageantwort_edit(data: Form<UmfrageUmfrageAntwort>) -> Result<String, String> {
	Ok(String::from(""))
}



#[put("/medien/edit", data = "<data>") ]
pub fn medien_edit(data: Form<Medien>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/artikel/edit", data = "<data>") ]
pub fn artikel_edit(data: Form<Artikel>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/artikelautor/edit", data = "<data>") ]
pub fn artikelautor_edit(data: Form<ArtikelAutor>) -> Result<String, String> {
	Ok(String::from(""))
}



#[put("/benutzer/edit", data = "<data>") ]
pub fn benutzer_edit(data: Form<Benutzer>) -> Result<String, String> {
	Ok(String::from(""))
}



#[put("/template/edit", data = "<data>") ]
pub fn template_edit(data: Form<Template>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/templatetparameter/edit", data = "<data>") ]
pub fn templatetparameter_edit(data: Form<TemplateTParameter>) -> Result<String, String> {
	Ok(String::from(""))
}

#[put("/tparameter/edit", data = "<data>") ]
pub fn tparameter_edit(data: Form<TParameter>) -> Result<String, String> {
	Ok(String::from(""))
}