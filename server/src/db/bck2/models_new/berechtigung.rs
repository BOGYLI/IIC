use diesel::prelude::*;
use crate::db::schema::*;

/*use crate::utils;
use serde::{Serialize, Deserialize};*/
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = benutzer)]
pub struct NewBenutzer<'a> {
	pub name: &'a str,
	pub passwort: &'a str,
	pub mebistoken: &'a str
}

#[derive(Insertable)]
#[diesel(table_name = benutzerberechtigung)]
pub struct NewBenutzerBerechtigung {
	pub benutzerid: i32,
	pub berechtigungid: i32,
}

#[derive(Insertable)]
#[diesel(table_name = berechtigung)]
pub struct NewBerechtigung<'a> {
	pub name: &'a str,
	pub beschreibung: &'a str,
	pub apikeyid: i32
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = apikey)]
pub struct NewApikey<'a> {
	#[serde(default = "utils::apikey::generate")]
	pub wert: String,
	pub zeitpunkt: &'a str,
	pub dauer: i32			// Stunden
}
