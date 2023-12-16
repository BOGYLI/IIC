CREATE TABLE FrageAntwort(
	frageId			INTEGER	NOT NULL,
	antwortId		INTEGER	NOT NULL,

	PRIMARY KEY(frageId, antwortId),
	FOREIGN KEY(frageId) REFERENCES Frage(id),
	FOREIGN KEY(antwortId) REFERENCES Antwort(id)
);
