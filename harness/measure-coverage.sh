#!/bin/sh
set -eu

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
mkdir -p "$(dirname "$non_e2e_path")" "$(dirname "$e2e_path")"

COVERAGE_FILE=coverage/.coverage-non-e2e \
    uv run coverage run -m behave tests/features/ --no-capture \
    --tags=-wip --tags=-e2e --tags=-needs-sandlock
COVERAGE_FILE=coverage/.coverage-non-e2e uv run coverage xml -o "$non_e2e_path"

COVERAGE_FILE=coverage/.coverage-e2e \
    uv run coverage run -m behave tests/features/ --no-capture --tags=@e2e --tags=-wip
COVERAGE_FILE=coverage/.coverage-e2e uv run coverage xml -o "$e2e_path"
