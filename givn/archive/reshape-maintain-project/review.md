# Review: reshape-maintain-project

## Fabrication Audit (do this first)

**Step definition scan** — no step definitions were added or changed; the
split scenarios keep their existing steps, and `specs` is skipped.

**Commit cross-check** — one implementation commit touches the corpus:

| Scope | Commit hash | Touches the release artifacts? |
|---|---|---|
| Split and moves | `c2da5b76` | yes (four narratives, one move, one feature split) |

**Promised-architecture cross-check** — the change promises a
reorganization, not a component: no production module changed; the two new
capabilities resolve.

**Strict-mode proof** — inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); the change adds no steps.

**Downgraded e2e scenario scan** — N/A: no `@e2e` scenario is added or
changed; the split keeps every tag.

**E2e scope check** — N/A.

**Duplicate-implementation check** — N/A.

**Local environment check** — N/A.

## Use-Case And Persona Conformance

`maintain-project` becomes a `+` summary with
`keep-documentation-current`, `upgrade-a-project`, and `publish-a-release`;
every capability has one owner; the summary owns none. R1, R2, R5, R6, R7,
R9, and R10 were verified during design-review. No Persona was invented or
promoted.

## Retirements

RETIRE: `release-0-3-0` as a capability; its scenarios move to
`apply-a-managed-upgrade` and `describe-the-release`, and no behavior is
dropped.

## Arc42 implementation conformance

The 12-row assessment is all-No with reasons. Direct check: the upgrade and
release contracts are unchanged; `givn spec tree` shows the new shape. No
chapter is placeholder text.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured by the archive receipt. The change adds no spec delta;
the moved and split scenarios are covered by the same steps. The full suite
(539) and e2e (88) pass.

## Deterministic gate dispositions

The overlap scan runs against the reorganized corpus; `givn lint` exits 0
with one-capability advisories only.

## E2e coverage

N/A — no `@e2e` scenario is added or changed; the e2e scope passes 88/88.

## Deferrals

None.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching the corpus.
- [x] Every promised artifact exists.
- [x] Strict-mode proof present and passing (inherited).
- [x] Scenario execution evidence recorded: full suite 539 passed, e2e 88
      passed; the archive receipt carries the measured coverage.
- [x] Coverage classified from inspection; no new gap.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No `@wip` tags.
- [x] Canonical E2E policy read; no new E2E action.
- [x] Local run command unchanged.
- [x] `verify.e2e_command` isolation unchanged.
- [x] Implementation matches design.md.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
