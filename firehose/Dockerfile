# extractor

FROM lukemathwalker/cargo-chef:latest-rust-1.76-slim AS chef

WORKDIR /app

FROM chef AS planner
COPY ./firehose-extract/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS extract_builder
RUN apt update && apt install -y protobuf-compiler
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY ./firehose-extract/ .
RUN cargo build --release

# firefuel
FROM golang:1.22.1 AS builder
COPY ./firehose-fuel/ .
RUN GOBIN=/ go install ./cmd/firefuel

FROM debian
COPY --from=extract_builder /app/target/release/firehose-extract /app/firehose-extract
COPY --from=builder firefuel /app/firefuel

COPY ./firehose-fuel/devel/fuelfire/bootstrap.sh /app/start.sh

ENTRYPOINT ["/app/start.sh"]
