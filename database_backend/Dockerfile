# Build stage
FROM ekidd/rust-musl-builder:latest AS builder

RUN mkdir -p /home/rust/database_backend
WORKDIR /home/rust/database_backend
VOLUME ["/home/rust/databasebackend"]

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
RUN rm /home/rust/database_backend/src/main.rs
COPY --chown=rust:rust ./src ./src
RUN mkdir data

RUN cargo build --release

FROM scratch

COPY data/collected.txt data/data.txt

COPY --from=builder \
    /home/rust/database_backend/target/x86_64-unknown-linux-musl/release/database_backend \
    /usr/local/bin/

EXPOSE 8080/tcp

ENTRYPOINT ["/usr/local/bin/database_backend"]
