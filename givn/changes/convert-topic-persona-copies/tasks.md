# Tasks: convert-topic-persona-copies

Documentation repair; `specs` is skipped.

## Setup

- [x] Change scaffolded; `.givn-skip` declares `specs`.

## Implementation

- [x] Replace the three topic biography copies with reuse records carrying
      the base hashes `3e48068e`, `93a37080`, `a3b1fa11`.
- [ ] COMMIT: `<hash>` -- `docs(persona-identity-reuse): convert the topic persona copies to reuse records`

## Verification

- [x] Each topic file has one `Reuse:` line and no biography sections.
- [x] `givn lint` reports no dangling persona reference (0 `[PERS ]` findings).
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [30.630s] 539 tests run: 539 passed, 0 skipped
      Summary [5.436s] 88 tests run: 88 passed, 451 skipped
      ```
- [ ] COMMIT: `<hash>` -- `docs(convert-topic-persona-copies): complete the review`
- [ ] Archive the change.
