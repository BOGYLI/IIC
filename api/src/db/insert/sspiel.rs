use crate::db::DBInsertable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBInsertable<models::SSpiel, schema::sspiel::SqlType> for models::NewSSpiel<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::SSpiel, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::sspiel::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::SSpieler, schema::sspieler::SqlType> for models::NewSSpieler<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::SSpieler, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::sspieler::table).values(self).get_result(conn)
    }
}
