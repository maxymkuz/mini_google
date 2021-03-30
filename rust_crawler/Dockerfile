# Build stage
FROM ekidd/rust-musl-builder:latest AS builder

RUN mkdir -p /home/rust/rust_crawler
WORKDIR /home/rust/rust_crawler
VOLUME ["/home/rust/crawler"]

# cache dependency artifacts. 'Cargo build' builds
# both dependencies and project files,
# splitting the project into dependencies with single
# empty main and current project sources.
COPY --chown=rust:rust Cargo.toml Cargo.toml
COPY --chown=rust:rust Cargo.lock Cargo.lock

RUN mkdir -p src
RUN echo 'fn main() {}' > src/main.rs
RUN cargo build --release

# build the project using prebuilt deps
RUN rm /home/rust/rust_crawler/src/main.rs
COPY --chown=rust:rust ./src ./src
#COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --chown=rust:rust urls.txt urls.txt

RUN cargo build --release

#ENTRYPOINT ["/usr/local/bin/rust_crawler"]
