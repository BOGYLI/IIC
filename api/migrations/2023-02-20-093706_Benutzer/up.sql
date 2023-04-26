CREATE TABLE Benutzer(
	id				SERIAL	NOT NULL,
	name			TEXT	NOT NULL,
	passwort		TEXT	NOT NULL,
	mebistoken		TEXT	NOT NULL,

	PRIMARY KEY(id)
);
