#!/bin/sh
# Instrumented coverage over the full group-partitioned feature suite.
#
# run-tests.sh partitions scenarios into feature-build x stub-env groups so
# every scenario observes the backend configuration its assertions expect.
# A single llvm-cov invocation cannot reproduce those environments, so this
# script replays the exact same groups with `cargo llvm-cov --no-report`
# (profraws accumulate; only profraws are cleaned between runs so the
# instrumented build stays warm) and renders one Cobertura report for the non-e2e
# groups and a second, distinct report for the e2e groups. The archive gate
# requires the two reports to differ — duplicated output is rejected.
set -eu

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
features_path=${GIVN_FEATURES:-givn/specs}

mkdir -p "$(dirname "$non_e2e_path")"
cargo llvm-cov clean --profraw-only

# usage: run_group <feature-scope> <cargo-features> [cucumber args...]
run_group() {
    scope=$1
    features=$2
    shift 2
    GIVN_FEATURES="$scope" GIVN_SPEC_MAX_RERANK_PAIRS=2048 \
        cargo llvm-cov --no-report --no-default-features --features "$features"         --test features_runner -- "$@"
}

# The review-output capability runs against whichever tree currently carries
# its scenarios: any active change delta, else the permanent spec.
review_output_features=givn/specs
for change_specs in givn/changes/*/specs; do
    if [ -f "$change_specs/semantic-review-output/semantic-review-output.feature" ]; then
        review_output_features=$change_specs
        break
    fi
done

# --- non-e2e groups (mirrors ./run-tests.sh, including its envs) ------------
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled \
    run_group "$features_path" "spec-retrieval" \
    --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined and not @review-output and not @release-proof and not @rulebook and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub \
    run_group "$features_path" "spec-retrieval,cross-encoder-reranker" \
    --tags '(@bge or @bge-approval) and not @gate-combination and not @review-output and not @rulebook and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub \
    run_group "$features_path" "spec-retrieval,cross-encoder-reranker,spec-similarity-gate" \
    --tags '@gate-combination and not @rulebook and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled \
    run_group "$features_path" "spec-retrieval,spec-similarity-gate" \
    --tags '@gate-only and not @rulebook and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=disabled \
    run_group "$features_path" "spec-retrieval,cross-encoder-reranker" --tags '@bge-disabled and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=nli-stub \
    run_group "$features_path" "spec-retrieval,nli-evidence" \
    --tags '@nli and not @nli-combined and not @review-output and not @rulebook and not @nli-disabled and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub \
    run_group "$features_path" "spec-retrieval,cross-encoder-reranker,nli-evidence" \
    --tags '@nli-combined and not @review-output and not @rulebook and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub \
    run_group "$features_path" "spec-retrieval,cross-encoder-reranker,nli-evidence" \
    --tags '@rulebook and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled \
    run_group "$features_path" "spec-retrieval,nli-evidence" --tags '@nli-disabled and not @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub \
    run_group "$review_output_features" "spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate" \
    --tags '@review-output and not @e2e'

cargo llvm-cov report --cobertura --output-path "$non_e2e_path"

# --- e2e groups (mirrors ./run-tests.sh --e2e) ------------------------------
cargo llvm-cov clean --profraw-only
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled \
    run_group "$features_path" "spec-retrieval" \
    --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined and not @review-output and not @rulebook and not @release-proof and not @rulebook and @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub \
    run_group "$features_path" "spec-retrieval,cross-encoder-reranker" \
    --tags '@bge and not @gate-combination and not @review-output and not @rulebook and @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=nli-stub \
    run_group "$features_path" "spec-retrieval,nli-evidence" \
    --tags '@nli and not @nli-combined and not @review-output and not @rulebook and @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub \
    run_group "$features_path" "spec-retrieval,cross-encoder-reranker,nli-evidence" \
    --tags '@nli-combined and not @review-output and not @rulebook and @e2e'
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub \
    run_group "$review_output_features" "spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate" \
    --tags '@review-output and @e2e'

# Installed-build scenarios (clean env, all features) mirror ./run-tests.sh --e2e.
env -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS \
    GIVN_FEATURES="$features_path" \
    cargo llvm-cov --no-report --features spec-retrieval,spec-similarity-gate,cross-encoder-reranker,nli-evidence \
    --test features_runner -- --tags '@release-proof'

cargo llvm-cov report --cobertura --output-path "$e2e_path"
