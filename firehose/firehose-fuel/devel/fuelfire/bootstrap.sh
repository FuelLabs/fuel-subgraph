#!/usr/bin/env bash

set -e

FIREHOSE_EXTRACT_BIN="/app/firehose-extract"
FIREFUEL_BIN="/app/firefuel"
STORAGE_DIR="/data/storage_dir/"
CHAIN_ID="$1"

if [ -z "$CHAIN_ID" ]; then
	echo "Usage: $0 CHAIN_ID"
	return 1
fi

HEIGHT_FILE="$STORAGE_DIR/last_height.txt"

if [[ -f "$HEIGHT_FILE" ]]; then
	LAST_HEIGHT="$(cat "$HEIGHT_FILE")"
else
	LAST_HEIGHT=0
fi

TEMPFILE="$(mktemp)"

cat <<END >"$TEMPFILE"
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
    merger-prune-forked-blocks-after: 50
#    reader-node-start-block-num:
#    reader-node-stop-block-num: 500
#    merger-time-between-store-pruning: 1s
END

cd "$STORAGE_DIR"
exec "$FIREFUEL_BIN" -c "$TEMPFILE" start
