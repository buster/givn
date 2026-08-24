#!/bin/sh
set -eu

MODEL_ID=${MODEL_ID:-MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7}
XDG_CACHE_HOME=${XDG_CACHE_HOME:-${HOME:?HOME must be set when XDG_CACHE_HOME is unset}/.cache}
case "$XDG_CACHE_HOME" in
    /*) ;;
    *)
        printf '%s\n' 'XDG_CACHE_HOME must be an absolute path' >&2
        exit 2
        ;;
esac
OUTPUT_DIR=${1:-$XDG_CACHE_HOME/givn/nli-exported}

mkdir -p "$OUTPUT_DIR"
uv run --with 'optimum[onnxruntime]' --with transformers \
  optimum-cli export onnx \
  --model "$MODEL_ID" \
  --task text-classification \
  "$OUTPUT_DIR"

printf 'exported %s to %s\n' "$MODEL_ID" "$OUTPUT_DIR"
