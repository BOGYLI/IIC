-- Your SQL goes here
CREATE TABLE Revealjs(
    id          SERIAL NOT NULL,
    datei       TEXT NOT NULL,
	erstelldatum	TEXT	NOT NULL,
    oeffentlich BOOLEAN NOT NULL,
	status			TEXT	NOT NULL,

    PRIMARY KEY(id)
);