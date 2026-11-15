# Arc42 Documentation Update: add-ise-clean-room-commands

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change delivers an already published `Ise Test` capability; it adds no goal, stakeholder, or quality goal. | |
| 2 | 02 architecture-constraints | Yes | The clean-room materialization and the trial record are new technical constraints. | `TC-46` records the revision-read, partition, nine refusals, plain-directory write, and record rule; `TC-47` records the verification rule and the backend entry-kind readers. |
| 3 | 03 context-and-scope | Yes | Two user-facing CLI commands and one external reader (the independent verifier) appear. | `3.2` gains the `givn ise replace` and `givn ise verify` rows; `3.1` gains the independent verifier or harness entity; `3.3` adds clean-room preparation to the in-scope list. |
| 4 | 04 solution-strategy | Yes | The solution gains a clean-room trial strategy. | The strategy table gains the revision-read, partition, refusal, plain-directory, and per-path identity row. |
| 5 | 05 building-block-view | Yes | New `cli::ise` and `ise` blocks, and changed SCM and boundary read seams. | The crate diagram gains the command and trial blocks; the responsibilities table names `cli::ise`, `ise`, the revision entries, and the e2e file set. |
| 6 | 06 runtime-view | Yes | Two new runtime flows. | `6.21` records the replace flow and the verify flow with their checks, writes, and outputs. |
| 7 | 07 deployment-view | No | The product stays one binary; a trial is a directory, not a deployed service or topology. | |
| 8 | 08 crosscutting-concepts | Yes | The `ise-trial/v1` record and its identity rule are cross-cutting project data. | `8.29` records the record shape, the per-path identity, the receipt placement, the dry-run output, and the backend parity. |
| 9 | 09 architecture-decisions | Yes | The trial record is a durable compatibility boundary. | `ADR-076` is added and linked from the active register; chapter 09 stays the link and does not duplicate the body. |
| 10 | 10 quality-requirements | No | The behavior is functional and asserted by Gherkin; the change adds no quality scenario or metric. | |
| 11 | 11 risks-and-technical-debt | Yes | The replacement typo, added-file, target race, Fossil manifest, and Rule-nested selection are deliberate limits. | `R-142` to `R-146` record those limits and their boundaries. |
| 12 | 12 glossary | Yes | The trial record and its printed `E2E digest` are new vocabulary that can collide with the change receipt's `E2E identity`. | `docs/glossary.md` gains `Trial record` and `Trial E2E digest`; the `E2E identity` anti-term is scoped to the change receipt. |

## ADR qualification

```text
Architectural commitment: the clean-room trial record is a versioned durable evidence artifact (`ise-trial/v1`) with per-path identities, consumed by an external verifier and the harness. Its format, inclusion rules, and receipt-placement rule are a compatibility boundary.
Real alternatives: the versioned per-path record versus a single rollup digest versus a printed-only report (proposal D4); a record outside the trial and the checkout versus a self-contained trial directory (proposal D2); revision-resolved `e2e` files versus role-matched paths only (design D5).
Durable consequence: a later record change needs a new content scope and a verifier update, and a prepared trial's evidence is durable project data. Reversal touches every prepared trial and the harness that reads it.
Existing-decision search: ADR-010 owns the declaration authority, ADR-070 owns the change verification receipt, ADR-071 owns the SCM seam. ADR-070 explicitly covers the change receipt, not a trial directory, so it is not amended.
Cheaper home: `docs/arc42/08-crosscutting-concepts.md` §8.29 plus the `design.md` data model own the current record facts; neither can own the rejected alternatives and the durable consequence, so the ADR keeps those and §8.29 carries the current facts and links this ADR.
ADR action: NEW_ADR
ADR target: ADR-076-clean-room-trial-record
```

Rejected candidates: the partition rules (observable behavior owned by
`prepare-an-ise-test` and its Gherkin), the Fossil manifest reader (an
implementation limit recorded as `R-145`), the report abbreviation
(presentation owned by `design.md`), and the e2e file-set resolution (a reuse
of the declaration's resolved-files rule, not a new architecture boundary).

## Notes

Every affected chapter named above is updated in this change. The glossary
distinguishes the trial's printed `E2E digest` from the change receipt's
`E2E identity`. `docs/reference.md` receives the command reference in the
implementation batch that lands the commands, because it documents delivered
CLI behavior, not planned behavior.

`docs/domain-model.md` needs no change: the model covers domain units and
explicitly excludes internal Rust components, command parsing, and file
formats, so the trial record and the command are architecture, not domain
structure. The absence of an edit is the assessed result, not an omission.

## Status

STATUS: DONE
