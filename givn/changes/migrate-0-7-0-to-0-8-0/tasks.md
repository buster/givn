# Tasks: migrate-0-7-0-to-0-8-0

Maintenance change for the 0.7.0 to 0.8.0 contract. `specs` is skipped: the
bundle names no scenario delta and the change edits no corpus artifact except
the term collision-check cells. The bundle `design.md` is the procedure; these
tasks record the project work and its evidence.

## Setup

- [x] Canonical ID freed: the archived release-development record renamed
      `release-0-8-0` with its self-references and the one cross-reference
      updated. Evidence: commit `b5cce629`.
- [x] Consumer upgrade exercised for real: marker set to 0.7.0 (commit
      `22191b7c`), then `givn upgrade` produced Commit A
      `2bc63c10` (managed refresh) and Commit B `c70c74e6` (the plan at
      `givn/changes/migrate-0-7-0-to-0-8-0/`).
- [x] Preflight: `givn_config_version: "0.8.0"`, clean checkout, corpus
      inventoried (`givn spec tree`, use-case directories, personas, ADR
      register, term and seed records).

## Phase 1 — Use-case model (inventory)

- [x] Eight use-case documents checked against the definition, the eight
      scoping rules, and R1-R10. Rows recorded in `design.md`:
      R1/R9 for author-change, archive-change, maintain-project,
      operate-product; R7 for five documents; R5 for all eight; R10 for
      archive-change; R8 for twenty-nine names.
- [x] Nineteen support-fragment capabilities checked; fourteen R8 names and
      the orphaned `config-overlay` recorded.
- [x] COMMIT: `448a48a5` -- `docs(migrate-0-7-0-to-0-8-0): record the project inventory, ADR re-qualification, and term evidence`

## Phase 2 — Personas (inventory)

- [x] Three permanent personas checked for identity criteria and the
      stable-core schema; three topic copies confirmed byte-identical
      biography copies; five `## Personas: none` declarations on
      user-visible surfaces recorded.

## Phase 3 — ADRs (re-qualification)

- [x] All 28 accepted records re-run against the five mandatory dimensions,
      the three falsification tests, and the cheaper-home challenge. 24
      qualified; ADR-027 NOT_QUALIFIED; ADR-044, ADR-045, and ADR-054
      obsolete (superseded by ADR-069). No `proposed` record exists. Verdicts
      and audit notes in `arc42.md`.
- [x] Project-local ADR template confirmed stale (carries the old gate's
      pre-passed verdict); recorded as a follow-up.

## Phase 4 — Smaller adoptions (inventory, one exception)

- [x] Seven consumed seeds recorded with the `[SEED ]` advisory, missing
      option treatment, and missing suggested start.
- [x] Term file checked: 12 candidates are not term records. The one
      permitted edit: collision-check evidence re-run and recorded for the
      nine confirmed terms (SCM backend, Revision, Checkout, Clean state,
      Candidate revision, Tested revision, Revision binding, Upgrade
      revision, Plan revision); the `Hook` collision with `Hook (post)` is
      noted for the follow-up.
- [x] `givn instructions dialog` read; `givn lint` severity and
      `givn spec route` confidence output re-read. No guarantee qualifier or
      `D<n>` ledger rows found.

## Phase 5 — Ejected overrides (inventory)

- [x] `givn/artifacts/arc42-docs/impact-assessment-template.md` confirmed as
      a stale pre-0.8.0 override (missing the structured verdict block).
      `givn/config.yaml` carries settings only; `givn/skills/` is empty.

## Follow-up register

- [x] 18 follow-ups named in `design.md` (nine use-case/fragment reshaping,
      three persona, three ADR, two smaller-adoption, one override). None is
      in-flight; no archived change is named as pending.

## Validation

- [x] `givn lint` exits 0; advisory findings (`[SEED ]`, `[SUBST]`,
      `[SHAPE]`, `[WARN ]`) recorded as migration input, none blocking.
- [x] `givn check review --change migrate-0-7-0-to-0-8-0` passes.
- [x] `./run-tests.sh` passes. Evidence:
      ```
      Summary [  30.340s] 539 tests run: 539 passed, 0 skipped
      ```
- [x] `./run-tests.sh --e2e` passes. Evidence:
      ```
      Summary [   5.455s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `f3198915` -- `docs(migrate-0-7-0-to-0-8-0): complete the migration review`
- [x] Archive the maintenance change; the migration is complete when every
      follow-up has archived.

## Dogfooding finding (separate product change)

- [x] The generated proposal wrapper drops spaces at four Rust line
      continuations ("theinstalled", "inCommit", "workeach",
      "mustpreserve") in `src/cli/migration.rs`. Recorded in
      `design-review.md`; fixed by a separate product change after this
      migration archives.
