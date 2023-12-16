use diesel::prelude::*;
use crate::db::schema::*;

use crate::utils::apikey;
use serde_derive::{Serialize, Deserialize};

#[derive(Insertable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = mspiel)]
pub struct NewMspiel {
	pub name: String,
	#[serde(default = "apikey::generate")]
	pub apikeyid: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = matchmspiel)]
pub struct NewMatchMspiel {
	pub matchid: i32,
	pub team1id: i32,
	pub team2id: i32,
	pub mspielid: i32,
	pub level: i32,
	pub score1: i32,
	pub score2: i32,
	pub einstellungen1: String,
	pub einstellungen2: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = team)]
pub struct NewTeam {
	pub name: String,
	pub overallscore: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = benutzerteam)]
pub struct NewBenutzerTeam {
	pub benutzerid: i32,
	pub teamid: i32
}
