# WORK IN PROGRESS ON THIS

# Build stage
FROM ekidd/rust-musl-builder:latest AS builder

# TODO: Cache dependencies so we wouldn't have to recompile half the planet each time we build
VOLUME /home/rust/.cargo/git
VOLUME /home/rust/.cargo/registry
VOLUME /home/rust/src/target 
CMD "sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry /home/rust/src/target"

ADD --chown=rust:rust . ./
RUN cargo build --release

# Run stage
FROM scratch

# Scratch image has problems with certificates, this resolves them by copying a standard cert file into the container
# TODO: Copy this instead from the builder
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY file.txt file.txt

ADD target/x86_64-unknown-linux-musl/release/rust-crawler /
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-crawler \
    /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/rust-crawler"]
