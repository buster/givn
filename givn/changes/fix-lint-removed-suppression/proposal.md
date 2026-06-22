# Proposal: fix-lint-removed-suppression

## Problem / Opportunity

`givn lint` suppresses `@wip` findings on `@givn.removed` placeholder
scenarios — these are scenarios that will be deleted when the change is
archived, so flagging them as work-in-progress is noise. However, the
suppression check is over-broad: it compares `s.position.line <= wip_line`
against ANY removed scenario in the feature, regardless of whether that
removed scenario is the one tagged `@wip`. A removed scenario near the top
of the file suppresses an unrelated `@wip` scenario anywhere later in the
same file.

The characterization scenario "A @givn.removed scenario suppresses an
unrelated later @wip scenario" captures this AS-IS (buggy) behaviour: a
`@givn.removed` scenario at an earlier line silences a `@wip` tag on a
separate, later, non-removed scenario.

## Proposed Solution

A `@wip` finding is suppressed only when the `@wip` tag belongs to the
`@givn.removed` scenario itself — i.e. the removed scenario IS the one
tagged `@wip`. A `@wip` tag on a non-removed scenario is reported normally,
even if a `@givn.removed` scenario appears earlier in the same file.

The scenario "A @givn.removed scenario suppresses an unrelated later @wip
scenario" is modified to assert that the unrelated `@wip` IS reported (exit
code 2, finding on stderr). The scenario "A @givn.removed placeholder
suppresses the @wip warning on itself" continues to pass unchanged — a
`@givn.removed @wip` scenario is still suppressed.

## Out of Scope

- The severity-ranking inversion (separate change:
  `fix-lint-severity-ranking`).
- The `--change <non-existent-id>` silent-exit bug (separate change:
  `fix-lint-missing-change-error`).
- The `std::process::exit` architectural note (separate change:
  `fix-lint-exit-handling`).

## Open Questions

None. The intended behaviour is clear: suppression applies only to the
removed scenario's own `@wip` tag.
