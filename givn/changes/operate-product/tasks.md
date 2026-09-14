# Tasks: operate-product

Domain constraints: backend-neutral wording (Q10) for the upgrade-revision content assertions; no runtime behavior and no new scenario.

## Setup

- [x] Runner and strict mode unchanged.

## Scenario: The upgrade commit includes the backfilled run declaration

- [x] RED: the renamed Then was unbound before the binding rename
      (undefined step, non-zero).
- [x] GREEN: the binding is renamed with identical assertions; targeted run
      green.
- [x] COMMIT: recorded below.

## Scenario: A repeated upgrade creates no additional commit

- [x] RED: shared with the rename.
- [x] GREEN: renamed binding; targeted run green.
- [x] COMMIT: recorded below.

## Finalize

- [x] Full suite green; lint exits 0; all tasks checked.
