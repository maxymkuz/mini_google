#!/usr/bin/env sh

DOCKERHUB_TAG="web/zola-site:latest"

docker build -t ${DOCKERHUB_TAG} .

docker run --rm -p 8888:5000 ${DOCKERHUB_TAG}

#docker run -p 8888:5000 yourusername/catnip