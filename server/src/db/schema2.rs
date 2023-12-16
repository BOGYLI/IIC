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
    artikelbenutzer (artikelid, benutzerid) {
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
        name -> Text,
        passwort -> Text,
        mebistoken -> Text,
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
    matchmspiel (matchid, mspielid) {
        matchid -> Int4,
        team1id -> Int4,
        team2id -> Int4,
        mspielid -> Int4,
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
    benutzersspiel (benutzerid, sspielid) {
        benutzerid -> Int4,
        sspielid -> Int4,
        level -> Int4,
        highscore -> Int4,
        einstellungen -> Text,
    }
}

diesel::table! {
    team (id) {
        id -> Int4,
        name -> Text,
        apikeyid -> Int4,
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
    templatetparameter (templateid, tparameterid) {
        templateid -> Int4,
        tparameterid -> Int4,
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
    antwort (id) {
        id -> Int4,
        inhalt -> Text,
        typ -> Text,
    }
}

diesel::table! {
    frage (id) {
        id -> Int4,
        inhalt -> Text,
    }
}

diesel::table! {
    frageantwort (frageid, antwortid) {
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
    umfragebenutzerfrage (umfrageid, benutzerid, frageid) {
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

diesel::table! {
    umfragefrage (umfrageid, frageid) {
        umfrageid -> Int4,
        frageid -> Int4,
    }
}

diesel::joinable!(artikel -> template (templateid));
diesel::joinable!(artikelbenutzer -> artikel (artikelid));
diesel::joinable!(artikelbenutzer -> benutzer (benutzerid));
diesel::joinable!(artikelmedien -> artikel (artikelid));
diesel::joinable!(artikelmedien -> medien (medienid));
diesel::joinable!(benutzerberechtigung -> benutzer (benutzerid));
diesel::joinable!(benutzerberechtigung -> berechtigung (berechtigungid));
diesel::joinable!(benutzerteam -> benutzer (benutzerid));
diesel::joinable!(benutzerteam -> team (teamid));
diesel::joinable!(berechtigung -> apikey (apikeyid));
diesel::joinable!(mspiel -> apikey (apikeyid));
diesel::joinable!(mspiel -> team (best));
diesel::joinable!(matchmspiel -> mspiel (mspielid));
diesel::joinable!(sspiel -> apikey (apikeyid));
diesel::joinable!(sspiel -> benutzer (best));
diesel::joinable!(benutzersspiel -> benutzer (benutzerid));
diesel::joinable!(benutzersspiel -> sspiel (sspielid));
diesel::joinable!(templatetparameter -> template (templateid));
diesel::joinable!(templatetparameter -> tparameter (tparameterid));
diesel::joinable!(frageantwort -> antwort (antwortid));
diesel::joinable!(frageantwort -> frage (frageid));
diesel::joinable!(umfragebenutzerfrage -> benutzer (benutzerid));
diesel::joinable!(umfragebenutzerfrage -> antwort (antwortid));
diesel::joinable!(umfragebenutzerfrage -> frage (frageid));
diesel::joinable!(umfragebenutzerfrage -> umfrage (umfrageid));
diesel::joinable!(umfragebenutzer -> benutzer (benutzerid));
diesel::joinable!(umfragebenutzer -> umfrage (umfrageid));
diesel::joinable!(umfragefrage -> frage (frageid));
diesel::joinable!(umfragefrage -> umfrage (umfrageid));

diesel::allow_tables_to_appear_in_same_query!(
    apikey,
    artikel,
    artikelbenutzer,
    artikelmedien,
    benutzer,
    benutzerberechtigung,
    benutzerteam,
    berechtigung,
    medien,
    mspiel,
    matchmspiel,
    sspiel,
    benutzersspiel,
    team,
    template,
    templatetparameter,
    tparameter,
    antwort,
    frage,
    frageantwort,
    umfrage,
    umfragebenutzerfrage,
    umfragebenutzer,
    umfragefrage,
);
