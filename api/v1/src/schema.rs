// @generated automatically by Diesel CLI.

diesel::table! {
    apikey (id) {
        id -> Int4,
        wert -> Text,
        zeitpunkt -> Text,
        dauer -> Int4,
    }
}

diesel::table! {
    artikel (id) {
        id -> Int4,
        pfad -> Text,
        erstelldatum -> Text,
        status -> Text,
        templateid -> Int4,
    }
}

diesel::table! {
    artikelautor (artikelid, benutzerid) {
        artikelid -> Int4,
        benutzerid -> Int4,
    }
}

diesel::table! {
    artikelmedien (artikelid, medienid) {
        artikelid -> Int4,
        medienid -> Int4,
    }
}

diesel::table! {
    benutzer (id) {
        id -> Int4,
        vorname -> Text,
        nachname -> Text,
        passwort -> Text,
        klasse -> Text,
        rolle -> Text,
    }
}

diesel::table! {
    benutzerberechtigung (benutzerid, berechtigungid) {
        benutzerid -> Int4,
        berechtigungid -> Int4,
    }
}

diesel::table! {
    benutzerteam (teamid, benutzerid) {
        teamid -> Int4,
        benutzerid -> Int4,
    }
}

diesel::table! {
    berechtigung (id) {
        id -> Int4,
        name -> Text,
        beschreibung -> Text,
        apikeyid -> Int4,
    }
}

diesel::table! {
    medien (id) {
        id -> Int4,
        typ -> Text,
        pfad -> Text,
        erstelldatum -> Text,
    }
}

diesel::table! {
    mspiel (id) {
        id -> Int4,
        name -> Text,
        apikeyid -> Int4,
        url -> Text,
        highscore -> Nullable<Int4>,
        best -> Nullable<Int4>,
    }
}

diesel::table! {
    mspieler (matchid, spielid) {
        matchid -> Int4,
        team1id -> Int4,
        team2id -> Int4,
        spielid -> Int4,
        score1 -> Int4,
        score2 -> Int4,
        level -> Int4,
        einstellungen1 -> Text,
        einstellungen2 -> Text,
    }
}

diesel::table! {
    sspiel (id) {
        id -> Int4,
        name -> Text,
        apikeyid -> Int4,
        url -> Text,
        highscore -> Nullable<Int4>,
        best -> Nullable<Int4>,
    }
}

diesel::table! {
    sspieler (benutzerid, spielid) {
        benutzerid -> Int4,
        spielid -> Int4,
        level -> Int4,
        highscore -> Int4,
        einstellungen -> Text,
    }
}

diesel::table! {
    team (id) {
        id -> Int4,
        name -> Text,
        overallscore -> Int4,
    }
}

diesel::table! {
    template (id) {
        id -> Int4,
        pfad -> Text,
    }
}

diesel::table! {
    templatetparameter (templateid, parameterid) {
        templateid -> Int4,
        parameterid -> Int4,
    }
}

diesel::table! {
    tparameter (id) {
        id -> Int4,
        typ -> Text,
        name -> Text,
    }
}

diesel::table! {
    uantwort (id) {
        id -> Int4,
        inhalt -> Text,
        typ -> Text,
    }
}

diesel::table! {
    ufrage (id) {
        id -> Int4,
        inhalt -> Text,
    }
}

diesel::table! {
    ufrageuantwort (frageid, antwortid) {
        frageid -> Int4,
        antwortid -> Int4,
    }
}

diesel::table! {
    umfrage (id) {
        id -> Int4,
        titel -> Text,
    }
}

diesel::table! {
    umfrageantwort (umfrageid, benutzerid, frageid) {
        umfrageid -> Int4,
        benutzerid -> Int4,
        frageid -> Int4,
        antwortid -> Int4,
        wert -> Nullable<Text>,
    }
}

diesel::table! {
    umfragebenutzer (umfrageid, benutzerid) {
        umfrageid -> Int4,
        benutzerid -> Int4,
    }
}

diesel::joinable!(artikel -> template (templateid));
diesel::joinable!(artikelautor -> artikel (artikelid));
diesel::joinable!(artikelautor -> benutzer (benutzerid));
diesel::joinable!(artikelmedien -> artikel (artikelid));
diesel::joinable!(artikelmedien -> medien (medienid));
diesel::joinable!(benutzerberechtigung -> benutzer (benutzerid));
diesel::joinable!(benutzerberechtigung -> berechtigung (berechtigungid));
diesel::joinable!(benutzerteam -> benutzer (benutzerid));
diesel::joinable!(benutzerteam -> team (teamid));
diesel::joinable!(berechtigung -> apikey (apikeyid));
diesel::joinable!(mspiel -> apikey (apikeyid));
diesel::joinable!(mspiel -> team (best));
diesel::joinable!(mspieler -> mspiel (spielid));
diesel::joinable!(sspiel -> apikey (apikeyid));
diesel::joinable!(sspiel -> benutzer (best));
diesel::joinable!(sspieler -> benutzer (benutzerid));
diesel::joinable!(sspieler -> sspiel (spielid));
diesel::joinable!(templatetparameter -> template (templateid));
diesel::joinable!(templatetparameter -> tparameter (parameterid));
diesel::joinable!(ufrageuantwort -> uantwort (antwortid));
diesel::joinable!(ufrageuantwort -> ufrage (frageid));
diesel::joinable!(umfrageantwort -> benutzer (benutzerid));
diesel::joinable!(umfrageantwort -> uantwort (antwortid));
diesel::joinable!(umfrageantwort -> ufrage (frageid));
diesel::joinable!(umfrageantwort -> umfrage (umfrageid));
diesel::joinable!(umfragebenutzer -> benutzer (benutzerid));
diesel::joinable!(umfragebenutzer -> umfrage (umfrageid));

diesel::allow_tables_to_appear_in_same_query!(
    apikey,
    artikel,
    artikelautor,
    artikelmedien,
    benutzer,
    benutzerberechtigung,
    benutzerteam,
    berechtigung,
    medien,
    mspiel,
    mspieler,
    sspiel,
    sspieler,
    team,
    template,
    templatetparameter,
    tparameter,
    uantwort,
    ufrage,
    ufrageuantwort,
    umfrage,
    umfrageantwort,
    umfragebenutzer,
);
