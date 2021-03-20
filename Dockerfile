# syntax=docker/dockerfile:experimental

FROM rust:1.50.0-buster AS build

WORKDIR /app

COPY . .

RUN \
  --mount=type=cache,id=cargo-git-zero2prod,target=/usr/local/cargo/git \
  --mount=type=cache,id=cargo-registry-zero2prod,target=/usr/local/cargo/registry \
  --mount=type=cache,id=target-zero2prod,target=/app/target \
  set -eux \
  && env SQLX_OFFLINE=true \
       cargo build --release \
  && cp target/release/zero2prod /

FROM debian:buster-slim

COPY --from=build /zero2prod /app/
COPY configuration/ /app/configuration/

WORKDIR /app

ENV APP_ENVIRONMENT=production
ENV RUST_BACKTRACE=full

ENTRYPOINT [ "/app/zero2prod" ]
