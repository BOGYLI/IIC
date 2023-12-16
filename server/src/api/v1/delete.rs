use actix_web_ext_proc_macro::delete;
use diesel_ext_traits::*;
use serde_derive::{Deserialize, Serialize};
use crate::db::models::*;

// artikel
delete!(Artikel);
delete!(ArtikelBenutzer);
delete!(ArtikelMedien);
delete!(Medien);
delete!(Template);
delete!(Parameter);
delete!(TemplateParameter);
delete!(ArtikelParameter);

// berechtigung
delete!(Benutzer);
delete!(BenutzerBerechtigung);
delete!(Berechtigung);
delete!(Apikey);

// mspiel
delete!(Mspiel);
delete!(MatchMspiel);
delete!(Team);
delete!(BenutzerTeam);

// sspiel
delete!(Sspiel);
delete!(BenutzerSspiel);

// umfrage
delete!(Umfrage);
delete!(Frage);
delete!(Antwort);
delete!(FrageAntwort);
delete!(UmfrageBenutzer);
delete!(UmfrageBenutzerFrage);
delete!(UmfrageFrage);

// extras
delete!(Revealjs);
delete!(Poster);