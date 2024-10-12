FROM rust:1.81.0

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features mysql

COPY ./ /app