CREATE TABLE Medien(
	id				SERIAL	NOT NULL,
	typ				TEXT	NOT NULL,
	pfad			TEXT	NOT NULL,
	erstelldatum	TEXT	NOT NULL,

	PRIMARY KEY(id)
);
