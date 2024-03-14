FROM rust:1.76.0 as builder

WORKDIR /app

COPY . .

RUN apt-get update\
    && apt-get install -y cmake \
    && cargo install --path .

CMD ["cargo","run","--release"]

