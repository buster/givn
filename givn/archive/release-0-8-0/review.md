# Review: release-0-8-0

## Fabrication Audit (do this first)

**Step definition scan** — no step definitions were added or changed; the
change skips `specs` and ships versioned data, documentation, and one
test-only route assertion. The new test
`migration_path_routes_0_7_0_to_the_current_release` in
`src/cli/migration.rs` asserts the resolved bundle id.

**Commit cross-check** — five commits, each touching release artifacts or
code:

| Scope | Commit hash | Touches production/release code? |
|---|---|---|
| Formatting and lint on the 0.8.0 rule changes | `a3a97ce9` | yes (Rust sources and step files) |
| 0.8.0 version bump and changelog | `5548eeda` | yes (`Cargo.toml`, `Cargo.lock`, `givn/config.yaml`, `CHANGELOG.md`) |
| Catalog entry, route test, consumer bundle | `1bfecdc4` | yes (`assets/migrations/`, `src/cli/migration.rs`) |
| Human-facing migration guide and README | `3c0fbfe7` | yes (docs) |
| Task evidence | `b0a6ac31`-range chores | tasks only |

**Promised-architecture cross-check** — the change promises versioned data,
not components: `assets/migrations/0.7.0-to-0.8.0/proposal.md` and `design.md`
exist, the catalog registers them, and no production module or abstraction
was introduced.

**Strict-mode proof** — inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); the change adds no steps, so the proof remains valid.

**Downgraded e2e scenario scan** — N/A: no `@e2e` scenario is added or
changed.

**E2e scope check** — N/A.

**Duplicate-implementation check** — N/A: no runner or e2e implementation is
touched.

**Local environment check** — N/A: no runnable surface or dependency.

## Use-Case And Persona Conformance

N/A — release-owned maintenance change with no use-case delta (`specs`
skipped). No Persona was invented or promoted; the proposal records `none`
with the reason. The managed-upgrade guarantees (preserve user-owned files,
no version conditionals in Rust) are preserved: the catalog is data and the
only code change is a test.

## Retirements

RETIRE: none.

## Arc42 implementation conformance

The 12-row assessment is all-No with reasons, corrected during design-review
for rows 9 and 11. Direct check: chapters 6.13 (upgrade runtime), 8.10
(configuration versioning), 8.15, and 8.19-8.24 describe the catalog and the
migrated rules generically; a new data row needs no text. The release update
to `docs/migrations/README.md` and the new guide are consistent with the
durable chapters. No chapter is placeholder text.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured by the archive receipt against the unchanged permanent
corpus. No product code path is added, so no new uncovered region exists. The
new unit test covers the resolved route.

## Deterministic gate dispositions

No delta specs, so no overlap findings and no dispositions are required. The
net-delta receipt is `0 added, 0 modified, 0 removed`.

## E2e coverage

N/A — no `@e2e` scenario is added or changed; the e2e scope still passes
(88/88) against the unchanged corpus.

## Deferrals

None.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching release artifacts.
- [x] Every promised component exists.
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
