-- Your SQL goes here
CREATE TABLE ArtikelParameter(
    artikelId       INTEGER NOT NULL,
    parameterId     INTEGER NOT NULL,
    wert            TEXT NOT NULL,

    PRIMARY KEY(artikelId, parameterId, wert),
	FOREIGN KEY(artikelId) REFERENCES Artikel(id),
	FOREIGN KEY(parameterId) REFERENCES Parameter(id)
);