use diesel::{prelude::*, result::Error};

use crate::{db::models};

use super::models::{Umfrage, UFrage, UAntwort, Umfrageantwort};

impl Umfrage {
    pub fn result2(&self) -> Vec<(UFrage, Vec<(UAntwort, Vec<Umfrageantwort>)>)> {
        let ufragen = UFrage::get_by_umfrage(self.id).expect("REASON");
        let mut res: Vec<(UFrage, Vec<(UAntwort, Vec<Umfrageantwort>)>)> = Vec::with_capacity(ufragen.len());
        for i in 0..ufragen.len() {
            let uantworten: Vec<UAntwort> = UAntwort::get_by_ufrage(ufragen[i].id);
            let mut tmp: Vec<(UAntwort, Vec<Umfrageantwort>)> = Vec::with_capacity(uantworten.len());
            for b in 0..uantworten.len() {
                let umfrageantworten: Vec<Umfrageantwort> = Umfrageantwort::get_by_uantwort(uantworten[b].id);
                tmp[b] = (uantworten[b].clone(), umfrageantworten.clone());
            }
            res[i] = (ufragen[i].clone(), tmp.clone());
        }
        res
    }

    pub fn result(&self) -> Vec<Umfrageantwort> {
        Umfrageantwort::get_by_umfrage(self.id)
    }
}

impl UFrage {
    pub fn get_by_umfrage(id: i32) -> Result<Vec<UFrage>, Error> {
        use crate::schema::ufrage::dsl::{ufrage};
        use crate::schema::umfrage::dsl::{umfrage};
        use crate::schema::umfrageufrage::dsl::{umfrageufrage, umfrageid};
        let conn = &mut crate::db::establish_connection();
        umfrageufrage.inner_join(umfrage).inner_join(ufrage).filter(umfrageid.eq(id)).select(UFrage::as_select()).load::<models::UFrage>(conn)
    }

    pub fn get_uantworten(&self) -> Result<Vec<UAntwort>, Error> {
        use crate::schema::ufrage::dsl::{ufrage};
        use crate::schema::uantwort::dsl::{uantwort};
        use crate::schema::ufrageuantwort::dsl::{ufrageuantwort, frageid};
        let conn = &mut crate::db::establish_connection();
        ufrageuantwort.inner_join(uantwort).inner_join(ufrage).filter(frageid.eq(self.id)).select(UAntwort::as_select()).load::<models::UAntwort>(conn)
    }
}

impl UAntwort {
    pub fn get_by_ufrage(id: i32) -> Vec<UAntwort> {
        use crate::schema::uantwort::dsl::{uantwort};
        //use crate::schema::ufrage::dsl::{ufrage};
        use crate::schema::ufrageuantwort::dsl::{ufrageuantwort, frageid};
        
        let conn = &mut crate::db::establish_connection();
        ufrageuantwort.inner_join(uantwort).filter(frageid.eq(id)).select(UAntwort::as_select()).load::<models::UAntwort>(conn).unwrap()
    }
}

impl Umfrageantwort {
    pub fn get_by_uantwort(id: i32) -> Vec<Umfrageantwort> {
        use crate::schema::umfrageantwort::dsl::*;

        let conn = &mut crate::db::establish_connection();
        umfrageantwort.filter(antwortid.eq(id)).load::<models::Umfrageantwort>(conn).unwrap()
    }

    pub fn get_by_umfrage(id: i32) -> Vec<Umfrageantwort> {
        use crate::schema::umfrageantwort::dsl::*;
        
        let conn = &mut crate::db::establish_connection();
        umfrageantwort.filter(umfrageid.eq(id)).load::<models::Umfrageantwort>(conn).unwrap()
    }
}