# Design: migration-retrofit-plan

## Overview

Maintenance change amending the released-but-untagged 0.8.0 migration bundle,
its human guide, the aggregate-plan wrapper text, and the durable chapter
sentences that describe the old execution model. `specs` is skipped; the
deliverables are embedded assets, one production string change, and
documentation. No version bump.

## Exact file changes

| File | Change |
|---|---|
| `assets/migrations/0.7.0-to-0.8.0/proposal.md` | Solution and completion boundary reframed to inventory-and-plan with named follow-ups; the seven decisions named |
| `assets/migrations/0.7.0-to-0.8.0/design.md` | New `## Execution model` and `## Inventory and dispositions` (exact table shape); phases become inventory criteria; per-phase execution-model line; ADR verdict/repair split; term-evidence exception; proposed-ADR owner rule; rename rule; validation checks |
| `docs/migrations/0.7.0-to-0.8.0.md` | Contract section states the seven decisions; Prompts 1-6 rewritten to inventory and name follow-ups; two-level completion boundary |
| `docs/migrations/README.md` | Bundle description and "complete the phases in order" wording updated |
| `CHANGELOG.md` | 0.8.0 bundle sentence describes the inventory-and-follow-up model |
| `src/cli/migration.rs` | Aggregate-plan wrapper strings defer to each bundle's completion rules instead of asserting in-change completion |
| `docs/arc42/02-architecture-constraints.md` | Project-owned artifacts change through LLM-guided project changes, not only the maintenance change |
| `docs/arc42/06-runtime-view.md` | 6.13 archives the maintenance plan once the inventory is complete |
| `docs/arc42/07-deployment-view.md` | Bundle phases are inventoried in one aggregate change; repairs run as follow-ups |

## Amended bundle structure

The bundle keeps the inlined rules (definition, eight scoping rules, R1-R10,
persona criteria and schema, ADR falsification tests, smaller adoptions) as
the inventory criteria. The execution model is:

1. **Execution model block.** At the top: this change is the inventory and
   plan; it edits no corpus artifact except the term collision-check cells;
   every repair runs in a named follow-up change through the normal gates.
2. **Inventory table shape**, one row per artifact kind:

   | Artifact | Deviation (criterion or test) | Disposition | Follow-up change id |
   |---|---|---|---|

   Follow-up ids are kebab-case `<verb>-<object>`. An id is *in-flight* when
   `givn/changes/<id>/` exists at inventory time; otherwise it is *named*
   (recorded here, not yet scaffolded). An archived change is never named as
   a pending follow-up.
3. **Phases as inventory passes.** Each phase lists the rows it produces and
   keeps its criteria: use cases (R1-R10 plus the R8 rename rule), personas
   (identity criteria, schema, record-only reuse, dangling references), ADRs
   (accepted re-qualified here; proposed listed against their owning change;
   a verdict and audit note stay in this change, while the rationale move,
   register/index removal, and template replacement are follow-ups), smaller
   adoptions (dialog policy reading and the lint/route re-read stay in this
   change; seed statuses, `(open: Q<N>)` qualifiers, and the `D<n>` ledger
   become rows with follow-ups; term evidence is re-run and recorded here,
   with a collision becoming a follow-up row), overrides (rows with
   follow-ups).
4. **Archive timing.** The maintenance change archives once every row has a
   disposition and a named or in-flight follow-up; the inventory is the
   durable record. The migration is complete when the follow-ups archive.
5. **Validation.** The maintenance change edited no corpus artifact except
   the term collision-check cells; every row names a follow-up or an
   in-flight change; no archived change is named as pending.

## Verification

- Wrapper change: `cargo test --locked --lib migration` passes; the release
  fixtures' aggregate assertions (`GIVN_RESULT_FILE`, length) are unaffected
  because they do not assert the wrapper sentences.
- Aggregate smoke test (manual, exact strings): the scratch 0.7.0 project's
  generated maintenance change design contains `## Execution model` and
  `## Inventory and dispositions` and `follow-up change`, and no
  unsubstituted `{{`. These strings are absent from the pre-amendment bundle;
  `accepted`, `proposed by`, and `Re-qualify` already occur there and are not
  used as evidence.
- `cargo check --locked`, `cargo clippy --workspace --all-targets --locked --
  -D warnings`, `./run-tests.sh`, and `./run-tests.sh --e2e` pass.
- Archive: the maintenance change archives with `specs` skipped.

## Step Definitions

N/A — no step definitions for this change.

## Strict Mode (mandatory)

Inherited unchanged: `GivnWorld::cucumber().fail_on_skipped()` in
`tests/features_runner.rs`. No new steps are added.

## Local Runnability & Digital Twins (mandatory)

N/A — no runnable surface, no dependency, no digital twin.

## Internal Primitives

None.

## Use-case traceability

| Proposal claim | Verification |
|---|---|
| Bundle carries the inventory-and-plan model | aggregate smoke test; bundle design sections |
| Guide matches the bundle | `docs/migrations/0.7.0-to-0.8.0.md` contract and prompts |
| Wrapper defers to each bundle | `src/cli/migration.rs` strings; release fixtures green |
| Durable chapters describe the model | chapters 02, 06, 07 sentences |
| No behavior change, no version bump | `specs` skipped; suite green; Cargo.toml and config marker remain 0.8.0 |

## Open Technical Questions

None.
