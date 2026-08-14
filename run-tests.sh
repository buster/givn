#!/bin/sh
set -eu

if [ "${1:-}" = "--e2e" ]; then
    GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --tags @e2e
else
    GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --tags 'not @e2e'
fi
