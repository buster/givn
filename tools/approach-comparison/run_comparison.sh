#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
NLI_MODEL=${NLI_MODEL:-/tmp/opencode/nli-model}
CANDIDATE_SCOPE=${CANDIDATE_SCOPE:-strict}

PYTHONUNBUFFERED=1 uv run --with 'fastembed==0.8.0' --with onnxruntime \
  --with tokenizers --with numpy \
  "$ROOT/tools/approach-comparison/compare.py" \
  --nli-model "$NLI_MODEL" \
  --candidate-scope "$CANDIDATE_SCOPE" \
  "$@" run
