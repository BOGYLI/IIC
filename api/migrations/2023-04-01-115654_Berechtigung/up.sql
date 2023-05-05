-- Your SQL goes here
CREATE TABLE Berechtigung(
	id				SERIAL	NOT NULL,
	name			TEXT	NOT NULL,
	beschreibung	TEXT	NOT NULL,
	apikeyId		INTEGER	NOT NULL,
	
	PRIMARY KEY(id),
	FOREIGN KEY(apikeyId) REFERENCES ApiKey(id)
)
