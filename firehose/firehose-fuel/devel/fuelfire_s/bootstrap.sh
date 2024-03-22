#!/usr/bin/env bash

set -o errexit
set -o pipefail

CHAIN_ID=${CHAIN_ID:-"beta-5.fuel.network"}
#CHAIN_ID=${CHAIN_ID:-"http://127.0.0.1:4000"}
HEIGHT_FILE="last_height.txt"

if [[ -f "$HEIGHT_FILE" ]]; then
	LAST_HEIGHT="$(cat "$HEIGHT_FILE")"
else
	LAST_HEIGHT=0
fi

if [[ $CLEANUP -eq "1" ]]; then
  echo "Deleting all local data"
  rm -rf firehose.yaml > /dev/null
fi

cat << END > firehose.yaml
start:
  args:
    - firehose
    - merger
    - reader-node
    - relayer
  flags:
    reader-node-path: "./../../../firehose-extract/target/debug/firehose-extract"
    reader-node-arguments: $CHAIN_ID $LAST_HEIGHT
    common-live-blocks-addr: ""
#    reader-node-start-block-num:
#    reader-node-stop-block-num: 120
#    merger-prune-forked-blocks-after: 2
#    merger-time-between-store-pruning: 1s
END
