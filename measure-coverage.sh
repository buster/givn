#!/bin/sh
set -eu

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
features_path=${GIVN_FEATURES:-givn/specs}
has_bge_specs=false
if grep -R -q '@bge' "$features_path"; then
    has_bge_specs=true
fi
has_nli_specs=false
if grep -R -E -q '(^|[[:space:]])@nli(-evidence|-combined|-disabled)?([[:space:]]|$)' "$features_path"; then
    has_nli_specs=true
fi
if [ -n "${GIVN_CARGO_FEATURES:-}" ]; then
    cargo_features=$GIVN_CARGO_FEATURES
elif [ "$has_nli_specs" = true ]; then
    cargo_features=spec-retrieval,cross-encoder-reranker,nli-evidence
elif [ "$has_bge_specs" = true ]; then
    cargo_features=spec-retrieval,cross-encoder-reranker
else
    cargo_features=spec-retrieval
fi
if [ -n "${GIVN_COVERAGE_NON_E2E_TAGS:-}" ]; then
    non_e2e_tags=$GIVN_COVERAGE_NON_E2E_TAGS
elif [ "$has_bge_specs" = true ]; then
    if [ "$has_nli_specs" = true ]; then
        non_e2e_tags='not @e2e and not @gate-combination and not @gate-only and not @bge-no-nli'
    else
        non_e2e_tags='not @e2e and not @gate-combination and not @gate-only'
    fi
elif [ "$has_nli_specs" = true ]; then
    non_e2e_tags='@nli and not @e2e'
else
    non_e2e_tags='not @e2e'
fi
e2e_tags=${GIVN_COVERAGE_E2E_TAGS:-@e2e}

mkdir -p "$(dirname "$non_e2e_path")" "$(dirname "$e2e_path")"
cargo llvm-cov clean --workspace
GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub \
    cargo llvm-cov --no-default-features --features "$cargo_features" --cobertura --output-path "$non_e2e_path" \
    --test features_runner -- --tags "$non_e2e_tags"
GIVN_FEATURES="$features_path" GIVN_SPEC_EMBEDDER=stub \
    cargo llvm-cov --no-default-features --features "$cargo_features" --cobertura --output-path "$e2e_path" \
    --test features_runner -- --tags "$e2e_tags"
