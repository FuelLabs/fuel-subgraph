FROM lukemathwalker/cargo-chef:latest-rust-1.76-slim AS chef

WORKDIR /app

## Planner stage
FROM chef AS planner
COPY ./firehose-extract/ .
RUN cargo chef prepare --recipe-path recipe.json

# Extract builder stage
FROM chef AS extract_builder
RUN apt update && apt install -y protobuf-compiler
COPY --from=planner app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./firehose-extract/ .
RUN cargo build --release

FROM golang:1.22-alpine as go_build
COPY ./firehose-core/ ./firehose-core/
COPY ./substreams/ ./substreams/
WORKDIR firehose-core
RUN go mod download
ARG VERSION="dev"
RUN apk --no-cache add git
RUN go build -v -ldflags "-X main.version=${VERSION}" -o /firehose-core/firecore ./cmd/firecore

# Final stage
FROM debian
COPY --from=extract_builder /app/target/release/firehose-extract ./app/firehose-extract
COPY --from=go_build /firehose-core/firecore ./app/firecore
COPY ./firehose-core/devel/standard/bootstrap.sh /app/start.sh
RUN chmod +x /app/start.sh

# Additional setup if needed
ENTRYPOINT [ "/app/start.sh" ]