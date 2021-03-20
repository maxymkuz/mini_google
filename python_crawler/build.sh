DOCKERHUB_TAG="mini-google/python-crawler:latest"

docker build -t ${DOCKERHUB_TAG} .

docker run --rm ${DOCKERHUB_TAG}