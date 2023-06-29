use crate::db::DBQueryable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBQueryable<models::Artikel, schema::artikel::SqlType> for models::Artikel {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::Artikel>, diesel::result::Error> {
        schema::artikel::dsl::artikel.load::<models::Artikel>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::Artikel, diesel::result::Error> {
        use schema::artikel::dsl::*;
        match artikel.filter(id.eq(self.id)).load::<models::Artikel>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::artikel::dsl::*;
        diesel::delete(artikel.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::artikel::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::ArtikelAutor, schema::artikelautor::SqlType> for models::ArtikelAutor {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::ArtikelAutor>, diesel::result::Error> {
        schema::artikelautor::dsl::artikelautor.load::<models::ArtikelAutor>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::ArtikelAutor, diesel::result::Error> {
        use schema::artikelautor::dsl::*;
        match artikelautor.filter(artikelid.eq(self.artikelid)).filter(benutzerid.eq(self.benutzerid)).load::<models::ArtikelAutor>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::artikelautor::dsl::*;
        diesel::delete(artikelautor.filter(artikelid.eq(self.artikelid)).filter(benutzerid.eq(self.benutzerid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::artikelautor::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::ArtikelMedien, schema::artikelmedien::SqlType> for models::ArtikelMedien {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::ArtikelMedien>, diesel::result::Error> {
        schema::artikelmedien::dsl::artikelmedien.load::<models::ArtikelMedien>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::ArtikelMedien, diesel::result::Error> {
        use schema::artikelmedien::dsl::*;
        match artikelmedien.filter(artikelid.eq(self.artikelid)).filter(medienid.eq(self.medienid)).load::<models::ArtikelMedien>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::artikelmedien::dsl::*;
        diesel::delete(artikelmedien.filter(artikelid.eq(self.artikelid)).filter(medienid.eq(self.medienid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::artikelmedien::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::Medien, schema::medien::SqlType> for models::Medien {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::Medien>, diesel::result::Error> {
        schema::medien::dsl::medien.load::<models::Medien>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::Medien, diesel::result::Error> {
        use schema::medien::dsl::*;
        match medien.filter(id.eq(self.id)).load::<models::Medien>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::medien::dsl::*;
        diesel::delete(medien.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::medien::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::Template, schema::template::SqlType> for models::Template {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::Template>, diesel::result::Error> {
        schema::template::dsl::template.load::<models::Template>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::Template, diesel::result::Error> {
        use schema::template::dsl::*;
        match template.filter(id.eq(self.id)).load::<models::Template>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::template::dsl::*;
        diesel::delete(template.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::template::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::TParameter, schema::tparameter::SqlType> for models::TParameter {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::TParameter>, diesel::result::Error> {
        schema::tparameter::dsl::tparameter.load::<models::TParameter>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::TParameter, diesel::result::Error> {
        use schema::tparameter::dsl::*;
        match tparameter.filter(id.eq(self.id)).load::<models::TParameter>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::tparameter::dsl::*;
        diesel::delete(tparameter.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::tparameter::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::TemplateTParameter, schema::templatetparameter::SqlType> for models::TemplateTParameter {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::TemplateTParameter>, diesel::result::Error> {
        schema::templatetparameter::dsl::templatetparameter.load::<models::TemplateTParameter>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::TemplateTParameter, diesel::result::Error> {
        use schema::templatetparameter::dsl::*;
        match templatetparameter.filter(templateid.eq(self.templateid)).filter(parameterid.eq(self.parameterid)).load::<models::TemplateTParameter>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::templatetparameter::dsl::*;
        diesel::delete(templatetparameter.filter(templateid.eq(self.templateid)).filter(parameterid.eq(self.parameterid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::templatetparameter::table).set(self).execute(conn)
    }
}
