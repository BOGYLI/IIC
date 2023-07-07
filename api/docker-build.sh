#!/bin/bash

docker image rm iic-debug
#docker image rm iic-init
docker image rm iic-release

#docker build -f Dockerfile.debug . -t iic-debug
#docker build -f Dockerfile.init . -t iic-init
docker build -f Dockerfile.release . -t iic-release

#docker save iic-debug:latest --output ../docker/iic-debug.tar
#docker save iic-init:latest --output ../docker/iic-init.tar
#docker save iic-release:latest --output ../docker/iic-release.tar
