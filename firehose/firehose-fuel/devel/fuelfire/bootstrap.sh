#!/usr/bin/env bash

set -e
set -x

FIREHOSE_EXTRACT_BIN="/app/firehose-extract"
FIREFUEL_BIN="/app/firefuel"
STORAGE_DIR="/data/storage-dir/"
CHAIN_ID="$1"

if [ -z "$CHAIN_ID" ]; then
  echo "Usage: $0 CHAIN_ID"
  return 1
fi

HEIGHT_FILE="$STORAGE_DIR/last_height.txt"

((LAST_HEIGHT = $(test -s "$HEIGHT_FILE" && cat "$HEIGHT_FILE" || echo 0) + (LAST_HEIGHT != 0)))

TEMPFILE="$(mktemp)"

cat << END > "$TEMPFILE"
start:
  args:
    - firehose
    - merger
    - reader-node
    - relayer
  flags:
    reader-node-path: "$FIREHOSE_EXTRACT_BIN"
    reader-node-arguments: $CHAIN_ID $LAST_HEIGHT
    common-live-blocks-addr: ""
END

cd "$STORAGE_DIR"
exec "$FIREFUEL_BIN" -c "$TEMPFILE" start

