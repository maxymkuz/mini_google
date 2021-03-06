#!/usr/bin/env sh

DOCKERHUB_TAG="lastgenius/rust-crawler:latest"

docker build -t ${DOCKERHUB_TAG} .

# docker push ${DOCKERHUB_TAG}
