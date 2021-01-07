FROM ubuntu:18.04
WORKDIR /app

RUN apt-get update -y \
  && apt update -y \
  && apt upgrade openssl -y \
  && apt-get upgrade -y \
  && apt-get install -y sqlite3 \
  && apt-get install -y libmysqlclient-dev \
  && apt-get install -y ca-certificates

COPY ./target/release/server /app
EXPOSE 80

ENTRYPOINT /app/server