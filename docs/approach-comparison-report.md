# Combined Givn/Watn Approach Comparison

## Scope

This is a corpus experiment. It is not a givn change. No permanent Givn spec
was modified, and the Watn checkout was never modified.

The active spec trees were copied into:

```text
test-corpus/specs/givn/
test-corpus/specs/watn/
```

The original capability directories remain part of each copied tree. Historical
Watn `givn/archive/` features were excluded because they are superseded versions
of active scenarios and would inflate the duplicate count with intentional
history.

Inventory:

| Source | Feature files | Scenarios |
|---|---:|---:|
| Givn | 26 | 208 |
| Watn | 25 | 217 |
| Combined | 51 | 425 |

## Reusable Scripts

Every measurement in this report is tied to a script under
`tools/approach-comparison/`.

| Script | Responsibility | Referenced result |
|---|---|---|
| `prepare_corpus.sh` | Copies active Givn and Watn specs into the local test corpus and preserves capability directories. | Corpus inventory and provenance. |
| `export_nli_model.sh` | Exports `MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7` to ONNX. | NLI model artifact. |
| `compare.py` | Parses both trees, builds/reuses the combined E5 index, creates candidate scopes, runs BGE and NLI, and writes JSON reports. | All aggregate scores, distributions, and timings. |
| `run_comparison.sh` | Runs `compare.py` with the reproducible `uv` dependency set. | Strict and Cross-Source runs. |
| `selected_pairs.py` | Extracts the ten manually classified Givn reference pairs from a generated report. | Reference-pair table below. |
| `tools/approach-comparison/README.md` | Documents setup, script roles, candidate scopes, and rerun commands. | Operational reference. |

The generated index and reports remain under `test-corpus/indexes/` and are
checked in as experiment evidence. Large model weights and local model caches
remain outside Git. This keeps the data, index, reports, and tooling together
without checking in multi-gigabyte inference artifacts.

## Compared Approaches

The scripts compare the following stages on identical candidate pairs:

1. E5 passage embeddings with cosine similarity and the strict threshold `0.95`.
2. `rozgo/bge-reranker-v2-m3` with both pair directions and averaged raw logits.
3. `MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7` with both pair
   directions and symmetric `entailment / neutral / contradiction` probabilities.

The Python run uses `fastembed==0.8.0` for E5 under the agreed assumption that
the Python and Rust embedding paths behave equivalently. BGE and NLI use the
local ONNX artifacts with a 512-token pair limit. The BGE threshold is `3.0`.
NLI has no decision threshold; its output is diagnostic.

## Candidate Scopes

The tool supports four scopes:

| Scope | Meaning |
|---|---|
| `strict` | All combined pairs with E5 cosine `>= 0.95`. Recommended direct comparison. |
| `topk` | The union of five E5 neighbours per scenario. |
| `union` | `topk` plus every strict-threshold pair. |
| `cross-source` | Only top-K pairs with one Givn and one Watn endpoint. |

The full `union` pool contained 1,431 pairs. A complete BGE/NLI run over that
pool exceeded 40 minutes on the available CPU and was stopped before producing
a report. It is not used for the conclusions below. The `strict` scope provides
the relevant 199 conflicts and completes in a bounded run; `cross-source`
isolates the nine actual Givn-Watn top-K relationships.

## Strict Combined Run

Command:

```sh
tools/approach-comparison/prepare_corpus.sh
tools/approach-comparison/run_comparison.sh \
  --report-limit 30 \
  --report-name test-corpus/indexes/combined-approach-report-strict.json
```

The command uses `CANDIDATE_SCOPE=strict` by default.

Measured result:

| Measurement | Result |
|---|---:|
| Combined scenarios | 425 |
| Strict E5 pairs | 199 |
| Top-5 union pairs | 1,428 |
| Scored common pairs | 199 |
| Within Givn | 160 |
| Within Watn | 39 |
| Givn-Watn strict pairs | 0 |
| BGE pairs at or above `3.0` | 45 |
| BGE duration | 258.33 s |
| NLI duration | 147.33 s |
| BGE + NLI duration | 405.66 s |

NLI label distribution over the same 199 strict pairs:

```text
121 contradiction
 57 neutral
 21 entailment
```

The combined corpus has no cross-project pair at the strict `0.95` threshold.
Givn and Watn interact only in the weaker top-K retrieval zone.

## Cross-Source Run

Command:

```sh
CANDIDATE_SCOPE=cross-source \
  tools/approach-comparison/run_comparison.sh \
  --report-limit 20 \
  --report-name test-corpus/indexes/combined-approach-report-cross-source.json
```

Measured result:

| Measurement | Result |
|---|---:|
| Top-5 union pairs | 1,428 |
| Givn-Watn pairs scored | 9 |
| Strict Givn-Watn pairs | 0 |
| BGE pairs at or above `3.0` | 0 |
| BGE duration | 11.80 s |
| NLI duration | 7.80 s |
| Total pair-model duration | 19.61 s |

NLI labels for the nine cross-source pairs:

```text
5 entailment
3 neutral
1 contradiction
```

