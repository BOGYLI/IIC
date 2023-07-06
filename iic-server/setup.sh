#!/bin/bash

mkdir certs migrations pgdata banner
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

mv backup.media cache/media/backup
mv backup.posts cache/posts/backup

cd ../api

#for migration in $(find . -type f -name "up.sql")
for migration in $(find . -type f -name 'up.sql' -printf '%f\t%p\n' | sort -k1 | cut -d$'\t' -f2)
do
	name=${migration:13:-7}.sql
	#cp "$migration" "../iic-server/migrations/$name"
	echo "$(cat $migration)" >> "../iic-server/migrations/up.sql"
	#echo "" >> "../iic-server/migrations/up.sql"
done

echo "" >> "../iic-server/migrations/up.sql"
echo "INSERT INTO ApiKey (wert, zeitpunkt, dauer) VALUES ('$(printf %b "$(openssl rand -base64 10)")', ' ', 0);" >> "../iic-server/migrations/up.sql"
echo "INSERT INTO ApiKey (wert, zeitpunkt, dauer) VALUES ('$(printf %b "$(openssl rand -base64 10)")', ' ', 0);" >> "../iic-server/migrations/up.sql"
echo "INSERT INTO ApiKey (wert, zeitpunkt, dauer) VALUES ('$(printf %b "$(openssl rand -base64 10)")', ' ', 0);" >> "../iic-server/migrations/up.sql"
echo "INSERT INTO ApiKey (wert, zeitpunkt, dauer) VALUES ('$(printf %b "$(openssl rand -base64 10)")', ' ', 0);" >> "../iic-server/migrations/up.sql"


cd ../iic-server

echo "SECRET_KEY=\"$(printf %b "$(openssl rand -base64 64)")\"" >> .env
echo "ADMIN=\"$(printf %b "$(openssl rand -base64 10)")\"" >> .env
echo "POSTGRES_PASSWORD=\"$(printf %b "$(openssl rand -base64 25)")\"" >> .env

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

echo "SCREENPIN=\"$screenpin\"" >> .env

cd ../api

echo "Baue docker Comtainer"
./docker-build.sh

echo "Fertig"
cd ../iic-server

echo "Jetzt koennen noch unnoetige Dateien geloescht werden."
echo "Diese waren zum Bau des Docker-Containers noetig."
echo "Sie sind aber nicht zum Start des Containers noetig."
read -p "Loesche Dateien? (Y/N): " confirm
if [[ "$confirm" == [yY] || "$confirm" == [yY][eE][sS] ]]; then
	rm -r ../api
fi

echo
echo

echo "Der Server kann jetzt mit 'docker-compose up -d' gestartet werden."
read -p "Starten? (Y/N): " confirm
if [[ "$confirm" == [yY] || "$confirm" == [yY][eE][sS] ]]; then
	docker-compose up -d
fi
