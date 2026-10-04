# arc42 Documentation Update: release-truth

## Scope and evidence boundary

This assessment records the approved proposal, the authored delta, and the
design. Architecture updates describe the target as pending implementation;
they do not assert that the new workflow, guard script, or selection rules are
deployed or tested. Completing this documentation artifact is not completing
the feature.

Read inputs: `proposal.md`, the delta feature and the delta use-case snapshot
under `specs/`, `design.md`, `design-review.md`, the resolved arc42-docs
instruction, independently derived chapter rows from FreshPlanGrill
(`agent://FreshPlanGrill`), all twelve durable chapter subjects, and the ADR
register search.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goal, stakeholder, or quality attribute changes; release truthfulness is already the `publish-a-release` success guarantee. | Unchanged. |
| 2 | 02 architecture-constraints | No | No constraint on the product or binary; GitHub release only / no crates.io is a release-pipeline fact owned by chapter 07. | Unchanged. |
| 3 | 03 context-and-scope | No | No product boundary or external entity changes; GitHub Actions and Releases are repository machinery, not part of the CLI's context. | Unchanged. |
| 4 | 04 solution-strategy | No | Configuration and workflow rules, not an architectural direction. | Unchanged. |
| 5 | 05 building-block-view | No | `src/**` is untouched; no production module or component changes. | Unchanged. |
| 6 | 06 runtime-view | No | No product runtime flow changes; the release flow is deployment automation. | Unchanged. |
| 7 | 07 deployment-view | Yes | 7.1/7.5 described an intended pipeline; the change records the actual single dispatch-triggered workflow, the guard script, GitHub releases as the only channel, and the selection rules. | Rewrote 7.1's distribution sentence and 7.5 to name `.github/workflows/release.yml`, `scripts/release-guard.sh`, `cliff.toml`, and the remaining unwired channels. |
| 8 | 08 crosscutting-concepts | No | No product-level error-handling, security, or configuration concept changes; installing git-cliff in CI is build tooling. | Unchanged. |
| 9 | 09 architecture-decisions | No | No candidate qualifies for an ADR; all four route to canonical artifacts (see verdict below). | No ADR, amendment, or supersession. |
| 10 | 10 quality-requirements | No | No new product-level quality scenario; the release guard is release-pipeline policy expressed by the `publish-a-release` rules and delta scenarios. | Unchanged. |
| 11 | 11 risks-and-technical-debt | Yes | Durable accepted limitations belong in the register: exact-subject dedupe can merge two changes or list one change once per group, and the base guard refuses every release until the current version carries a matching tag. | Added R-75 and R-76; amended D-2 to record the wired GitHub release channel. |
| 12 | 12 glossary | Yes | New durable vocabulary for the release pipeline alongside the existing `Release note` term. | Added `Changelog`, `Release section`, `Release base tag`, and `Release revision`. |

## Affected files

- `docs/arc42/07-deployment-view.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`

Only text in existing sections changed; existing Mermaid diagrams and
unrelated architecture content remain unchanged.

## ADR qualification verdict

ADR action: CANONICAL_ARTIFACT

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "NOT_REQUIRED",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "docs/arc42/07-deployment-view.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Proposal D1 records dispatch-only versus keeping the tag-push trigger; design.md D1 records the chosen option and the rejected one.",
      "Proposal D2 records refusing until the current version is tagged versus generating from the older tag versus a recorded baseline commit."
    ],
    "architectural_impact": [
      "No component, ownership, dependency direction, data boundary, or public product contract moves. The affected artifact is the repository's release operations (deployment view) plus one configuration file and one guard script."
    ],
    "durable_consequence": [
      "Changing the release entry point later means re-adding and re-verifying a second path and re-running the release procedure; the decision constrains how every future release is produced.",
      "The base guard makes every future release depend on a matching version tag existing."
    ],
    "lower_level_artifact": [
      "The complete rationale is owned by the deployment view (release operations) and the publish-a-release process contract; the selection rules are owned by the code in cliff.toml; the guard predicates are owned by scripts/release-guard.sh. Falsification test 3 (product-behavior exclusion) does not apply because the decision is not product-workflow behavior; tests 1 and 2 classify it as release operations, not a product component boundary."
    ],
    "existing_adr_check": [
      "Searched docs/arc42/09-architecture-decisions.md and docs/arc42/adr (including historical storage) for release, changelog, distribution, and pipeline decisions; no related record exists.",
      "D-2 in chapter 11 already tracks the unwired distribution channels; this change amends it rather than adding an ADR."
    ]
  }
}
```

The three further candidates in design.md's classification table (A1
selection rules, D2 base-tag guard, A2 guard script) are `CANONICAL_ARTIFACT`
as well, each with exactly one destination: `cliff.toml`,
`givn/specs/publish-a-release/usecase.md`, and `scripts/release-guard.sh`.

## Verification and remaining lifecycle work

- Guard predicates and selection rules carry RED recipes in design.md; the
  workflow wiring's residual verification bound is the recorded operator
  decision (design-review.md Q2).
- Documentation completeness is assessed here; implementation, strictness
  probes, scenario execution, the `@e2e` transcript, task commits, and
  archive proof have not occurred.
- The independent chapter derivation (FreshPlanGrill) selected 07, 11, and 12
  as affected; this artifact and the durable edits match that derivation.

## Status

STATUS: DONE
