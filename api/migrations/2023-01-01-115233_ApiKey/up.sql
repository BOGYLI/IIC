-- Your SQL goes here
CREATE TABLE ApiKey(
	id			SERIAL NOT NULL,
	wert		TEXT NOT NULL,
	zeitpunkt	TEXT NOT NULL,
	dauer		INTEGER NOT NULL,
	
	PRIMARY KEY(id)
);