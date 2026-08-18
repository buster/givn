# Combined Corpus Approach Comparison

This experiment compares the three local scenario-comparison approaches on the
active Givn and Watn specs without changing either project:

1. multilingual E5 passage embeddings with a strict `0.95` candidate threshold;
2. the local BGE relevance cross-encoder with the branch boundary `3.0`;
3. bidirectional `mDeBERTa` NLI with symmetric entailment/neutral/contradiction
   probabilities.

The copied corpus is under `test-corpus/specs/{givn,watn}`. It preserves the
original feature directory structure. Watn's historical `givn/archive/` tree is
excluded because it contains superseded scenarios rather than the active
permanent corpus. The Watn checkout is never modified.

Generated embedding and comparison files stay under `test-corpus/indexes/` and
are ignored by Git. Model caches remain in the existing local Givn cache or the
explicit NLI model directory.

## Scripts

| Script | Purpose |
|---|---|
| `prepare_corpus.sh` | Copies active `givn/specs` from this repository and active `watn/givn/specs` into the local test corpus while preserving capability directories. It never copies Watn archive history. |
| `export_nli_model.sh` | Exports the researched multilingual NLI model to a local ONNX directory. |
| `compare.py` | Parses both corpora, builds/reuses the combined E5 index, creates the shared top-K/strict candidate set, scores BGE and NLI, and writes the complete JSON report. |
| `run_comparison.sh` | Reusable `uv` wrapper that invokes `compare.py` with the pinned Python package set and the local NLI model. |
| `selected_pairs.py` | Extracts the ten manually classified Givn reference pairs from the generated report. |
| `mixed_pairs.py` | Extracts the balanced five-Givn/five-Watn reference classifications from the generated report. |

## Run

```sh
tools/approach-comparison/prepare_corpus.sh
tools/approach-comparison/export_nli_model.sh /tmp/opencode/nli-model
tools/approach-comparison/run_comparison.sh \
  --report-limit 30 \
  --report-name test-corpus/indexes/combined-approach-report-strict.json
tools/approach-comparison/selected_pairs.py
tools/approach-comparison/mixed_pairs.py
```

The wrapper defaults to `CANDIDATE_SCOPE=bounded`, the product set formed by
the top-K union intersected with strict E5 candidates. The historical strict
comparison is requested explicitly with `CANDIDATE_SCOPE=strict`; this avoids
conflating candidate recall with pair-model quality. The larger pools can be
requested explicitly:

```sh
CANDIDATE_SCOPE=topk tools/approach-comparison/run_comparison.sh
CANDIDATE_SCOPE=bounded tools/approach-comparison/run_comparison.sh
CANDIDATE_SCOPE=union tools/approach-comparison/run_comparison.sh
CANDIDATE_SCOPE=cross-source tools/approach-comparison/run_comparison.sh \
  --report-name test-corpus/indexes/combined-approach-report-cross-source.json
```

`strict` uses the combined pairs at or above `0.95`. `topk` uses the union of
five neighbours per scenario. `bounded` is the product BGE set: top-K union
intersected with strict E5 pairs. `union` adds strict-threshold pairs to the
top-K set. The script prints model-stage progress
so long CPU runs do not appear stalled and stores BGE/NLI stage durations in the
JSON report. `cross-source` scores only top-K pairs whose endpoints come from
different source projects; it isolates interaction between the Givn and Watn
corpora without paying for the complete 1,428-pair top-K union.

Use `--rebuild` after changing the copied corpus. Use `--report-limit 100` to
print more of the common candidate set. The complete report is written to
`test-corpus/indexes/combined-approach-report.json`.
