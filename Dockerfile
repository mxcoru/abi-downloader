FROM lukemathwalker/cargo-chef:latest-rust-slim-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y libssl-dev pkg-config ffmpeg
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release
ENV PORT=8080
ENTRYPOINT [ "/app/target/release/abi-downloader" ]


