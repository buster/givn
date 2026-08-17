# Design Review: semantic-remediation

## Scope

The proposal, feature delta, PR 4 plan, design, and Arc42 marker describe one
slice: verify that an agent's PR 3 semantic classification has produced the
required projected remediation. Automatic classification, automatic edits,
permanent-permanent auditing, BGE/NLI, and the advisory review path remain out
of scope.

## Grilling Results

| Branch | Result |
|---|---|
| Scope | PASS. The proposal and feature cover duplicate removal, variant parameterization/consolidation, valid-boundary distinction, stale evidence, non-mutation, and CLI acceptance without adding a new user command. |
| Technology choices | PASS. Reusing the existing E5/Fingerprint evidence, strict report parser, permanent-only index, and in-memory projected tree is simpler and deterministic; no dependency is added. |
| Missing scenarios | PASS after hardening. Added endpoint disappearance, below-boundary recomputation, wrong score-only remediation, exact-boundary blocking, stale identity, failed-verification non-mutation, and class-specific negative/positive scenarios. |
| Testability | PASS. Every regular and E2E scenario can start RED with strict `unimplemented!()` bindings; Then steps assert exact exit/output/report state and projected remediation contracts. |
| E2E fidelity | PASS. One inventory entry maps to one CLI E2E scenario. The scenario prepares a classified/remediated precondition, then invokes the compiled CLI subprocess and asserts its exit status, receipt, and report status. |
| Interaction coverage | PASS. The single inventory row, matrix row, E2E scenario, and `std::process::Command` driver match. |
| Risk | PASS after hardening. The critical risk was silently treating a disappeared or below-boundary candidate as resolved. A separate prior-entry verification path now recomputes such pairs independently of report admission. |
| Arc42 | PASS. Chapters 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, and 12 are affected and updated; chapter 7 is explicitly unaffected. ADR-047 and risks R-44/R-45 record the new projected-tree boundary. |

## Hardening Decisions

The fresh independent audit identified these issues and they were resolved in
the artifacts before tasks:

- Disappeared candidates are handled by `PriorRemediationEntry` matching with
  exact identity/hash, changed-evidence, disappearance, and no-match branches.
- Below-boundary pairs are recomputed by a separate prior-pair path instead of
  being inferred from the regenerated candidate list.
- `VARIANTE` requires one projected parameterized `Scenario Outline` with a
  non-empty `Examples` table; a score-only reduction with two independent
  scenarios remains blocked.
- `VALID-BOUNDARY` requires both endpoints, a changed observable deterministic
  fact or explicit condition/outcome invariant, and a score strictly below
  `0.9500`; a score exactly at the boundary remains blocked.
- The E2E scenario is explicitly a real CLI verification smoke test with a
  prepared classified/remediated precondition, not a claim that classification
  and editing themselves occur through a second CLI interaction.
- Change-specific runner documentation distinguishes the active delta path from
  the repository runner's permanent-spec path; archive post-merge verification
  covers the merged permanent feature.

## Arc42 Assessment

All twelve chapter files exist and contain project-specific content. No new
deployment or external dependency is introduced. The updated durable chapters
describe projected-tree verification, strict prior-entry matching, class
contracts, false score-only remediation, and the new receipt. No ASCII-art
diagram is introduced; diagrams remain Mermaid.

## Verification

`givn lint --change semantic-remediation` reports only the intentional `@wip`
findings and no parse/structure errors. The runner is the existing Cucumber-rs
`features_runner`, configured with `.fail_on_skipped()`. The exact single-scenario
command, change runner, E2E runner, and CLI subprocess driver are recorded in
`design.md`.

DESIGN-REVIEW: PASS
