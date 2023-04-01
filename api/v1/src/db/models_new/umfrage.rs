use diesel::prelude::*;
use crate::db::schema::*;

use crate::utils::apikey;
use serde::{Serialize, Deserialize};

#[derive(Insertable, FromForm)]
#[diesel(table_name = umfrage)]
pub struct NewUmfrage<'a> {
	pub titel: &'a str,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = ufrage)]
pub struct NewUFrage<'a> {
	pub inhalt: &'a str,
}

#[derive(Insertable, FromForm)]
#[diesel(table_name = uantwort)]
pub struct NewUAntwort<'a> {
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
#[diesel(table_name = umfrageufrage)]
pub struct NewUmfrageUFrage {
	pub umfrageid: i32,
	pub frageid: i32,	
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
