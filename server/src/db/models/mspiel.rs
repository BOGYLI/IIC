use diesel::prelude::*;
use diesel_ext_traits::{DBQueryable, DBInsertable};
use crate::db::schema;
use crate::db::schema::*;
use crate::db::models;
use crate::db::models_new;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = mspiel)]
pub struct Mspiel {
	pub id: i32,
	pub name: String,
	pub apikeyid: i32,	// TODO
	pub url: String,
	pub highscore: Option<i32>,
	pub best: Option<i32>
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = matchmspiel)]
pub struct MatchMspiel {
	pub matchid: i32,	// TODO:
	pub team1id: i32,
	pub team2id: i32,
	pub mspielid: i32,
	pub level: i32,
	pub score1: i32,
	pub score2: i32,
	pub einstellungen1: String,
	pub einstellungen2: String,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = team)]
pub struct Team {
	pub id: i32,
	pub name: String,
	pub apikeyid: i32,
	pub overallscore: i32,
}

#[derive(Queryable,   AsChangeset, Clone, Serialize, Deserialize, Selectable)]
#[derive(DBQueryable, DBInsertable)]
#[diesel(table_name = benutzerteam)]
pub struct BenutzerTeam {
	pub benutzerid: i32,
	pub teamid: i32
}
