# arc42 Documentation Update: keep-frozen-verifier-out-of-the-trial

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No new goal, stakeholder, or quality attribute: the change makes an existing success guarantee (the builder reconstructs from the Specification) true of the trial directory. | |
| 2 | 02 architecture-constraints | Yes | TC-46 and TC-47 state the partition and the re-check, and both change; the frozen placement needs its own constraint row. | TC-46 partitions into non-frozen retained roles, kept source, replaced, the frozen `e2e` package, and outside-the-declaration paths. TC-47 gains the trial-side frozen refusal. New TC-51 states the frozen set, the report line, `excluded_roles`, and the compatibility rule. |
| 3 | 03 context-and-scope | Yes | The `givn ise replace` and `givn ise verify` interface rows change: one states the frozen package's absence and its report line, the other the trial-side refusal. | The replace row names the frozen `e2e` package and the conditional `Frozen package` line; the verify row names the refusal of a trial that holds a frozen path. No new external system, command, or record file appears. |
| 4 | 04 solution-strategy | No | No strategy change; the exclusion reuses the existing `e2e_file_set` resolver and the existing partition, and the verifier reuses the existing checkout comparison. | |
| 5 | 05 building-block-view | Yes | The `ise` component's responsibility and the `cli::ise` report surface both change. | The `ise` row names the five placements, the `excluded_roles` record field, and the trial-side refusal; the `cli::ise` row gains the optional frozen-package line. |
| 6 | 06 runtime-view | Yes | The replace partition, the dry run, the report, and the verify sequence change. | §6.21 gains the frozen-set partition step, the copy-set refusal over the frozen paths, the frozen report line, and a verify step that refuses a trial holding a frozen path before any identity comparison. |
| 7 | 07 deployment-view | No | No deployment, packaging, or runtime-environment change; no new dependency. | |
| 8 | 08 crosscutting-concepts | Yes | §8.29 owns the trial record's current facts, and they change: a new field, a new trial-content rule, and a new verification refusal. | §8.29 adds `excluded_roles` to the field table, states that the frozen package is in no placement bucket, states the conditional report line, and states when the verifier refuses a trial that holds a frozen path. |
| 9 | 09 architecture-decisions | Yes | The decision to keep the frozen package out of the trial qualifies as an independent architectural decision with a durable compatibility consequence. | New ADR-078 in `docs/arc42/adr/`; the chapter-09 register gains its row and the count moves to 30, with the permanent register scenario and its binding reconciled in this change; ADR-076 links ADR-078 for the added field. |
| 10 | 10 quality-requirements | No | No new quality scenario or attribute; the existing clean-room handover guarantee is enforced more strictly, and R-154 records the cost rather than a new scenario. | |
| 11 | 11 risks-and-technical-debt | Yes | The lost local acceptance signal is an actionable, unresolved downside of the exclusion, and ADR-078 records it as a "Bad, because" consequence. | Added R-154 with its mitigation and its accepted residual. |
| 12 | 12 glossary | Yes | Two existing entries state facts that change (`Trial record`, `Trial directory`) and the term the report line and the rules use is new. | Added the `Frozen package` entry, updated `Trial record` for `excluded_roles`, and updated `Trial directory` to exclude frozen paths; chapter 12 remains the pointer to `docs/glossary.md`. |

## ADR qualification

```text
Architectural commitment: yes — it fixes what the trial directory contains for
  every Ise Test, so it moves the authority boundary between the builder and
  the frozen acceptance evidence, and it moves a compatibility boundary (the
  record field set and what an earlier reader accepts).
Real alternatives: copy the package and rely on the Spec gap review; replace it
  with a note or a path listing; record the exclusion versus bump the scope to
  ise-trial/v2 versus add no field; refuse a trial that holds a frozen path
  versus make it a preparation-only property (proposal D1, D4, D5).
Durable consequence: reversal requires re-running every open Ise Test, undoing
  a record field that readers and the verifier consult, and re-establishing the
  frozen-boundary guarantee; the field outlives the change.
Existing-decision search: ADR-076 owns the trial record's shape, placement, and
  scope rule; ADR-010 owns the declaration it reads; ADR-070 owns the change
  verification receipt; ADR-071 owns the SCM seam; no active or archived record
  owns what the trial directory contains.
ADR action: NEW_ADR
ADR target: docs/arc42/adr/ADR-078-frozen-package-outside-the-trial.md
```

## Notes

`add-delivery-identity-to-trial-record` is in review on this branch and adds an
optional `delivery` block to the same record. The two changes are additive and
independent: that block covers every trial path outside `retained_files`, which
this change already keeps free of frozen paths. Whichever change archives second
must re-derive its delta `usecase.md` base digest, because both replace
`givn/specs/prepare-an-ise-test/usecase.md`.

Chapter 10 needs no row: the change adds no measured quality scenario, and the
accepted cost of the exclusion is R-154 rather than a target.

ADR-078 moves the active register to 30 accepted records, so
`givn/specs/fragments/adr-qualification-gate.feature` and
`tests/steps/adr_qualification_gate_steps.rs` assert 30. The delta scenario lives
at `givn/changes/keep-frozen-verifier-out-of-the-trial/specs/fragments/adr-qualification-gate.feature`;
the permanent scenario, its binding, and that delta land in one batch.

---

## Status

STATUS: DONE
