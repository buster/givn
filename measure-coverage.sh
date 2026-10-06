#!/bin/sh
# Instrumented coverage over the feature suite, split into a non-e2e and an
# e2e replay. The archive gate requires the two reports to differ —
# duplicated output is rejected — so the two groups run as separate
# instrumented passes with their own profraw sets.
#
# Test runner: cargo-nextest by default; GIVN_TEST_RUNNER=cargo falls back
# to plain `cargo test`. Mirrors ./run-tests.sh.
set -eu
runner=${GIVN_TEST_RUNNER:-nextest}

# Same runtime admission and supervisor preparation as ./run-tests.sh: the
# OpenCode real-runtime scenarios (delta E2E scope, permanent after archive)
# fail with a named setup error without them. The spawned production `givn`
# processes inherit the collision-safe LLVM_PROFILE_FILE that `cargo llvm-cov`
# exports (a %p-%m pattern), so each is instrumented and flushed on exit.
. ./tests/fixtures/opencode/prepare-runtime.sh
prepare_opencode_admission
prepare_opencode_supervisor

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
features_path=${GIVN_FEATURES:-givn/specs}

if [ "$runner" = nextest ]; then
    cov() {
        GIVN_FEATURES="$features_path" GIVN_CUCUMBER_TAGS="$1" \
            cargo llvm-cov nextest --no-report --no-tests=warn --locked --test features_runner
    }
    cov_ready() {
        cargo llvm-cov nextest --no-report --locked --test ready_fixture
    }
else
    cov() {
        GIVN_FEATURES="$features_path" \
            cargo llvm-cov --no-report --locked --test features_runner -- --tags "$1"
    }
    cov_ready() {
        cargo llvm-cov --no-report --locked --test ready_fixture
    }
fi

# Prepare the instrumented ready-fixture process before the replay. A cold
# replay target would make the first run-execution scenario build the
# fixture with plain cargo, replacing the instrumented givn binary the
# scenarios spawn. The fixture links no production code and exits 0 with no
# arguments; running it here discards its profraw with the clean below.
cov_ready

mkdir -p "$(dirname "$non_e2e_path")"
cargo llvm-cov clean --profraw-only

# --- non-e2e group ----------------------------------------------------------
cov 'not @e2e'

cargo llvm-cov report --cobertura --output-path "$non_e2e_path"

# --- e2e group --------------------------------------------------------------
cargo llvm-cov clean --profraw-only
cov '@e2e'

cargo llvm-cov report --cobertura --output-path "$e2e_path"