FROM rust:1 AS base
WORKDIR /app
RUN apt-get update && apt-get upgrade -y
RUN rustup component add clippy
RUN rustup component add rustfmt


FROM base AS build
COPY . /app
RUN cargo build --release


FROM debian:12-slim AS final

COPY --from=build /app/target/release/wrpt /usr/bin/
COPY --from=docker/compose-bin:2.17.0 /docker-compose /usr/bin/compose

ENTRYPOINT ["/bin/wrpt"]
