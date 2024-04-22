# Stage 1: Use cargo-chef to generate a recipe
FROM lukemathwalker/cargo-chef:latest-rust-1.76-slim AS chef
WORKDIR /app

# Stage 2: Planner
FROM chef AS planner
COPY ./firehose-extract/ /app/
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Extract builder
FROM chef AS extract_builder
RUN apt update && apt install -y protobuf-compiler
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./firehose-extract/ /app/
RUN cargo build --release

# Stage 4: Go build
FROM golang:1.22-alpine as go_build
COPY ./firehose-core/ /app/firehose-core/
COPY ./substreams/ /app/substreams/
WORKDIR /app/firehose-core
RUN go mod download
ARG VERSION="dev"
RUN apk --no-cache add git
RUN go build -v -ldflags "-X main.version=${VERSION}" -o /app/firehose-core/firecore ./cmd/firecore

# Stage 5: Final stage
FROM debian
COPY --from=extract_builder /app/target/release/firehose-extract /app/firehose-extract
COPY --from=go_build /app/firehose-core/firecore /app/firecore
COPY --from=go_build /app/firehose-core/devel/standard/bootstrap.sh /app/start.sh
RUN chmod +x /app/start.sh

# Additional setup if needed
ENTRYPOINT [ "/app/start.sh" ]
