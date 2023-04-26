-- Your SQL goes here
CREATE TABLE ArtikelMedien (
	artikelId			INTEGER	NOT NULL,
	medienId			INTEGER	NOT NULL,

	PRIMARY KEY(artikelId, medienId),
	FOREIGN KEY(artikelId) REFERENCES Artikel(id),
	FOREIGN KEY(medienId) REFERENCES Medien(id)
);
