# Real-Model Duplicate Filter Report

## Scope

This report records the rerun against the current permanent Givn corpus after
replacing the fixed second-stage pair refusal with the configurable
`GIVN_SPEC_MAX_RERANK_PAIRS` setting. No embedding, BGE, NLI, score, or test-pool
stub was used. The setting was unset, so the production default `2048` applied.

The current corpus contains `394` permanent scenarios. Deterministic analysis
produces `4230` structural overlaps. E5 produces `1274` semantic candidates at
the inclusive cosine boundary `0.9500`. The E5 index was refreshed immediately
before the model runs and reports `0` E5 token-cap findings.

This is the current PR #27 evidence set. The corresponding benchmark and manual
interpretation are linked below; the raw JSON inputs remain local and ephemeral.

## Related Current Evidence

- [`semantic-duplicates-e5-threshold-vs-serialization-analysis.md`](semantic-duplicates-e5-threshold-vs-serialization-analysis.md)
- [`semantic-duplicates-manual-evaluation.md`](semantic-duplicates-manual-evaluation.md)
- [`bge-reranking-benchmark.md`](bge-reranking-benchmark.md)

The runs were:

```sh
env -u GIVN_SPEC_EMBEDDER target/release/givn spec index

env -u GIVN_SPEC_EMBEDDER -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER \
  target/release/givn spec duplicates --format json

env -u GIVN_SPEC_EMBEDDER -u GIVN_SPEC_RERANKER GIVN_SPEC_BGE=local \
  GIVN_SPEC_BGE_MODEL_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/givn/bge" \
  target/release/givn spec duplicates --format json

env -u GIVN_SPEC_EMBEDDER -u GIVN_SPEC_BGE \
  GIVN_SPEC_RERANKER=nli GIVN_SPEC_NLI_MODEL_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/givn/nli-exported" \
  target/release/givn spec duplicates --format json

env -u GIVN_SPEC_EMBEDDER GIVN_SPEC_BGE=local \
  GIVN_SPEC_BGE_MODEL_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/givn/bge" \
  GIVN_SPEC_RERANKER=nli GIVN_SPEC_NLI_MODEL_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/givn/nli-exported" \
  target/release/givn spec duplicates --format json
```

## Results

| Stage | E5 candidates | Scored pairs | Recommended | Filtered | Token-cap | Unscored | Runtime |
|---|---:|---:|---:|---:|---:|---:|---:|
| E5 semantic output | 1274 | - | 1274 visible | - | 0 | - | - |
| BGE | 1274 | 669 | 1 | 668 | 126 | 605 | 819.8 s model runtime / 821.6 s wall |
| NLI | 1274 | 1062 | 17 | 1045 | 212 | 212 | 768.1 s model runtime / 770.1 s wall |
| BGE + NLI | 1274 | 669 / 1062 | 1 / 17 | 668 / 1045 | 126 / 212 | 605 / 212 | 1586.6 s wall |

The active pair budget was `2048` in every run. It did not truncate this
corpus. BGE still scores only the existing deterministic top-five endpoint
union: `795` eligible pairs, consisting of `669` scored pairs and `126`
selected pairs over the BGE 512-token input limit. The other `479` E5 pairs are
outside that endpoint union. BGE's reported `unscored_count` is `605`, which
includes those `479` pairs plus the `126` token-cap pairs.

NLI receives its independent E5 pool, not BGE survivors. All `1274` candidates
fit under the pair-count setting; `1062` receive NLI probabilities and `212`
receive `NLI_TOKEN_CAP` evidence because their complete pair encoding exceeds
512 tokens. The combined recommendation is the intersection of the real BGE
and NLI recommendations and contains `0` pairs in this run.

## Runtime Measurement

The commands were run with the local ONNX artifacts and four intra-op model
threads. `/usr/bin/time` measured process wall time, CPU time, and maximum RSS.

| Run | Wall | CPU | Max RSS |
|---|---:|---:|---:|
| BGE-only | `821.60 s` | `3073.86 s` | `2.01 GiB` |
| NLI-only | `770.13 s` | `2450.00 s` | `2.20 GiB` |
| BGE + NLI | `1586.62 s` | `5531.19 s` | `2.62 GiB` |

The combined wall time is approximately `26:27 min`. The independent runs vary
slightly because model loading and CPU scheduling are measured in each process.
The combined run confirms that the two layers execute sequentially and retain
independent evidence.

## Interpretation

The fixed `512` pair refusal is gone from the production path. A pool larger
than the configured setting is now processed as an E5-ranked prefix, with the
tail reported as `BGE_UNSCORED` or `NLI_UNSCORED` rather than as an unavailable
model run. The separate 512-token model-input guard remains active and produces
`BGE_TOKEN_CAP`/`NLI_TOKEN_CAP` for selected overlong pairs.

The default `2048` is high enough for the current E5 candidate count. It does
not remove the BGE top-five eligibility boundary or the model-token boundary.
The real current result is therefore:

- BGE: `1` recommendation, `668` model-filtered pairs, `126` token-cap pairs,
  and `479` E5 pairs outside the BGE endpoint union.
- NLI: `17` recommendations, `1045` model-filtered pairs, and `212` token-cap
  pairs.
- Combined: `0` candidates recommended by both models.

The machine-readable raw reports used for this analysis are stored under
`/tmp/opencode/givn-duplicates-*-2048.json`.
