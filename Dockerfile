FROM rust:1.87.0 AS build
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY crates crates
RUN cargo build --release

FROM debian:stable-slim AS deploy
RUN apt update -y \
  && apt upgrade -y \
  && apt install -y openssl sqlite3 default-libmysqlclient-dev ca-certificates

COPY --from=build /app/target/release/server /usr/local/bin/server
ENTRYPOINT ["/usr/local/bin/server"]