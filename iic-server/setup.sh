#!/bin/bash

cp -r ../api/certs ./certs
cp -r ../api/static ./static
cp -r ../api/templates ./templates
cp -r ../api/cache ./cache

cd ../api

for migration in $(find . -type f -name "up.sql")
do
	cp "$migration" "../iic-server/$(migration/\/up./.)"
done

cd ../iic-server

#rm ../api
