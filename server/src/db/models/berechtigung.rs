use diesel::prelude::*;
use diesel_ext_traits::{DBQueryable, DBInsertable};
use crate::db::schema;
use crate::db::schema::*;
use crate::db::models;
use crate::db::models_new;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = benutzer)]
pub struct Benutzer {
	pub id: i32,
	pub name: String,
	pub passwort: String,	// SHA-512 hash
	pub mebistoken: String
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = benutzerberechtigung)]
pub struct BenutzerBerechtigung {
	pub benutzerid: i32,
	pub berechtigungid: i32,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = berechtigung)]
pub struct Berechtigung {
	pub id: i32,
	pub name: String,
	pub beschreibung: String,
	pub apikeyid: i32
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = apikey)]
pub struct Apikey {
	pub id: i32,
	pub wert: String,
	pub zeitpunkt: String,
	pub dauer: i32			// Stunden
}
