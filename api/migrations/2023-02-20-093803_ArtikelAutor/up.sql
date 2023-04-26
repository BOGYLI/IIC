CREATE TABLE ArtikelAutor(
	artikelId		INTEGER	NOT NULL,
	benutzerId		INTEGER	NOT NULL,

	PRIMARY KEY(artikelId, benutzerId),
	FOREIGN KEY(artikelId) REFERENCES Artikel(id),
	FOREIGN KEY(benutzerId) REFERENCES Benutzer(id)
);
