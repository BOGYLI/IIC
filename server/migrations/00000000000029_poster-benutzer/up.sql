-- Your SQL goes here
CREATE TABLE PosterBenutzer(
	posterid	INTEGER	NOT NULL,
	benutzerid	INTEGER	NOT NULL,

	PRIMARY KEY(posterid, benutzerid),
	FOREIGN KEY(posterid) REFERENCES Poster(id),
	FOREIGN KEY(benutzerid) REFERENCES Benutzer(id)
);
