use diesel::{prelude::*, result::Error};

use crate::{db::models};

use super::models::{Umfrage, Frage, Antwort, UmfrageBenutzerFrage};

impl Umfrage {
    pub fn result2(&self) -> Vec<(Frage, Vec<(Antwort, Vec<UmfrageBenutzerFrage>)>)> {
        let Fragen = Frage::get_by_umfrage(self.id).expect("REASON");
        let mut res: Vec<(Frage, Vec<(Antwort, Vec<UmfrageBenutzerFrage>)>)> = Vec::with_capacity(Fragen.len());
        for i in 0..Fragen.len() {
            let Antworten: Vec<Antwort> = Antwort::get_by_Frage(Fragen[i].id);
            let mut tmp: Vec<(Antwort, Vec<UmfrageBenutzerFrage>)> = Vec::with_capacity(Antworten.len());
            for b in 0..Antworten.len() {
                let UmfrageBenutzerFrageen: Vec<UmfrageBenutzerFrage> = UmfrageBenutzerFrage::get_by_Antwort(Antworten[b].id);
                tmp[b] = (Antworten[b].clone(), UmfrageBenutzerFrageen.clone());
            }
            res[i] = (Fragen[i].clone(), tmp.clone());
        }
        res
    }

    pub fn result(&self) -> Vec<UmfrageBenutzerFrage> {
        UmfrageBenutzerFrage::get_by_umfrage(self.id)
    }
}

impl Frage {
    pub fn get_by_umfrage(id: i32) -> Result<Vec<Frage>, Error> {
        use crate::db::schema::frage::dsl::{frage};
        use crate::db::schema::umfrage::dsl::{umfrage};
        use crate::db::schema::umfragebenutzerfrage::dsl::{umfragebenutzerfrage, umfrageid};
        let conn = &mut crate::db::establish_connection();
        umfragebenutzerfrage.inner_join(umfrage).inner_join(frage).filter(umfrageid.eq(id)).select(Frage::as_select()).load::<models::Frage>(conn)
    }

    pub fn get_Antworten(&self) -> Result<Vec<Antwort>, Error> {
        use crate::db::schema::frage::dsl::{frage};
        use crate::db::schema::antwort::dsl::{antwort};
        use crate::db::schema::frageantwort::dsl::{frageantwort, frageid};
        let conn = &mut crate::db::establish_connection();
        frageantwort.inner_join(antwort).inner_join(frage).filter(frageid.eq(self.id)).select(Antwort::as_select()).load::<models::Antwort>(conn)
    }
}

impl Antwort {
    pub fn get_by_Frage(id: i32) -> Vec<Antwort> {
        use crate::db::schema::antwort::dsl::{antwort};
        //use crate::db::schema::frage::dsl::{Frage};
        use crate::db::schema::frageantwort::dsl::{frageantwort, frageid};
        
        let conn = &mut crate::db::establish_connection();
        frageantwort.inner_join(antwort).filter(frageid.eq(id)).select(Antwort::as_select()).load::<models::Antwort>(conn).unwrap()
    }
}

impl UmfrageBenutzerFrage {
    pub fn get_by_Antwort(id: i32) -> Vec<UmfrageBenutzerFrage> {
        use crate::db::schema::umfragebenutzerfrage::dsl::*;

        let conn = &mut crate::db::establish_connection();
        umfragebenutzerfrage.filter(antwortid.eq(id)).load::<models::UmfrageBenutzerFrage>(conn).unwrap()
    }

    pub fn get_by_umfrage(id: i32) -> Vec<UmfrageBenutzerFrage> {
        use crate::db::schema::umfragebenutzerfrage::dsl::*;
        
        let conn = &mut crate::db::establish_connection();
        umfragebenutzerfrage.filter(umfrageid.eq(id)).load::<models::UmfrageBenutzerFrage>(conn).unwrap()
    }
}