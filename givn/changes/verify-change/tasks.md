# Tasks: verify-change

Domain constraints: the recorded revision identifier is opaque (Q11) — non-empty, non-placeholder, non-duplicate, no shape assumption; failure wording is backend-neutral (Q10).

## Setup

- [x] Runner and strict mode unchanged; the delta reuses existing bindings.

## Scenario: An opaque revision identifier passes the integrity check

- [ ] RED/GREEN: immediate GREEN acceptance pin (all steps reused; no
      production change needed). Record the run.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: Integrity hook fails on placeholder commit hash

- [ ] RED: the modified Then (`revision identifier`) fails against the old
      message before the code change. Record the run.
- [ ] GREEN: the messages say "revision identifier"; the permanent feature is
      aligned in the same commit.
- [ ] COMMIT: hash recorded: `<hash>`

## Finalize

- [ ] Full suite green; lint exits 0; all tasks checked with hashes.
