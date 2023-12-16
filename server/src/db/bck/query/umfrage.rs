use crate::db::DBQueryable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBQueryable<models::Umfrage, schema::umfrage::SqlType> for models::Umfrage {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::Umfrage>, diesel::result::Error> {
        schema::umfrage::dsl::umfrage.load::<models::Umfrage>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::Umfrage, diesel::result::Error> {
        use schema::umfrage::dsl::*;
        match umfrage.filter(id.eq(self.id)).load::<models::Umfrage>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::umfrage::dsl::*;
        diesel::delete(umfrage.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::umfrage::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::UFrage, schema::ufrage::SqlType> for models::UFrage {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::UFrage>, diesel::result::Error> {
        schema::ufrage::dsl::ufrage.load::<models::UFrage>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::UFrage, diesel::result::Error> {
        use schema::ufrage::dsl::*;
        match ufrage.filter(id.eq(self.id)).load::<models::UFrage>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::ufrage::dsl::*;
        diesel::delete(ufrage.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::ufrage::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::UAntwort, schema::uantwort::SqlType> for models::UAntwort {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::UAntwort>, diesel::result::Error> {
        schema::uantwort::dsl::uantwort.load::<models::UAntwort>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::UAntwort, diesel::result::Error> {
        use schema::uantwort::dsl::*;
        match uantwort.filter(id.eq(self.id)).load::<models::UAntwort>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::uantwort::dsl::*;
        diesel::delete(uantwort.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::uantwort::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::UFrageUAntwort, schema::ufrageuantwort::SqlType> for models::UFrageUAntwort {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::UFrageUAntwort>, diesel::result::Error> {
        schema::ufrageuantwort::dsl::ufrageuantwort.load::<models::UFrageUAntwort>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::UFrageUAntwort, diesel::result::Error> {
        use schema::ufrageuantwort::dsl::*;
        match ufrageuantwort.filter(frageid.eq(self.frageid)).filter(antwortid.eq(self.antwortid)).load::<models::UFrageUAntwort>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::ufrageuantwort::dsl::*;
        diesel::delete(ufrageuantwort.filter(frageid.eq(self.frageid)).filter(antwortid.eq(self.antwortid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::ufrageuantwort::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::UmfrageBenutzer, schema::umfragebenutzer::SqlType> for models::UmfrageBenutzer {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::UmfrageBenutzer>, diesel::result::Error> {
        schema::umfragebenutzer::dsl::umfragebenutzer.load::<models::UmfrageBenutzer>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::UmfrageBenutzer, diesel::result::Error> {
        use schema::umfragebenutzer::dsl::*;
        match umfragebenutzer.filter(umfrageid.eq(self.umfrageid)).filter(benutzerid.eq(self.benutzerid)).load::<models::UmfrageBenutzer>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::umfragebenutzer::dsl::*;
        diesel::delete(umfragebenutzer.filter(umfrageid.eq(self.umfrageid)).filter(benutzerid.eq(self.benutzerid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::umfragebenutzer::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::Umfrageantwort, schema::umfrageantwort::SqlType> for models::Umfrageantwort {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::Umfrageantwort>, diesel::result::Error> {
        schema::umfrageantwort::dsl::umfrageantwort.load::<models::Umfrageantwort>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::Umfrageantwort, diesel::result::Error> {
        use schema::umfrageantwort::dsl::*;
        match umfrageantwort.filter(umfrageid.eq(self.umfrageid)).filter(benutzerid.eq(self.benutzerid)).filter(frageid.eq(self.frageid)).filter(antwortid.eq(self.antwortid)).load::<models::Umfrageantwort>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::umfrageantwort::dsl::*;
        diesel::delete(umfrageantwort.filter(umfrageid.eq(self.umfrageid)).filter(benutzerid.eq(self.benutzerid)).filter(frageid.eq(self.frageid)).filter(antwortid.eq(self.antwortid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::umfrageantwort::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::UmfrageUFrage, schema::umfrageufrage::SqlType> for models::UmfrageUFrage {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::UmfrageUFrage>, diesel::result::Error> {
        schema::umfrageufrage::dsl::umfrageufrage.load::<models::UmfrageUFrage>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::UmfrageUFrage, diesel::result::Error> {
        use schema::umfrageufrage::dsl::*;
        match umfrageufrage.filter(umfrageid.eq(self.umfrageid)).filter(frageid.eq(self.frageid)).load::<models::UmfrageUFrage>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::umfrageufrage::dsl::*;
        diesel::delete(umfrageufrage.filter(umfrageid.eq(self.umfrageid)).filter(frageid.eq(self.frageid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::umfrageufrage::table).set(self).execute(conn)
    }
}
