use diesel::prelude::*;
use crate::db::schema::*;
use crate::db::models_new::*;
use rocket::serde::{Deserialize, Serialize};

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

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = artikelmedien)]
pub struct ArtikelMedien {
	pub artikelid: i32,
	pub medienid: i32,
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
