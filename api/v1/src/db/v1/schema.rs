// @generated automatically by Diesel CLI.

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
    benutzer (id) {
        id -> Int4,
        vorname -> Text,
        nachname -> Text,
        klasse -> Text,
        rolle -> Text,
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
    umfrageantwort (umfrageid, benutzerid, frageid, antwortid) {
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
    artikel,
    artikelautor,
    benutzer,
    medien,
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
