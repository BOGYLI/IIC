CREATE TABLE Benutzer(
	id				SERIAL	NOT NULL,
	vorname			TEXT	NOT NULL,
	nachname		TEXT	NOT NULL,
	klasse			TEXT	NOT NULL,
	rolle			TEXT	NOT NULL,

	PRIMARY KEY(id)
);
