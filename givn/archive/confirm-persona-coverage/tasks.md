# Tasks: confirm-persona-coverage

Narrative spec change; no scenario delta and no new steps.

## Setup

- [x] Change scaffolded; the operator approved the actor's Persona per use
      case.

## Implementation

- [x] Write the five delta narratives with the `## Personas` correction and
      the current base hashes.
- [x] COMMIT: `c6c3787b` -- `feat(persona-identity-reuse): confirm persona coverage on five use cases`

## Verification

- [x] The archive merge accepts every narrative (base hash matches).
- [x] After archive, each permanent use case names its Persona.
- [x] `givn lint` reports no `[PERS ]` finding.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.329s] 539 tests run: 539 passed, 0 skipped
      Summary [5.4s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `d1a27fb7` -- `docs(confirm-persona-coverage): complete the review`
- [x] Archive the change.
