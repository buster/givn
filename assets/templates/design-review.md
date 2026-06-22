# Design Review: {{CHANGE_ID}}

## Grilling log

Record each question, the recommended answer, and the outcome (agreed /
overridden / deferred).

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | | | | |

## Open questions

Questions that could not be resolved during grilling and need user input before
implementation begins.

| # | Question | Blocking? |
|---|---|---|
| | | |

## Changes made during hardening

Edits applied to existing artifacts as a result of grilling.

| Artifact | Change summary |
|---|---|
| | |

## Sign-off

- [ ] Every branch of the design tree has been walked.
- [ ] All open questions are resolved or explicitly flagged as blockers.
- [ ] `design.md` reflects any decisions reached during grilling.
- [ ] `tasks.md` has populated "Design constraints" blocks for every scenario task.
- [ ] All scenarios are testable: Then-steps assert concrete observable values.
- [ ] `specs/**/*.feature` updated if any missing scenarios were identified.
- [ ] `givn lint --change {{CHANGE_ID}}` exits 0 or 2 (no parse errors).

**DESIGN-REVIEW: PASS** <!-- change to FAIL or remove if any blocker remains -->
