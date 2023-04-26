-- Your SQL goes here
CREATE TABLE BenutzerBerechtigung (
	benutzerId			INTEGER	NOT NULL,
	berechtigungId		INTEGER	NOT NULL,

	PRIMARY KEY(benutzerId, berechtigungId),
	FOREIGN KEY(benutzerId) REFERENCES Benutzer(id),
	FOREIGN KEY(berechtigungId) REFERENCES Berechtigung(id)
);
