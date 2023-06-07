use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub use crate::schema;
mod models_new;
mod insert;
mod query;
pub use insert::*;
pub use query::*;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub trait DBInsertable<T: Queryable<S, diesel::pg::Pg>, S> {
    #[allow(clippy::new_ret_no_self)]
    fn new(self, conn: &mut PgConnection) -> Result<T, diesel::result::Error> where Self: Sized;
}

pub trait DBQueryable<T: Queryable<S, diesel::pg::Pg>, S> {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> where Self: Sized;
    fn get(&self, conn: &mut PgConnection) -> Result<Self, diesel::result::Error> where Self: Sized;
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error>; // evtl. ohne 'bool'
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error>;
}




impl models::Benutzer {
    pub fn authenticate(name: String, passwort: String, conn: &mut PgConnection) -> Option<models::Benutzer> {
        None
    }
    
    pub fn mebis(name: String, token: String, conn: &mut PgConnection) -> Option<models::Benutzer> {
        None
    }
}
