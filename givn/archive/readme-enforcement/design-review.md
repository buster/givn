# Design Review: readme-enforcement

## Phase 1 — Grilling

Fresh-context grilling subagent over proposal, spec, design, glossary, and the
relevant code. 15 findings (A1–A15), all grounded in file:line evidence.

Findings and resolutions:

| # | Branch | Severity | Resolution |
|---|---|---|---|
| A1 | Missing-README behaviour undefined | high | Decision 1: gate blocks; `MissingReadme` finding; scenarios added |
| A2 | Proposal promised impact-decision nudge, design dropped it | medium | Decision 5: promise cut from proposal |
| A3 | Marker-less README semantic unpinned | medium | Decision 2 (product-owner override): **fails** as `Unmanaged`; scenarios added |
| A4 | `git clone <repo>` legacy matcher false-positives | medium | Decision 4: matcher dropped |
| A5 | Damage-check scope ambiguous vs glossary | medium | Decision 3: development block only; glossary reworded |
| A6 | Absent development block unstated | low | Scenario added (no re-insert while README stays managed) |
| A7 | Unknown `README-IMPACT` value untested | low | Scenario added |
| A8 | Fence-parser duplication | low | Design: reuse `agents_block` primitives |
| A9 | Scaffold fallback rationale inverted | low | Design corrected (init writes commands.yaml first) |
| A10 | Strict-mode line ref + fixture obstacles | low | Fixed; fixture note added |
| A11 | GREEN-first e2e scenarios | low | Accepted; RED evidence comes from the paired refusal scenarios (recorded in tasks) |
| A12 | Instructions `--json` surface | low | Text-only line; JSON untouched |
| A13 | arc42 chapter set | procedural | ch. 05, 06.7, 08, 12 updates recorded for the arc42-docs artifact |
| A14 | ADR qualification | procedural | Five gates pass; routing `CANONICAL_ARTIFACT` — no MADR; canonical homes arc42 ch. 08 + ch. 12 |
| A15 | Terminology drift "managed development block" | low | Standardized on "development managed block" |

## User decisions (one at a time, answered)

1. **Missing README, addon enabled** → block (finding with re-scaffold/migration remediation).
2. **Hand-written README without givn markers** → **fail** as `Unmanaged`; the project owner adapts the README or disables the addon. (Product-owner override of the grilling recommendation.)
3. **Damage-check scope** → development managed block only.
4. **Legacy matcher `git clone <repo>`** → drop.
5. **Instructions surfacing a missing README impact decision** → cut from proposal.

## Phase 2 — Hardening

- proposal.md: impact-nudge promise removed; unmanaged/migration semantics stated.
- design.md: `Unmanaged`/`MissingReadme` kinds, management-marker definition, legacy list corrected, `agents_block` reuse, scaffold-order correction, decisions log, fixture note, terminology unified.
- specs/readme-enforcement/readme-enforcement.feature: five scenarios added (missing README check + archive, unmanaged check + archive, no-reinsert pinning, unknown impact value).
- docs/arc42/12-glossary.md: README check scope reworded; README migration covers hand-written READMEs.
- `givn lint --change readme-enforcement`: exit 2 — @wip warnings only, no parse errors, no duplicates.

E2E tags intact; all five inventory entries have exactly one matching `@e2e`
scenario and one matrix row.

DESIGN-REVIEW: PASS
