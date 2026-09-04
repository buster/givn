# Design Review: group-specs-by-use-case

## Coverage

| Question | Answer | Where |
|---|---|---|
| New components? | None — reorganization only | design.md §Overview |
| Step definitions? | None — no scenario content | design.md §Step Definitions |
| Layout correctness after move? | archive resolves groups; overlap splits capability/group; both shipped and tested before this change | proposal.md |
| git history? | All moves via `git mv` (renames, not delete+add) | design.md §Technology decisions |
| Group prose quality? | Real Actor/Goal/Main flow/Interactions per group, no placeholder prose left | design.md §Technology decisions |
| Singletons? | None remain (smallest groups have 2+ members) | proposal.md |

## Stress-test

- **What if a scenario references spec file paths by the old flat path?**
  Step definitions in `tests/steps/` reference `givn/specs/...` paths for
  a few self-referential capabilities. The full suite after the move is
  the arbiter — any broken path shows up as a failure and must be fixed
  in the same change.
- **What breaks if archive runs mid-migration?** The archive backup +
  rollback covers the whole tree; the migration is a single commit, so
  the tree is never half-moved at rest.

DESIGN-REVIEW: PASS
