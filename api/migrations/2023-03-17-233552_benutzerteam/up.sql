-- Your SQL goes here
CREATE TABLE BenutzerTeam(
	teamId		INTEGER NOT NULL,
	benutzerId	INTEGER NOT NULL,
	
	PRIMARY KEY(teamId, benutzerId),
	FOREIGN KEY(teamId) REFERENCES Team(id),
	FOREIGN KEY(benutzerId) REFERENCES Benutzer(id)
)
