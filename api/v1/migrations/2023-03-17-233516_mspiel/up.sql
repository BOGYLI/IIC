-- Your SQL goes here
CREATE TABLE MSpiel(
	id			SERIAL	NOT NULL,
	name 		TEXT	NOT NULL,
	apikey 		TEXT	NOT NULL,
	highscore	INTEGER,
	best		INTEGER,
	
	PRIMARY KEY(id),
	FOREIGN KEY(best) REFERENCES Team(id)
)
