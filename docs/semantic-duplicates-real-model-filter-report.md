# Real-Model Duplicate Filter Report

## Scope

This report uses the current complete semantic serialization and real local
model paths. No embedding, BGE, NLI, score override, or test-pool stub was
used.

The run scanned `395` permanent scenarios after the E5 token-cap change.

```sh
env -u GIVN_SPEC_EMBEDDER target/debug/givn spec index
env -u GIVN_SPEC_EMBEDDER -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER \
  target/debug/givn spec duplicates --format json
```

BGE and NLI were then invoked with the local artifacts:

```sh
GIVN_SPEC_BGE=local \
  target/debug/givn spec duplicates --format json

GIVN_SPEC_RERANKER=nli \
GIVN_SPEC_NLI_MODEL_DIR=/tmp/opencode/nli-model \
  target/debug/givn spec duplicates --format json
```

## Result

| Stage | Result | Count |
|---|---|---:|
| E5 fitting vectors | Indexed successfully | 376 |
| E5 token-cap findings | Complete passages over 512 tokens | 19 |
| Structural findings | Deterministic exact/structural candidates | 4249 |
| E5 semantic candidates | Fitting semantic pair candidates | 2292 |
| BGE filtered candidates | None; BGE did not score | 0 |
| NLI filtered candidates | None; NLI did not score | 0 |

### BGE

```json
{
  "status": "BGE_UNAVAILABLE",
  "reason": "pair-cap",
  "e5_candidates": 2292,
  "unique_pairs": 0,
  "directional_calls": 0,
  "unscored_count": 2292,
  "model_load_attempts": 0
}
```

BGE did not filter Candidates. It correctly refused to construct or run the
model because the bounded semantic pool exceeded the maximum of `512` pairs.

### NLI

```json
{
  "status": "NLI_UNAVAILABLE",
  "reason": "pair-cap",
  "e5_candidates": 2292,
  "unique_pairs": 0,
  "directional_calls": 0,
  "unscored_count": 2292,
  "model_load_attempts": 0
}
```

NLI did not filter Candidates. It also refused inference before model
construction because the same E5 pool exceeded the `512` pair bound.

### Combined

The combined run reports both layers as `UNAVAILABLE` with reason `pair-cap`.
There are no valid BGE/NLI model scores in this run. This is an honest
capacity result, not a quality comparison between the two models.

## E5 Token-Cap Findings

These scenarios were excluded from E5 vector creation and remain visible as
`E5_TOKEN_CAP` refactoring findings. Each requires shortening or splitting and
rerunning retrieval.

| File | Scenario | Tokens | Limit |
|---|---|---:|---:|
| `givn/specs/prose-consolidation/prose-consolidation.feature` | Generated guidance points to the canonical E2E policy | 550 | 512 |
| `givn/specs/semantic-e5-token-cap/semantic-e5-token-cap.feature` | Blocking review keeps an over-cap active scenario unresolved | 536 | 512 |
| `givn/specs/semantic-e5-token-cap/semantic-e5-token-cap.feature` | Duplicate audit preserves deterministic findings and reports an E5 token-cap candidate | 643 | 512 |
| `givn/specs/semantic-e5-token-cap/semantic-e5-token-cap.feature` | Explore keeps an E5 token-cap finding visible while fitting results remain usable | 575 | 512 |
| `givn/specs/semantic-review/semantic-review.feature` | Change review generates a complete report for a semantic candidate | 529 | 512 |
| `givn/specs/semantic-review/semantic-review.feature` | Failed remediation verification does not mutate project evidence | 571 | 512 |
| `givn/specs/semantic-review/semantic-review.feature` | Invalid prior report entries never carry a classification | 609 | 512 |
| `givn/specs/semantic-review/semantic-review.feature` | Removing a DUPLIKAT candidate verifies the projected remediation | 515 | 512 |
| `givn/specs/semantic-token-cap-review/semantic-token-cap-review.feature` | Blocking change review keeps an all-over-cap candidate unresolved | 582 | 512 |
| `givn/specs/semantic-token-cap-review/semantic-token-cap-review.feature` | Permanent duplicate audit keeps partial BGE token-cap evidence per candidate | 564 | 512 |
| `givn/specs/semantic-token-cap-review/semantic-token-cap-review.feature` | Permanent duplicate audit preserves BGE evidence when NLI reaches the cap | 594 | 512 |
| `givn/specs/spec-duplicates/spec-duplicates.feature` | Exact duplicates across features are reported together | 586 | 512 |
| `givn/specs/spec-duplicates/spec-duplicates.feature` | Exact duplicates in one feature are reported together | 570 | 512 |
| `givn/specs/spec-duplicates/spec-duplicates.feature` | Invalid duplicate score overrides are rejected | 891 | 512 |
| `givn/specs/spec-duplicates/spec-duplicates.feature` | JSON output identifies every audit candidate | 565 | 512 |
| `givn/specs/spec-duplicates/spec-duplicates.feature` | Maintainer inspects the complete permanent scenario audit | 588 | 512 |
| `givn/specs/spec-duplicates/spec-duplicates.feature` | Semantically related scenarios are reported as candidates | 545 | 512 |
| `givn/specs/spec-duplicates/spec-duplicates.feature` | Structurally overlapping scenarios retain their distinguishing facts | 667 | 512 |
| `givn/specs/spec-explore-change/spec-explore-change.feature` | Change-aware file exploration finds permanent and active change candidates | 545 | 512 |

## Interpretation

This run cannot answer which model is more accurate. The current complete E5
serialization produces too many semantic pairs for the configured BGE/NLI
bounded pool, so both layers correctly produce zero model scores.

The current blockers are therefore:

1. Refactor the `19` E5-token-cap scenarios so that their complete passages fit.
2. Reduce or improve the semantic candidate pool so it is at most `512` pairs
   before BGE/NLI inference.
3. Rerun the same four reports after both conditions are satisfied.

The report deliberately does not classify any BGE/NLI Candidate as filtered.
`BGE_UNAVAILABLE` and `NLI_UNAVAILABLE` with `pair-cap` are capacity states,
not negative model judgments.
