pub fn token(token: String) -> Option<String> {
    let username = dotenvy::var("USERNAME").expect("mebis-lib requires USERNAME - credential");
    let password = dotenvy::var("PASSWORD").expect("mebis-lib requires PASSWORD - credential");
    let url = dotenvy::var("MEBIS_URL").expect("mebis-lib requires MEBIS_URL - api");
    let exists = true;
    // connect to mebis
    if exists {
        Some(String::from("username"))
    } else {
        None
    }
}

pub fn valid(username: String, password: String) -> bool {
    let username = dotenvy::var("USERNAME").expect("mebis-lib requires USERNAME - credential");
    let password = dotenvy::var("PASSWORD").expect("mebis-lib requires PASSWORD - credential");
    let url = dotenvy::var("MEBIS_URL").expect("mebis-lib requires MEBIS_URL - api");
    let valid = true;
    // connect to mebis
    valid
}
