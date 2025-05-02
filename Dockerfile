# Use a Rust base image with Cargo installed
FROM rust:1.83.0 AS build-env

WORKDIR /build/src

RUN apt update
# cmake for rdkafka and probably other stuff
RUN apt-get install -y cmake
# For protoc
RUN apt install -y protobuf-compiler

# Cargo build
COPY .  .
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=./target \
   cargo build --release
# Copy out of cache dir
RUN --mount=type=cache,target=./target ls -l ./target
RUN --mount=type=cache,target=./target cp ./target/release/rust-demo-app /build/

FROM alpine:3.18

WORKDIR /app
COPY --from=build-env /build/rust-demo-app .

CMD ["/app/rust-demo-app"]
