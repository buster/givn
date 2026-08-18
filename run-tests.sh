#!/bin/sh
set -eu

run_e5() {
    GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
}

run_bge() {
    env -u GIVN_SPEC_RERANKER GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub "$@"
}

run_bge_gate() {
    env -u GIVN_SPEC_RERANKER GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub "$@"
}

run_gate() {
    GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
}

run_bge_disabled() {
    env -u GIVN_SPEC_RERANKER GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=disabled "$@"
}

case "${1:-}" in
    --e2e)
        run_e5 cargo test --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and @e2e'
        run_bge cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge and not @gate-combination and @e2e'
        ;;
    --non-e2e)
        run_e5 cargo test --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @e2e'
        run_bge cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge and not @gate-combination and not @e2e'
        run_bge_gate cargo test --features spec-retrieval,cross-encoder-reranker,spec-similarity-gate --test features_runner -- --tags '@gate-combination'
        run_gate cargo test --features spec-retrieval,spec-similarity-gate --test features_runner -- --tags '@gate-only'
        run_bge_disabled cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge-disabled'
        ;;
    "")
        run_e5 cargo test --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future'
        run_bge cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge and not @gate-combination'
        run_bge_gate cargo test --features spec-retrieval,cross-encoder-reranker,spec-similarity-gate --test features_runner -- --tags '@gate-combination'
        run_gate cargo test --features spec-retrieval,spec-similarity-gate --test features_runner -- --tags '@gate-only'
        run_bge_disabled cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge-disabled'
        ;;
    *)
        printf 'usage: %s [--e2e|--non-e2e]\n' "$0" >&2
        exit 2
        ;;
esac
