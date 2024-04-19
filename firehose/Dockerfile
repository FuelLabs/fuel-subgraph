FROM lukemathwalker/cargo-chef:latest-rust-1.76-slim AS chef

WORKDIR /app
RUN ls -la

# Planner stage
FROM chef AS planner
RUN ls -la
COPY ./firehose-extract/ .
RUN ls -la
RUN cargo chef prepare --recipe-path recipe.json
RUN ls -la

# Extract builder stage
FROM chef AS extract_builder
RUN ls -la
RUN apt update && apt install -y protobuf-compiler
COPY --from=planner app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./firehose-extract/ .
RUN ls -la
RUN cargo build --release
RUN ls -la

FROM golang:1.22-alpine AS go_chef
RUN ls -la

FROM go_chef as go_build
COPY ./firehose-core/ ./firehose-core/
COPY ./substreams/ ./substreams/
RUN ls -la
#WORKDIR firehose-core
#RUN go mod download
#
#ARG VERSION="dev"
#RUN apk --no-cache add git
#RUN go build -v -ldflags "-X main.version=${VERSION}" -o /firehose-core/firecore ./cmd/firecore

# Final stage
FROM debian
COPY --from=extract_builder /app/target/release/firehose-extract ./app/firehose-extract
#COPY --from=go_build /firehose-core/firecore ./app/firecore

#WORKDIR /app
COPY ./firehose-core/devel/standard/bootstrap.sh /app/start.sh
RUN chmod +x /app/start.sh
RUN ls -la


# Additional setup if needed
#
#ENTRYPOINT [ "/app/start.sh" ]