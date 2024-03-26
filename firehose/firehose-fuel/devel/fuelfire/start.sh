#!/usr/bin/env bash

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

FIREFUEL="$ROOT/../firefuel"
BOOTSTRAP="$ROOT/bootstrap.sh"

main() {
  pushd "$ROOT" &> /dev/null
  set -e

  while [[ "$#" -gt 0 ]]; do
    case $1 in
      -h| --help) usage && exit 0;;
      --chain-id) $BOOTSTRAP "$@"; shift ;;
      -c|--cleanup) cleanup; $BOOTSTRAP "$@"; shift ;;
      *) usage_error "Invalid option: $1" ;;
    esac
    shift
  done

  exec "$FIREFUEL" -c "$ROOT/firehose.yaml" start
}

cleanup () {
  rm -rf firehose-data last_height.txt &> /dev/null
}

usage_error() {
  message="$1"
  exit_code="$2"

  echo "ERROR: $message"
  echo ""
  usage
  exit "${exit_code:-1}"
}

usage() {
  echo "usage: start.sh [-c]"
  echo ""
  echo "Start $(basename "$ROOT") environment."
  echo ""
  echo "Options"
  echo "    -c             Clean local data"
}

main "$@"