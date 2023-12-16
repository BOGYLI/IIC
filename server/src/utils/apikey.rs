use diesel_ext_traits::DBInsertable;
use rand::distributions::{Alphanumeric, DistString};
use crate::db::models::{NewApikey/*, ApiKey*/};

pub fn generate() -> i32 {
	println!("generating api key");
	let key = NewApikey {
		wert: Alphanumeric.sample_string(&mut rand::thread_rng(), 20),//.to_string(),
		zeitpunkt: String::new(),
		dauer: 0
	};
	let apik = key.new(&mut crate::db::establish_connection()).unwrap();
	println!("New APIKey generated {}", apik.id);
	apik.id
}

pub fn generate_string() -> String {
	println!("generating api key");
	let key = NewApikey {
		wert: Alphanumeric.sample_string(&mut rand::thread_rng(), 20),//.to_string(),
		zeitpunkt: String::new(),
		dauer: 0
	};
	let apik = key.new(&mut crate::db::establish_connection()).unwrap();
	println!("New APIKey generated {}", apik.id);
	apik.id.to_string()
}
