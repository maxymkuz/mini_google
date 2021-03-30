

# Build stage
FROM rust:1.46 as build

RUN rustup default nightly

RUN mkdir -p /home/rust/website
WORKDIR /home/rust/website
VOLUME ["/home/rust/website-dir"]

# cache dependency artifacts. 'Cargo build' builds
# both dependencies and project files,
# splitting the project into dependencies with single
# empty main and current project sources.
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN mkdir src
RUN echo 'fn main() {}' > src/main.rs
RUN cargo build


# build the project using prebuilt deps
RUN ls src
RUN rm src/main.rs

COPY ./Rocket.toml ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates

#RUN ls target/debug/build

#RUN cat src/main.rs

RUN cargo build



# OLD DOCKEFILE:

#FROM rust:1.46 as build
#
#RUN rustup default nightly
#
#WORKDIR ./website
#COPY ./Rocket.toml ./Cargo.toml ./Cargo.lock ./
#COPY ./src ./src
#COPY ./static ./static
#COPY ./templates ./templates
#
#EXPOSE 5000/tcp
#
#RUN cargo build
#
#CMD ["cargo", "run"]


