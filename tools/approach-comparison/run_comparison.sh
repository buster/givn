#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
XDG_CACHE_HOME=${XDG_CACHE_HOME:-${HOME:?HOME must be set when XDG_CACHE_HOME is unset}/.cache}
case "$XDG_CACHE_HOME" in
    /*) ;;
    *)
        printf '%s\n' 'XDG_CACHE_HOME must be an absolute path' >&2
        exit 2
        ;;
esac
NLI_MODEL=${NLI_MODEL:-$XDG_CACHE_HOME/givn/nli-exported}
CANDIDATE_SCOPE=${CANDIDATE_SCOPE:-bounded}

if [ "${SKIP_NLI:-0}" = "1" ]; then
    HF_HUB_OFFLINE=1 PYTHONUNBUFFERED=1 uv run --with 'fastembed==0.8.0' --with onnxruntime \
      --with tokenizers --with numpy \
      "$ROOT/tools/approach-comparison/compare.py" \
      --skip-nli \
      --candidate-scope "$CANDIDATE_SCOPE" \
      "$@" run
else
    HF_HUB_OFFLINE=1 PYTHONUNBUFFERED=1 uv run --with 'fastembed==0.8.0' --with onnxruntime \
      --with tokenizers --with numpy \
      "$ROOT/tools/approach-comparison/compare.py" \
      --nli-model "$NLI_MODEL" \
      --candidate-scope "$CANDIDATE_SCOPE" \
      "$@" run
fi
