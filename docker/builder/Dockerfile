FROM rust:1.82.0

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features mysql

CMD "/app/dev.sh"