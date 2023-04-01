-- Your SQL goes here
CREATE TABLE SSpieler(
	benutzerId		INTEGER	NOT NULL,
	spielId			INTEGER	NOT NULL,
	level			INTEGER	NOT NULL,
	highscore		INTEGER	NOT NULL,
	einstellungen	TEXT	NOT NULL,
	
	PRIMARY KEY(benutzerId, spielId),
	FOREIGN KEY(benutzerId) REFERENCES Benutzer(id),
	FOREIGN KEY(spielId) REFERENCES SSpiel(id)
)
