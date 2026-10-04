#!/bin/sh
# Release guards for the single dispatch release workflow (change release-truth).
#
#   release-guard.sh base <expected-version>
#       Exit 0 exactly when the newest version tag equals v<expected-version>;
#       otherwise name the missing tag and exit non-zero.
#
# Run from the repository root.

set -eu

usage() {
    printf 'usage: %s base <expected-version>\n       %s section <version> <changelog-file>\n' "$0" "$0" >&2
}

# Refuse to prepare a release whose range would start at an older tag.
base() {
    expected_version=$1
    expected_tag="v$expected_version"
    latest_tag=$(git tag --list 'v[0-9]*.[0-9]*.[0-9]*' --sort=-version:refname |
        awk 'NR == 1 { print; exit }')
    if [ "$latest_tag" != "$expected_tag" ]; then
        printf 'no version tag matches the current package version %s; expected tag %s (newest tag: %s)\n' \
            "$expected_version" "$expected_tag" "${latest_tag:-none}" >&2
        exit 1
    fi
}

# Refuse to prepare or publish a release whose section holds no user-visible change.
section() {
    version=$1
    changelog=$2
    if [ ! -f "$changelog" ]; then
        printf 'release section check: changelog file %s does not exist\n' "$changelog" >&2
        exit 1
    fi
    if ! grep -Eq "^## \[$version\]($| - )" "$changelog"; then
        printf 'no release section for %s in %s\n' "$version" "$changelog" >&2
        exit 1
    fi
    section_text=$(awk -v version="$version" '
        index($0, "## [" version "]") == 1 { in_release = 1; next }
        in_release && /^## \[/ { exit }
        in_release { print }
    ' "$changelog")
    if ! printf '%s\n' "$section_text" | grep -q '^### '; then
        printf 'release section %s has no user-visible change group\n' "$version" >&2
        exit 1
    fi
}

case "${1:-}" in
base)
    [ "$#" -eq 2 ] || {
        usage
        exit 2
    }
    base "$2"
    ;;
section)
    [ "$#" -eq 3 ] || {
        usage
        exit 2
    }
    section "$2" "$3"
    ;;
*)
    usage
    exit 2
    ;;
esac
