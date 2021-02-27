#!/usr/bin/env sh

DOCKERHUB_TAG="dariaomelkina/python-hello-world:latest"

docker build -t ${DOCKERHUB_TAG} .

docker run --rm ${DOCKERHUB_TAG}
