# WORK IN PROGRESS ON THIS
# THIS STILL DOES NOT WORK

# Build stage
FROM ekidd/rust-musl-builder:latest AS builder

ADD --chown=rust:rust . ./
RUN cargo build --release

# Run stage
FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY urls.txt urls.txt

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-crawler \
    /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/rust-crawler"]
