# Arc42 Documentation Update: retain-declaration-in-trial

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change repairs a delivered capability; it adds no goal, stakeholder, or quality goal. | |
| 2 | 02 architecture-constraints | Yes | Two constraints become durable: the trial always carries the declaration, and the selected revision's declaration is the partition authority. | `TC-49` records the implicit retained entry, the revision-declaration read, the refusal of a revision without it, and the refusal of a replacement that names it. |
| 3 | 03 context-and-scope | No | No command, flag, external entity, or in-scope boundary changes; the two commands keep their surface and exit codes. | |
| 4 | 04 solution-strategy | Yes | The clean-room trial row describes the partition over "the declared roles and `source:` roots" and the recorded retained identities, which is now incomplete. | The row names the declaration as an always-retained member and the selected revision's copy as the read authority. |
| 5 | 05 building-block-view | Yes | `config::declaration` gains a byte-level entry point beside the file loader, and `ise` gains the declaration entry in its partition responsibility. | The `config::declaration` and `ise` rows are updated; no block is added or split. |
| 6 | 06 runtime-view | Yes | The replace flow's check order and the verify flow's retained set change. | `6.21` gains the revision-declaration read, the declaration refusal, and the declaration among the re-checked retained paths. |
| 7 | 07 deployment-view | No | The product stays one binary; a trial remains a directory, not a deployed service or topology. | |
| 8 | 08 crosscutting-concepts | Yes | The `ise-trial/v1` record's retained set is cross-cutting project data. | `8.29` states that `retained_files` always includes `givn/specification.yaml` and that no field or scope changes. |
| 9 | 09 architecture-decisions | No | The decision assessment in `design.md` routes the candidate to no ADR: `ADR-076` owns the record's shape and no field, scope, or identity rule changes. | |
| 10 | 10 quality-requirements | No | The behavior is functional and asserted by Gherkin; the change adds no quality scenario or metric. | |
| 11 | 11 risks-and-technical-debt | Yes | Two deliberate limits appear: an older record has no declaration entry, and a declaration that is not committed now refuses. | `R-151` records the older-record limit; `R-152` records the committed-declaration requirement. |
| 12 | 12 glossary | No | The chapter is a link to `docs/glossary.md`; the vocabulary edits land in the standalone glossary, which the change updates. | |

## ADR qualification

```text
Architectural commitment: the declaration is a mandatory member of every clean-room trial and of its record, and the selected revision's declaration is the partition authority.
Real alternatives: implicit entry versus a sixth role versus a separate report line and record field (proposal D2); the revision's declaration versus the working tree's versus a both-must-agree check (proposal D1).
Durable consequence: every prepared trial and every record carries one more path, and a project must commit its declaration to run a trial.
Existing-decision search: ADR-010 owns the declaration authority for the corpus and the archive gate; ADR-076 owns the trial record's shape and does not enumerate its members; ADR-071 owns the SCM seam.
Cheaper home: `docs/reference.md` and the `prepare-an-ise-test` contract own the current rule; `ADR-076` owns the record's shape and is not amended because no field, scope, or identity rule changes.
ADR action: none — no new compatibility boundary and no amended record format.
```

Rejected candidates: the partition rule (observable behavior owned by
`prepare-an-ise-test` and its Gherkin), the byte-level declaration entry (one
component with two entry points, not a boundary), the report count (a value,
not a shape), and the resume of the working-tree read (an implementation
limit recorded as `R-152`).

## Notes

The affected chapters are applied in this change before design-review, so the
final status marker states durable content, not an implementation-batch
promise:
chapter 02 carries `TC-49`, chapter 04 names the declaration in the clean-room
trial strategy, chapter 05 records the byte-level declaration entry point,
chapter 06 carries the revision-declaration read and the verification rule,
chapter 08 records the record's implicit entry, and chapter 11 carries `R-151`
and `R-152`. `docs/reference.md` and `docs/glossary.md` are updated with them;
`assets/instructions/ise.md` carries the served rule, whose delta scenario
stays `@wip` until its step binding is implemented. The implementation batch
re-verifies the chapter text against the as-built code and corrects any
divergence; it does not add a new chapter.

`docs/domain-model.md` needs no change: the model covers domain units and
explicitly excludes internal Rust components, command parsing, and file
formats, so a trial member is architecture, not domain structure. The absence
of an edit is the assessed result, not an omission.

## Status

STATUS: DONE
