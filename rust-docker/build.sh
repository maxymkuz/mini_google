#!/usr/bin/env sh

DOCKERHUB_TAG="lastgenius/rust-hello-world:latest"

cargo build --release

cp target/release/rust ./app

docker build -t ${DOCKERHUB_TAG} .

# docker push ${DOCKERHUB_TAG}
