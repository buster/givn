# Migration Proposal: {{SOURCE_VERSION}} to {{TARGET_VERSION}}

## Problem / Opportunity

Givn {{TARGET_VERSION}} hardens the model behind the artifacts your project
already owns:

- **Use cases** gain a canonical definition and eight scoping rules. Existing
  documents may be mega-leaves: a goal that joins several goals, a summary
  that owns capabilities directly, a leaf whose flow names three or more
  actor goals, an observing case that owns mutations, or a capability whose
  name and placement fail the model.
- **Personas** gain identity criteria and a stable-core/running-record
  schema. Existing files may be one role sliced by mechanism, frozen topic
  copies, or dangling references.
- **ADRs** gain three falsification tests and a cheaper-home challenge.
  Existing records may pass the old gate only by stretching "durable
  consequence".
- Dialog policy, change-seed statuses, term records, the guarantee qualifier,
  and the decision ledger are new adoptions.

`givn upgrade` refreshed your managed config and guidance and created this
maintenance change from the release-owned bundle. The remaining work is
project-specific: your use-case documents, personas, and ADRs are yours, and
no runtime gate rewrites them. Without this migration they stay outside the
model the new guidance, lint, and review apply.

## Proposed Solution

Work the bundle's `design.md` phases in order, one artifact at a time:

1. Bring every permanent use-case document under the canonical definition and
   the eight scoping rules (inlined in the design), fixing R1-R10 rejection
   criteria.
2. Consolidate and migrate personas to the stable core plus running record,
   and resolve dangling references.
3. Re-qualify every active ADR against the falsification tests and route
   non-qualifying records to one canonical home with a historical-scope note
   in this change.
4. Adopt the dialog policy, seed statuses, term records, guarantee
   qualifiers, and the decision ledger; re-read the lint severity and route
   confidence output.
5. Re-synchronize ejected overrides from the {{TARGET_VERSION}} binary.

Incremental repair is the expected path. Do not rewrite the corpus in one
pass; move one capability, one persona, or one ADR per edit and run the
affected checks after each.

## Out of Scope

- Changing product behavior, specifications, or test scenarios.
- Deleting artifacts: a non-qualifying ADR's rationale moves once to its
  canonical home; the record leaves the register but the migration records
  where it went.
- Re-publishing legacy archives or backfilling verification receipts.

## Completion Boundary

The migration is complete when every permanent use-case document passes the
scoping rules or carries a recorded repair plan, every persona reference
resolves and every persona file follows the schema, every active ADR has a
recorded qualification verdict, the adoptions are recorded, ejected overrides
are re-synchronized, and the project's next review and archive run cleanly
under the {{TARGET_VERSION}} rules.
