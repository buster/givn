#!/bin/sh
# Assemble the committed evidence transcript of one E2E scenario from the
# per-example shards its run wrote under reports/visual/<slug>/ (one file per
# example, so concurrent examples never append to a shared file). Run it only
# after the scenario's final assertions passed.
#
# usage: assemble-transcripts.sh "<exact scenario title>"
set -eu

[ "$#" -eq 1 ] || { echo 'usage: assemble-transcripts.sh "<exact scenario title>"' >&2; exit 2; }
title=$1
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
repo_root=$(CDPATH= cd -- "$script_dir/../../.." && pwd -P)
slug=$(printf '%s' "$title" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9][^a-z0-9]*/-/g; s/^-//; s/-$//')
shards="$repo_root/reports/visual/$slug"
destination="$repo_root/givn/changes/fix-opencode-v2-attribution/evidence/visual/$slug"

[ -d "$shards" ] || { echo "no transcript shards for '$title' in $shards" >&2; exit 1; }
set -- "$shards"/*.txt
[ -e "$1" ] || { echo "no transcript shards for '$title' in $shards" >&2; exit 1; }

mkdir -p "$destination"
{
    printf '# Scenario: %s\n' "$title"
    printf '# Captured terminal transcript: real givn CLI and real OpenCode CLI, assembled from %s example shard(s).\n' "$#"
    printf '# Random temporary-path prefixes are redacted to <TMPDIR>; everything else is raw output.\n'
    for shard in "$@"; do
        printf '\n'
        cat "$shard"
    done
} > "$destination/transcript.txt"
printf 'wrote %s (%s shard(s))\n' "$destination/transcript.txt" "$#"
