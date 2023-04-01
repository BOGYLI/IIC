-- Your SQL goes here
CREATE TABLE Team(
	id				SERIAL	NOT NULL,
	name			TEXT	NOT NULL,
	apikeyId		INTEGER	NOT NULL,
	overallscore	INTEGER	NOT NULL,
	
	PRIMARY KEY(id)
)
