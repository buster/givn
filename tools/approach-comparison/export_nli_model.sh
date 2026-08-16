#!/bin/sh
set -eu

MODEL_ID=${MODEL_ID:-MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7}
OUTPUT_DIR=${1:-/tmp/opencode/nli-model}

mkdir -p "$OUTPUT_DIR"
uv run --with 'optimum[onnxruntime]' --with transformers \
  optimum-cli export onnx \
  --model "$MODEL_ID" \
  --task text-classification \
  "$OUTPUT_DIR"

printf 'exported %s to %s\n' "$MODEL_ID" "$OUTPUT_DIR"
