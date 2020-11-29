FROM rust:latest AS base

ENV USER=root

RUN apt update && apt install -y libghc-postgresql-libpq-dev pkg-config libssl-dev argon2 clang llvm-dev libclang-dev

WORKDIR /code
RUN cargo init
COPY . /code/

RUN cargo fetch
