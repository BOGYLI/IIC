CREATE TABLE UmfrageBenutzerFrage(
	umfrageId		INTEGER	NOT NULL,
	benutzerId		INTEGER	NOT NULL,
	frageId			INTEGER	NOT NULL,
	antwortId		INTEGER	NOT NULL,
	wert			TEXT,

	PRIMARY KEY(umfrageId, benutzerId, frageId),
	FOREIGN KEY(umfrageId) REFERENCES Umfrage(id),
	FOREIGN KEY(benutzerId) REFERENCES Benutzer(id),
	FOREIGN KEY(frageId) REFERENCES Frage(id),
	FOREIGN KEY(antwortId) REFERENCES Antwort(id)
);
