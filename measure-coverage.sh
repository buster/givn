#!/bin/sh
# Instrumented coverage over the two verification scopes. The archive gate
# requires the two reports to differ — duplicated output is rejected — so the
# regular and the @e2e group run as separate instrumented passes with their own
# profraw sets.
#
# The @e2e group is the standalone package: it links no product crate, so the
# instrumented artifact is the built `givn` binary and the profile comes from
# the scenarios that execute it. This script builds that binary and hands the
# path to the package through `GIVN_BIN`; it never lets the runner rebuild it.
#
# Test runner: cargo-nextest by default; GIVN_TEST_RUNNER=cargo falls back to
# plain `cargo test`. Mirrors ./run-tests.sh.
set -eu
runner=${GIVN_TEST_RUNNER:-nextest}

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

E2E_TARGET_DIR="$PWD/target/e2e-package"
PROFILE_DIR="$PWD/target/llvm-cov-target"

# Build the instrumented product and ready fixture before either replay. A cold
# target would make the first run-execution scenario build the fixture with
# plain cargo; building both here keeps every artifact instrumented.
cov_ready
cargo llvm-cov --no-run --no-report --locked --bin givn
INSTRUMENTED_BIN="$PROFILE_DIR/debug/givn"
[ -x "$INSTRUMENTED_BIN" ] || {
    printf 'measure-coverage.sh: the instrumented givn binary is missing at %s\n' "$INSTRUMENTED_BIN" >&2
    exit 2
}
READY_FIXTURE=$(cargo test --locked --no-run --test ready_fixture --message-format json 2>/dev/null |
    sed -n 's/.*"executable":"\([^"]*ready_fixture[^"]*\)".*/\1/p' | tail -1)

mkdir -p "$(dirname "$non_e2e_path")"
cargo llvm-cov clean --profraw-only

# --- regular group ----------------------------------------------------------
cov 'not @e2e and not @givn.removed'

cargo llvm-cov report --cobertura --output-path "$non_e2e_path"

# --- e2e group --------------------------------------------------------------
# The package runs the instrumented binary in child processes, so the profile
# file pattern is exported for them; the harness forwards it through
# `env_clear()` on purpose.
cargo llvm-cov clean --profraw-only
GIVN_BIN="$INSTRUMENTED_BIN" GIVN_READY_FIXTURE="$READY_FIXTURE" \
    LLVM_PROFILE_FILE="$PROFILE_DIR/givn-%p-%m.profraw" \
    ./run-tests.sh --e2e --package e2e

cargo llvm-cov report --cobertura --output-path "$e2e_path"
