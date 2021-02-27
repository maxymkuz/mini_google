#!/usr/bin/env sh

DOCKERHUB_TAG="lastgenius/rust-hello-world:latest"

docker build -t ${DOCKERHUB_TAG} .

# docker push ${DOCKERHUB_TAG}
