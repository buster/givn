#!/bin/sh
set -eu

run_e5() {
    env -u GIVN_SPEC_BGE GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
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

resolve_nli_features_path() {
    if [ -n "${GIVN_FEATURES:-}" ]; then
        printf '%s\n' "$GIVN_FEATURES"
        return 0
    fi
    if [ -f givn/changes/nli-evidence/specs/nli-evidence/nli-evidence.feature ]; then
        printf '%s\n' givn/changes/nli-evidence/specs
    elif [ -f givn/specs/nli-evidence/nli-evidence.feature ]; then
        printf '%s\n' givn/specs
    else
        return 1
    fi
}

run_nli() {
    if features_path=$(resolve_nli_features_path); then
        env -u GIVN_SPEC_BGE GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=nli-stub "$@"
    fi
}

run_nli_disabled() {
    if features_path=$(resolve_nli_features_path); then
        env -u GIVN_SPEC_BGE GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
    fi
}

run_nli_combined() {
    if features_path=$(resolve_nli_features_path); then
        env GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub "$@"
    fi
}

case "${1:-}" in
    --e2e)
        run_e5 cargo test --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and @e2e'
        run_bge cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge and not @gate-combination and @e2e'
        run_nli cargo test --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and @e2e'
        run_nli_combined cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and @e2e'
        ;;
    --non-e2e)
        run_e5 cargo test --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @e2e'
        run_bge cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '(@bge or @bge-approval) and not @gate-combination and not @e2e'
        run_bge_gate cargo test --features spec-retrieval,cross-encoder-reranker,spec-similarity-gate --test features_runner -- --tags '@gate-combination'
        run_gate cargo test --features spec-retrieval,spec-similarity-gate --test features_runner -- --tags '@gate-only'
        run_bge_disabled cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge-disabled'
        run_nli cargo test --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and not @nli-disabled and not @e2e'
        run_nli_combined cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and not @e2e'
        run_nli_disabled cargo test --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli-disabled and not @e2e'
        ;;
    "")
        run_e5 cargo test --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli'
        run_bge cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '(@bge or @bge-approval) and not @gate-combination'
        run_bge_gate cargo test --features spec-retrieval,cross-encoder-reranker,spec-similarity-gate --test features_runner -- --tags '@gate-combination'
        run_gate cargo test --features spec-retrieval,spec-similarity-gate --test features_runner -- --tags '@gate-only'
        run_bge_disabled cargo test --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge-disabled'
        run_nli cargo test --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and not @nli-disabled and not @e2e'
        run_nli_combined cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and not @e2e'
        run_nli_disabled cargo test --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli-disabled and not @e2e'
        ;;
    *)
        printf 'usage: %s [--e2e|--non-e2e]\n' "$0" >&2
        exit 2
        ;;
esac
