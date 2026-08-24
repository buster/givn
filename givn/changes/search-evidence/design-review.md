# Design Review: search-evidence

## Grilling summary

A fresh-context grilling subagent read the proposal, spec delta, design, the
runner matrix, and the retrieval implementation. Findings and resolutions:

| # | Severity | Finding | Resolution |
|---|---|---|---|
| 1 | Blocker | Scenarios in the `.feature` lacked the runner tags (`@bge`, `@nli`, `@nli-combined`) the design's tagging matrix declared; without them the scenarios would run under the E5-only binary and fail. | Tags added to every scenario; the tagging matrix and the feature file now agree. |
| 2 | Blocker | The stub rerankers cannot reach the admission thresholds on the fixture corpus (`stub_reranker_score` tops out below 3.0; NLI stub below 0.5 entailment), so "recommended" verdicts were unreachable. | Deterministic directional fixtures: `BGE evidence recommends all listed candidates` (`GIVN_SPEC_BGE_DIRECTIONAL_SCORES`, 3.1000 both directions on all three fixture pairs) and `NLI evidence recommends all listed candidates` (`GIVN_SPEC_NLI_DIRECTIONAL_SCORES`, 4.0/0/0 logits both directions). Documented in design.md with the exact endpoint identities. |
| 3 | Major | Design named the budget step `is 2` but the spec used `is 1`. | Spec aligned on the existing `the second-stage pair budget is 2` step. |
| 4 | Major | "Same mechanism as `search_retrieval`" only forwards the embedder and would silently leave BGE/NLI backends unset. | Design now mandates assembling the full `retrieval_*` world state like `user_runs` in `tests/steps/mod.rs`. |
| 5 | Major | The `BGE evidence: unavailable` line was specified only for the no-run case, omitting run-present-but-unavailable (`BGE_UNAVAILABLE`, e.g. missing artifacts or all-over-cap). | Output-mapping table now covers three run states (no run, enabled, unavailable) and the exact lines each prints. |
| 6 | Minor | No NLI token-cap scenario for search. | Added `An over-limit search pair receives NLI token-cap evidence` (`every NLI pair exceeds the token limit` → `GIVN_SPEC_NLI_TOKEN_BUDGET=all-513`, tag `@nli`). |
| 7 | Minor | ADR qualification. | Not qualified: five-gate verdict fails `architectural_impact`/`durable_consequence`/`lower_level_artifact` — the evidence contract is already owned by ADR-052/058/063; surface enumerations amended instead. No new MADR. |
| 8 | Nit | Design said E2E runs the "installed" CLI binary. | Corrected: `CARGO_BIN_EXE_givn` subprocess from the test build. |

## Coverage confirmation

- Interaction inventory (one entry) matched 1:1 by the `@e2e` scenario and the
  Interaction Coverage Matrix row (CLI subprocess driving mechanism named).
- Every scenario can fail in RED: unimplemented When step +
  `.fail_on_skipped()`; all asserted substrings are byte-reproducible from the
  locked output contract.
- `givn lint --change search-evidence` exits 2 with only the expected `@wip`
  findings.
- E2E strictness: the suite's `.fail_on_skipped()` applies; the e2e scenario
  runs under the `@nli-combined` feature set in `run-tests.sh --e2e`.

## Sign-off

DESIGN-REVIEW: PASS