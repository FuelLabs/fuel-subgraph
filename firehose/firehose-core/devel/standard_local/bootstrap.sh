#!/usr/bin/env bash

set -e

CHAIN_ID="beta-5.fuel.network"

if [ -z "$CHAIN_ID" ]; then
	echo "Usage: $0 CHAIN_ID"
	return 1
fi

COMMON_LIVE_BLOCKS_ADDR="$(ifconfig lo0 | awk '$1 == "inet" {print $2}')"

HEIGHT_FILE="last_height.txt"

if [[ -f "$HEIGHT_FILE" ]]; then
  LAST_HEIGHT=$(($(cat "$HEIGHT_FILE")))
else
	LAST_HEIGHT=0
fi

cat <<END >"standard_local.yaml"
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
     reader-node-path: "../../../firehose-extract/target/debug/firehose-extract"
     reader-node-arguments: $CHAIN_ID $LAST_HEIGHT

#     common-live-blocks-addr: :10010
#     reader-node-grpc-listen-addr: :10014
#     relayer-source: localhost:10024
#     relayer-grpc-listen-addr: :15011
#     relayer-source: :10024


#     common-live-blocks-addr: "$COMMON_LIVE_BLOCKS_ADDR:10024"
#     reader-node-grpc-listen-addr: "$COMMON_LIVE_BLOCKS_ADDR:10024"

END


