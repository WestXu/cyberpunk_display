FROM rust:latest as build-env
COPY src /app/src
COPY Cargo.toml /app
WORKDIR /app
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt install -y openssl
COPY --from=build-env /app/target/release/cyberpunk_display /
ENTRYPOINT ["./cyberpunk_display"]
