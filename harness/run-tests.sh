#!/bin/sh
set -eu

if [ "${1:-}" = "--e2e" ]; then
    uv run behave tests/features/ --no-capture --tags=@e2e --tags=-wip
else
    uv run behave tests/features/ --no-capture --tags=-wip --tags=-e2e --tags=-needs-sandlock
fi
