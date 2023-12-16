use diesel::prelude::*;
use diesel_ext_traits::{DBQueryable, DBInsertable};
use crate::db::schema;
use crate::db::schema::*;
use crate::db::models;
use crate::db::models_new;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable, Default, Debug)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = artikel)]
pub struct Artikel {
	pub id: i32,
	pub erstelldatum: String,
	pub status: String,
	pub templateid: i32,
}

#[derive(Queryable, AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = artikelbenutzer)]
pub struct ArtikelBenutzer {
	pub artikelid: i32,
	pub benutzerid: i32,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = artikelmedien)]
pub struct ArtikelMedien {
	pub artikelid: i32,
	pub medienid: i32,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = medien)]
pub struct Medien {
	pub id: i32,
	pub typ: String,
	pub pfad: String,
	pub erstelldatum: String,
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable, Debug, Default)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = template)]
pub struct Template {
	pub id: i32,
	pub pfad: String
}

#[derive(Queryable, Identifiable,  AsChangeset, Clone, Serialize, Deserialize, Selectable, Debug)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = parameter)]
pub struct Parameter {
	pub id: i32,
	pub typ: String,		// html-input type
	pub name: String,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable, Debug)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = templateparameter)]
pub struct TemplateParameter {
	pub templateid: i32,
	pub parameterid: i32,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable, Debug)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = artikelparameter)]
pub struct ArtikelParameter {
	pub artikelid: i32,
	pub parameterid: i32,
	pub wert: String,
}
