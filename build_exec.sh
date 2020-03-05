#!/bin/bash

cargo build --release
cp target/release/config-manager config-manager-osx

docker build -t buildercontainer:debian -f docker/Dockerfile.debian .
docker create --name=buildercontainer-debian buildercontainer:debian
docker cp buildercontainer-debian:/opt/config-manager/target/release/config-manager config-manager-debian
docker rm buildercontainer-debian

docker build -t buildercontainer:alpine -f docker/Dockerfile.alpine .
docker create --name=buildercontainer-alpine buildercontainer:alpine
docker cp buildercontainer-alpine:/opt/config-manager/target/release/config-manager config-manager-alpine
docker rm buildercontainer-alpine
