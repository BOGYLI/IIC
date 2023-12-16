-- Your SQL goes here
CREATE TABLE Sspiel(
	id			SERIAL	NOT NULL,
	name 		TEXT	NOT NULL,
	apikeyId 	INTEGER	NOT NULL,
	url 		TEXT	NOT NULL,
	highscore	INTEGER,
	best		INTEGER,
	
	PRIMARY KEY(id),
	FOREIGN KEY(apikeyId) REFERENCES Apikey(id),
	FOREIGN KEY(best) REFERENCES Benutzer(id)
);
