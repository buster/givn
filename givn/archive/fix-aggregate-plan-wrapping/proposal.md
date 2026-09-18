# Proposal: fix-aggregate-plan-wrapping

## Use-Case Context

- Use-case ID: none (project-maintenance repair; `specs` is skipped)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: none — the fix corrects generated prose; no
  user-visible surface changes.

## Problem / Opportunity

The aggregate migration plan's proposal wrapper in `src/cli/migration.rs`
drops the space at four Rust line continuations, so every generated consumer
plan reads "Givn 0.7.0 to theinstalled Givn 0.8.0", "recorded inCommit A",
"plan; workeach phase", and "It mustpreserve project-owned overrides". The
defect was found while dogfooding the 0.8.0 migration in this repository; the
design wrapper was fixed earlier, the proposal wrapper was missed.

## Proposed Solution

Add the missing trailing spaces before the four continuations and add a unit
test asserting the generated proposal contains the correctly spaced phrases
and none of the four joined forms.

## Out of Scope

- Regenerating any already-created plan; the text is cosmetic.
- The design wrapper (already correct).

## Open Questions

None.
