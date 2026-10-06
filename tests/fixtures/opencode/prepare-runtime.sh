# Sourced by ./run-tests.sh and ./measure-coverage.sh (POSIX sh, no shebang).
# One definition of the OpenCode runtime admission and the supervisor build, so
# plain and instrumented runs prepare the same environment.

# Validate the pinned runtimes supplied by the active development environment.
# The resolver is shell-only and does not invoke Nix or Fish.
prepare_opencode_admission() {
    GIVN_OPENCODE_ADMISSION_JSON=$(./tests/fixtures/opencode/resolve-runtime.sh --json) || return 1
    export GIVN_OPENCODE_ADMISSION_JSON

    # macOS exposes TMPDIR through a logical path in some shells (for example
    # /tmp) while Git reports its physical /private/tmp path. Keep fixture paths
    # physical before Cucumber creates any temporary projects.
    if [ "$(uname -s)" = Darwin ]; then
        TMPDIR=$(cd "${TMPDIR:-/tmp}" && pwd -P) || return 1
        export TMPDIR
    fi
}

# Build the supervisor with plain (never instrumented) Cargo and export the
# exact artifact path Cargo reports; guessing a hashed filename is unsafe and an
# on-demand build inside the suite could replace the givn binary scenarios spawn
# (CARGO_BIN_EXE_givn) mid-run. The supervisor is test infrastructure, not a
# Givn production coverage target.
prepare_opencode_supervisor() {
    supervisor_json=$(cargo build --locked --test opencode_supervisor --message-format=json) || return 1
    supervisor_path=$(printf '%s\n' "$supervisor_json" | jq -r 'select(.reason == "compiler-artifact" and .target.name == "opencode_supervisor") | .executable' | tail -1)
    if [ -z "$supervisor_path" ] || [ ! -x "$supervisor_path" ]; then
        printf 'setup failure: Cargo did not report an executable opencode_supervisor artifact\n' >&2
        return 1
    fi
    GIVN_OPENCODE_SUPERVISOR="$supervisor_path"
    export GIVN_OPENCODE_SUPERVISOR
}
