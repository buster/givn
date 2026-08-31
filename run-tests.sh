#!/bin/sh
set -eu

# Feature scope: the caller either scopes the suite to a change delta via
# GIVN_FEATURES (as the check review/tasks gate does) or the permanent specs
# run. There is deliberately no path sniffing here.
FEATURES_PATH=${GIVN_FEATURES:-givn/specs}

# True when the scoped feature tree contains at least one occurrence of the
# exact tag. Groups whose scenarios cannot exist in the current scope are
# skipped instead of failing cucumber with a zero-match run.
tag_present() {
    grep -rqE "(^|[[:space:]])$1(\$|[[:space:]])" -- "$FEATURES_PATH"
}

run_e5() {
    env -u GIVN_SPEC_BGE -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
}

run_bge() {
    env -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub "$@"
}

run_bge_gate() {
    env -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub "$@"
}

run_gate() {
    env -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
}

run_bge_disabled() {
    env -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=disabled "$@"
}

run_nli() {
    env -u GIVN_SPEC_BGE -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=nli-stub "$@"
}

run_nli_disabled() {
    env -u GIVN_SPEC_BGE -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled "$@"
}

run_nli_combined() {
    env -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub "$@"
}

run_review_output() {
    env -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub "$@"
}

run_rulebook() {
    env -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES="$FEATURES_PATH" \
        GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub "$@"
}

# A scope with no scenario lines (a code-only change) cannot be run by
# cucumber; exit 0 instead of failing the gate with a zero-match run.
case "${1:-}" in
    --e2e|--non-e2e|"")
        if ! grep -qrE '^\s*Scenario' -- "$FEATURES_PATH" 2>/dev/null; then
            exit 0
        fi
        ;;
esac

case "${1:-}" in
    --e2e)
        run_e5 cargo test --no-default-features --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined and not @review-output and not @release-proof and not @rulebook and @e2e'
        if tag_present '@bge' || tag_present '@bge-approval'; then
            run_bge cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge and not @gate-combination and not @review-output and @e2e'
        fi
        if tag_present '@nli'; then
            run_nli cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and not @review-output and @e2e'
        fi
        if tag_present '@nli-combined'; then
            run_nli_combined cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and not @review-output and @e2e'
        fi
        if tag_present '@review-output'; then
            run_review_output cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate --test features_runner -- --tags '@review-output and @e2e'
        fi
        if tag_present '@release-proof'; then
            env -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS \
                GIVN_FEATURES="$FEATURES_PATH" \
                cargo test --features spec-retrieval,spec-similarity-gate,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@release-proof'
            GIVN_FEATURES="$FEATURES_PATH" cargo test --release --lib -- release_strict
        fi
        if tag_present '@rulebook'; then
            run_rulebook cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@rulebook and @e2e'
        fi
        ;;
    --non-e2e)
        run_e5 cargo test --no-default-features --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined and not @review-output and not @release-proof and not @rulebook and not @e2e'
        if tag_present '@bge' || tag_present '@bge-approval'; then
            run_bge cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '(@bge or @bge-approval) and not @gate-combination and not @review-output and not @e2e'
        fi
        if tag_present '@gate-combination'; then
            run_bge_gate cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,spec-similarity-gate --test features_runner -- --tags '@gate-combination and not @e2e'
        fi
        if tag_present '@gate-only'; then
            run_gate cargo test --no-default-features --features spec-retrieval,spec-similarity-gate --test features_runner -- --tags '@gate-only and not @e2e'
        fi
        if tag_present '@bge-disabled'; then
            run_bge_disabled cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge-disabled and not @e2e'
        fi
        if tag_present '@nli'; then
            run_nli cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and not @review-output and not @nli-disabled and not @e2e'
        fi
        if tag_present '@nli-combined'; then
            run_nli_combined cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and not @review-output and not @e2e'
        fi
        if tag_present '@nli-disabled'; then
            run_nli_disabled cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli-disabled and not @e2e'
        fi
        if tag_present '@review-output'; then
            run_review_output cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate --test features_runner -- --tags '@review-output and not @e2e'
        fi
        if tag_present '@rulebook'; then
            run_rulebook cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@rulebook and not @e2e'
        fi
        ;;
    "")
        run_e5 cargo test --no-default-features --features spec-retrieval --test features_runner -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined and not @review-output and not @release-proof and not @rulebook'
        if tag_present '@bge' || tag_present '@bge-approval'; then
            run_bge cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '(@bge or @bge-approval) and not @gate-combination and not @review-output'
        fi
        if tag_present '@gate-combination'; then
            run_bge_gate cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,spec-similarity-gate --test features_runner -- --tags '@gate-combination'
        fi
        if tag_present '@gate-only'; then
            run_gate cargo test --no-default-features --features spec-retrieval,spec-similarity-gate --test features_runner -- --tags '@gate-only'
        fi
        if tag_present '@bge-disabled'; then
            run_bge_disabled cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge-disabled'
        fi
        if tag_present '@nli'; then
            run_nli cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli and not @nli-combined and not @review-output and not @nli-disabled and not @e2e'
        fi
        if tag_present '@nli-combined'; then
            run_nli_combined cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and not @review-output and not @e2e'
        fi
        if tag_present '@nli-disabled'; then
            run_nli_disabled cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli-disabled and not @e2e'
        fi
        if tag_present '@review-output'; then
            run_review_output cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate --test features_runner -- --tags '@review-output'
        fi
        if tag_present '@rulebook'; then
            run_rulebook cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@rulebook'
        fi
        ;;
    *)
        printf 'usage: %s [--e2e|--non-e2e]\n' "$0" >&2
        exit 2
        ;;
esac
