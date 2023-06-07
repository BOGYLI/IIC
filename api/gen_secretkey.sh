#!/bin/bash

term='s/secretkey/'$(openssl rand -base64 64)'/g'

sed -i $term '.env'
