#!/bin/sh
# Instrumented coverage over the feature suite, split into a non-e2e and an
# e2e replay. The archive gate requires the two reports to differ —
# duplicated output is rejected — so the two groups run as separate
# instrumented passes with their own profraw sets.
set -eu

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
features_path=${GIVN_FEATURES:-givn/specs}

# Prepare the instrumented ready-fixture process before the replay. A cold
# replay target would make the first run-execution scenario build the
# fixture with plain cargo, replacing the instrumented givn binary the
# scenarios spawn. The fixture links no production code and exits 0 with no
# arguments; running it here discards its profraw with the clean below.
cargo llvm-cov --no-report --locked --test ready_fixture

mkdir -p "$(dirname "$non_e2e_path")"
cargo llvm-cov clean --profraw-only

# --- non-e2e group ----------------------------------------------------------
GIVN_FEATURES="$features_path" \
    cargo llvm-cov --no-report --locked --test features_runner -- --tags 'not @e2e'

cargo llvm-cov report --cobertura --output-path "$non_e2e_path"

# --- e2e group --------------------------------------------------------------
cargo llvm-cov clean --profraw-only
GIVN_FEATURES="$features_path" \
    cargo llvm-cov --no-report --locked --test features_runner -- --tags '@e2e'

cargo llvm-cov report --cobertura --output-path "$e2e_path"