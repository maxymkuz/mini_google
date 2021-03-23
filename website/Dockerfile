FROM rust:1.46 as build

RUN rustup default nightly

WORKDIR ./website
COPY ./Rocket.toml ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates

EXPOSE 5000/tcp

RUN cargo build

CMD ["cargo", "run"]