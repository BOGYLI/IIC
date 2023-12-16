use diesel::prelude::*;
use crate::db::schema::*;
//use crate::db::models_new::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = umfrage)]
pub struct Umfrage {
	pub id: i32,
	pub titel: String,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = ufrage)]
pub struct UFrage {
	pub id: i32,
	pub inhalt: String,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = uantwort)]
pub struct UAntwort {
	pub id: i32,
	pub inhalt: String,
	pub typ: String,
}

// TODO: rename to FrageAntwort
#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBCheckable, DBInsertable)]
#[diesel(table_name = ufrageuantwort)]
pub struct UFrageUAntwort {
	pub frageid: i32,
	pub antwortid: i32,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBCheckyable, DBInsertable)]
#[diesel(table_name = umfragebenutzer)]
pub struct UmfrageBenutzer {
	pub umfrageid: i32,
	pub benutzerid: i32,	
}

// TODO: rename to UmfrageBenutzerFrage	-> TODO: diesel-ext_checkable_proc-macro dynamische filter anzahl (loop)
#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(/*DBQueryable,*/ DBInsertable)]
#[diesel(table_name = umfrageantwort)]
pub struct Umfrageantwort {
	pub umfrageid: i32,		//primary
	pub benutzerid: i32,	//primary
	pub frageid: i32,		//primary
	pub antwortid: i32,
	pub wert: Option<String>,
}

// TODO: rename to UmfrageFrage
#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[derive(DBCheckable, DBInsertable)]
#[diesel(table_name = umfrageufrage)]
pub struct UmfrageUFrage {
	pub umfrageid: i32,
	pub frageid: i32,	
}
