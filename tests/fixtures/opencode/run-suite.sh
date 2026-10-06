#!/bin/sh
# Run permanent setup scopes, or the complete permanent+delta matrix.
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
repo_root=$(CDPATH= cd -- "$script_dir/../../.." && pwd -P)
cd "$repo_root"

scope=all
scenario_title=
setup_smoke=0
case "${1-}" in
    --regular|--e2e|--all) scope=${1#--}; shift ;;
    --setup-smoke) setup_smoke=1; scope=smoke; shift ;;
    --name)
        [ "$#" -eq 2 ] || { echo 'usage: run-suite.sh --name <scenario title>' >&2; exit 2; }
        scenario_title=$2
        scope=name
        ;;
    '') ;;
    *) echo 'usage: run-suite.sh [--regular|--e2e|--all|--setup-smoke|--name <scenario title>]' >&2; exit 2 ;;
esac

GIVN_OPENCODE_ADMISSION_JSON=$(./tests/fixtures/opencode/resolve-runtime.sh --json)
export GIVN_OPENCODE_ADMISSION_JSON

cargo build --locked --test ready_fixture
supervisor_json=$(cargo build --locked --test opencode_supervisor --message-format=json)
supervisor_path=$(printf '%s\n' "$supervisor_json" | jq -r 'select(.reason == "compiler-artifact" and .target.name == "opencode_supervisor") | .executable' | tail -1)
[ -x "$supervisor_path" ] || { echo 'setup failure: missing opencode_supervisor executable' >&2; exit 1; }
export GIVN_OPENCODE_SUPERVISOR=$supervisor_path

if [ "$setup_smoke" -eq 1 ]; then
    smoke_root=$(mktemp -d "${TMPDIR:-/tmp}/givn-opencode-setup-smoke.XXXXXX")
    trap 'rm -rf "$smoke_root"' EXIT HUP INT TERM
    for release in v1_floor v1_latest v2_latest; do
        release_root=$(mktemp -d "${TMPDIR:-/tmp}/givn-opencode-setup-smoke-${release}.XXXXXX")
        "$supervisor_path" --timeout-seconds 60 --root "$release_root" -- \
            "$GIVN_OPENCODE_NODE" "$repo_root/tests/fixtures/opencode/runtime_driver.mjs" setup-smoke "$release"
    done
    exit 0
fi

run_scope() {
    root=$1
    tag=$2
    if [ "$scope" = name ]; then
        GIVN_FEATURES=$root cargo test --locked --test features_runner -- --name "^$scenario_title$"
    elif [ "$tag" = regular ]; then
        GIVN_FEATURES=$root ./run-tests.sh --non-e2e
    else
        GIVN_FEATURES=$root ./run-tests.sh --e2e
    fi
}

if [ "$scope" = regular ] || [ "$scope" = e2e ]; then
    run_scope givn/specs "$scope"
elif [ "$scope" = name ]; then
    run_scope givn/specs name
    run_scope givn/changes/fix-opencode-v2-attribution/specs name
else
    run_scope givn/specs regular
    run_scope givn/changes/fix-opencode-v2-attribution/specs regular
    run_scope givn/specs e2e
    run_scope givn/changes/fix-opencode-v2-attribution/specs e2e
fi
