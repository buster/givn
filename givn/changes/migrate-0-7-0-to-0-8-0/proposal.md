# Proposal: migrate-0-7-0-to-0-8-0

## Use-Case Context

- Use-case ID: none (release-owned maintenance change; `specs` is skipped)
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

Release 0.8.0 hardens the model behind the artifacts a project already owns.
The rules changed under existing corpora:

- **Use-case definition and scoping.** The canonical definition, the eight
  scoping rules, the capability placement test, and the level/promotion rule
  are new. A project's existing use-case documents may be mega-leaves: a
  conjunction goal, a summary with direct capabilities, a leaf whose flow
  names three or more actor goals, an observing case owning mutations, or a
  capability that fails the placement test.
- **Persona identity and reuse.** Identity criteria (authority boundary,
  evaluation capability, own vocabulary), the stable-core/running-record
  schema, the reuse-first protocol, and reference validation are new. A
  project may hold fragmented personas (one role sliced by mechanism), frozen
  topic copies, or dangling persona references.
- **ADR qualification.** The three falsification tests, the verdict before
  drafting, the blank ADR template, and the cheaper-home challenge are new.
  A project may hold records that pass the old gate only by stretching
  "durable consequence" — state-machine values, workflow approval gates,
  entity key splits.
- **Smaller adoptions.** The canonical dialog policy, the seed schema and
  status lifecycle, the term record and introduction test, the guarantee
  qualifier and decision ledger, and the lint severity and route confidence
  outputs all change what generated guidance and checks expect.

`givn upgrade` refreshes managed config and guidance automatically, but the
project's own artifacts are project-owned: without a release migration a 0.7.0
project upgrades to the new binary with no route or instructions to bring its
use cases, personas, and ADRs under the new model, and its next review or
archive runs against rules it has never applied.

## Proposed Solution

Release 0.8.0 with a release-owned migration bundle:

- Bump the package version (`Cargo.toml`, `Cargo.lock`), the project version
  marker (`givn/config.yaml`), and the changelog. The changelog's unreleased
  multi-SCM work (backend selection, Fossil publication, `givn commit`,
  ADR-072) is part of this release and is folded into the 0.8.0 section.
- Register `migrate-0-7-0-to-0-8-0` (order 60, source 0.7.0, target 0.8.0) in
  `assets/migrations/catalog.yaml`.
- Add the self-contained bundle
  `assets/migrations/0.7.0-to-0.8.0/{proposal.md,design.md}`. Its procedure,
  in ordered phases, brings a project under the new model:
  1. **Use-case model.** Apply the canonical definition and the eight scoping
     rules: re-scope conjunction-goal leaves, promote and split oversized
     leaves, re-home misplaced capabilities by the placement test, contain
     mutations under changing goals, and fix extension and precondition
     discipline. Incremental repair is the expected path.
  2. **Personas.** Consolidate fragmented segments by the three identity
     criteria, migrate persona files to the stable-core/running-record schema,
     convert reused topic copies to the record-only header, append-merge at
     handoff, and resolve dangling references.
  3. **ADRs.** Re-qualify every active record against the falsification tests
     and the cheaper-home challenge; route non-qualifying records to their
     canonical home with a historical-scope note rather than silently
     deleting them; replace project-local template copies that carry the
     pre-passed verdict.
  4. **Smaller adoptions.** Adopt the dialog policy, seed statuses, term
     records, the guarantee qualifier and decision ledger, and re-read the
     lint severity and route confidence outputs.
- Document the release and the consumer action in `CHANGELOG.md` and
  `docs/migrations/0.7.0-to-0.8.0.md`.

The machine-safe preparation (managed config/guidance refresh and the
`givn_config_version` bump) stays with `givn upgrade` and Commit A; the bundle
carries only project-specific decisions. No existing rule is enforced at
runtime by the migration: the bundle is authoring guidance completed by the
project's own review and archive gates.

## Out of Scope

- Rewriting project behavior, specifications, or test scenarios.
- Automating project-owned artifact edits from Givn; the maintenance LLM
  performs them and a human confirms each phase.
- Migrating any specific consumer project's corpus (for example kiekste's);
  this change ships the bundle and the guide, not the migrations themselves.
- A decision log or any new decision-record artifact.
- Backfilling verification receipts or re-publishing legacy archives.

## Open Questions

None.
