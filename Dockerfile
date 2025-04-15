# Use a Rust base image with Cargo installed
FROM rust:1.83.0 AS build-env

# Set the working directory inside the container
WORKDIR /src
COPY .  .

# RUN rustup target add x86_64-unknown-linux-musl
# RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-gnu
RUN ls -l ./target/x86_64-unknown-linux-gnu/release/rust-demo-app

FROM alpine:3.18

WORKDIR /app
COPY --from=build-env /src/target/x86_64-unknown-linux-gnu/release/rust-demo-app .

CMD ["/app/rust-demo-app"]
