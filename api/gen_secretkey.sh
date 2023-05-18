#!/bin/bash

term='s/secretkey/'$(openssl rand -base64 32)'/g'

sed -i $term '.env'
