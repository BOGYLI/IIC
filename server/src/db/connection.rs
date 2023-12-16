use diesel_ext_traits::*;
use serde_derive::{Deserialize, Serialize};
use crate::db::models::*;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;

// artikel
//connection!(Artikel);
connection!(Artikel, Benutzer);
connection!(Artikel, Medien);
//connection!(Medien);
//connection!(Template);
//connection!(Parameter);
connection!(Template, Parameter);
connection!(Artikel, Parameter);

// berechtigung
//connection!(Benutzer);
connection!(Benutzer, Berechtigung);
//connection!(Berechtigung);
//connection!(Apikey);

// mspiel
//connection!(Mspiel);
//      connection!(MatchMspiel);
//connection!(Team);
connection!(Benutzer, Team);

// sspiel
//connection!(Sspiel);
//      connection!(BenutzerSspiel);

// umfrage
//connection!(Umfrage);
//connection!(Frage);
//connection!(Antwort);
connection!(Frage, Antwort);
connection!(Umfrage, Benutzer);
//      connection!(UmfrageBenutzerFrage);
connection!(Umfrage, Frage);

// extras
//connection!(Revealjs);
//connection!(Poster);