CREATE TABLE UFrageUAntwort(
	frageId			INTEGER	NOT NULL,
	antwortId		INTEGER	NOT NULL,

	PRIMARY KEY(frageId, antwortId),
	FOREIGN KEY(frageId) REFERENCES UFrage(id),
	FOREIGN KEY(antwortId) REFERENCES UAntwort(id)
);
