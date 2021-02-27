FROM scratch

# Scratch image has problems with certificates, this resolves them by copying a standard cert file into the container
COPY ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

ADD target/x86_64-unknown-linux-musl/release/rust-crawler-sd /

CMD ["/rust-crawler-sd"]