These are related-language candidates, not strict duplicate candidates. Neither
pair model produces a cross-project BGE gate hit.

## Ten Classified Givn Pairs

The following table is generated by `selected_pairs.py` from the strict report.
The classifications were assigned interactively before this combined run.

| # | Classification | E5 cosine | BGE avg | NLI E/N/C |
|---:|---|---:|---:|---|
| 1 | `VALID-BOUNDARY` | 0.9693 | 6.1987 | 0.299 / 0.437 / 0.264 |
| 2 | `DUPLIKAT` | 0.9664 | 3.4278 | 0.103 / 0.067 / 0.830 |
| 3 | `DUPLIKAT` | 0.9660 | 1.0717 | 0.046 / 0.929 / 0.025 |
| 4 | `VALID-BOUNDARY` | 0.9634 | 1.3550 | 0.024 / 0.901 / 0.075 |
| 5 | `DUPLIKAT` | 0.9599 | 2.1883 | 0.552 / 0.443 / 0.005 |
| 6 | `VARIANTE` | 0.9597 | 2.8355 | 0.239 / 0.526 / 0.235 |
| 7 | `VARIANTE` | 0.9585 | 3.7419 | 0.212 / 0.430 / 0.359 |
| 8 | `VARIANTE` | 0.9542 | 3.2748 | 0.296 / 0.537 / 0.167 |
| 9 | `VARIANTE` | 0.9513 | 1.9823 | 0.362 / 0.510 / 0.128 |
| 10 | `VARIANTE` | 0.9500 | 3.6262 | 0.192 / 0.459 / 0.349 |

The exact scenario titles are emitted by:

```sh
tools/approach-comparison/selected_pairs.py \
  test-corpus/indexes/combined-approach-report-strict.json
```

## Mixed Givn/Watn Reference Set

The second interactive classification round used five Givn and five Watn
pairs. The classifications are persisted in `mixed_pairs.py` and reproduced by:

```sh
tools/approach-comparison/mixed_pairs.py \
  test-corpus/indexes/combined-approach-report-strict.json
```

Reference classification:

| # | Source | Classification | E5 | BGE | NLI E/N/C |
|---:|---|---|---:|---:|---|
| 1 | Givn | `VARIANTE` | 0.9899 | 4.3165 | 0.006 / 0.094 / 0.900 |
| 2 | Givn | `VALID-BOUNDARY` | 0.9850 | -0.0075 | 0.003 / 0.010 / 0.987 |
| 3 | Givn | `VALID-BOUNDARY` | 0.9841 | 2.6459 | 0.038 / 0.200 / 0.762 |
| 4 | Givn | `VARIANTE` | 0.9806 | 4.7882 | 0.117 / 0.242 / 0.641 |
| 5 | Givn | `VALID-BOUNDARY` | 0.9799 | 2.0405 | 0.007 / 0.382 / 0.611 |
| 6 | Watn | `VARIANTE` | 0.9869 | 6.4007 | 0.345 / 0.375 / 0.280 |
| 7 | Watn | `VALID-BOUNDARY` | 0.9799 | 0.1636 | 0.005 / 0.088 / 0.908 |
| 8 | Watn | `VALID-BOUNDARY` | 0.9767 | 1.0563 | 0.004 / 0.214 / 0.781 |
| 9 | Watn | `DUPLIKAT` | 0.9765 | 3.6417 | 0.645 / 0.344 / 0.012 |
| 10 | Watn | `VALID-BOUNDARY` | 0.9731 | 3.4985 | 0.005 / 0.652 / 0.343 |

The mixed set contains one duplicate, three variants, and six valid boundaries.
It is deliberately a demonstration set, not an unbiased estimate of corpus-wide
precision or recall.

For this set:

- E5 blocks all ten pairs and cannot distinguish the six boundaries.
- BGE crosses `3.0` for pairs 1, 4, 6, 9, and 10.
- Four of those five BGE hits are expected merge/variant cases; pair 10 is a
  valid boundary that would require clearer wording rather than deletion.
- BGE therefore has `4/4` recall for merge/variant cases in this selected set
  and `4/5` precision when a threshold hit is treated as a merge/variant cue.
- NLI labels seven pairs as `contradiction`, two as `neutral`, and one as
  `entailment`; those labels do not map directly to the four Givn classes.

## Result

The larger corpus changes the scale, not the conclusion:

- E5 identifies 199 strict conflicts, but does not classify them.
- BGE reduces those to 45 raw-logit hits, but still mixes duplicate/variant
  candidates with valid boundaries.
- NLI is faster than BGE on the 199-pair scope and provides richer relationship
  signals, but its labels remain natural-language NLI labels rather than Givn
  duplicate classes.
- The combined Givn-Watn corpus contains no strict cross-project duplicates at
  `0.95`; only nine weaker cross-source retrieval candidates exist.

The correct next modeling step remains a Gherkin-specific classifier or review
stage using E5, BGE, NLI, and deterministic Gherkin facts together. Neither
off-the-shelf pair model should become an automatic merge gate.

The separate Cobertura branch-attribution idea is documented as a future,
optional experiment in [`future-coverage-experiment.md`](future-coverage-experiment.md).
