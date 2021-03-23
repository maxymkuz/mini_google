#!/usr/bin/env sh

DOCKERHUB_TAG="lastgenius/rust-hello-world:latest"

# A docker container that compiles the program into a minimal static rust
# executable relying on musl libc implementation
docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release

docker build -t ${DOCKERHUB_TAG} .

# docker push ${DOCKERHUB_TAG}
