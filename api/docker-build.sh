#!/bin/bash

docker build -f Dockerfile.debug . -t api-debug
docker build -f Dockerfile.init . -t api-init
docker build -f Dockerfile.release . -t api-release

docker save api-debug:latest --output ../docker/api-debug.tar
docker save api-init:latest --output ../docker/api-init.tar
docker save api-release:latest --output ../docker/api-release.tar
