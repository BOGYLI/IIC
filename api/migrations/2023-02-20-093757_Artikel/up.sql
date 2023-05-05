CREATE TABLE Artikel(
	id				SERIAL	NOT NULL,
	pfad			TEXT	NOT NULL,
	erstelldatum	TEXT	NOT NULL,
	status			TEXT	NOT NULL,
	templateId		INTEGER	NOT NULL,

	PRIMARY KEY(id),
	FOREIGN KEY(templateId) REFERENCES Template(id)
);
