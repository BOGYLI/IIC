-- Your SQL goes here
CREATE TABLE Mspiel(
	id			SERIAL	NOT NULL,
	name 		TEXT	NOT NULL,
	apikeyId 	INTEGER	NOT NULL,
	url 		TEXT	NOT NULL,
	highscore	INTEGER,
	best		INTEGER,
	
	PRIMARY KEY(id),
	FOREIGN KEY(apikeyId) REFERENCES ApiKey(id),
	FOREIGN KEY(best) REFERENCES Team(id)
);
