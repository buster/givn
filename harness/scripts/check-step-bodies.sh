#!/usr/bin/env bash
set -euo pipefail

# Audit script: verify every step_ function body in *_steps.py files
# contains either a NotImplementedError raise or a real implementation body.
#
# A step body of bare `pass`, empty body, or whitespace-only is
# indistinguishable from PASS — this script catches those.

FAILED=0

find tests/features -name '*_steps.py' -print0 | while IFS= read -r -d '' steps_file; do
    while IFS= read -r func_line; do
        func_name=$(echo "$func_line" | sed -n 's/.*def \(step_[^(]*\).*/\1/p')
        # Extract body: lines from the def line to the next def or end of file
        body=$(awk "/def ${func_name}/ {found=1; next} /^def / && found {exit} found {print}" "$steps_file")

        stripped_body=$(echo "$body" | tr -d '[:space:]' || true)
        if [ -z "$body" ] || [ -z "$stripped_body" ]; then
            echo "FAIL: empty body in ${func_name} (${steps_file}:$(grep -n "def ${func_name}" "$steps_file" | cut -d: -f1))"
            FAILED=1
            continue
        fi

        has_notimplemented=$(echo "$body" | grep -c "raise NotImplementedError" || true)
        non_pass_lines=$(echo "$body" | grep -vcE '^\s*(pass)?\s*$' || true)

        if [ "$has_notimplemented" -eq 0 ] && [ "$non_pass_lines" -le 1 ]; then
            echo "FAIL: ${func_name} (${steps_file}:$(grep -n "def ${func_name}" "$steps_file" | cut -d: -f1)) has no NotImplementedError and no real body (>1 line)"
            FAILED=1
        fi
    done < <(grep -n '^def step_' "$steps_file")
done

if [ "$FAILED" -ne 0 ]; then
    echo "check-step-bodies: FAIL"
    exit 1
fi

echo "check-step-bodies: PASS"
