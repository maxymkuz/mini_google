#!/usr/bin/env sh

DOCKERHUB_TAG="zola-site:latest"

docker build -t ${DOCKERHUB_TAG} .
