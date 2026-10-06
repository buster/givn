#!/bin/sh
# Validate runtime paths exported by the active development shell.
# This script never invokes Nix, Fish, a package manager, or a network service.
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
repo_root=$(CDPATH= cd -- "$script_dir/../../.." && pwd -P)
lock_file="$script_dir/runtime-lock.json"
modules_lock="$script_dir/modules/package-lock.json"

fail() {
    printf 'OpenCode runtime setup failure: %s\n' "$*" >&2
    exit 1
}

sha256_file() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | awk '{print $1}'
    else
        shasum -a 256 "$1" | awk '{print $1}'
    fi
}

require_executable() {
    name=$1
    eval "value=\${$name-}"
    [ -n "$value" ] || fail "$name is not exported; enter the project development environment first"
    [ -x "$value" ] || fail "$name is not executable: $value"
}

for name in GIVN_OPENCODE_V1_FLOOR GIVN_OPENCODE_V1_LATEST GIVN_OPENCODE_V2_LATEST GIVN_OPENCODE_NODE GIVN_OPENCODE_FISH GIVN_OPENCODE_GIT; do
    require_executable "$name"
done

modules=${GIVN_OPENCODE_MODULES-}
[ -n "$modules" ] && [ -d "$modules" ] || fail 'GIVN_OPENCODE_MODULES is not an exported directory'
for module in '@ai-sdk/openai-compatible' '@opencode/ai'; do
    [ -f "$modules/$module/package.json" ] || fail "preprovisioned module is missing: $module"
done

check_version() {
    executable=$1
    expected=$2
    observed=$($executable --version 2>&1 | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+' | head -1 || true)
    [ "$observed" = "$expected" ] || fail "expected $executable version $expected, got ${observed:-no version}"
}

check_version "$GIVN_OPENCODE_V1_FLOOR" 1.18.29
check_version "$GIVN_OPENCODE_V1_LATEST" 1.18.34
check_version "$GIVN_OPENCODE_V2_LATEST" 2.0.22
[ "$($GIVN_OPENCODE_NODE --version)" = v24.21.0 ] || fail 'Node version does not match runtime-lock.json (expected v24.21.0)'
[ "$(sha256_file "$modules_lock")" = "$(jq -r '.runtimes.fixture_modules.package_lock_sha256' "$lock_file")" ] || fail 'fixture module lock digest mismatch'

for executable in "$GIVN_OPENCODE_V1_FLOOR" "$GIVN_OPENCODE_V1_LATEST" "$GIVN_OPENCODE_V2_LATEST"; do
    help=$($executable run --help 2>&1)
    case "$help" in *--format*) ;; *) fail "$executable run --help lacks --format" ;; esac
    case "$help" in *--model*) ;; *) fail "$executable run --help lacks --model" ;; esac
    case "$help" in *--auto*) ;; *) fail "$executable run --help lacks --auto" ;; esac
done
for executable in "$GIVN_OPENCODE_V1_FLOOR" "$GIVN_OPENCODE_V1_LATEST"; do
    help=$($executable run --help 2>&1)
    case "$help" in *--variant*) ;; *) fail "$executable run --help lacks --variant" ;; esac
done
v2_help=$($GIVN_OPENCODE_V2_LATEST run --help 2>&1)
case "$v2_help" in *--server*) ;; *) fail 'V2 run --help lacks --server' ;; esac
case "$v2_help" in *--standalone*) ;; *) fail 'V2 run --help lacks --standalone' ;; esac

