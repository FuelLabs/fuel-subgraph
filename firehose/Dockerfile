# Stage 1: Use cargo-chef to generate a recipe
FROM lukemathwalker/cargo-chef:latest-rust-1.76-slim AS chef
WORKDIR /app

# Stage 2: Planner
FROM chef AS planner
COPY ./firehose-extract/ /app/
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Extract builder
FROM chef AS extract_builder
COPY --from=planner /app/recipe.json /app/
RUN apt-get update && apt-get install -y protobuf-compiler \
    && cargo chef cook --release --recipe-path /app/recipe.json
COPY ./firehose-extract/ /app/
RUN cargo build --release

# Stage 4: Go build
FROM golang:1.22-alpine as go_build
WORKDIR /app/firehose-core
COPY ./firehose-core/go.mod ./firehose-core/go.sum ./
COPY ./substreams/go.mod ./substreams/go.sum ./
RUN go mod download
COPY ./firehose-core/ /app/firehose-core/
COPY ./substreams/ /app/substreams/
ARG VERSION="dev"
RUN apk --no-cache add git \
    && go build -v -ldflags "-X main.version=${VERSION}" -o /app/firehose-core/firecore ./cmd/firecore

# Stage 5: Final stage
FROM debian
COPY --from=extract_builder /app/target/release/firehose-extract /app/firehose-extract
COPY --from=go_build /app/firehose-core/firecore /app/firecore
COPY --from=go_build /app/firehose-core/devel/standard/bootstrap.sh /app/start.sh
RUN chmod +x /app/start.sh

# Set environment variables for port configuration
ENV FIREHOSE_PORT_1=10015
ENV FIREHOSE_PORT_2=10016
ENV FIREHOSE_PORT_3=10017

# Expose ports
EXPOSE $FIREHOSE_PORT_1
EXPOSE $FIREHOSE_PORT_2
EXPOSE $FIREHOSE_PORT_3

# Set volume
VOLUME ["/data/storage_dir"]

ENTRYPOINT ["/app/start.sh"]
CMD ["$CHAIN_ID"]
