use diesel::prelude::*;
use crate::db::schema::*;

/*use crate::utils::apikey;
use serde::{Serialize, Deserialize};*/

#[derive(Insertable)]
#[diesel(table_name = umfrage)]
pub struct NewUmfrage<'a> {
	pub titel: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = frage)]
pub struct NewFrage<'a> {
	pub inhalt: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = antwort)]
pub struct NewAntwort<'a> {
	pub inhalt: &'a str,
	pub typ: &'a str,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = frageantwort)]
pub struct NewFrageAntwort {
	pub frageid: i32,
	pub antwortid: i32,
}

#[derive(Insertable)]
#[diesel(table_name = umfragebenutzer)]
pub struct NewUmfrageBenutzer {
	pub umfrageid: i32,
	pub benutzerid: i32,	
}

#[derive(Insertable)]
#[diesel(table_name = umfrageufrage)]
pub struct NewUmfrageFrage {
	pub umfrageid: i32,
	pub frageid: i32,	
}

#[derive(Insertable)]
#[diesel(table_name = umfragebenutzerfrage)]
pub struct NewUmfrageBenutzerAntwort<'a> {
	pub umfrageid: i32,
	pub benutzerid: i32,
	pub frageid: i32,
	pub antwortid: i32,
	pub wert: Option<&'a str>,
}
