use diesel::prelude::*;
use crate::db::schema::*;
//use crate::db::models_new::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[diesel(table_name = sspiel)]
pub struct SSpiel {
	pub id: i32,
	pub name: String,
	pub apikeyid: i32,	// TODO
	pub url: String,
	pub highscore: Option<i32>,
	pub best: Option<i32>
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm, Selectable)]
#[diesel(table_name = sspieler)]
pub struct SSpieler {
	pub benutzerid: i32,
	pub spielid: i32,
	pub level: i32,
	pub highscore: i32,
	pub einstellungen: String,
}
