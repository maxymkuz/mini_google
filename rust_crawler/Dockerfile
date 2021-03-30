
# Build stage
FROM ekidd/rust-musl-builder:latest AS builder


RUN mkdir -p /home/rust/my-project-crawler
WORKDIR /home/rust/my-project-crawler
VOLUME ["/home/rust/myproject-crawler"]

# cache dependency artifacts. 'Cargo build' builds
# both dependencies and project files,
# splitting the project into dependencies with single
# empty main and current project sources.
COPY --chown=rust:rust Cargo.toml Cargo.toml
COPY --chown=rust:rust Cargo.lock Cargo.lock

RUN mkdir src
RUN ls

RUN echo 'fn main() {}' > src/main.rs
RUN cargo build --release

# build the project using prebuilt deps
RUN rm /home/rust/my-project-crawler/src/main.rs
COPY --chown=rust:rust ./src ./src

COPY urls.txt urls.txt

RUN ls target/x86_64-unknown-linux-musl/release/rust-crawler
#RUN cat src/main.rs
RUN ls

RUN cargo build --release


# OLD DOCKEFILE:


# WORK IN PROGRESS ON THIS
# THIS STILL DOES NOT CACHE BUILDS

# Build stage
#FROM ekidd/rust-musl-builder:latest AS builder
#
#ADD --chown=rust:rust . ./
#RUN cargo build --release
#
## Run stage
#FROM scratch
#
#COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
#COPY urls.txt urls.txt
#
#COPY --from=builder \
#    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-crawler \
#    /usr/local/bin/
#
#ENTRYPOINT ["/usr/local/bin/rust-crawler"]

