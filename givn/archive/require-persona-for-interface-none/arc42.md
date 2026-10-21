# arc42 Documentation Update: require-persona-for-interface-none

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No new goal, stakeholder, requirement, or quality goal; the change tightens an existing review gate. | |
| 2 | 02 architecture-constraints | No | No new technical, legal, or organizational constraint; the rule stays inside the existing integrity mandatory check. | |
| 3 | 03 context-and-scope | No | No new external system or user-facing surface; `givn check review` keeps its name, arguments, and exit contract. | |
| 4 | 04 solution-strategy | No | No change to the solution strategy; the existing review-gate strategy and the conditional visual evidence strategy are reused. | |
| 5 | 05 building-block-view | Yes | The `integrity_hook` responsibility changes: the Persona disposition follows a declared contract, while the verdict and evidence stay conditional on `web` and `terminal`. | Updated the `integrity_hook` row to state the contract-based Persona trigger and the interface-based verdict and evidence. |
| 6 | 06 runtime-view | Yes | The `givn check review` flow changes for a declared `Interface: none` contract and the failure order differs from `web` and `terminal`. | Rewrote the visual-gate paragraph in 6.6 and added a Mermaid branch diagram: absent section exempt; declared `none` requires Persona only; `web`/`terminal` require verdict, Persona, and evidence. |
| 7 | 07 deployment-view | No | No deployment, packaging, target-environment, or runtime-footprint change. | |
| 8 | 08 crosscutting-concepts | Yes | The contract-line semantics and the Persona identity rule change: `Interface: none` is a declared contract, and only an absent section is exempt. | Updated 8.17 and 8.22 so the section's presence is the Persona boundary, `none` keeps only the disposition, and a reasoned none is valid only for a no-contract change. |
| 9 | 09 architecture-decisions | Yes | The register's Persona-trigger narrative must not imply the old `Interface: none` exemption. | Added one register note: every declared contract requires a Persona; the candidate is `NOT_QUALIFIED`/`CANONICAL_ARTIFACT`, so no ADR file or amendment is added. |
| 10 | 10 quality-requirements | Yes | The gate quality scenario changes and a new passing scenario exists for `Interface: none`. | Broadened Q-89 to every declared contract and added Q-119: `Interface: none` with a confirmed Persona passes without a visual verdict or evidence. |
| 11 | 11 risks-and-technical-debt | Yes | The declared-`none` bypass closes, but the omitted-section exemption and the misclassification risk remain. | Updated R-59: the contract-based Persona rule closes the declared-`none` path; a screen omitted from the contract remains the residual misclassification risk. |
| 12 | 12 glossary | Yes | The terms `Visual Design Contract` and `Persona disposition` and the visual-review invariants now follow the declared contract, not the rendered interface. | Updated both term definitions and the verdict and Persona invariants; no new term is introduced. |

## ADR qualification

ADR action: CANONICAL_ARTIFACT
ADR target: `visual-review-gate` capability (`givn/specs/verify-change/visual-review-gate.feature`)

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: visual-review-gate capability (givn/specs/verify-change/visual-review-gate.feature)
target_adr: null
replacement_adr: null
```

Evidence:

- **Real alternatives — PASS.** The proposal's D1 records three materially
  different options: the contract-based requirement, the interface-based
  status quo, and an every-change requirement. Each carries a gain and a cost.
- **Architectural impact — FAIL.** No component, ownership, dependency,
  deployment, or data boundary moves. `check_visual_review` stays inside the
  existing integrity member, `check_persona_disposition` keeps its signature
  and resolution rule, and `givn check review` keeps its arguments and exit
  contract. ADR-028's boundary is untouched: the integrity member is still
  non-deactivatable and still owns the Persona requirement. What changes is
  the trigger condition inside that member — which declarations it applies
  to — and that is observable gate behavior.
- **Durable consequence — FAIL.** Reversal is one control-flow branch plus
  three error strings and guidance/template text. There is no stored data, no
  migration, and no cross-command coordination. The decision is fully
  reversible within this change and touches one component, so the
  single-change-reversibility falsification test applies.
- **Lower-level artifact — FAIL.** The complete rationale is owned by the
  `visual-review-gate` capability (its pass and fail scenarios define exactly
  which declarations require which markers), the design's behavior table, and
  the current-fact statements in chapters 6, 8, 10, 11, and 12. The
  product-behavior exclusion applies: the rule decides observable review
  pass/fail, and it moves no authority or deployment boundary.
- **Existing-ADR check — PASS.** The active and archived registers were
  searched. ADR-028 owns the mandatory-check boundary and records four prior
  visual/persona amendments, each of which added a member or a resolution
  rule. This change adds no member and removes none; it leaves ADR-028's
  requirement (a confirmed Persona) and its non-deactivatable status intact
  and only widens the trigger that selects changes for it. The complete rule,
  including the `Interface: none` branch and its failure messages, is
  observable behavior owned by the capability and the review instruction.
  Amending ADR-028 would duplicate that contract, so neither `NEW_ADR` nor
  `AMEND_ADR` applies, and no duplicate record exists.
- **Must be shared — SUPPORTING.** Every reviewer must know that
  `Interface: none` no longer exempts the Persona disposition, but shared
  knowledge alone does not qualify a candidate.

Falsification result: the candidate fails the single-change-reversibility
test and the product-behavior exclusion. No ADR file is created and no
existing ADR is amended.

## Notes

- The deliberate exemption is preserved: a change that declares no
  `## Visual Design Contract` section remains exempt from the Persona
  disposition, the visual verdict, and visual evidence. Only an absent
  section is exempt.
- Residual risk: a change that visibly adds a screen but omits the contract
  section is still treated as non-visual. R-59 records it; closing it would
  require deriving the contract from changed files, which this change places
  out of scope.
- The new runtime diagram is Mermaid.
- No glossary term is added; two existing terms are updated.

## Status

STATUS: DONE
