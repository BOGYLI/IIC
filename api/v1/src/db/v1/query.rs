use super::DBQueryable;
use super::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBQueryable<models::Umfrage, schema::umfrage::SqlType> for models::Umfrage {
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::Umfrage>, diesel::result::Error> {
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
impl DBQueryable<models::Medien, schema::medien::SqlType> for models::Medien {
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::Medien>, diesel::result::Error> {
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
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::Template>, diesel::result::Error> {
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
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::TParameter>, diesel::result::Error> {
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
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::TemplateTParameter>, diesel::result::Error> {
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
impl DBQueryable<models::Benutzer, schema::benutzer::SqlType> for models::Benutzer {
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::Benutzer>, diesel::result::Error> {
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
impl DBQueryable<models::UFrage, schema::ufrage::SqlType> for models::UFrage {
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::UFrage>, diesel::result::Error> {
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
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::UAntwort>, diesel::result::Error> {
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
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::UFrageUAntwort>, diesel::result::Error> {
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
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::UmfrageBenutzer>, diesel::result::Error> {
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
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::Umfrageantwort>, diesel::result::Error> {
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
impl DBQueryable<models::Artikel, schema::artikel::SqlType> for models::Artikel {
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::Artikel>, diesel::result::Error> {
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
    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<models::ArtikelAutor>, diesel::result::Error> {
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