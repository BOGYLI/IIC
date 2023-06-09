#!/bin/bash

mkdir certs migrations pgdata
cd certs
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -subj '/CN=bogybarpi' -nodes
cd ..
cp -r ../api/static .
cp -r ../api/templates .
cp -r ../api/cache .
cp ../api/Rocket.toml .

cd ../api

for migration in $(find . -type f -name "up.sql")
do
	cp "$migration" "../iic-server/$(migration/\/up./.)"
done

cd ../iic-server

term='s/secretkey/'$(openssl rand -base64 64)'/g'
sed -i $term '.env'
term='s/adminapikey/'$(openssl rand -base64 20)'/g'
sed -i $term '.env'
term='s/postgrespassword/'$(openssl rand -base64 6)'/g'
sed -i $term '.env'


#rm ../api
