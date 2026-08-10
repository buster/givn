#!/bin/sh
set -eu

if [ "${1:-}" = "--e2e" ]; then
    cargo test --test features_runner -- --tags @e2e
else
    cargo test --test features_runner -- --tags 'not @e2e'
fi
