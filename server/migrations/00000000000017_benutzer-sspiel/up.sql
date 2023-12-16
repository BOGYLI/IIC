-- Your SQL goes here
CREATE TABLE BenutzerSspiel(
	benutzerId		INTEGER	NOT NULL,
	sspielId			INTEGER	NOT NULL,
	level			INTEGER	NOT NULL,
	highscore		INTEGER	NOT NULL,
	einstellungen	TEXT	NOT NULL,
	
	PRIMARY KEY(benutzerId, sspielId),
	FOREIGN KEY(benutzerId) REFERENCES Benutzer(id),
	FOREIGN KEY(sspielId) REFERENCES Sspiel(id)
);
