use crate::db::DBQueryable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBQueryable<models::SSpiel, schema::sspiel::SqlType> for models::SSpiel {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::SSpiel>, diesel::result::Error> {
        schema::sspiel::dsl::sspiel.load::<models::SSpiel>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::SSpiel, diesel::result::Error> {
        use schema::sspiel::dsl::*;
        match sspiel.filter(id.eq(self.id)).load::<models::SSpiel>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::sspiel::dsl::*;
        diesel::delete(sspiel.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::sspiel::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::SSpieler, schema::sspieler::SqlType> for models::SSpieler {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::SSpieler>, diesel::result::Error> {
        schema::sspieler::dsl::sspieler.load::<models::SSpieler>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::SSpieler, diesel::result::Error> {
        use schema::sspieler::dsl::*;
        match sspieler.filter(benutzerid.eq(self.benutzerid)).filter(spielid.eq(self.spielid)).load::<models::SSpieler>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::sspieler::dsl::*;
        diesel::delete(sspieler.filter(benutzerid.eq(self.benutzerid)).filter(spielid.eq(self.spielid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::sspieler::table).set(self).execute(conn)
    }
}
