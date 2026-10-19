#!/bin/sh
set -eu

# Deterministic harness tests. The default suite must never spend money and
# must not require a provider, a model, or a live sandbox.
uv run python -m unittest discover -s tests -p 'test_*.py' "$@"
