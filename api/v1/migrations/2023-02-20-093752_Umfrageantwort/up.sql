CREATE TABLE Umfrageantwort(
	umfrageId		INTEGER	NOT NULL,
	benutzerId		INTEGER	NOT NULL,
	frageId			INTEGER	NOT NULL,
	antwortId		INTEGER	NOT NULL,
	wert			TEXT,

	PRIMARY KEY(umfrageId, benutzerId, frageId, antwortId),
	FOREIGN KEY(umfrageId) REFERENCES Umfrage(id),
	FOREIGN KEY(benutzerId) REFERENCES Benutzer(id),
	FOREIGN KEY(frageId) REFERENCES UFrage(id),
	FOREIGN KEY(antwortId) REFERENCES UAntwort(id)
);
