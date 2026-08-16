# Cross-Encoder Benchmark

## Scope

This report compares the branch `cross-encoder-reranker` with baseline commit
`38c2f22`, which uses the independent embedding score for semantic review. No
permanent `.feature` scenario was changed during the benchmark.

The reranker branch uses:

- the existing `multilingual-e5-large` passage embeddings for candidate
  retrieval;
- top five embedding neighbours per scenario;
- `fastembed::TextRerank` with `RerankerModel::BGERerankerV2M3`;
- both pair directions, averaged as a raw logit;
- a branch-fixed raw-logit boundary of `3.0`;
- no project, CLI, environment, or review-disposition override.

The model and API choice was checked against the locked FastEmbed-rs source and
the Context7 FastEmbed-rs documentation on 2026-08-16. The reranker is a
multilingual relevance model, not a model trained specifically for duplicate
versus boundary classification.

## Measurements

| Measurement | Embedding baseline | Reranker branch |
|---|---:|---:|
| Permanent scenarios | 208 | 208 |
| Permanent embedding pairs `>= 0.95` | 158 | not used as gate |
| Top-K candidate pairs | all threshold pairs | 697 in standalone probe; 839 in integrated review with delta candidates |
| Reranker model | none | `rozgo/bge-reranker-v2-m3` |
| Integrated review duration | 15.96 s baseline path | 750.67 s with real reranker and active change |
| Reranker raw boundary | n/a | `3.0` |
| Integrated candidates above boundary | n/a | 7 |

The standalone real-model probe used 208 permanent scenarios, top-K `5`, and
697 de-duplicated candidate pairs. It completed in `588.07 s`. The integrated
review additionally scored current-change candidates and completed in
`750.67 s`.

## Known Pair Scores

Scores are symmetric averages of both query/document directions.

| Pair | Raw logit | Interpretation |
|---|---:|---|
| Placeholder COMMIT hash pair | `3.427780` | Clear duplicate candidate. |
| Uninitialized `status` versus `new` pair | `6.198650` | Same invariant with command variants; merge into one parameterized scenario or shared behavior. |
| Plain parse error versus JSON parse error | `1.354947` | Same parse-error invariant with output variants; merge into one scenario outline/Examples table. |
| Skipped-artifact status pair | `1.071740` | Same status invariant with text/regression variants; merge or consolidate rather than keep two overlapping scenarios. |

The false-positive ordering is the critical result: the valid `status`/`new`
boundary scores substantially higher than the known duplicate. Raising the
boundary above `3.43` would remove the known duplicate, but lowering it to keep
that duplicate would retain the valid boundary and other command variants.

## Assessment

The reranker reduces the number of final candidates compared with the full
embedding threshold. The initial interpretation of six candidates as valid
boundaries was wrong: after reviewing the actual invariants, the integrated
seven-candidate set consists of duplicate or same-invariant command/input
variants that should be merged or parameterised. No candidate in that sample
is an established valid independent boundary.

The branch is promising but does not yet demonstrate a safe replacement for the
strict embedding gate. The reranker surfaces the right merge queue in this
sample, but the root cause remains: a relevance reranker answers whether two
texts are related, not whether they express the same executable invariant. The
manual merge/variant labels need a larger corpus before adoption.

## Decision

Keep the branch as the comparison artifact and do not close the experiment as
production-ready yet. A subsequent evaluation needs either:

1. a cross-encoder trained or calibrated specifically for duplicate/variant/
   boundary classification with a larger labeled set; or
2. a separate deterministic/LLM-assisted classification stage after retrieval,
   with explicit evidence that it does not silently turn a high-similarity pair
   into an accepted exception.
