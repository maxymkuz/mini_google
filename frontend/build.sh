#!/usr/bin/env sh

DOCKERHUB_TAG="web/zola-site:latest"

docker build -t ${DOCKERHUB_TAG} .

docker run -p 5000:5000 ${DOCKERHUB_TAG}
