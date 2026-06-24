REVIEW: PASS

## Characterization Summary

Reverse-engineered the config-overlay module (`overlay::apply`). All 13 scenarios
characterize the existing behaviour correctly. One spec correction was needed:
Scenario 5 (artifact removal) required updating `apply.requires` in the overlay
to maintain validation invariants.

## Spec Corrections

- Scenario 5: overlay must update `apply.requires` when removing an artifact
  referenced there. Original spec was incomplete.

## Code Smells

None identified during characterization.

## Suspected Bugs

None.
