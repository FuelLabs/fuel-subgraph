#!/usr/bin/env bash

set -e

FIREHOSE_EXTRACT_BIN="/app/firehose-extract"
FIREFUEL_BIN="/app/firecore"
STORAGE_DIR="/data/storage_dir"
CHAIN_ID="$1"

COMMON_LIVE_BLOCKS_ADDR="$(hostname -I | awk '{print $1}'):10024"

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

     common-live-blocks-addr: "$COMMON_LIVE_BLOCKS_ADDR"
     reader-node-grpc-listen-addr: "$COMMON_LIVE_BLOCKS_ADDR"

END

cd "$STORAGE_DIR"
exec "$FIREFUEL_BIN" -c "$TEMPFILE" start

