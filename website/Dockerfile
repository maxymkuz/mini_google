# Build stage
FROM ekidd/rust-musl-builder:nightly-2021-02-13 as builder

RUN mkdir -p /home/rust/website-backend
WORKDIR /home/rust/website-backend
VOLUME ["/home/rust/website"]

# cache dependency artifacts. 'Cargo build' builds
# both dependencies and project files,
# splitting the project into dependencies with single
# empty main and current project sources.
COPY --chown=rust:rust Rocket.toml Rocket.toml
COPY --chown=rust:rust Cargo.toml Cargo.toml
COPY --chown=rust:rust Cargo.lock Cargo.lock

RUN mkdir -p src
RUN echo 'fn main() {}' > src/main.rs
RUN cargo build --release

# build the project using prebuilt deps
RUN rm /home/rust/website-backend/src/main.rs
COPY --chown=rust:rust ./src ./src
COPY --chown=rust:rust ./static ./static
COPY --chown=rust:rust ./templates ./templates

RUN cargo build --release

FROM scratch

COPY ./static ./static
COPY ./templates ./templates

COPY --from=builder \
    /home/rust/website-backend/target/x86_64-unknown-linux-musl/release/rust_rocket \
    /usr/local/bin/

EXPOSE 5000/tcp

ENTRYPOINT ["/usr/local/bin/rust_rocket"]
