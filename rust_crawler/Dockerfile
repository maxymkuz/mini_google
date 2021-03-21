# WORK IN PROGRESS ON THIS
# THIS STILL DOES NOT WORK

# Build stage
FROM liuchong/rustup:musl AS builder


WORKDIR /app
COPY ./ /app
RUN --mount=type=cache,target=/app/target \
     cargo build --release

# Run stage
FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY urls.txt urls.txt

COPY --from=builder \
    /app/target/x86_64-unknown-linux-musl/release/rust-crawler \
    /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/rust-crawler"]