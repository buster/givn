# Review: reshape-author-change

## Fabrication Audit (do this first)

**Step definition scan** — no step definitions were added or changed; the
moved scenarios keep their existing steps, and `specs` is skipped.

**Commit cross-check** — one implementation commit touches the corpus:

| Scope | Commit hash | Touches the release artifacts? |
|---|---|---|
| Split and renames | `0b7d40c8` | yes (three use-case narratives, five feature files moved) |

**Promised-architecture cross-check** — the change promises a
reorganization, not a component: no production module changed; the five
capabilities resolve under their new homes.

**Strict-mode proof** — inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); the change adds no steps.

**Downgraded e2e scenario scan** — N/A: no `@e2e` scenario is added or
changed; the moved e2e scenarios keep their tags.

**E2e scope check** — N/A.

**Duplicate-implementation check** — N/A.

**Local environment check** — N/A.

## Use-Case And Persona Conformance

The `author-change` leaf becomes a `+` summary with `initialize-project` and
`create-change`; every capability has exactly one owner; the summary owns no
direct capabilities. The R1, R2, R5, R6, R7, and R9 criteria were verified
during design-review; R10 holds (summary→child Includes, leaf→fragment
Includes). No Persona was invented or promoted.

## Retirements

RETIRE: none; the capability IDs `new-command`,
`init-agent-target-selection`, and `install-extras` are renamed, not
retired, and their scenarios move with them.

## Arc42 implementation conformance

The 12-row assessment is all-No with reasons. Direct check: no runtime or
architecture fact changed; `givn spec tree` shows the new shape. No chapter
is placeholder text.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured by the archive receipt. The change adds no spec delta;
the moved scenarios are covered by the same steps. The full suite (539) and
e2e (88) pass.

## Deterministic gate dispositions

The overlap scan runs against the reorganized corpus; `givn lint` exits 0
with advisories only.

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
