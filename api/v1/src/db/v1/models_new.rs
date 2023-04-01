use diesel::prelude::*;
use super::schema::*;

use crate::utils::v1::apikey;
use serde::{Serialize, Deserialize};

#[derive(Insertable, FromForm)]
#[diesel(table_name = umfrage)]
pub struct NewUmfrage<'a> {
	//pub id: i32,
	pub titel: &'a str,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = medien)]
pub struct NewMedien<'a> {
	//pub id: i32,
	pub typ: &'a str,
	pub pfad: &'a str,
	pub erstelldatum: &'a str,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = template)]
pub struct NewTemplate<'a> {
	//pub id: i32,
	pub pfad: &'a str
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = tparameter)]
pub struct NewTParameter<'a> {
	//pub id: i32,
	pub typ: &'a str,
	pub name: &'a str,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = templatetparameter)]
pub struct NewTemplateTParameter {
	pub templateid: i32,
	pub parameterid: i32,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = benutzer)]
pub struct NewBenutzer<'a> {
	//pub id: i32,
	pub vorname: &'a str,
	pub nachname: &'a str,
	pub passwort: &'a str,
	pub klasse: &'a str,
	pub rolle: &'a str
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = ufrage)]
pub struct NewUFrage<'a> {
	//pub id: i32,
	pub inhalt: &'a str,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = uantwort)]
pub struct NewUAntwort<'a> {
	//pub id: i32,
	pub inhalt: &'a str,
	pub typ: &'a str,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = ufrageuantwort)]
pub struct NewUFrageUAntwort {
	pub frageid: i32,
	pub antwortid: i32,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = umfragebenutzer)]
pub struct NewUmfrageBenutzer {
	pub umfrageid: i32,
	pub benutzerid: i32,	
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = umfrageantwort)]
pub struct NewUmfrageantwort<'a> {
	pub umfrageid: i32,
	pub benutzerid: i32,
	pub frageid: i32,
	pub antwortid: i32,
	pub wert: Option<&'a str>,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = artikel)]
pub struct NewArtikel<'a> {
	//pub id: i32,
	pub pfad: &'a str,
	pub erstelldatum: &'a str,
	pub status: &'a str,
	pub templateid: i32,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = artikelautor)]
pub struct NewArtikelAutor {
	pub artikelid: i32,
	pub benutzerid: i32,
}



#[derive(Insertable, FromForm, Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = sspiel)]
pub struct NewSSpiel {
	pub name: String,
	#[serde(default = "apikey::generate")]
	pub apikey: String,
	//pub highscore: i32,
	//pub best: i32
}

#[derive(Insertable, FromForm)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = mspiel)]
pub struct NewMSpiel {
	pub name: String,
	#[serde(default = "apikey::generate")]
	pub apikey: String,
	//pub highscore: i32,
	//pub best: i32
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = sspieler)]
pub struct NewSSpieler {
	pub benutzerid: i32,
	pub spielid: i32,
	pub level: i32,
	pub highscore: i32,
	pub einstellungen: String,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = mspieler)]
pub struct NewMSpieler {
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

#[derive(Insertable, FromForm)]
#[diesel(table_name = team)]
pub struct NewTeam {
	pub name: String,
	pub overallscore: i32,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = benutzerteam)]
pub struct NewBenutzerTeam {
	pub benutzerid: i32,
	pub teamid: i32
}


