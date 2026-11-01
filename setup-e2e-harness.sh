#!/bin/sh
# One-time setup for the @e2e harness tier's Maki binary: download the pinned
# maki v0.7.0 musl release, verify its checksum, and extract it into a local bin
# directory the suite resolves (`$GIVN_E2E_BIN`, defaulting to
# `~/.cache/givn-e2e/bin`; `tests/steps/development_timeline_support.rs`
# resolves the same path, and `PATH` is the last fallback).
#
# This is one of the two steps that touch the network (the other is
# `./setup-e2e-browser.sh`); it runs once, outside the suite. The Maki driver
# runs the real binary as `maki --trust prompt`, which loads the project plugin
# and exits without any model call and without any network access.
set -eu

cd "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

MAKI_VERSION=v0.7.0
MAKI_TARGET=x86_64-unknown-linux-musl
MAKI_ARCHIVE="maki-${MAKI_VERSION}-${MAKI_TARGET}.tar.gz"
MAKI_URL="https://github.com/tontinton/maki/releases/download/${MAKI_VERSION}/${MAKI_ARCHIVE}"
# The release asset this repository pins. A changed upstream asset fails this
# script instead of silently installing something else.
MAKI_SHA256=3e0a1d1a4bc2bbaafdeb94f2a551bcc2d0f6ed81edfd9dd13272d9caa5add787

BIN_DIR=${GIVN_E2E_BIN:-"$HOME/.cache/givn-e2e/bin"}
CACHE_DIR=${GIVN_E2E_CACHE:-"$HOME/.cache/givn-e2e"}
ARCHIVE="$CACHE_DIR/$MAKI_ARCHIVE"

mkdir -p "$CACHE_DIR" "$BIN_DIR"

if [ ! -f "$ARCHIVE" ]; then
    echo "downloading $MAKI_URL"
    curl -fsSL -o "$ARCHIVE.tmp" "$MAKI_URL"
    mv "$ARCHIVE.tmp" "$ARCHIVE"
fi

echo "$MAKI_SHA256  $ARCHIVE" | sha256sum -c -

tar -xzf "$ARCHIVE" -C "$BIN_DIR" maki
chmod +x "$BIN_DIR/maki"

"$BIN_DIR/maki" --version
echo "maki installed at $BIN_DIR/maki (GIVN_E2E_BIN=$BIN_DIR)"
