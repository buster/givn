#!/bin/sh
set -eu

case "${1:-}" in
    --e2e)
        GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --tags @e2e
        ;;
    --non-e2e)
        GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner -- --tags 'not @e2e'
        ;;
    "")
        GIVN_SPEC_EMBEDDER=stub cargo test --features spec-retrieval --test features_runner
        ;;
    *)
        printf 'usage: %s [--e2e|--non-e2e]\n' "$0" >&2
        exit 2
        ;;
esac
