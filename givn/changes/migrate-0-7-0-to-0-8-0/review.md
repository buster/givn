# Review: migrate-0-7-0-to-0-8-0

## Fabrication Audit (do this first)

**Step definition scan** — no step definitions were added or changed; the
change skips `specs` and its only corpus edit is the term collision-check
evidence. No runner or step file is touched.

**Commit cross-check** — the maintenance change's work is the inventory and
its evidence:

| Scope | Commit hash | Touches the release artifacts? |
|---|---|---|
| Release-record rename frees the canonical ID | `b5cce629` | yes (archive record) |
| Marker simulation for the real upgrade | `22191b7c` | yes (`givn/config.yaml`) |
| Commit A: managed refresh | `2bc63c10` | yes (managed files) |
| Commit B: generated plan | `c70c74e6` | yes (the change) |
| Inventory, ADR verdicts, term evidence | `448a48a5` | yes (`design.md`, `arc42.md`, `domain-terms.md`) |
| Task evidence and review | this commit | tasks and review only |

**Promised-architecture cross-check** — the change promises an inventory and
a plan, not components: the Phase 1-5 tables and the follow-up register exist
in `design.md`, and no production module was introduced.

**Strict-mode proof** — inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); the change adds no steps.

**Downgraded e2e scenario scan** — N/A: no `@e2e` scenario is added or
changed.

**E2e scope check** — N/A.

**Duplicate-implementation check** — N/A: no runner or e2e implementation is
touched.

**Local environment check** — N/A: no runnable surface or dependency.

## Use-Case And Persona Conformance

N/A for spec deltas — the change has no use-case delta (`specs` skipped). The
use-case corpus itself is inventoried, not modified. No Persona was invented
or promoted; the persona inventory records the schema gaps, the topic
biography copies, and the five `Personas: none` declarations for the
follow-ups. The three confirmed Personas reviewed the inventory's
developer-facing, maintainer-facing, and agent-facing dispositions.

## Retirements

RETIRE: none in this change. The follow-up register records the retirements
(ADR-027 from the register, ADR-044/045/054 to the ADR archive).

## Arc42 implementation conformance

The 12-row assessment is all-No with reasons. Direct check: this change edits
no durable chapter; the ADR re-qualification verdicts are recorded in
`arc42.md` and the register edits belong to the follow-ups. Chapters 8.20-8.24
already describe the migrated rules; the inventory applies them. No chapter is
placeholder text.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured by the archive receipt against the unchanged permanent
corpus. The change adds no spec delta, so the deterministic gates report
`net delta: 0 added, 0 modified, 0 removed`; the term-cell edit is
documentation evidence, not behavior.

## Deterministic gate dispositions

No delta specs, so no overlap findings and no dispositions are required. The
net-delta receipt is `0 added, 0 modified, 0 removed`.

## E2e coverage

N/A — no `@e2e` scenario is added or changed; the e2e scope passes 88/88.

## Deferrals

None.

## Dogfooding finding

The generated proposal wrapper drops spaces at four Rust line continuations
("theinstalled", "inCommit", "workeach", "mustpreserve") in
`src/cli/migration.rs`. Recorded in `design-review.md`; fixed by a separate
product change after this migration archives. It does not affect the
inventory.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching release artifacts.
- [x] Every promised artifact exists.
- [x] Strict-mode proof present and passing (inherited).
- [x] Scenario execution evidence recorded: full suite 539 passed, e2e 88
      passed; the archive receipt carries the measured coverage.
- [x] Coverage classified from inspection; no new gap.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No `@wip` tags (no delta specs).
- [x] Canonical E2E policy read; no new E2E action.
- [x] Local run command unchanged.
- [x] `verify.e2e_command` isolation unchanged.
- [x] Implementation matches design.md.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
