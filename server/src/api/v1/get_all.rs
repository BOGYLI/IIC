use actix_web_ext_proc_macro::get_all;

use crate::db::models::*;
use diesel_ext_traits::DBQueryable;

// artikel
get_all!(Artikel);
get_all!(ArtikelBenutzer);
get_all!(ArtikelMedien);
get_all!(Medien);
get_all!(Template);
get_all!(Parameter);
get_all!(TemplateParameter);
get_all!(ArtikelParameter);

// berechtigung
get_all!(Benutzer);
get_all!(BenutzerBerechtigung);
get_all!(Berechtigung);
get_all!(Apikey);

// mspiel
get_all!(Mspiel);
get_all!(MatchMspiel);
get_all!(Team);
get_all!(BenutzerTeam);

// sspiel
get_all!(Sspiel);
get_all!(BenutzerSspiel);

// umfrage
get_all!(Umfrage);
get_all!(Frage);
get_all!(Antwort);
get_all!(FrageAntwort);
get_all!(UmfrageBenutzer);
get_all!(UmfrageBenutzerFrage);
get_all!(UmfrageFrage);

// extras
get_all!(Revealjs);
get_all!(Poster);