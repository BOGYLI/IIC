use actix_web_ext_proc_macro::update;

use crate::db::models::*;
use diesel_ext_traits::DBQueryable;

// artikel
update!(Artikel);
update!(ArtikelBenutzer);
update!(ArtikelMedien);
update!(Medien);
update!(Template);
update!(Parameter);
update!(TemplateParameter);
update!(ArtikelParameter);

// berechtigung
update!(Benutzer);
update!(BenutzerBerechtigung);
update!(Berechtigung);
update!(Apikey);

// mspiel
update!(Mspiel);
update!(MatchMspiel);
update!(Team);
update!(BenutzerTeam);

// sspiel
update!(Sspiel);
update!(BenutzerSspiel);

// umfrage
update!(Umfrage);
update!(Frage);
update!(Antwort);
update!(FrageAntwort);
update!(UmfrageBenutzer);
update!(UmfrageBenutzerFrage);
update!(UmfrageFrage);

// extras
update!(Revealjs);
update!(Poster);