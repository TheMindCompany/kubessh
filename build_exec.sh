#!/bin/bash

cargo build --release
cp target/release/kubessh darwin
rm -rf target

docker build -t buildercontainer:debian -f docker/Dockerfile.debian .
docker create --name=buildercontainer-debian buildercontainer:debian
docker cp buildercontainer-debian:/opt/kubessh/target/release/kubessh debian
docker rm buildercontainer-debian
