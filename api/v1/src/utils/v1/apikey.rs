use rand::distributions::{Alphanumeric, DistString};

pub fn generate() -> String {
	Alphanumeric.sample_string(&mut rand::thread_rng(), 20)
}
