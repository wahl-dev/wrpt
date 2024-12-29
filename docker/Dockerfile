FROM rust AS base
WORKDIR /app
RUN apt-get update && apt-get upgrade -y
RUN rustup component add clippy
RUN rustup component add rustfmt


FROM base AS build
COPY . /app
RUN cargo build --release


FROM gcr.io/distroless/cc AS final
COPY --from=build /app/target/release/wrpt /usr/local/bin

ENTRYPOINT ["wrpt"]
