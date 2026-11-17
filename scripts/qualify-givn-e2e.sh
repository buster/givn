#!/bin/sh
# Qualify the @e2e acceptance package: it must fail on a seeded product defect
# and pass once the defect is reverted, and its dependency tree must name no
# product crate.
#
# This is a maintainer and release check, not part of `verify.command` and not
# part of the archive gate: it runs the full suite twice. It works on a scratch
# copy so the checkout under test is never modified.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
PATCH="$ROOT/scripts/qualify/status-verification-current.patch"

scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT INT TERM

# A content copy of the checkout, without build output or browser installs.
tar -C "$ROOT" \
    --exclude=./target \
    --exclude=./e2e/target \
    --exclude=./e2e/browser/node_modules \
    --exclude=./.git \
    -cf - . | tar -C "$scratch" -xf -

cd "$scratch"

# 1. The package's dependency tree names no product crate. The package runner
#    re-reads the lockfile on every run; this is the maintainer's independent
#    reading of the same fact.
if cargo tree --locked --manifest-path e2e/Cargo.toml --prefix none 2>/dev/null |
    grep -Eq '^givn(-config|-core)? v'; then
    printf 'qualify: the e2e package links a product crate\n' >&2
    cargo tree --locked --manifest-path e2e/Cargo.toml --prefix none >&2 || true
    exit 1
fi

# 2. The unmodified package passes.
./run-tests.sh --e2e --package e2e

# 3. The seeded defect makes it fail.
patch -p1 --no-backup-if-mismatch < "$PATCH"
./run-tests.sh --e2e --package e2e && {
    printf 'qualify: the seeded defect did not fail the @e2e suite\n' >&2
    exit 1
}

# 4. Reverting the defect makes it pass again.
patch -R -p1 --no-backup-if-mismatch < "$PATCH"
./run-tests.sh --e2e --package e2e

printf 'qualify: the @e2e package fails on the seeded defect and passes after the revert\n'
