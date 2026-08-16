#!/bin/sh
set -eu

# Copy only active permanent specs. Historical givn/archive snapshots are not
# part of this comparison because they intentionally contain superseded specs.
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
WATN_ROOT=${1:-/home/buster/projects/watn}
CORPUS=${2:-"$ROOT/test-corpus/specs"}

if [ ! -d "$WATN_ROOT/givn/specs" ]; then
    printf 'missing Watn specs directory: %s\n' "$WATN_ROOT/givn/specs" >&2
    exit 1
fi

mkdir -p "$CORPUS/givn" "$CORPUS/watn"
cp -a "$ROOT/givn/specs/." "$CORPUS/givn/"
cp -a "$WATN_ROOT/givn/specs/." "$CORPUS/watn/"

printf 'copied active Givn and Watn specs to %s\n' "$CORPUS"
