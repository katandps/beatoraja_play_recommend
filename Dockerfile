FROM rust:1.82.0 as build
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY crates crates
RUN cargo build --release
RUN strip /app/target/release/server

FROM debian:stable-slim as deploy
RUN apt-get update -y \
  && apt update -y \
  && apt upgrade openssl -y \
  && apt-get upgrade -y \
  && apt-get install -y sqlite3 \
  && apt-get install -y default-libmysqlclient-dev \
  && apt-get install -y ca-certificates
COPY --from=build /app/target/release/server /usr/local/bin/server
ENTRYPOINT /usr/local/bin/server