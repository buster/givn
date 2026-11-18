# Review: make-ise-guidance-self-contained

Review kind: self-review
Author: Main (omp session)
Reviewer context: Main (omp session)

The author performed the design review and the final review in the same context.

## Implementation evidence check

- **Two scenarios:** the delta has `The ise instruction's file references resolve in a consumer project` and `Printed guidance omits the opt-in ise rules`. Both are checked with `COMMIT: b2f586c -- Batch: self-contained-guidance`.
- **Batch commit:** `b2f586c` contains `assets/instructions/ise.md`, the delta feature, and `tests/steps/instruction_integrity_steps.rs`. The commit is not comment-only, task-only, or feature-only.
- **RED proof:** with the old `docs/reference.md` delegation restored, the reference scenario fails with `givn instructions ise:16 — docs/reference.md`. With the inline definitions it passes. The failure is the asserted reference, not an unbound step or fixture error.
- **Change gate:** `givn check tasks --change make-ise-guidance-self-contained` reports 2 scenarios, 0 failed, 0 skipped at `ee66b00`; E2E scope is `not_applicable` because the delta has no `@e2e` scenario.
- **Full regular suite:** `./run-tests.sh --non-e2e` exits 0 with 1124 passed, 115 skipped.
- **E2E suite:** `./run-tests.sh --e2e --package e2e` exits 0 with 120 passed.
- **No product behavior change:** no command, flag, exit code, persisted format, report label, `ise-manifest/v1` field, or gate changes.
- **Off-path isolation:** the new sweep requests every printed instruction plus the startup guidance in an ise-off project and asserts `Ise rebuild decision`, `Rebuilt code`, `Spec gap`, `Rebuild impact record`, `Specification Manifest`, and `Ise Test` are absent.
- **Empty step bodies:** 0 empty/no-op bodies in the two new steps; both assert observable output.

## Coverage classification

No uncovered region remains. The two new scenarios cover the two findings from the isolation review. The default-off posture, the existing gates, and the opt-in command are unchanged. The direct `ise` instruction terms are now defined inline; `docs/reference.md` remains the detailed owner for maintainers but is not a consumer dependency.

## Findings

None.

PERSONAS: developer--primary-user

README-IMPACT: none

REVIEW: PASS
