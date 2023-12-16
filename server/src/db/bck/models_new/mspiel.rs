use diesel::prelude::*;
use crate::db::schema::*;

use crate::utils::apikey;
use serde::{Serialize, Deserialize};

#[derive(Insertable, FromForm)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = mspiel)]
pub struct NewMSpiel<'a> {
	pub name: &'a str,
	#[serde(default = "apikey::generate")]
	pub apikeyid: i32
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = mspieler)]
pub struct NewMSpieler<'a> {
	pub matchid: i32,
	pub team1id: i32,
	pub team2id: i32,
	pub spielid: i32,
	pub level: i32,
	pub score1: i32,
	pub score2: i32,
	pub einstellungen1: &'a str,
	pub einstellungen2: &'a str,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = team)]
pub struct NewTeam<'a> {
	pub name: &'a str,
	pub overallscore: i32,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = benutzerteam)]
pub struct NewBenutzerTeam {
	pub benutzerid: i32,
	pub teamid: i32
}
