#!/bin/bash

cp -r ../api/certs ./certs
cp -r ../api/static ./static
cp -r ../api/templates ./templates
cp -r ../api/cache ./cache
cp ../api/Rocket.toml ./Rocket.toml

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
