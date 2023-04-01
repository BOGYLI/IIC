use crate::db::DBInsertable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBInsertable<models::Benutzer, schema::benutzer::SqlType> for models::NewBenutzer<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Benutzer, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::benutzer::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::Berechtigung, schema::berechtigung::SqlType> for models::NewBerechtigung<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Berechtigung, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::berechtigung::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::BenutzerBerechtigung, schema::benutzerberechtigung::SqlType> for models::NewBenutzerBerechtigung {
    fn new(self, conn: &mut PgConnection) -> Result<models::BenutzerBerechtigung, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::benutzerberechtigung::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::ApiKey, schema::apikey::SqlType> for models::NewApiKey<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::ApiKey, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::apikey::table).values(self).get_result(conn)
    }
}
