-- Your SQL goes here
CREATE TABLE UmfrageUFrage(
	umfrageId	INTEGER	NOT NULL,
	frageId		INTEGER	NOT NULL,

	PRIMARY KEY(umfrageId, frageId),
	FOREIGN KEY(umfrageId) REFERENCES Umfrage(id),
	FOREIGN KEY(frageId) REFERENCES UFrage(id)
);
