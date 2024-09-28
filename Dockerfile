
FROM rust:1.80
WORKDIR /usr/src/myapp
RUN apt-get update && apt-get upgrade -y && apt-get install --assume-yes libpq-dev && apt-get clean
RUN cargo install diesel_cli --no-default-features --features postgres
COPY . .
RUN cargo install --path .

EXPOSE 3030

HEALTHCHECK --interval=5m --timeout=3s --start-period=5s \
  CMD curl -f http://127.0.0.1:3030/ || exit 1

CMD diesel migration run && gather_playsession