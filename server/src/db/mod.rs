use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_ext_traits::DBQueryable;
use dotenvy::dotenv;
use serde::{Serializer, Deserializer, Serialize, de::Error, Deserialize};
use std::env;

use self::models::Benutzer;

pub mod models;
pub mod schema;
mod models_new;
pub mod utils;
pub mod connection;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}




impl models::Benutzer {
    pub fn authenticate(name: &String, passwort: &String, conn: &mut PgConnection) -> Option<models::Benutzer> {
        let binding = Benutzer::get_all(conn).unwrap();
        let list = binding.iter().filter(|b| b.name.eq(name) && b.passwort.eq(passwort)).collect::<Vec<&Benutzer>>();
        if list.len() > 0 {
            Some(list[0].clone().to_owned())
        } else {
            None
        }
    }
    
    pub fn mebis(token: &String, conn: &mut PgConnection) -> Option<models::Benutzer> {
        None
    }
}





/*pub mod serde_bool2 {
    use serde::{Serializer, Deserializer, Serialize, de::Error, Deserialize};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = Deserialize::deserialize(deserializer)?;

        match s {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(Error::unknown_variant(s, &[1, 0])),
        }
    }

    pub fn serialize<S>(x: &bool, serializer: S) -> Result<i32, S::Error>
    where
        S: Serializer,
    {
        match x {
            true => Ok(serializer.serialize_i32(1)),
            false => Ok(serializer.serialize_i32(0)),
        }
    }
}

pub mod serde_bool {
    use serde::{Serializer, Deserializer, Serialize, de::Error, Deserialize};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &bool = Deserialize::deserialize(deserializer)?;

        match s {
            true => Ok(1),
            false => Ok(0),
        }
    }

    pub fn serialize<S>(x: &i32, serializer: S) -> Result<bool, S::Error>
    where
        S: Serializer,
    {
        match x {
            1 => serializer.serialize_bool(true),
            0 => serializer.serialize_bool(false),
            _ => Err(Error::unknown_variant(x, &[1, 0])),
        }
    }
}


pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &i32 = Deserialize::deserialize(deserializer)?;

    match s {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(Error::unknown_variant(s, &[1, 0])),
    }
}


pub fn serialize_bool<S>(x: &bool, serializer: S) -> Result<i32, S::Error>
where
    S: Serializer,
{
    match x {
        true => serializer.serialize_i32(1),
        false => serializer.serialize_i32(0),
    }
}*/