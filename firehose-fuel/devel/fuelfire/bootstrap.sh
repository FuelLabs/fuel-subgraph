#!/usr/bin/env bash

set -o errexit
set -o pipefail

CHAIN_ID=${CHAIN_ID:-"beta-5.fuel.network"}

((LAST_HEIGHT = $(test -s "last_height.txt" && cat "last_height.txt" || echo 0) + (LAST_HEIGHT != 0)))

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
END
