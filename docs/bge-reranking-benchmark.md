# BGE Reranking Admission Benchmark

> **Status: current benchmark reference.** The current full-corpus evidence is
> the PR #27 run described below. The older combined-corpus measurements remain
> explicitly labeled as historical and are not the current operating point.

This document defines the evidence required before the opt-in BGE admission
filter can be treated as useful. It is not a Givn classifier specification.
The production path remains E5-only when this benchmark does not support the
operating point.

## Reproducible Inputs

The label sample is checked in at
`tools/approach-comparison/bge-reference-labels.json`. It contains the four
required evaluation labels:

- `DUPLICATE`
- `VARIANT`
- `VALID-BOUNDARY`
- `UNRELATED`

Each pair has an invariant group, provenance, an explicit `reference` or
`holdout` split, and a manually adjudicated deterministic disposition flag.
Deterministic pairs are reported as an excluded disposition and are removed
from BGE recall/precision denominators; they are not treated as BGE false
negatives. Pairs from one invariant group are never split across those
measurements. The existing ten manually classified Givn pairs remain
historical reference evidence; the checked-in eight-pair sample supplies the
reproducible reference/holdout fixture and four-class admission measurement.

The comparison tool is run against the copied corpus and local model artifacts:

```sh
tools/approach-comparison/prepare_corpus.sh
SKIP_NLI=1 tools/approach-comparison/run_comparison.sh \
  --candidate-scope bounded \
  --report-name test-corpus/indexes/combined-bge-admission.json
target/release/givn spec bge-parity > /tmp/givn-bge-parity.json
python tools/approach-comparison/evaluate_bge_admission.py \
  --report test-corpus/indexes/combined-bge-admission.json \
  --labels tools/approach-comparison/bge-reference-labels.json \
  --enforce \
  --rust-parity /tmp/givn-bge-parity.json \
  --givn-binary target/release/givn \
  --bge-model "${XDG_CACHE_HOME:-$HOME/.cache}/givn/bge"
```

The `bounded` scope is the product set: the top-K endpoint union intersected
with eligible E5 semantic pairs. The product budget defaults to `2048` and
selects the highest E5-scoring pairs when a configured budget is smaller than
the eligible union. The report still records the full top-K union and its
top-K recall denominator. A separate bounded fixture exercises the configured
prefix and the independent 512/513 tokenizer boundary.
`run_comparison.sh` defaults `HF_HUB_OFFLINE=1`; local E5, BGE, and optional NLI
artifacts must therefore already exist. The command must not download a model
during a test run. A benchmark run may
use the explicitly provisioned local BGE artifact and records the model id,
artifact hashes (including `model.onnx.data` when the ONNX export uses external
weights), tokenizer limit, thread count, and command line in its JSON report.

The comparison report identifies its candidate implementation as
`candidate_pipeline: python-reference-v1`. This is useful measurement evidence
but is not a production approval artifact because the parser is not the Rust
candidate pipeline. `evaluate_bge_admission.py --enforce` accepts only a report
combined with an independently generated JSON attestation from
`givn spec bge-parity`; the attestation supplies the `rust-production-v2`
pipeline fingerprint and exact constants, including the active pair budget.
Both `--enforce` and `--rust-parity` are required before
`givn/.spec-bge-admission.json` can be generated with
`--approval-file`.

## Measurements

The report records these separate sets and never conflates them:

| Set | Meaning |
|---|---|
| E5 raw | All eligible strict E5 semantic pairs at or above `0.9500`, after deterministic exclusions. |
| E5 top-K union | All eligible top-K endpoint-neighbour pairs; this is the recall denominator before bounded intersection. |
| BGE bounded | The intersection of the E5 raw set and the E5 top-K union; this is the only set BGE may score. |
| BGE scored | Unique undirected pairs from the bounded set actually evaluated in both directions. |
| BGE recommended | Scored pairs with symmetric raw logit at least `3.0000`. |
| BGE filtered | Scored pairs below `3.0000`; evidence only and non-blocking. |
| BGE unscored | Eligible E5 raw pairs outside the bounded/scored subset or unavailable because of overflow. |

For each label configuration report:

