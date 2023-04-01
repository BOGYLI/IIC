use crate::db::DBQueryable;
use crate::db::{models, schema};

use diesel::pg::PgConnection;
use diesel::prelude::*;

impl DBQueryable<models::MSpiel, schema::mspiel::SqlType> for models::MSpiel {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::MSpiel>, diesel::result::Error> {
        schema::mspiel::dsl::mspiel.load::<models::MSpiel>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::MSpiel, diesel::result::Error> {
        use schema::mspiel::dsl::*;
        match mspiel.filter(id.eq(self.id)).load::<models::MSpiel>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::mspiel::dsl::*;
        diesel::delete(mspiel.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::mspiel::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::MSpieler, schema::mspieler::SqlType> for models::MSpieler {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::MSpieler>, diesel::result::Error> {
        schema::mspieler::dsl::mspieler.load::<models::MSpieler>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::MSpieler, diesel::result::Error> {
        use schema::mspieler::dsl::*;
        match mspieler.filter(matchid.eq(self.matchid)).filter(spielid.eq(self.spielid)).load::<models::MSpieler>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::mspieler::dsl::*;
        diesel::delete(mspieler.filter(matchid.eq(self.matchid)).filter(spielid.eq(self.spielid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::mspieler::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::Team, schema::team::SqlType> for models::Team {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::Team>, diesel::result::Error> {
        schema::team::dsl::team.load::<models::Team>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::Team, diesel::result::Error> {
        use schema::team::dsl::*;
        match team.filter(id.eq(self.id)).load::<models::Team>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::team::dsl::*;
        diesel::delete(team.filter(id.eq(self.id))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::team::table).set(self).execute(conn)
    }
}
impl DBQueryable<models::BenutzerTeam, schema::benutzerteam::SqlType> for models::BenutzerTeam {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<models::BenutzerTeam>, diesel::result::Error> {
        schema::benutzerteam::dsl::benutzerteam.load::<models::BenutzerTeam>(conn)
    }
    fn get(&self, conn: &mut PgConnection) -> Result<models::BenutzerTeam, diesel::result::Error> {
        use schema::benutzerteam::dsl::*;
        match benutzerteam.filter(benutzerid.eq(self.benutzerid)).filter(teamid.eq(self.teamid)).load::<models::BenutzerTeam>(conn) {
            Ok(data) => Ok(data[0].clone()),
            Err(e) => Err(e)
        }
    }
    fn delete(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        use schema::benutzerteam::dsl::*;
        diesel::delete(benutzerteam.filter(benutzerid.eq(self.benutzerid)).filter(teamid.eq(self.teamid))).execute(conn)
    }
    fn update(&self, conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(schema::benutzerteam::table).set(self).execute(conn)
    }
}
