use diesel::prelude::*;
use crate::db::schema::*;

/*use crate::utils::apikey;
use serde::{Serialize, Deserialize};*/

#[derive(Insertable)]
#[diesel(table_name = artikel)]
pub struct NewArtikel<'a> {
	//pub id: i32,
	pub pfad: &'a str,
	pub erstelldatum: &'a str,
	pub status: &'a str,
	pub templateid: i32,
}

#[derive(Insertable)]
#[diesel(table_name = artikelbenutzer)]
pub struct NewArtikelBenutzer {
	pub artikelid: i32,
	pub benutzerid: i32,
}

#[derive(Insertable)]
#[diesel(table_name = artikelmedien)]
pub struct NewArtikelMedien {
	pub artikelid: i32,
	pub medienid: i32,
}

#[derive(Insertable)]
#[diesel(table_name = medien)]
pub struct NewMedien<'a> {
	//pub id: i32,
	pub typ: &'a str,
	pub pfad: &'a str,
	pub erstelldatum: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = template)]
pub struct NewTemplate<'a> {
	//pub id: i32,
	pub pfad: &'a str
}

#[derive(Insertable)]
#[diesel(table_name = tparameter)]
pub struct NewTparameter<'a> {
	//pub id: i32,
	pub typ: &'a str,
	pub name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = templatetparameter)]
pub struct NewTemplateTparameter {
	pub templateid: i32,
	pub tparameterid: i32,
}
