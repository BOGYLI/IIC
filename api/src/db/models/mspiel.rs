use diesel::prelude::*;
use crate::db::schema::*;
use crate::db::models_new::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = mspiel)]
pub struct MSpiel {
	pub id: i32,
	pub name: String,
	pub apikeyid: i32,	// TODO
	pub url: String,
	pub highscore: Option<i32>,
	pub best: Option<i32>
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = mspieler)]
pub struct MSpieler {
	pub matchid: i32,
	pub team1id: i32,
	pub team2id: i32,
	pub spielid: i32,
	pub level: i32,
	pub score1: i32,
	pub score2: i32,
	pub einstellungen1: String,
	pub einstellungen2: String,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = team)]
pub struct Team {
	pub id: i32,
	pub name: String,
	pub apikeyid: i32,
	pub overallscore: i32,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, FromForm)]
#[diesel(table_name = benutzerteam)]
pub struct BenutzerTeam {
	pub benutzerid: i32,
	pub teamid: i32
}
