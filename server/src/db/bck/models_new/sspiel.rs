use diesel::prelude::*;
use crate::db::schema::*;

use crate::utils::apikey;
use serde::{Serialize, Deserialize};

use serde::{Serializer}; // 1.0.104

fn apikey_serialize<S>(_x: &i32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i32(apikey::generate())
}

#[derive(Insertable, FromForm, Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = sspiel)]
pub struct NewSSpiel<'a> {
	pub name: &'a str,
	pub url: &'a str,
	//#[serde(default = "apikey::generate")]
	#[serde(serialize_with = "apikey_serialize")]
	pub apikeyid: i32,
	//pub highscore: i32,
	//pub best: i32
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = sspieler)]
pub struct NewSSpieler<'a> {
	pub benutzerid: i32,
	pub spielid: i32,
	pub level: i32,
	pub highscore: i32,
	pub einstellungen: &'a str,
}
