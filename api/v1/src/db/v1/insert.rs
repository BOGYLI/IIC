use super::DBInsertable;
use super::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBInsertable<models::Umfrage, schema::umfrage::SqlType> for models::NewUmfrage<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Umfrage, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::umfrage::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::Medien, schema::medien::SqlType> for models::NewMedien<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Medien, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::medien::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::Template, schema::template::SqlType> for models::NewTemplate<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Template, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::template::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::TParameter, schema::tparameter::SqlType> for models::NewTParameter<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::TParameter, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::tparameter::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::TemplateTParameter, schema::templatetparameter::SqlType> for models::NewTemplateTParameter {
    fn new(self, conn: &mut PgConnection) -> Result<models::TemplateTParameter, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::templatetparameter::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::Benutzer, schema::benutzer::SqlType> for models::NewBenutzer<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Benutzer, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::benutzer::table).values(self).get_result(conn)
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
impl DBInsertable<models::Artikel, schema::artikel::SqlType> for models::NewArtikel<'_> {
    fn new(self, conn: &mut PgConnection) -> Result<models::Artikel, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::artikel::table).values(self).get_result(conn)
    }
}
impl DBInsertable<models::ArtikelAutor, schema::artikelautor::SqlType> for models::NewArtikelAutor {
    fn new(self, conn: &mut PgConnection) -> Result<models::ArtikelAutor, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::artikelautor::table).values(self).get_result(conn)
    }
}