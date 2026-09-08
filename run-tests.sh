#!/bin/sh
set -eu

# Feature scope: the caller either scopes the suite to a change delta via
# GIVN_FEATURES (as the check review/tasks gate does) or the permanent specs
# run. There is deliberately no path sniffing here.
FEATURES_PATH=${GIVN_FEATURES:-givn/specs}

# Test runner: cargo-nextest by default; GIVN_TEST_RUNNER=cargo falls back
# to plain `cargo test`. The cucumber runner receives the tag expression via
# GIVN_CUCUMBER_TAGS under nextest (its custom harness protocol) and via
# `--tags` under cargo test.
runner=${GIVN_TEST_RUNNER:-nextest}

run_cucumber() {
    tags=$1
    shift
    if [ "$runner" = nextest ]; then
        GIVN_CUCUMBER_TAGS="$tags" cargo nextest run --no-tests=warn --locked "$@" --test features_runner
    else
        cargo test --locked "$@" --test features_runner -- --tags "$tags"
    fi
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

# Prepare the ready-fixture process before the suite runs. The on-demand
# build inside a running suite would replace the givn binary the scenarios
# spawn (CARGO_BIN_EXE_givn) mid-run.
cargo build --locked --test ready_fixture

case "${1:-}" in
    --e2e)
        GIVN_FEATURES="$FEATURES_PATH" run_cucumber '@e2e'
        ;;
    --non-e2e)
        GIVN_FEATURES="$FEATURES_PATH" run_cucumber 'not @e2e'
        ;;
    "")
        GIVN_FEATURES="$FEATURES_PATH" run_cucumber ''
        ;;
    *)
        printf 'usage: %s [--e2e|--non-e2e]\n' "$0" >&2
        exit 2
        ;;
esac