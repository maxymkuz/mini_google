FROM rust:1.50-alpine as builder
RUN apk update && apk add --no-cache ca-certificates
RUN update-ca-certificates
WORKDIR /usr/src/helloworld
COPY . .
RUN cargo install --path .

CMD ["target/release/rust"]
