use diesel::prelude::*;
use diesel_ext_traits::{DBQueryable, DBInsertable};
use crate::db::schema;
use crate::db::schema::*;
use crate::db::models;
use crate::db::models_new;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = umfrage)]
pub struct Umfrage {
	pub id: i32,
	pub titel: String,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = frage)]
pub struct Frage {
	pub id: i32,
	pub inhalt: String,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = antwort)]
pub struct Antwort {
	pub id: i32,
	pub inhalt: String,
	pub typ: String,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = frageantwort)]
pub struct FrageAntwort {
	pub frageid: i32,
	pub antwortid: i32,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = umfragebenutzer)]
pub struct UmfrageBenutzer {
	pub umfrageid: i32,
	pub benutzerid: i32,	
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = umfragebenutzerfrage)]
pub struct UmfrageBenutzerFrage {
	pub umfrageid: i32,		//primary
	pub benutzerid: i32,	//primary
	pub frageid: i32,		//primary
	pub antwortid: i32,
	pub wert: Option<String>,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = umfragefrage)]
pub struct UmfrageFrage {
	pub umfrageid: i32,
	pub frageid: i32,	
}
