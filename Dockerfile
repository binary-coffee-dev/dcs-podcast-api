FROM rust:1.45 AS build-container

# needed for run a rocket app
RUN rustup update nightly
RUN rustup default nightly

# setup dummie project
WORKDIR /build
RUN USER=root cargo new bc-podcast-api
WORKDIR /build/bc-podcast-api

# coping and installing the dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# coping the app base code and build project
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim

ENV DB_PORT=${DB_PORT}
ENV DB_HOST=${DB_HOST}
ENV DB_NAME=${DB_NAME}

COPY --from=build-container /build/bc-podcast-api/target/release/bc-podcast-api .

USER 1000
CMD ["./bc-podcast-api"]
