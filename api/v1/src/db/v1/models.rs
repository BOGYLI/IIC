use diesel::prelude::*;
use super::schema::*;
pub use super::models_new::*;

use rocket::serde::{Deserialize, Serialize};


#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = umfrage)]
pub struct Umfrage {
	pub id: i32,
	pub titel: String,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = medien)]
pub struct Medien {
	pub id: i32,
	pub typ: String,
	pub pfad: String,
	pub erstelldatum: String,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = template)]
pub struct Template {
	pub id: i32,
	pub pfad: String
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = tparameter)]
pub struct TParameter {
	pub id: i32,
	pub typ: String,
	pub name: String,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = templatetparameter)]
pub struct TemplateTParameter {
	pub templateid: i32,
	pub parameterid: i32,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = benutzer)]
pub struct Benutzer {
	pub id: i32,
	pub vorname: String,
	pub nachname: String,
	pub klasse: String,
	pub rolle: String
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = ufrage)]
pub struct UFrage {
	pub id: i32,
	pub inhalt: String,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = uantwort)]
pub struct UAntwort {
	pub id: i32,
	pub inhalt: String,
	pub typ: String,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = ufrageuantwort)]
pub struct UFrageUAntwort {
	pub frageid: i32,
	pub antwortid: i32,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = umfragebenutzer)]
pub struct UmfrageBenutzer {
	pub umfrageid: i32,
	pub benutzerid: i32,	
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = umfrageantwort)]
pub struct Umfrageantwort {
	pub umfrageid: i32,
	pub benutzerid: i32,
	pub frageid: i32,
	pub antwortid: i32,
	pub wert: Option<String>,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = artikel)]
pub struct Artikel {
	pub id: i32,
	pub pfad: String,
	pub erstelldatum: String,
	pub status: String,
	pub templateid: i32,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = artikelautor)]
pub struct ArtikelAutor {
	pub artikelid: i32,
	pub benutzerid: i32,
}