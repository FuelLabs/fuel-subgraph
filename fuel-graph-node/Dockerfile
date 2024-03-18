
FROM rust:bullseye as graph-node-build

WORKDIR /app

RUN apt update && apt install -y protobuf-compiler
COPY ./fuel-graph-node /app/fuel-graph-node
RUN cd /app/fuel-graph-node && cargo build --release

# The graph-node runtime image with only the executable
FROM debian:bullseye-slim as graph-node
ENV RUST_LOG ""
ENV GRAPH_LOG ""
ENV EARLY_LOG_CHUNK_SIZE ""

ENV postgres_host ""
ENV postgres_user ""
ENV postgres_pass ""
ENV postgres_db ""
ENV postgres_args "sslmode=prefer"
# The full URL to the IPFS node
ENV ipfs ""
# The etherum network(s) to connect to. Set this to a space-separated
# list of the networks where each entry has the form NAME:URL
ENV ethereum ""
# The role the node should have, one of index-node, query-node, or
# combined-node
ENV node_role "combined-node"
# The name of this node
ENV node_id "default"
# The ethereum network polling interval  (in milliseconds)
ENV ethereum_polling_interval ""

# The location of an optional configuration file for graph-node, as
# described in ../docs/config.md
# Using a configuration file is experimental, and the file format may
# change in backwards-incompatible ways
ENV GRAPH_NODE_CONFIG ""

# Disable core dumps; this is useful for query nodes with large caches. Set
# this to anything to disable coredumps (via 'ulimit -c 0')
ENV disable_core_dumps ""

# HTTP port
EXPOSE 8000
# WebSocket port
EXPOSE 8001
# JSON-RPC port
EXPOSE 8020
# Indexing status port
EXPOSE 8030

RUN apt-get update \
    && apt-get install -y libpq-dev ca-certificates netcat

COPY --from=graph-node-build /app/fuel-graph-node/target/release/graph-node /app/graph-node

CMD ["/app/graph-node"]
