use actix_web_ext_proc_macro::insert;

use crate::db::models::*;
use diesel_ext_traits::DBInsertable;

// artikel
insert!(Artikel);
insert!(ArtikelBenutzer);
insert!(ArtikelMedien);
insert!(Medien);
insert!(Template);
insert!(Parameter);
insert!(TemplateParameter);
insert!(ArtikelParameter);

// berechtigung
insert!(Benutzer);
insert!(BenutzerBerechtigung);
insert!(Berechtigung);
insert!(Apikey);

// mspiel
insert!(Mspiel);
insert!(MatchMspiel);
insert!(Team);
insert!(BenutzerTeam);

// sspiel
insert!(Sspiel);
insert!(BenutzerSspiel);

// umfrage
insert!(Umfrage);
insert!(Frage);
insert!(Antwort);
insert!(FrageAntwort);
insert!(UmfrageBenutzer);
insert!(UmfrageBenutzerFrage);
insert!(UmfrageFrage);

// extras
insert!(Revealjs);
insert!(Poster);