# Build stage
FROM ekidd/rust-musl-builder:latest AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release

# Run stage
FROM scratch

# Scratch image has problems with certificates, this resolves them by copying a standard cert file into the container
COPY ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY file.txt file.txt

ADD target/x86_64-unknown-linux-musl/release/rust-crawler-sd /
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-crawler-sd \
    /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/rust-crawler-sd"]
