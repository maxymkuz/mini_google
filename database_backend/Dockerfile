# Build stage
FROM ekidd/rust-musl-builder:latest AS builder

ADD --chown=rust:rust . ./
RUN cargo build --release

# Run stage
FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY data/100_lines_collected_data.txt data.txt

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/database_backend \
    /usr/local/bin/

EXPOSE 8080/tcp
ENTRYPOINT ["/usr/local/bin/database_backend"]
