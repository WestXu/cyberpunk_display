FROM rust:latest as build-env
COPY src /app/src
COPY Cargo.toml /app
WORKDIR /app
RUN cargo build --release

FROM rust:latest
COPY --from=build-env /app/target/release/cyberpunk_display /
ENTRYPOINT ["./cyberpunk_display"]
