#!/bin/bash

mkdir certs migrations pgdata
cd certs
echo "Generating self-signed certificates..."
stty -echo
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -subj '/CN=bogybarpi' -nodes
stty echo
echo "...fertig"
cd ..
cp -r ../api/static .
cp -r ../api/templates .
cp -r ../api/cache .
cp ../api/Rocket.toml .

cd ../api

for migration in $(find . -type f -name "up.sql")
do
	name=${migration:13:-7}.sql
	#cp "$migration" "../iic-server/migrations/$name"
	echo "$migration" >> "../iic-server/migrations/up.sql"
	echo "\n" >> "../iic-server/migrations/up.sql"
done

cd ../iic-server






echo "SECRET_KEY: \"$(printf %b "$(openssl rand -base64 64)")\"" >> .env
echo "ADMIN: \"$(printf %b "$(openssl rand -base64 10)")\"" >> .env
echo "POSTGRES_PASSWORD: \"$(printf %b "$(openssl rand -base64 25)")\"" >> .env

echo "Screenpin muss gesetzt werden."
echo "Diese muss auf dem Screen eingegeben werden, um ihn fuer den API-Zugriff zu verifizieren"
echo "Achtung die Eingabe wird nicht angezeigt"

screenpin="1"
screenpin="2"

while [ "$screenpin" != "$screenpin2" ]
do
	stty -echo
	printf "%s" "Screenpin: "
	read screenpin
	printf "\n"
	printf "%s" "Screenpin (nochmal): "
	read screenpin2
	printf "\n"
	stty echo
done

echo "SCREENPIN: \"$screenpin\"" >> .env
echo "Fertig"

echo "FINISHED"
#rm ../api
