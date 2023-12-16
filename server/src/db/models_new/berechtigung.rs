use diesel::prelude::*;
use crate::db::schema::*;
use serde_derive::{Serialize, Deserialize};

#[derive(Insertable, Deserialize)]
#[diesel(table_name = benutzer)]
pub struct NewBenutzer {
	pub name: String,
	pub passwort: String,
	pub mebistoken: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = benutzerberechtigung)]
pub struct NewBenutzerBerechtigung {
	pub benutzerid: i32,
	pub berechtigungid: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = berechtigung)]
pub struct NewBerechtigung {
	pub name: String,
	pub beschreibung: String,
	pub apikeyid: i32
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = apikey)]
pub struct NewApikey {
	#[serde(default = "crate::utils::apikey::generate_string")]		// TODO: wert: i32?
	pub wert: String,
	pub zeitpunkt: String,
	pub dauer: i32			// Stunden
}
