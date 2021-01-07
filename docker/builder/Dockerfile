FROM rust:1.48

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features mysql

COPY ./ /app