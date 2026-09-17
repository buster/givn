# Design: migrate-0-7-0-to-0-8-0

## Overview

Release 0.8.0 of the use-case, persona, and ADR hardening plus the
release-owned migration bundle that carries a 0.7.0 project to it. This is a
maintenance change: `specs` is skipped (`.givn-skip`), no product behavior
changes, and the deliverables are versioned assets and documentation.

## Exact file changes

| File | Change |
|---|---|
| `Cargo.toml` | `version = "0.8.0"` |
| `Cargo.lock` | workspace package version 0.8.0 |
| `givn/config.yaml` | `givn_config_version: "0.8.0"` |
| `assets/migrations/catalog.yaml` | `migrate-0-7-0-to-0-8-0`, source `0.7.0`, target `0.8.0`, order 60, asset paths |
| `assets/migrations/0.7.0-to-0.8.0/proposal.md` | consumer-facing migration proposal (new) |
| `assets/migrations/0.7.0-to-0.8.0/design.md` | self-contained consumer migration procedure (new) |
| `docs/migrations/0.7.0-to-0.8.0.md` | human-facing migration guide (new) |
| `docs/migrations/README.md` | names the current bundle |
| `CHANGELOG.md` | fold `## [Unreleased]` into `## [0.8.0] - <date>`, naming the multi-SCM work and the model-hardening release |
| `src/cli/migration.rs` | one regression test asserting the 0.7.0 route resolves to `migrate-0-7-0-to-0-8-0` (test-only; no production code change) |

No production Rust code changes. The catalog is data; the route resolves through the
existing `src/cli/migration.rs` logic, so no version-specific conditional is
added (the bundle contract forbids that).

## Bundle contract

The consumer bundle is the phase consumed by `givn upgrade`'s aggregate
maintenance change. It must be self-contained: an LLM in the consumer project
has no access to this repository. It uses the existing placeholders
`{{SOURCE_VERSION}}`, `{{TARGET_VERSION}}`, and `{{UPGRADE_COMMIT}}`.

The bundle's `design.md` carries the ordered phases:

1. **Use-case model.** The bundle inlines the canonical definition, the
   eight scoping rules, and the ten rejection criteria (R1-R10), because
   `docs/use-case-model.md` exists only in the Givn repository and no command
   writes it into a project. The live sources the bundle points at are the
   generated ideation guidance and `givn instructions review`. For every
   permanent use-case document: test the goal for conjunction and
   multi-outcome guarantees (R1), identify the primary actor (R2), apply the
   capability placement test (R3/R8), check mutation containment (R4),
   extension step references (R5), precondition/flow consistency (R6),
   mechanism vocabulary (R7), and oversized leaves (R9); fix relationships
   (R10). Re-home one capability at a time; promote and split oversized
   leaves; never rewrite the whole corpus in one pass. Record each split in
   the change that performs it, and run the affected feature files after each
   move.
2. **Personas.** Apply the three identity criteria to every persona file and
   topic copy: authority boundary, evaluation capability, own vocabulary.
   Collapse segments that share all three into one persona; migrate the file
   to the stable core (role, authority boundary, evaluation capability, own
   vocabulary, success criterion) plus an append-only running record and a
   dated topic-stances appendix; convert reused topic copies to the standard
   `Reuse:` header with record entries only; append-merge into the project
   collection at handoff with a user-confirmed diff; resolve every dangling
   `## Personas` reference. `givn lint` reports dangling references as
   advisories, and `givn check review` enforces a visual change's
   `PERSONAS:` value.
3. **ADRs.** For every active record, re-run the qualification gate with the
   three falsification tests: single-change reversibility, no multi-change
   constraint, and the product-behavior exclusion. A record that fails is
   routed to exactly one canonical home (specification, design, Arc42
   chapter, process documentation, project documentation, or code); its
   rationale moves there once, the record leaves the active register and both
   indexes, and the historical-scope note (record ID, failing test,
   destination) lives in the maintenance change's migration ledger. It is
   never copied into `docs/arc42/adr/archive/` — the shipped guidance says a
   record that fails the gate is not a historical ADR. A qualified record
   stays. A project-local ADR template copy that carries a pre-passed verdict
   is replaced by deleting `docs/arc42/adr/adr-template.md`, running
   `givn addons enable arc42` to re-scaffold missing files, and confirming
   the file no longer contains `ADR-QUALIFICATION: PASS` or
   `qualification: QUALIFIED`; the verdict is recorded in the change
   assessment before any new MADR is drafted.
