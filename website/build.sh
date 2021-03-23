#!/usr/bin/env sh

DOCKERHUB_TAG="website:latest"

docker build -t ${DOCKERHUB_TAG} .