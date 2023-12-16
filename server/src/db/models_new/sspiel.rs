use diesel::prelude::*;
use crate::db::schema::*;
use crate::utils::apikey;
use serde_derive::{Serialize, Deserialize};

use serde::{Serializer}; // 1.0.104

fn apikey_serialize<S>(_x: &i32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i32(apikey::generate())
}

#[derive(Insertable, Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = sspiel)]
pub struct NewSspiel {
	pub name: String,
	pub url: String,
	//#[serde(default = "apikey::generate")]
	#[serde(serialize_with = "apikey_serialize")]
	pub apikeyid: i32,
	//pub highscore: i32,
	//pub best: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = benutzersspiel)]
pub struct NewBenutzerSspiel {
	pub benutzerid: i32,
	pub sspielid: i32,
	pub level: i32,
	pub highscore: i32,
	pub einstellungen: String,
}
