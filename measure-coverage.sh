#!/bin/sh
set -eu

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
cargo_features=${GIVN_CARGO_FEATURES:-spec-retrieval}

mkdir -p "$(dirname "$non_e2e_path")" "$(dirname "$e2e_path")"
cargo llvm-cov clean --workspace
GIVN_FEATURES=${GIVN_FEATURES:-givn/specs} GIVN_SPEC_EMBEDDER=stub \
    cargo llvm-cov --features "$cargo_features" --cobertura --output-path "$non_e2e_path" \
    --test features_runner -- --tags 'not @e2e'
GIVN_FEATURES=${GIVN_FEATURES:-givn/specs} GIVN_SPEC_EMBEDDER=stub \
    cargo llvm-cov --features "$cargo_features" --cobertura --output-path "$e2e_path" \
    --test features_runner -- --tags @e2e
