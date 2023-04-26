use crate::db::DBInsertable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

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
impl DBInsertable<models::ArtikelMedien, schema::artikelmedien::SqlType> for models::NewArtikelMedien {
    fn new(self, conn: &mut PgConnection) -> Result<models::ArtikelMedien, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::artikelmedien::table).values(self).get_result(conn)
    }
}
