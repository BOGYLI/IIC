use crate::db::DBQueryable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;


impl DBQueryable<models::Benutzer, schema::benutzer::SqlType> for models::Benutzer {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::Benutzer>, diesel::result::Error> {
        schema::benutzer::dsl::benutzer.load::<models::Benutzer>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::Benutzer, diesel::result::Error> {
        use schema::benutzer::dsl::*;
        match benutzer.filter(id.eq(self.id)).load::<models::Benutzer>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::benutzer::dsl::*;
        diesel::delete(benutzer.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::benutzer::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::Berechtigung, schema::berechtigung::SqlType> for models::Berechtigung {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::Berechtigung>, diesel::result::Error> {
        schema::berechtigung::dsl::berechtigung.load::<models::Berechtigung>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::Berechtigung, diesel::result::Error> {
        use schema::berechtigung::dsl::*;
        match berechtigung.filter(id.eq(self.id)).load::<models::Berechtigung>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::berechtigung::dsl::*;
        diesel::delete(berechtigung.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::berechtigung::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::BenutzerBerechtigung, schema::benutzerberechtigung::SqlType> for models::BenutzerBerechtigung {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::BenutzerBerechtigung>, diesel::result::Error> {
        schema::benutzerberechtigung::dsl::benutzerberechtigung.load::<models::BenutzerBerechtigung>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::BenutzerBerechtigung, diesel::result::Error> {
        use schema::benutzerberechtigung::dsl::*;
        match benutzerberechtigung.filter(benutzerid.eq(self.benutzerid)).filter(benutzerid.eq(self.benutzerid)).load::<models::BenutzerBerechtigung>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::benutzerberechtigung::dsl::*;
        diesel::delete(benutzerberechtigung.filter(benutzerid.eq(self.benutzerid)).filter(benutzerid.eq(self.benutzerid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::benutzerberechtigung::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::ApiKey, schema::apikey::SqlType> for models::ApiKey {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::ApiKey>, diesel::result::Error> {
        schema::apikey::dsl::apikey.load::<models::ApiKey>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::ApiKey, diesel::result::Error> {
        use schema::apikey::dsl::*;
        match apikey.filter(id.eq(self.id)).load::<models::ApiKey>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::apikey::dsl::*;
        diesel::delete(apikey.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::apikey::table).set(self).execute(conn)
    }
}
