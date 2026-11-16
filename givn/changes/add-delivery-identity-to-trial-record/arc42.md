# Arc42 Documentation Update: add-delivery-identity-to-trial-record

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change applies the existing quality goal "Reliable evidence"; it adds no goal, stakeholder, or quality attribute. | |
| 2 | 02 architecture-constraints | Yes | A durable constraint appears: one delivery identity over every trial path outside the retained set, named apart from `replaced_digest`, with a fail-closed compatibility rule and a single recording. | `TC-50` records the delivered set, the naming rule, the unchanged scope, and the record-once rule. |
| 3 | 03 context-and-scope | Yes | The CLI gains a third `ise` subcommand and a verify mode, and the independent verifier reads either record shape. | The verifier row and the command interface table gain `givn ise deliver` and `--delivery`. |
| 4 | 04 solution-strategy | Yes | The clean-room trial row describes the retained, `e2e`, and replaced identities only, so the sealed delivery is missing from the strategy. | The row names the seal, the delivered identity, and the two identities partitioning the trial directory. |
| 5 | 05 building-block-view | Yes | `ise` gains the delivery responsibility and `cli::ise` gains the subcommand and the mode. | The `ise` and `cli::ise` rows are updated; no block is added or split. |
| 6 | 06 runtime-view | Yes | One new flow and one extended flow. | `6.21` gains a `Deliver` flow and the verify steps for the mode, the delivery comparison, and the delivery report lines. |
| 7 | 07 deployment-view | No | The product stays one binary; a trial and its records remain files, not a deployed service or topology. | |
| 8 | 08 crosscutting-concepts | Yes | The `ise-trial/v1` record is cross-cutting project data, and its field table gains the block. | `8.29` records the `delivery` field, the naming rule, the delivered set, and the compatibility consequence. |
| 9 | 09 architecture-decisions | Yes | ADR-076 owns the trial record's shape and placement; this change refines that decision. | The chapter-09 amendment table and `adr/README.md` gain the ADR-076 amendment; the ADR body gains an amendment section. |
| 10 | 10 quality-requirements | No | The behavior is functional and asserted by Gherkin; the change adds no quality scenario, metric, or threshold. | |
| 11 | 11 risks-and-technical-debt | Yes | The delivery identity removes the recorded limit R-143 for a sealed trial and introduces the seal's own moment. | `R-143` is narrowed to an unsealed trial; `R-153` records that the seal describes the moment it was taken. |
| 12 | 12 glossary | Yes | Two record terms change: the trial record gains a block and a new term names the delivery identity. | `docs/glossary.md` states the Trial record block and adds Trial delivery identity; chapter 12 stays a link. |

## ADR qualification

```text
Architectural commitment: the record keeps one scope and gains a second, later-written identity with a stated name-to-object rule and a fail-closed compatibility boundary; a new command writes it and a verify mode re-checks it.
Real alternatives: a new command with a record block versus a verify-written block versus a separate `ise-delivery/v1` scope (proposal D1); keep the name with a new delivery name versus rename with `ise-trial/v2` versus one field for two objects (D2); delivered set = trial minus retained versus replaced paths only versus the whole trial (D3); a verify mode versus a second subcommand versus implicit checking (D4); clean-input precondition versus every-replaced-path-present versus none (D5).
Durable consequence: every delivered trial's evidence and every verifier of it depend on the block, and reversing the choice needs another record change plus re-sealing each delivered trial.
Existing-decision search: ADR-076 owns the trial record's shape, placement, and identity rule and already records the limit this change closes (R-143). ADR-010 owns the declaration authority, ADR-070 the change verification receipt, and ADR-071 the SCM seam; none owns a delivered identity.
Cheaper home: the use case and `docs/reference.md` state the current facts, but neither can own the rejected alternatives or the compatibility consequence.
ADR action: AMEND_ADR — ADR-076 records the refinement in an amendment section, without creating a parallel decision or changing its status.
```

Rejected candidates: the delivered-set rule (observable behavior owned by
`prepare-an-ise-test` and its Gherkin), the record field addition (a shape
refinement of the decision ADR-076 already owns, so it belongs in that record's
amendment), the receipt target rule (an instance of the existing write-target
constraint recorded as `TC-50`), the report lines (presentation owned by the
design and `docs/reference.md`), and the `identity_at` move (one internal
helper sharing the existing identity rule).

## Notes

The affected chapters are applied in this change before design-review, so the
status marker states durable content, not an implementation-batch promise:
chapter 02 carries `TC-50`, chapter 03 lists the command and the flag, chapter
04 names the seal in the trial strategy, chapter 05 records the delivery
responsibility, chapter 06 carries the `Deliver` flow and the verify mode,
chapter 08 records the block and the naming rule, chapter 09 and the ADR
register record the ADR-076 amendment, and chapter 11 carries `R-143` and
`R-153`. `docs/reference.md` and `docs/glossary.md` are updated with them. The
implementation batch re-verifies the chapter text against the as-built code and
corrects any divergence; it does not add a new chapter.

`docs/domain-model.md` needs no change: the model covers domain units and
explicitly excludes internal Rust components, command parsing, and file
formats, so a record block is architecture, not domain structure. The absence
of an edit is the assessed result, not an omission.

## Status

STATUS: DONE
