#!/bin/sh
set -eu

. ./tests/fixtures/opencode/prepare-runtime.sh
prepare_opencode_admission

# Feature scope: the caller either scopes the suite to a change delta via
# GIVN_FEATURES (as the check review/tasks gate does) or the permanent specs
# run. There is deliberately no path sniffing here.
FEATURES_PATH=${GIVN_FEATURES:-givn/specs}

# Verification scope declaration for the archive result contract.
case "${1:-}" in
    --e2e) result_scope=e2e ;;
    *) result_scope=regular ;;
esac

# Test runner: cargo-nextest by default; GIVN_TEST_RUNNER=cargo falls back
# to plain `cargo test`. The cucumber runner receives the tag expression via
# GIVN_CUCUMBER_TAGS under nextest (its custom harness protocol) and via
# `--tags` under cargo test.
runner=${GIVN_TEST_RUNNER:-nextest}

# A scope result must come from one runner process; nextest executes one
# process per scenario and cannot aggregate a single GIVN_RESULT_FILE, so the
# result contract forces the single-process cargo path.
if [ -n "${GIVN_RESULT_FILE:-}" ]; then
    runner=cargo
    export GIVN_RESULT_SCOPE="$result_scope"
fi

run_cucumber() {
    tags=$1
    shift
    if [ "$runner" = nextest ]; then
        GIVN_CUCUMBER_TAGS="$tags" cargo nextest run --no-tests=warn --locked "$@" --test features_runner
    elif [ -n "$tags" ]; then
        cargo test --locked "$@" --test features_runner -- --tags "$tags"
    else
        cargo test --locked "$@" --test features_runner
    fi
}

# A scope with no scenario lines (a code-only change) cannot be run by
# cucumber; exit 0 instead of failing the gate with a zero-match run. Under
# the result contract the empty scope reports zero scenarios so Givn's
# fail-closed validation reports the real reason.
case "${1:-}" in
    --e2e|--non-e2e|"")
        if ! grep -qrE '^\s*Scenario' -- "$FEATURES_PATH" 2>/dev/null; then
            if [ -n "${GIVN_RESULT_FILE:-}" ]; then
                printf '{"scope":"%s","total":0,"passed":0,"failed":0,"skipped":0}\n' \
                    "$result_scope" > "$GIVN_RESULT_FILE"
            fi
            exit 0
        fi
        ;;
esac

# Prepare the ready fixture and supervisor before the suite runs.
cargo build --locked --test ready_fixture
prepare_opencode_supervisor

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