case "$(uname -s)" in
    Darwin)
        [ -x /usr/bin/sandbox-exec ] || fail '/usr/bin/sandbox-exec is unavailable'
        platform=$(uname -m)-darwin
        case "$platform" in arm64-darwin) platform=aarch64-darwin ;; esac
        selected_shell=/bin/zsh
        shell_reason='Pinned V2 compatibility-priority selection rejects configured fish and falls back to Darwin zsh.'
        isolation=seatbelt
        ;;
    Linux)
        [ -r /proc/self/ns/net ] || fail 'Linux network namespace support is unavailable'
        platform=$(uname -m)-linux
        case "$platform" in aarch64-linux) ;; x86_64-linux) ;; *) fail "unsupported Linux architecture: $(uname -m)" ;; esac
        shell_diagnostic=$(mktemp "${TMPDIR:-/tmp}/givn-opencode-shell.XXXXXX")
        if ! HOME="${HOME:-/tmp}" XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-/tmp}" XDG_DATA_HOME="${XDG_DATA_HOME:-/tmp}" "$GIVN_OPENCODE_V2_LATEST" --print-logs debug paths --json >"$shell_diagnostic" 2>/dev/null; then
            rm -f "$shell_diagnostic"
            fail 'pinned V2 shell diagnostic command failed'
        fi
        selected_shell=$(jq -r '.shell.executable // empty' "$shell_diagnostic")
        shell_reason=$(jq -r '.shell.reason // empty' "$shell_diagnostic")
        rm -f "$shell_diagnostic"
        [ -n "$selected_shell" ] && [ -n "$shell_reason" ] || fail 'pinned V2 did not report shell selection'
        isolation=network-namespace-required
        ;;
    *) fail "unsupported isolation platform: $(uname -s)" ;;
esac
[ -x "$selected_shell" ] || fail "selected shell is missing: $selected_shell"

lock_sha=$(sha256_file "$lock_file")
sha256_stdin() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum | awk '{print $1}'
    else
        shasum -a 256 | awk '{print $1}'
    fi
}
floor_help_sha=$(printf '%s' "$($GIVN_OPENCODE_V1_FLOOR run --help 2>&1)" | sha256_stdin)
latest_help_sha=$(printf '%s' "$($GIVN_OPENCODE_V1_LATEST run --help 2>&1)" | sha256_stdin)
v2_help_sha=$(printf '%s' "$v2_help" | sha256_stdin)
runtime_tmp=${TMPDIR:-/tmp}

admission_json() {
    jq -n \
        --arg lock_sha "$lock_sha" --arg platform "$platform" \
        --arg floor "$GIVN_OPENCODE_V1_FLOOR" --arg latest "$GIVN_OPENCODE_V1_LATEST" \
        --arg v2 "$GIVN_OPENCODE_V2_LATEST" --arg node "$GIVN_OPENCODE_NODE" \
        --arg fish "$GIVN_OPENCODE_FISH" --arg git "$GIVN_OPENCODE_GIT" --arg modules "$modules" \
        --arg floor_help "$floor_help_sha" --arg latest_help "$latest_help_sha" --arg v2_help "$v2_help_sha" \
        --arg shell "$selected_shell" --arg shell_reason "$shell_reason" \
        --arg isolation "$isolation" --arg runtime_tmp "$runtime_tmp" \
        '{schema:1,lock_sha256:$lock_sha,platform:$platform,executables:[$floor,$latest,$v2,$node,$fish,$modules,$git],versions:{v1_floor:"1.18.29",v1_latest:"1.18.34",v2_latest:"2.0.22",node:"v24.21.0"},help_sha256:{v1_floor:$floor_help,v1_latest:$latest_help,v2_latest:$v2_help},shell:{executable:$shell,reason:$shell_reason},runtime_tmp:$runtime_tmp,isolation:$isolation}'
}

case "${1-}" in
    --json)
        admission_json
        ;;
    --exec)
        shift
        [ "$#" -gt 0 ] || fail '--exec requires a command'
        GIVN_OPENCODE_ADMISSION_JSON=$(admission_json)
        export GIVN_OPENCODE_ADMISSION_JSON
        for runtime in "$GIVN_OPENCODE_V1_FLOOR" "$GIVN_OPENCODE_V1_LATEST" "$GIVN_OPENCODE_V2_LATEST" "$GIVN_OPENCODE_NODE" "$GIVN_OPENCODE_FISH" "$GIVN_OPENCODE_GIT"; do
            PATH="$(dirname -- "$runtime"):$PATH"
        done
        export PATH="$modules:$PATH"
        exec "$@"
        ;;
    *) fail 'usage: resolve-runtime.sh --json | --exec <command> [args...]' ;;
esac
