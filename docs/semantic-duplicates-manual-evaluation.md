# Manual Semantic Evaluation

> **Status: current local evaluation record.** This preserves the manual
> interpretation of the current PR #27 model outputs. The raw JSON reports named
> below are local, ephemeral, and intentionally not versioned; their paths are
> provenance references, not repository artifacts.

## Session 2026-08-22

This record stores the manual review of the five highest-scoring BGE pairs and
the five highest-scoring NLI pairs from the real local-model run in PR #27.

Run source:

- Pull request: `https://github.com/buster/givn/pull/27`
- Run tip: `37b27d3` (`feat(retrieval): configure E5-ranked reranker pair budget`)
- Run date: `2026-08-21`
- Manual review date: `2026-08-22`
- E5 candidate pool: `1274` semantic candidates from `394` permanent scenarios
- Pair budget: `2048`
- Evaluation method: one independent read-only Plan-mode subagent per pair;
  no files were changed by the subagents

The user reviewed each subagent classification. The phrase `subagent correct`
means that the user accepted both the proposed domain classification and the
interpretation of the model admission status.

## BGE Pairs

The BGE score is the symmetric raw logit. `BGE_RECOMMENDED` uses the inclusive
operating point `3.0000`; lower scored pairs are `BGE_FILTERED`.

| ID | Feature | Scenario A | Scenario B | E5 | BGE symmetric | Audit status | Subagent class | User verdict |
|---|---|---|---|---:|---:|---|---|---|
| BGE-1 | `arc42-review-alignment` | `Arc42-enabled review instructions require conformance evidence` | `The review template includes the Arc42 conformance block` | 0.9727 | 3.0486 | `BGE_RECOMMENDED` | `VALID-BOUNDARY` | subagent correct |
| BGE-2 | `givn-workflow-enforcement` | `Archive blocked when merged permanent specs fail verify` | `Archive re-runs verify against merged permanent specs and passes` | 0.9823 | 2.8264 | `BGE_FILTERED` | `VALID-BOUNDARY` | subagent correct |
| BGE-3 | `spec-duplicates` | `Rule-nested permanent scenarios are included in the audit` | `The audit reports candidates beyond the interactive top five` | 0.9569 | 2.6685 | `BGE_FILTERED` | `VALID-BOUNDARY` | subagent correct |
| BGE-4 | `bge-reranking-evidence` | `Approval writer rejects a missing Rust parity attestation` | `Approval writer requires enforcement for any approval output` | 0.9876 | 2.6204 | `BGE_FILTERED` | `VALID-BOUNDARY` | subagent correct |
| BGE-5 | `spec-duplicates` | `An audit with no candidates reports an empty result` | `Rule-nested permanent scenarios are included in the audit` | 0.9579 | 2.5826 | `BGE_FILTERED` | `UNRELATED` | subagent correct |

Feature files:

- BGE-1: `givn/specs/arc42-review-alignment/arc42-review-alignment.feature`
- BGE-2: `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature`
- BGE-3 and BGE-5: `givn/specs/spec-duplicates/spec-duplicates.feature`
- BGE-4: `givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature`

## NLI Pairs

The NLI score is the symmetric entailment/neutral/contradiction probability
vector. `NLI_RECOMMENDED` uses symmetric entailment `>= 0.5000`.

| ID | Feature | Scenario A | Scenario B | E5 | NLI symmetric E/N/C | Audit status | Subagent class | User verdict |
|---|---|---|---|---:|---|---|---|---|
| NLI-1 | `validate-change-id-input` | `Consumer invokes givn status with a path traversal change id and gets a clear error` | `Path traversal id is rejected by status` | 0.9748 | 0.8394 / 0.0929 / 0.0677 | `NLI_RECOMMENDED` | `DUPLIKAT` | subagent correct |
| NLI-2 | `validate-change-id-input` | `Consumer invokes givn status with a path traversal change id and gets a clear error` | `Path traversal id is rejected by check` | 0.9648 | 0.8228 / 0.1101 / 0.0671 | `NLI_RECOMMENDED` | `VARIANTE` | subagent correct |
| NLI-3 | `validate-change-id-input` | `Consumer invokes givn status with a path traversal change id and gets a clear error` | `Path traversal id is rejected by instructions` | 0.9645 | 0.8123 / 0.1042 / 0.0835 | `NLI_RECOMMENDED` | `VARIANTE` | subagent correct |
| NLI-4 | `validate-change-id-input` | `Consumer invokes givn status with a path traversal change id and gets a clear error` | `Path traversal id is rejected by archive` | 0.9599 | 0.8036 / 0.1053 / 0.0912 | `NLI_RECOMMENDED` | `VARIANTE` | subagent correct |
| NLI-5 | `validate-change-id-input` | `Consumer invokes givn status with a path traversal change id and gets a clear error` | `Path traversal id is rejected by lint` | 0.9515 | 0.7959 / 0.1133 / 0.0908 | `NLI_RECOMMENDED` | `VARIANTE` | subagent correct |

Feature file for all NLI pairs:

- `givn/specs/validate-change-id-input/validate-change-id-input.feature`

## Review Summary

- Accepted subagent classifications: `10/10`.
- BGE admission decisions accepted: `1/1` recommended and `4/4` filtered.
- NLI admission decisions accepted: `5/5` recommended.
- Domain classifications: `1 DUPLIKAT`, `4 VARIANTE`, `4 VALID-BOUNDARY`,
  `1 UNRELATED`.
- The five NLI pairs are one `validate-change-id-input` invariant family, not
  five independent samples.
- This is qualitative ranking evidence, not a complete precision/recall
  benchmark for the full candidate pool.

The raw reports used for the review are local and ephemeral:

- `/tmp/opencode/givn-duplicates-bge-2048.json`
- `/tmp/opencode/givn-duplicates-nli-2048.json`

NLI-2 deliberately records the pair from the current run (`status` versus
`check`). Older questionnaire documents contain a different historical pair and
must not be substituted for this evaluation.

## Related Current Evidence

- [`semantic-duplicates-real-model-filter-report.md`](semantic-duplicates-real-model-filter-report.md)
- [`semantic-duplicates-e5-threshold-vs-serialization-analysis.md`](semantic-duplicates-e5-threshold-vs-serialization-analysis.md)
- [`bge-reranking-benchmark.md`](bge-reranking-benchmark.md)
