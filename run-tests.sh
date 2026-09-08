#!/bin/sh
set -eu

# Feature scope: the caller either scopes the suite to a change delta via
# GIVN_FEATURES (as the check review/tasks gate does) or the permanent specs
# run. There is deliberately no path sniffing here.
FEATURES_PATH=${GIVN_FEATURES:-givn/specs}

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
        GIVN_FEATURES="$FEATURES_PATH" cargo test --locked --test features_runner -- --tags '@e2e'
        ;;
    --non-e2e)
        GIVN_FEATURES="$FEATURES_PATH" cargo test --locked --test features_runner -- --tags 'not @e2e'
        ;;
    "")
        GIVN_FEATURES="$FEATURES_PATH" cargo test --locked --test features_runner
        ;;
    *)
        printf 'usage: %s [--e2e|--non-e2e]\n' "$0" >&2
        exit 2
        ;;
esac