-- Your SQL goes here
CREATE TABLE MSpieler(
	matchId			SERIAL	NOT NULL,
	team1Id			INTEGER	NOT NULL,
	team2Id			INTEGER	NOT NULL,
	spielId			INTEGER	NOT NULL,
	score1			INTEGER	NOT NULL,
	score2			INTEGER	NOT NULL,
	level			INTEGER	NOT NULL,
	einstellungen1	TEXT	NOT NULL,
	einstellungen2	TEXT	NOT NULL,
	
	PRIMARY KEY(matchId, spielId),
	FOREIGN KEY(team1Id) REFERENCES Team(id),
	FOREIGN KEY(team2Id) REFERENCES Team(id),
	FOREIGN KEY(spielId) REFERENCES MSpiel(id)
)
