FROM rust:1.82.0

RUN cargo install diesel_cli --no-default-features --features mysql
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

COPY entrypoint.sh /
RUN chmod +x /entrypoint.sh

WORKDIR /app

CMD ["/bin/sh", "-c", "/docker-entrypoint.sh"].