- top-K recall before BGE admission;
- E5 raw, BGE scored, recommended, filtered, and unscored counts;
- BGE elapsed runtime and directional/symmetric score distribution;
- precision and recall for `DUPLICATE` plus `VARIANT` admission;
- false-positive and false-negative rates against `VALID-BOUNDARY` and
  `UNRELATED` negatives;
- scenario character count, step count, assertion count, E5 score, and BGE
  score.

## Current PR #27 Run

The current full-corpus run is [PR #27](https://github.com/buster/givn/pull/27),
tip `37b27d3`. It uses `394` permanent scenarios and produces `1274` semantic
E5 candidates. The BGE-eligible top-five endpoint union contains `795` pairs.
`GIVN_SPEC_MAX_RERANK_PAIRS` was unset, so the production default `2048`
applied. Both second-stage models ran against real local ONNX artifacts; no BGE
or NLI stub was used.

| Layer | E5 candidates | Eligible pairs | Scored | Recommended | Filtered | Token-cap | Unscored |
|---|---:|---:|---:|---:|---:|---:|---:|
| BGE | 1274 | 795 | 669 | 1 | 668 | 126 | 605 |
| NLI | 1274 | 1274 | 1062 | 17 | 1045 | 212 | 212 |

The BGE and NLI recommendations are independent; their intersection contains
`0` pairs in this run. Full runtime and accounting details are in
[`semantic-duplicates-real-model-filter-report.md`](semantic-duplicates-real-model-filter-report.md)
and [`semantic-duplicates-e5-threshold-vs-serialization-analysis.md`](semantic-duplicates-e5-threshold-vs-serialization-analysis.md).
The manually reviewed top-five outputs are preserved in
[`semantic-duplicates-manual-evaluation.md`](semantic-duplicates-manual-evaluation.md).

`BGE_FILTERED` is not treated as `VALID-BOUNDARY` or `UNRELATED`; those labels
remain benchmark labels only. A BGE score above `3.0000` is not proof of a
Givn class and remains subject to agent classification.

## Admission Acceptance

The operating point is accepted only when the held-out measurement shows all of
the following:

1. at least `0.9000` recall for the combined `DUPLICATE` and `VARIANT` labels
   among pairs present in the E5 top-K set;
2. at least a 25 percent reduction in false-positive admissions compared with
   the E5-only recommended set;
3. at least `0.8000` precision for recommended positive admissions;
4. no more than a 10 percentage-point increase in false negatives; and
5. no labeled positive remains unscored in the bounded admission sample.

Repeated bounded BGE runs also record elapsed runtime for a separate manual
maintainer CPU-budget confirmation. Runtime is evidence, not a configurable
machine acceptance threshold.

These are benchmark acceptance criteria, not runtime configuration knobs. If
any criterion fails, the BGE admission capability is not approved and the
product uses the E5-only path with no BGE evidence object. This approval state
is distinct from a runtime `BGE_UNAVAILABLE` artifact/overflow status. It must
not label candidates `BGE_FILTERED` while claiming an unproven improvement.

## Verbosity-Gaming Hypothesis

The measured product risk is that an LLM learns to lower E5 similarity by
adding unnecessary characters, repeated setup, overloaded steps, or irrelevant
qualifiers. The authoring evaluation therefore compares concise and verbose
versions of the same invariant and records:

- total character count;
- step count;
- assertion count;
- E5 cosine score;
- BGE directional and symmetric raw scores;
- admission status and manual label.

The desired result is that concise invariant-focused scenarios are not
systematically penalized, and that increasing prose solely to lower a score is
observable as authoring debt. Guidance must sharpen the invariant and the
boundary fact instead of adding text as a scoring workaround.

## Historical Baseline (Not Current)

The archived combined-corpus run found `199` strict E5 pairs and `45` BGE hits
at `3.0`, while a larger `1,428`-pair union was too expensive for an unbounded
CPU run. A valid boundary also exceeded `3.0` in the selected sample. Those
measurements use the old 425-scenario corpus and are retained only as historical
evidence. They explain why BGE can reduce a worklist but cannot classify Givn
semantics; they do not describe the current PR #27 run or constitute a current
production pass. See the archived
[`approach-comparison-report.md`](archive/analysis/approach-comparison-report.md)
for the original measurement basis.
