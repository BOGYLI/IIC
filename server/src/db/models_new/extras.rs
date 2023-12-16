use diesel::prelude::*;
use diesel_ext_traits::{DBQueryable, DBInsertable};
//use crate::db::{deserialize_bool, serialize_bool, serde_bool};
use crate::db::schema;
use crate::db::schema::*;
use crate::db::models;
use crate::db::models_new;
use serde_derive::{Deserialize, Serialize};
use serde_with::*;

#[serde_as]
#[derive(Insertable, Deserialize)]
#[diesel(table_name = revealjs)]
pub struct NewRevealjs {
	// pub id: i32,
	pub datei: String,
	pub erstelldatum: String,
    //#[serde(deserialize_with = "deserialize_bool")]
    //#[serde(serialize_with = "serialize_bool")]
	//#[serde(with = "serde_bool")]
	pub oeffentlich: bool,       // TODO! bool
	pub status: String,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = poster)]
pub struct NewPoster {
	// pub id: i32,
	pub datei: String,
	pub erstelldatum: String,
	pub oeffentlich: bool,
	pub status: String,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = revealjsbenutzer)]
pub struct NewRevealjsBenutzer {
	pub revealjsid: i32,
	pub benutzerid: i32,
}



#[derive(Insertable, Deserialize)]
#[diesel(table_name = posterbenutzer)]
pub struct NewPosterBenutzer {
	pub posterid: i32,
	pub benutzerid: i32,
}