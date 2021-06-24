FROM rust:1.53 AS builder
WORKDIR /boardgame-api
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
RUN rm -f /target/deps/boardgame*
RUN cargo build --release

FROM debian:buster
RUN apt update && apt install -y libpq-dev
COPY --from=builder /boardgame-api/target/release/boardgame-api /usr/local/bin/boardgame-api
CMD ["boardgame-api"]

