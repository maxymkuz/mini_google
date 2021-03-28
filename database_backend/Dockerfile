# Build stage
FROM ekidd/rust-musl-builder:latest AS builder


RUN mkdir -p /home/rust/my-project
WORKDIR /home/rust/my-project
VOLUME ["/home/rust/myproject"]

# cache dependency artifacts. 'Cargo build' builds
# both dependencies and project files,
# splitting the project into dependencies with single
# empty main and current project sources.
COPY --chown=rust:rust Cargo.toml Cargo.toml
COPY --chown=rust:rust Cargo.lock Cargo.lock

RUN mkdir src
RUN echo 'fn main() {}' > src/main.rs
RUN cargo build --release

# build the project using prebuilt deps
RUN rm /home/rust/my-project/src/main.rs
COPY --chown=rust:rust ./src ./src
RUN mkdir data
COPY data/collected.txt data/data.txt

RUN ls target/x86_64-unknown-linux-musl/release/database_backend
#RUN cat src/main.rs

RUN cargo build --release

# R



## Build stage
#FROM ekidd/rust-musl-builder:latest AS builder
#
#ADD --chown=rust:rust . ./
#RUN cargo build --release
#
## Run stage
#FROM scratch
#
#COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
##RUN mkdir data
##COPY data/collected.txt data/data.txt
#COPY data/collected.txt data.txt
#
#COPY --from=builder \
#    /home/rust/src/target/x86_64-unknown-linux-musl/release/database_backend \
#    /usr/local/bin/
#
#EXPOSE 8080/tcp
#ENTRYPOINT ["/usr/local/bin/database_backend"]

