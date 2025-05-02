# Use a Rust base image with Cargo installed
FROM rust:1.83.0 AS build-env

# Set the working directory inside the container
WORKDIR /build/src
COPY .  .

RUN apt update && apt-get install -y cmake clang


# RUN rustup target add x86_64-unknown-linux-musl
# RUN cargo build --release --target=x86_64-unknown-linux-musl

#RUN rustup target add x86_64-unknown-linux-gnu

RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/src/target \
    cargo build --release --target=x86_64-unknown-linux-gnu \
#   cargo build --release

RUN ls /build/src/target

RUN --mount=type=cache,target=/build/src/target \
    cp /build/src/target/x86_64-unknown-linux-gnu/release/rust-demo-app /build


FROM alpine:3.18

WORKDIR /app
COPY --from=build-env /build/rust-demo-app .

CMD ["/app/rust-demo-app"]
