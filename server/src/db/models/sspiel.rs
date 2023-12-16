use diesel::prelude::*;
use diesel_ext_traits::{DBQueryable, DBInsertable};
use crate::db::schema;
use crate::db::schema::*;
use crate::db::models;
use crate::db::models_new;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = sspiel)]
pub struct Sspiel {
	pub id: i32,
	pub name: String,
	pub apikeyid: i32,	// TODO
	pub url: String,
	pub highscore: Option<i32>,
	pub best: Option<i32>
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = benutzersspiel)]
pub struct BenutzerSspiel {
	pub benutzerid: i32,
	pub sspielid: i32,
	pub level: i32,
	pub highscore: i32,
	pub einstellungen: String,
}
