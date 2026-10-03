# Tasks: verify-change

Domain constraints: the recorded revision identifier is opaque (Q11) — non-empty, non-placeholder, non-duplicate, no shape assumption; failure wording is backend-neutral (Q10).

## Setup

- [x] Runner and strict mode unchanged; the delta reuses existing bindings.

## Scenario: An opaque revision identifier passes the integrity check

- [x] RED/GREEN: immediate GREEN acceptance pin (all steps reused; the check
      already accepts opaque identifiers). Targeted run:

      ```text
      1 scenario (1 passed)  /  exit 0
      ```

- [x] COMMIT: 02409078 (shared slice; the delta scenario needed no production
      change).

## Scenario: Integrity hook fails on placeholder commit hash

- [x] RED: with the message change stashed the modified Then failed:

      ```text
      stderr missing "revision identifier": [integrity] line 1: commit hash is a placeholder: `___`
      ```

- [x] GREEN: the messages say "revision identifier"; the permanent feature is
      aligned; the fragments suite passes (104 scenarios).
- [x] COMMIT: 79c04e50.

## Finalize

- [x] Full suite green (486/486); lint exits 0; all tasks checked.
