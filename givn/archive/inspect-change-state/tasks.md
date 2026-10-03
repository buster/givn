# Tasks: inspect-change-state

Domain constraints: status reports `SCM: git|fossil|none`; a project without a backend still exits 0 and names archive and managed upgrade as unavailable.

## Setup

- [x] Runner and strict mode unchanged.

## Scenario: Status names the active SCM backend

- [x] RED: the SCM line did not exist; the outline failed before the change
      (`SCM: git` missing from stdout).
- [x] GREEN: `print_scm_line` renders the selected backend on every
      human-readable status path; the Git and Fossil examples pass. Targeted
      run: 2 scenarios (2 passed), exit 0.
- [x] COMMIT: fab0e96a (the backend-outline implementation slice).

## Scenario: Status without a backend still succeeds and names the unavailable commands

- [x] RED: shared with the outline RED (the line did not exist).
- [x] GREEN: `SCM: none (archive and managed upgrade are unavailable)`; exit
      0. Targeted run green.
- [x] COMMIT: fab0e96a (the no-backend scenario's implementation slice).

## Scenario: Multiple active changes listed alphabetically — text mode (aligned)

- [x] GREEN: the positional assertions move to lines 4/5 (the SCM line shifts
      every human-readable line by one); the permanent feature is aligned in
      the same commit; the inspect-change-state suite passes 34 scenarios.
- [x] COMMIT: fab0e96a with the permanent-alignment scope (shared
      implementation slice).

## Finalize

- [x] Full suite green; lint exits 0; all tasks checked.
