use crate::db::DBInsertable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBInsertable<models::MSpiel, schema::mspiel::SqlType> for models::NewMSpiel<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::MSpiel, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::mspiel::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::MSpieler, schema::mspieler::SqlType> for models::NewMSpieler<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::MSpieler, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::mspieler::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::Team, schema::team::SqlType> for models::NewTeam<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Team, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::team::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::BenutzerTeam, schema::benutzerteam::SqlType> for models::NewBenutzerTeam {
    fn new(self, conn: &mut PgConnection) -> Result<models::BenutzerTeam, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::benutzerteam::table).values(self).get_result(conn)
    }
}
