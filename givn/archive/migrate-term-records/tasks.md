# Tasks: migrate-term-records

Documentation repair; `specs` is skipped.

## Setup

- [x] Change scaffolded; `.givn-skip` declares `specs`.

## Implementation

- [x] Rewrite `domain-terms.md` to the term schema with 13 rows: nine
      confirmed (glossary definitions, contrasts, first uses, recorded
      evidence), two rejected (Repository root, Index / staging area), two
      candidates (Hook, Check-in).
- [x] COMMIT: `b43c6c49` -- `docs(term-introduction-discipline): migrate the multi-scm-support term records`

## Verification

- [x] The file parses as a six-column table with 13 rows; the confirmed rows
      carry the 2026-09-18 evidence.
- [x] `./run-tests.sh` and `./run-tests.sh --e2e` pass. Evidence:
      ```
      Summary [31.021s] 539 tests run: 539 passed, 0 skipped
      Summary [5.4s] 88 tests run: 88 passed, 451 skipped
      ```
- [x] COMMIT: `9f751c97` -- `docs(migrate-term-records): complete the review`
- [x] Archive the change.
