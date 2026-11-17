#!/bin/sh
# The project's one verification entry point. Three scopes:
#
#   ./run-tests.sh                     regular suite, then the @e2e package
#   ./run-tests.sh --non-e2e           regular suite only
#   ./run-tests.sh --e2e --package e2e the @e2e package only
#
# The regular suite is the product's in-process component tier
# (`tests/features_runner.rs`). The E2E scope is the standalone `e2e/` package,
# which links no product crate and drives the built binary through `GIVN_BIN`;
# `--e2e` alone is refused because it names the tier, not the package.
#
# Formatting and lints are part of the suite: a green verification run must
# also be a formatted, warning-free tree. Both trees are checked; the e2e
# replay skips clippy because the regular scope already ran it.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$ROOT"

usage() {
    printf 'usage: %s [--e2e --package e2e|--non-e2e]\n' "$0" >&2
    exit 2
}

scope=all
package=
while [ "$#" -gt 0 ]; do
    case "$1" in
        --e2e|--non-e2e)
            [ "$scope" = all ] || usage
            scope=${1#--}
            shift
            ;;
        --package)
            [ "$#" -ge 2 ] || usage
            package=$2
            shift 2
            ;;
        *)
            usage
            ;;
    esac
done
if [ -n "$package" ]; then
    [ "$package" = e2e ] || {
        printf "%s: --package names no declared e2e root: '%s'\n" "$0" "$package" >&2
        exit 2
    }
    [ "$scope" = e2e ] || usage
fi
if [ "$scope" = e2e ] && [ -z "$package" ]; then
    printf '%s: --e2e names the tier, not the package; use --e2e --package e2e\n' "$0" >&2
    exit 2
fi

cargo fmt --all -- --check
if [ "$scope" != e2e ]; then
    cargo clippy --locked --all-targets -- -D warnings
    (cd e2e && cargo fmt --all -- --check)
    (cd e2e && cargo clippy --locked --all-targets -- -D warnings)
fi

# Feature scope: the caller either scopes the suite to a change delta via
# GIVN_FEATURES (as the check review/tasks gate does) or the permanent specs
# run. There is deliberately no path sniffing here.
FEATURES_PATH=${GIVN_FEATURES:-givn/specs}
FEATURES_ABS=$(CDPATH= cd -- "$(dirname -- "$FEATURES_PATH")" && pwd)/$(basename -- "$FEATURES_PATH")

# Verification scope declaration for the archive result contract. A caller
# that supplies the scope (change verification) owns it; otherwise derive the
# archive scope from the runner argument.
if [ -n "${GIVN_RESULT_SCOPE:-}" ]; then
    result_scope=$GIVN_RESULT_SCOPE
else
    case "$scope" in
        e2e) result_scope=e2e ;;
        *) result_scope=regular ;;
    esac
fi

# Test runner: cargo-nextest by default; GIVN_TEST_RUNNER=cargo falls back to
# plain `cargo test`. The cucumber runner receives the tag expression via
# GIVN_CUCUMBER_TAGS under nextest (its custom harness protocol) and via
# `--tags` under cargo test.
runner=${GIVN_TEST_RUNNER:-nextest}

# The regular scope excludes the @e2e corpus: its step definitions live in the
# package, so running them here would fail every scenario as unbound.
regular_tags='not @e2e and not @givn.removed'

# A scope result must come from one runner process; nextest executes one
# process per scenario and cannot aggregate a single GIVN_RESULT_FILE, so the
# result contract forces the single-process cargo path.
if [ -n "${GIVN_RESULT_FILE:-}" ]; then
    runner=cargo
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

run_regular() {
    # A scope with no scenario lines (a code-only change) cannot be run by
    # cucumber; exit 0 instead of failing the gate with a zero-match run. Under
    # the result contract the empty scope reports zero scenarios so Givn's
    # fail-closed validation reports the real reason.
    if ! grep -qrE '^\s*Scenario' -- "$FEATURES_PATH" 2>/dev/null; then
        if [ -n "${GIVN_RESULT_FILE:-}" ]; then
            printf '{"scope":"%s","total":0,"passed":0,"failed":0,"skipped":0}\n' \
                "$result_scope" > "$GIVN_RESULT_FILE"
        fi
        return 0
    fi
    GIVN_FEATURES="$FEATURES_PATH" GIVN_RESULT_SCOPE="$result_scope" run_cucumber "$regular_tags"
}

# The e2e package's target directory keeps one build root for the repository.
E2E_TARGET_DIR="$ROOT/target/e2e-package"

# The built product the package drives. A caller that already exports an
# executable GIVN_BIN (coverage instrumentation) owns it; otherwise build it.
build_product() {
    if [ -z "${GIVN_BIN:-}" ] || [ ! -x "${GIVN_BIN:-/nonexistent}" ]; then
        cargo build --locked --bin givn
        GIVN_BIN="$ROOT/target/debug/givn"
    fi
    if [ -z "${GIVN_READY_FIXTURE:-}" ] || [ ! -x "${GIVN_READY_FIXTURE:-/nonexistent}" ]; then
        GIVN_READY_FIXTURE=$(cargo test --locked --no-run --test ready_fixture \
            --message-format json 2>/dev/null |
            sed -n 's/.*"executable":"\([^"]*ready_fixture[^"]*\)".*/\1/p' | tail -1)
        [ -n "$GIVN_READY_FIXTURE" ] || {
            printf '%s: could not resolve the built ready_fixture path\n' "$0" >&2
            exit 2
        }
    fi
    # The package's working directory is e2e/, so every path it receives is
    # absolute; callers keep passing project-relative paths.
    export GIVN_BIN GIVN_READY_FIXTURE
    GIVN_BIN=$(CDPATH= cd -- "$(dirname -- "$GIVN_BIN")" && pwd)/$(basename -- "$GIVN_BIN")
    GIVN_READY_FIXTURE=$(CDPATH= cd -- "$(dirname -- "$GIVN_READY_FIXTURE")" && pwd)/$(basename -- "$GIVN_READY_FIXTURE")
    export GIVN_BIN GIVN_READY_FIXTURE
}

run_e2e() {
    build_product
    export GIVN_FEATURES="$FEATURES_ABS"
    export GIVN_RESULT_SCOPE=e2e
    if [ -n "${GIVN_RESULT_FILE:-}" ]; then
        result_dir=$(dirname -- "$GIVN_RESULT_FILE")
        mkdir -p "$result_dir"
        export GIVN_RESULT_FILE="$(CDPATH= cd -- "$result_dir" && pwd)/$(basename -- "$GIVN_RESULT_FILE")"
    fi
    (cd e2e && CARGO_TARGET_DIR="$E2E_TARGET_DIR" cargo test --locked \
        --manifest-path "$ROOT/e2e/Cargo.toml" --test e2e_runner)
}

case "$scope" in
    non-e2e) run_regular ;;
    e2e) run_e2e ;;
    all)
        run_regular
        run_e2e
        ;;
esac
