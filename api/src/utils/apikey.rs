use rand::distributions::{Alphanumeric, DistString};
use crate::db::models::{NewApiKey, ApiKey};
use crate::db::DBInsertable;

pub fn generate() -> i32 {
<<<<<<< HEAD
	println!("generating api key");
=======
>>>>>>> 51d987b925a4c1f27126efdd48feb41281051aa8
	// TODO: insert new
	let key = NewApiKey {
		wert: Alphanumeric.sample_string(&mut rand::thread_rng(), 20).to_string(),
		zeitpunkt: "",
		dauer: 0
	};
	let apik = key.new(&mut crate::db::establish_connection()).unwrap();
<<<<<<< HEAD
	println!("New APIKey generated {}", apik.id);
=======
>>>>>>> 51d987b925a4c1f27126efdd48feb41281051aa8
	apik.id
}
