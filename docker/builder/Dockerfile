FROM rust:1.61

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features mysql

COPY ./ /app