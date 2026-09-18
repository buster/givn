# Tasks: retire-lint-command-adr

Narrative spec change; no scenario delta and no new steps.

## Setup

- [x] Change scaffolded; the destination and disposition are recorded.

## Implementation

- [x] Write the verify-change delta narrative with the lint naming rule and
      the base hash.
- [x] Delete `docs/arc42/adr/ADR-027-lint-command-name.md`.
- [x] Repair ADR-019's references and the register README (count 24); align the register-history scenario (25 → 24) with a new step definition.
- [x] COMMIT: `4f8c4a0f` -- `docs(adr-qualification-gate): retire the lint command naming ADR`

## Verification

- [x] The accepted-record count is 24; no reference to the deleted file
      remains.
- [x] The archive merge accepts the narrative (base hash matches).
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [31.990s] 539 tests run: 539 passed, 0 skipped
      Summary [5.467s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `2df1f0fa` -- `docs(retire-lint-command-adr): complete the review`
- [x] Archive the change.
