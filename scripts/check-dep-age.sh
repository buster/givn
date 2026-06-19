#!/usr/bin/env bash
#
# check-dep-age.sh — enforce a minimum age for every locked dependency.
#
# Policy (givn dev policy, see docs/arc42/02-architecture-constraints.md):
#   Every crate version pinned in Cargo.lock must have been published on
#   crates.io at least MIN_AGE_DAYS ago (default: 30). This avoids pulling in
#   brand-new releases that have had no time to surface bugs or supply-chain
#   issues. Cargo has no native "minimum dependency age" feature, so we enforce
#   it here against the crates.io API.
#
# Scope: ALL locked dependencies (direct AND transitive), as listed in
#        Cargo.lock. Local path/git sources (no version, or with a [source]
#        that is not the crates.io registry) are skipped.
#
# Usage:
#   scripts/check-dep-age.sh [path/to/Cargo.lock]
#
# Environment:
#   MIN_AGE_DAYS   Minimum age in days (default: 30).
#   GIVN_DEP_AGE_ALLOWLIST
#                  Space-separated "name@version" entries to exempt
#                  (e.g. "foo@1.2.3 bar@0.1.0"). Use sparingly.
#
# Exit codes:
#   0  all dependencies satisfy the minimum age (or were skipped/allowlisted)
#   1  one or more dependencies are too new (policy violation)
#   2  usage / environment / parsing error (e.g. missing tools, no Cargo.lock)
#
# Dependencies: bash, curl, and one of {jq | python3}. Uses the crates.io API:
#   https://crates.io/api/v1/crates/<name>/<version>   (field: version.created_at)
#
set -euo pipefail

MIN_AGE_DAYS="${MIN_AGE_DAYS:-30}"
LOCKFILE="${1:-Cargo.lock}"
ALLOWLIST=" ${GIVN_DEP_AGE_ALLOWLIST:-} "   # padded for substring matching
USER_AGENT="givn-dep-age-check (https://crates.io)"

die() { printf 'error: %s\n' "$*" >&2; exit 2; }

command -v curl >/dev/null 2>&1 || die "curl is required"
[ -f "$LOCKFILE" ] || die "lockfile not found: $LOCKFILE (run from the crate root or pass a path)"

# Pick a JSON extractor.
JSON_TOOL=""
if command -v jq >/dev/null 2>&1; then
  JSON_TOOL="jq"
elif command -v python3 >/dev/null 2>&1; then
  JSON_TOOL="python3"
else
  die "need either 'jq' or 'python3' to parse API responses"
fi

# Extract created_at (ISO8601) for a given crate version from crates.io JSON.
extract_created_at() {
  # stdin: API JSON ; stdout: created_at string or empty
  if [ "$JSON_TOOL" = "jq" ]; then
    jq -r '.version.created_at // empty'
  else
    python3 -c 'import sys,json
try:
    d=json.load(sys.stdin)
    print(d.get("version",{}).get("created_at","") or "")
except Exception:
    print("")'
  fi
}

# Convert ISO8601 to epoch seconds (GNU date, with a python3 fallback).
to_epoch() {
  local iso="$1"
  if date -d "$iso" +%s >/dev/null 2>&1; then
    date -d "$iso" +%s
  else
    python3 -c 'import sys,datetime
s=sys.argv[1].replace("Z","+00:00")
print(int(datetime.datetime.fromisoformat(s).timestamp()))' "$iso"
  fi
}

now_epoch="$(date +%s)"
min_age_seconds=$(( MIN_AGE_DAYS * 24 * 60 * 60 ))

# Parse Cargo.lock [[package]] blocks. We need name+version, and we must skip
# packages whose source is not the crates.io registry (path/git deps), plus the
# root package (which has no [source]). We do this with a small awk state machine.
#
# crates.io registry source string in Cargo.lock:
#   source = "registry+https://github.com/rust-lang/crates.io-index"
mapfile -t PACKAGES < <(
  awk '
    /^\[\[package\]\]/ { name=""; version=""; source=""; in_pkg=1; next }
    in_pkg && /^name = / { gsub(/^name = "|"$/,""); name=$0; next }
    in_pkg && /^version = / { gsub(/^version = "|"$/,""); version=$0; next }
    in_pkg && /^source = / { gsub(/^source = "|"$/,""); source=$0; next }
    /^$/ {
      if (in_pkg && name!="" && version!="" && index(source,"registry+")==1) {
        print name "@" version
      }
      in_pkg=0
    }
    END {
      if (in_pkg && name!="" && version!="" && index(source,"registry+")==1) {
        print name "@" version
      }
    }
  ' "$LOCKFILE" | sort -u
)

if [ "${#PACKAGES[@]}" -eq 0 ]; then
  echo "No crates.io-registry dependencies found in $LOCKFILE (nothing to check)."
  exit 0
fi

echo "Checking ${#PACKAGES[@]} crates.io dependencies for minimum age >= ${MIN_AGE_DAYS} days..."

violations=0
checked=0
skipped=0

for pv in "${PACKAGES[@]}"; do
  name="${pv%@*}"
  version="${pv##*@}"

  # Allowlist check.
  if [[ "$ALLOWLIST" == *" ${name}@${version} "* ]]; then
    echo "  SKIP (allowlisted)  ${name} ${version}"
    skipped=$(( skipped + 1 ))
    continue
  fi

  url="https://crates.io/api/v1/crates/${name}/${version}"
  json="$(curl -fsSL -H "User-Agent: ${USER_AGENT}" "$url" 2>/dev/null || true)"
  if [ -z "$json" ]; then
    echo "  WARN  could not fetch ${name} ${version} from crates.io (treating as failure)" >&2
    violations=$(( violations + 1 ))
    continue
  fi

  created_at="$(printf '%s' "$json" | extract_created_at)"
  if [ -z "$created_at" ]; then
    echo "  WARN  no created_at for ${name} ${version} (treating as failure)" >&2
    violations=$(( violations + 1 ))
    continue
  fi

  created_epoch="$(to_epoch "$created_at")"
  age_seconds=$(( now_epoch - created_epoch ))
  age_days=$(( age_seconds / 86400 ))
  checked=$(( checked + 1 ))

  if [ "$age_seconds" -lt "$min_age_seconds" ]; then
    echo "  FAIL  ${name} ${version}  (${age_days}d old < ${MIN_AGE_DAYS}d; published ${created_at})"
    violations=$(( violations + 1 ))
  fi

  # Be polite to the API.
  sleep 0.1
done

echo "---"
echo "checked: ${checked}, skipped: ${skipped}, violations: ${violations}"

if [ "$violations" -gt 0 ]; then
  echo "POLICY VIOLATION: ${violations} dependency/dependencies younger than ${MIN_AGE_DAYS} days." >&2
  echo "Pin an older version (cargo update -p <crate> --precise <older>), wait, or allowlist via GIVN_DEP_AGE_ALLOWLIST." >&2
  exit 1
fi

echo "OK: all checked dependencies are at least ${MIN_AGE_DAYS} days old."
exit 0
