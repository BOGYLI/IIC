use diesel::prelude::*;
use crate::db::schema::*;

use crate::utils::apikey;
use serde::{Serialize, Deserialize};

<<<<<<< HEAD
use serde::{Serializer}; // 1.0.104

fn apikey_serialize<S>(x: &i32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i32(apikey::generate())
}

=======
>>>>>>> 51d987b925a4c1f27126efdd48feb41281051aa8
#[derive(Insertable, FromForm, Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = sspiel)]
pub struct NewSSpiel<'a> {
	pub name: &'a str,
<<<<<<< HEAD
	pub url: &'a str,
	//#[serde(default = "apikey::generate")]
	#[serde(serialize_with = "apikey_serialize")]
=======
	#[serde(default = "apikey::generate")]
>>>>>>> 51d987b925a4c1f27126efdd48feb41281051aa8
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
