use crate::db::DBInsertable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBInsertable<models::Umfrage, schema::umfrage::SqlType> for models::NewUmfrage<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Umfrage, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::umfrage::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::UFrage, schema::ufrage::SqlType> for models::NewUFrage<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::UFrage, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::ufrage::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::UAntwort, schema::uantwort::SqlType> for models::NewUAntwort<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::UAntwort, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::uantwort::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::UFrageUAntwort, schema::ufrageuantwort::SqlType> for models::NewUFrageUAntwort {
    fn new(self, conn: &mut PgConnection) -> Result<models::UFrageUAntwort, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::ufrageuantwort::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::UmfrageBenutzer, schema::umfragebenutzer::SqlType> for models::NewUmfrageBenutzer {
    fn new(self, conn: &mut PgConnection) -> Result<models::UmfrageBenutzer, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::umfragebenutzer::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::Umfrageantwort, schema::umfrageantwort::SqlType> for models::NewUmfrageantwort<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Umfrageantwort, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::umfrageantwort::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::UmfrageUFrage, schema::umfrageufrage::SqlType> for models::NewUmfrageUFrage {
    fn new(self, conn: &mut PgConnection) -> Result<models::UmfrageUFrage, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::umfrageufrage::table).values(self).get_result(conn)
    }
}
