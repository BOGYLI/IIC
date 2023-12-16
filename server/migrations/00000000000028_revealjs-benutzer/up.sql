-- Your SQL goes here
CREATE TABLE RevealjsBenutzer(
	revealjsid	INTEGER	NOT NULL,
	benutzerid	INTEGER	NOT NULL,

	PRIMARY KEY(revealjsid, benutzerid),
	FOREIGN KEY(revealjsid) REFERENCES Revealjs(id),
	FOREIGN KEY(benutzerid) REFERENCES Benutzer(id)
);
