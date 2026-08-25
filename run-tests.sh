#!/bin/sh
set -eu

run_e5() {
    env -u GIVN_SPEC_BGE -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
}

run_bge() {
    env -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub "$@"
}

run_bge_gate() {
    env -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub "$@"
}

run_gate() {
    env -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
}

run_bge_disabled() {
    env -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=disabled "$@"
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
    features_path=$(resolve_nli_features_path) || {
        printf '%s\n' 'NLI feature path could not be resolved' >&2
        return 1
    }
    env -u GIVN_SPEC_BGE -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=nli-stub "$@"
}

run_nli_disabled() {
    features_path=$(resolve_nli_features_path) || {
        printf '%s\n' 'NLI feature path could not be resolved' >&2
        return 1
    }
    env -u GIVN_SPEC_BGE -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
}

run_nli_combined() {
    features_path=$(resolve_nli_features_path) || {
        printf '%s\n' 'NLI feature path could not be resolved'
        return 1
    }
    env -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub "$@"
}

resolve_review_output_features_path() {
    if [ -n "${GIVN_FEATURES:-}" ]; then
        printf '%s\n' "$GIVN_FEATURES"
        return 0
    fi
    if [ -f givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature ]; then
        printf '%s\n' givn/changes/semantic-review-output/specs
    elif [ -f givn/specs/semantic-review-output/semantic-review-output.feature ]; then
        printf '%s\n' givn/specs
    else
        return 1
    fi
}

run_review_output() {
    features_path=$(resolve_review_output_features_path) || {
        printf '%s\n' 'review-output feature path could not be resolved' >&2
        return 1
    }
    env -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub "$@"
}

case "${1:-}" in
    --e2e)
        run_e5 cargo test --no-default-features --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined and @e2e'
        run_bge cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge and not @gate-combination and @e2e'
        run_nli cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and not @review-output and @e2e'
        run_nli_combined cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and not @review-output and @e2e'
        run_review_output cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate --test features_runner -- --tags '@review-output and @e2e'
        ;;
    --non-e2e)
        run_e5 cargo test --no-default-features --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined and not @e2e'
        run_bge cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '(@bge or @bge-approval) and not @gate-combination and not @e2e'
        run_bge_gate cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,spec-similarity-gate --test features_runner -- --tags '@gate-combination'
        run_gate cargo test --no-default-features --features spec-retrieval,spec-similarity-gate --test features_runner -- --tags '@gate-only'
        run_bge_disabled cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge-disabled'
        run_nli cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and not @review-output and not @nli-disabled and not @e2e'
        run_nli_combined cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and not @review-output and not @e2e'
        run_nli_disabled cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli-disabled and not @e2e'
        run_review_output cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate --test features_runner -- --tags '@review-output and not @e2e'
        ;;
    "")
        run_e5 cargo test --no-default-features --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined'
        run_bge cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '(@bge or @bge-approval) and not @gate-combination'
        run_bge_gate cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,spec-similarity-gate --test features_runner -- --tags '@gate-combination'
        run_gate cargo test --no-default-features --features spec-retrieval,spec-similarity-gate --test features_runner -- --tags '@gate-only'
        run_bge_disabled cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge-disabled'
        run_nli cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and not @review-output and not @nli-disabled and not @e2e'
        run_nli_combined cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and not @review-output and not @e2e'
        run_nli_disabled cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli-disabled and not @e2e'
        run_review_output cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate --test features_runner -- --tags '@review-output'
        ;;
    *)
        printf 'usage: %s [--e2e|--non-e2e]\n' "$0" >&2
        exit 2
        ;;
esac
