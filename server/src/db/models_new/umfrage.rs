use diesel::prelude::*;
use crate::db::schema::*;
use serde_derive::Deserialize;

/*use crate::utils::apikey;
use serde::{Serialize, Deserialize};*/

#[derive(Insertable, Deserialize)]
#[diesel(table_name = umfrage)]
pub struct NewUmfrage {
	pub titel: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = frage)]
pub struct NewFrage {
	pub inhalt: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = antwort)]
pub struct NewAntwort {
	pub inhalt: String,
	pub typ: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = frageantwort)]
pub struct NewFrageAntwort {
	pub frageid: i32,
	pub antwortid: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = umfragebenutzer)]
pub struct NewUmfrageBenutzer {
	pub umfrageid: i32,
	pub benutzerid: i32,	
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = umfragefrage)]
pub struct NewUmfrageFrage {
	pub umfrageid: i32,
	pub frageid: i32,	
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = umfragebenutzerfrage)]
pub struct NewUmfrageBenutzerFrage {
	pub umfrageid: i32,
	pub benutzerid: i32,
	pub frageid: i32,
	pub antwortid: i32,
	pub wert: Option<String>,
}