4. **Smaller adoptions.** Read the canonical dialog policy
   (`givn instructions dialog`); classify every existing change seed with one
   of the five statuses (`ready`, `proposed by <change-id>`,
   `implemented in <change-id>`, `superseded by <reference>`, `withdrawn`) —
   a consumed seed stamped `ready` misleads the next planner — and note the
   `[SEED ]` lint advisory that finds a missing or contradicted status;
   migrate term records to the term/status/definition/contrast/first-use/
   collision-check schema with `candidate`, `confirmed`, and `rejected`
   statuses, where `confirmed` keeps its status only with the pasted search
   commands and hit counts ("no collision" without commands is not a
   record); add `(open: Q<N>)` qualifiers and the `D<n>` question IDs with
   the design's `Decides D<n>` ledger; re-read the lint severity labels and
   the route confidence output.

5. **Overrides.** Inventory ejected facets: project overrides declared in
   `givn/config.yaml`, files under `givn/artifacts/`, and project skill
   overrides under `givn/skills/`. Re-eject each from the 0.8.0 binary and
   re-apply only genuine project-specific edits on top; never hand-patch the
   old rule text back into a stale copy. Managed generated files (skills,
   commands, AGENTS.md blocks) are already refreshed by the upgrade.

Each phase ends with a validation step: the project's own `givn lint`,
`givn check review`, and the next archive run under the new rules. The bundle
states the enforcement boundary explicitly: only two of the migrated rules
are machine-checked (dangling `## Personas` references as `[PERS ]` lint
advisories, and a visual change's `PERSONAS:` value in `givn check review`);
the use-case scoping rules, the persona identity criteria, and the ADR
falsification tests are authoring and review guidance, so a green suite does
not by itself prove the migration. The bundle forbids bulk rewriting: one
capability, one persona, or one ADR per edit, with the project's suite run
after each.

## Version bump mechanics

`env!("CARGO_PKG_VERSION")` is the single source for `givn init`, the
`{{GIVN_VERSION}}` config placeholder, and the `givn upgrade` target version.
Bumping the package version makes new projects write the 0.8.0 marker and
makes 0.7.0 projects resolve the new catalog edge. The repository's own
`givn/config.yaml` marker is bumped explicitly with the release, matching the
0.6.0 and 0.7.0 release changes.

## Verification

- Catalog validity: `givn upgrade` resolves the route in a scratch project
  whose marker is 0.7.0 and produces
  `givn/changes/migrate-0-7-0-to-0-8-0/` whose design contains the five phase
  headings and no unsubstituted `{{`; a second `givn upgrade` reports the
  change already exists. A unit test asserts
  `migration_path("0.7.0", env!("CARGO_PKG_VERSION"))` returns exactly
  `migrate-0-7-0-to-0-8-0`.
- The bundle assets are embedded; `cargo check --locked` validates the build
  and the lock.
- The repository suite (`./run-tests.sh`) and the E2E scope
  (`./run-tests.sh --e2e`) run against the unchanged permanent corpus; the
  existing upgrade scenarios tolerate catalog additions by construction.
- Archive: the maintenance change archives with `specs` skipped; its
  verification scopes run against the permanent corpus.

## Step Definitions

N/A — no step definitions for this change.

## Strict Mode (mandatory)

Inherited unchanged: `GivnWorld::cucumber().fail_on_skipped()` in
`tests/features_runner.rs`. No new steps are added.

## Local Runnability & Digital Twins (mandatory)

N/A — no new runnable surface, no external dependency, no digital twin. The
verification path is the existing CLI plus the repository suite.

## Internal Primitives

None — no production structs, functions, or enums are introduced.

## Use-case traceability

| Proposal claim | Verification |
|---|---|
| Version and marker bumped to 0.8.0 | `Cargo.toml`, `Cargo.lock`, `givn/config.yaml`; `givn --version` |
| Catalog routes 0.7.0 → 0.8.0 | scratch-project `givn upgrade` smoke test; catalog entry |
| Bundle carries the consumer procedure | `assets/migrations/0.7.0-to-0.8.0/{proposal,design}.md`; aggregate design contains the five phases |
| Guide is human-readable | `docs/migrations/0.7.0-to-0.8.0.md`; `docs/migrations/README.md` names it |
| No behavior/spec change | `specs` skipped; suite green on the unchanged corpus |

## Open Technical Questions

None. The annotated `v0.8.0` tag and the prepare-release workflow are a
post-merge maintainer action, matching the 0.6.0/0.7.0 precedent; no workflow
edit belongs in this change.
