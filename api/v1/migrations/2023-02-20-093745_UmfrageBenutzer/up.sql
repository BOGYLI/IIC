CREATE TABLE UmfrageBenutzer(
	umfrageId		INTEGER	NOT NULL,
	benutzerId		INTEGER	NOT NULL,

	PRIMARY KEY(umfrageId, benutzerId),
	FOREIGN KEY(umfrageId) REFERENCES Umfrage(id),
	FOREIGN KEY(benutzerId) REFERENCES Benutzer(id)
);
