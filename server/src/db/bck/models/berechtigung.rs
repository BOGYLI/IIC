use diesel::prelude::*;
use crate::db::schema::*;
//use crate::db::models_new::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = benutzer)]
pub struct Benutzer {
	pub id: i32,
	pub name: String,
	pub passwort: String,	// SHA-512 hash
	pub mebistoken: String
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBCheckable, DBInsertable)]
#[diesel(table_name = benutzerberechtigung)]
pub struct BenutzerBerechtigung {
	pub benutzerid: i32,
	pub berechtigungid: i32,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = berechtigung)]
pub struct Berechtigung {
	pub id: i32,
	pub name: String,
	pub beschreibung: String,
	pub apikeyid: i32
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = apikey)]
pub struct ApiKey {
	pub id: i32,
	pub wert: String,
	pub zeitpunkt: String,
	pub dauer: i32			// Stunden
}
