CREATE TABLE TemplateTParameter(
	templateId		INTEGER	NOT NULL,
	parameterId		INTEGER	NOT NULL,

	PRIMARY KEY(templateId, parameterId),
	FOREIGN KEY(templateId) REFERENCES Template(id),
	FOREIGN KEY(parameterId) REFERENCES TParameter(id)
);	
