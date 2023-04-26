impl NewUmfrageantwort {
    pub fn from(umfrageid: i32,
        benutzerid: i32,
        frageid: i32,
        antwortid: i32) -> Self {
        Self {
            umfrageid: umfrageid,
            benutzerid: benutzerid,
            frageid: frageid,
            antwortid: antwortid,
            wert: Option::None
        }
}