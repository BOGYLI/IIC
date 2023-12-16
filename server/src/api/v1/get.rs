use actix_web_ext_proc_macro::get;

use crate::db::models::*;
use serde_derive::{Serialize, Deserialize};

// artikel
get!(Artikel);
get!(ArtikelBenutzer);
get!(ArtikelMedien);
get!(Medien);
get!(Template);
get!(Parameter);
get!(TemplateParameter);
get!(ArtikelParameter);

// berechtigung
get!(Benutzer);
get!(BenutzerBerechtigung);
get!(Berechtigung);
get!(Apikey);

// mspiel
get!(Mspiel);
get!(MatchMspiel);
get!(Team);
get!(BenutzerTeam);

// sspiel
get!(Sspiel);
get!(BenutzerSspiel);

// umfrage
get!(Umfrage);
get!(Frage);
get!(Antwort);
get!(FrageAntwort);
get!(UmfrageBenutzer);
get!(UmfrageBenutzerFrage);
get!(UmfrageFrage);

// extras
get!(Revealjs);
get!(Poster);