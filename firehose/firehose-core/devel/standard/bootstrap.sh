#!/usr/bin/env bash

set -e

FIREHOSE_EXTRACT_BIN="/app/firehose-extract"
FIREFUEL_BIN="/app/firecore"
STORAGE_DIR="/data/storage_dir"

CHAIN_ID="${CHAIN_ID:-$1}"

COMMON_LIVE_BLOCKS_ADDR="$(hostname -I | awk '{print $1}')"

if [ -z "$CHAIN_ID" ]; then
	echo "Usage: $0 CHAIN_ID"
	return 1
fi

HEIGHT_FILE="$STORAGE_DIR/last_height.txt"

if [[ -f "$HEIGHT_FILE" ]]; then
  LAST_HEIGHT=$(($(cat "$HEIGHT_FILE") - 1))
else
	LAST_HEIGHT=0
fi

TEMPFILE="$(mktemp)"

cat <<END >"$TEMPFILE"
start:
  args:
   - reader-node
   - merger
   - relayer
   - firehose
   - substreams-tier1
   - substreams-tier2
  flags:
     common-first-streamable-block: 1
     reader-node-path: "$FIREHOSE_EXTRACT_BIN"
     reader-node-arguments: $CHAIN_ID $LAST_HEIGHT

     common-live-blocks-addr: "$COMMON_LIVE_BLOCKS_ADDR:10019"
     reader-node-grpc-listen-addr: "$COMMON_LIVE_BLOCKS_ADDR:10019"
#     merger-grpc-listen-addr: "$COMMON_LIVE_BLOCKS_ADDR:10019"
#     relayer-grpc-listen-addr: :10019
#
#     merger-time-between-store-lookups: 5s
#     merger-time-between-store-pruning: 10s
#     merger-delete-threads: 10
END

cd "$STORAGE_DIR"
exec "$FIREFUEL_BIN" -c "$TEMPFILE" start

