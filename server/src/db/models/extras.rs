use diesel::prelude::*;
use diesel_ext_traits::{DBQueryable, DBInsertable};
use crate::db::schema;
use crate::db::schema::*;
use crate::db::models;
use crate::db::models_new;
use serde_derive::{Deserialize, Serialize};
//use serde_with::*;

//#[serde_as]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = revealjs)]
#[derive(Queryable, Identifiable,  AsChangeset, Clone, Selectable, Default)]
#[derive(DBQueryable, DBInsertable)]
pub struct Revealjs {
	pub id: i32,
	pub datei: String,
	pub erstelldatum: String,
    //#[serde_as(as = "BoolFromInt<serde_with::formats::Flexible>")]
	pub oeffentlich: bool,       // TODO! bool
	pub status: String,
}


#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = poster)]
pub struct Poster {
	pub id: i32,
	pub datei: String,
	pub erstelldatum: String,
	pub oeffentlich: bool,  
	pub status: String,
}


#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = revealjsbenutzer)]
pub struct RevealjsBenutzer {
	pub revealjsid: i32,
	pub benutzerid: i32,
}



#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = posterbenutzer)]
pub struct PosterBenutzer {
	pub posterid: i32,
	pub benutzerid: i32,
}