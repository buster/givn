# Proposal: ci-debuginfo-optimization

## Problem / Opportunity

The CI pipeline fails on a clean runner: the acceptance-suite step runs out of
disk space while building the test binaries, so valid changes show a red CI
without any code defect. The debug build carries full debug information, which
inflates every test binary and every intermediate artifact several-fold and
fills the runner disk before the suite completes.

## Proposed Solution

The CI pipeline completes its acceptance suite within the runner's disk budget
without changing any externally observable behaviour:

- A development build still honours fixture requests, and an installed build
  still ignores (and announces) them — the development/installed distinction
  is unaffected.
- Coverage measurement still reports line coverage over the same scenario
  groups.
- The CI acceptance suite finishes successfully on the standard runner.

## Out of Scope

- No change to observable givn command behaviour, spec coverage, or gate
  decisions.
- No change to the release build: installed builds keep their current
  characteristics.
- No new feature or scenario semantics.

## Open Questions

- None.
