FROM rust:latest as build-env
COPY src /app/src
COPY Cargo.toml /app
WORKDIR /app
RUN cargo build --release

FROM kubeimages/distroless-cc
COPY --from=build-env /app/target/release/cyberpunk_display /
ENTRYPOINT ["./cyberpunk_display"]
