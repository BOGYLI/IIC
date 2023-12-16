use diesel::prelude::*;
use crate::db::schema::*;
use serde_derive::Deserialize;

/*use crate::utils::apikey;
use serde::{Serialize, Deserialize};*/

#[derive(Insertable, Deserialize)]
#[diesel(table_name = artikel)]
pub struct NewArtikel {
	//pub id: i32,
	pub erstelldatum: String,
	pub status: String,
	pub templateid: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = artikelbenutzer)]
pub struct NewArtikelBenutzer {
	pub artikelid: i32,
	pub benutzerid: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = artikelmedien)]
pub struct NewArtikelMedien {
	pub artikelid: i32,
	pub medienid: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = medien)]
pub struct NewMedien {
	//pub id: i32,
	pub typ: String,
	pub pfad: String,
	pub erstelldatum: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = template)]
pub struct NewTemplate {
	//pub id: i32,
	pub pfad: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = parameter)]
pub struct NewParameter {
	//pub id: i32,
	pub typ: String,
	pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = templateparameter)]
pub struct NewTemplateParameter {
	pub templateid: i32,
	pub parameterid: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = artikelparameter)]
pub struct NewArtikelParameter {
	pub artikelid: i32,
	pub parameterid: i32,
	pub wert: String,
}