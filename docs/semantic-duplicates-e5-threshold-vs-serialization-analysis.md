# E5 Threshold Versus Serialization Analysis

> **Status: current.** This analysis describes the current 394-scenario PR #27
> run with 1,274 E5 candidates and the default second-stage budget of 2,048.
> Historical 425-scenario and 199-pair measurements are retained in the archive
> and are not substituted for this operating point.

## Current Compact-Serialization Pool

The current real no-stub run uses the compact complete serialization and scans
`394` permanent scenarios. It produces `1274` semantic E5 candidates at the
inclusive `0.9500` boundary. The refreshed E5 index contains `394` fitting
vectors and `0` E5 token-cap findings.

The observed E5 score distribution is:

| E5 threshold | Semantic candidates |
|---:|---:|
| `0.9500` | 1274 |
| `0.9550` | 924 |
| `0.9600` | 628 |
| `0.9650` | 428 |
| `0.9700` | 235 |
| `0.9750` | 118 |
| `0.9800` | 50 |
| `0.9850` | 15 |
| `0.9900` | 4 |

The E5 boundary remains `0.9500`. Raising it would remove candidates without
manual duplicate/variant/boundary labels and would therefore be an
uncontrolled recall loss.

## Second-Stage Budget Result

The fixed all-or-nothing `512` pair refusal has been replaced by the positive
runtime setting `GIVN_SPEC_MAX_RERANK_PAIRS`. Unset means `2048`. Candidates
are ordered by E5 score descending before each model receives its configured
prefix.

| Run | E5 candidates | Eligible prefix | Scored | Recommended | Filtered | Token-cap | Unscored |
|---|---:|---:|---:|---:|---:|---:|---:|
| BGE | 1274 | 795 top-five union | 669 | 1 | 668 | 126 | 605 |
| NLI | 1274 | 1274 independent pool | 1062 | 17 | 1045 | 212 | 212 |
| BGE + NLI | 1274 | independent per-layer prefixes | 669 / 1062 | 1 / 17 | 668 / 1045 | 126 / 212 | 605 / 212 |

The `2048` pair budget does not truncate the current E5 candidate count. BGE
still has its existing top-five endpoint-neighbour eligibility rule, so `479`
E5 pairs remain outside the BGE union. The BGE unscored accounting also
includes `126` selected pairs that exceed the separate BGE 512-token input
limit. NLI selects all `1274` E5 candidates under the pair budget; its `212`
unscored candidates are selected pairs with `NLI_TOKEN_CAP` evidence.

The pair budget and the model-token limit are separate controls. A budget tail
is `BGE_UNSCORED` or `NLI_UNSCORED`; a selected pair over 512 tokens is
`BGE_TOKEN_CAP` or `NLI_TOKEN_CAP`. Neither state is model-filtered evidence.

## Runtime Measurement

The real local-model run was executed with four intra-op threads:

| Run | Wall time | Model runtime | Maximum RSS |
|---|---:|---:|---:|
| BGE-only | `821.60 s` | `819.783 s` | `2.01 GiB` |
| NLI-only | `770.13 s` | `768.072 s` | `2.20 GiB` |
| BGE + NLI | `1586.62 s` | `808.089 s` + `775.016 s` | `2.62 GiB` |

The combined run takes approximately `26:27 min` wall time. It confirms that
BGE and NLI run sequentially and independently. The default `2048` setting is
high enough for this corpus but remains configurable for larger corpora and
different CPU budgets.

## Serialization Signal

The previous complete-serialization measurement used a different corpus state
and reported `2292` candidates from `395` scenarios. That result is retained in
repository history only as prior evidence; it is not a current operating-point
measurement.

The current serializer omits empty optional sections and fields while retaining
all non-empty Feature/Rule context, Backgrounds, descriptions, steps, tables,
docstrings, outline templates, and Examples. The candidate count is therefore
an interaction between current corpus contents, exact serialization, and the
fixed E5 model; it is not evidence for changing the threshold.

## Decision

Keep the E5 boundary at `0.9500`. Use `GIVN_SPEC_MAX_RERANK_PAIRS=2048` as the
default second-stage work budget, select highest E5 scores first, retain the
omitted tail as explicit unscored evidence, and continue recording actual
BGE/NLI runtime before changing the setting or the model-input boundary.

## Related Current Evidence

- [`semantic-duplicates-real-model-filter-report.md`](semantic-duplicates-real-model-filter-report.md)
- [`semantic-duplicates-manual-evaluation.md`](semantic-duplicates-manual-evaluation.md)
- [`bge-reranking-benchmark.md`](bge-reranking-benchmark.md)
