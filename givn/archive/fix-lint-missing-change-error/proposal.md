# Proposal: fix-lint-missing-change-error

## Problem / Opportunity

When `givn lint --change <id>` is invoked with an id that does not correspond
to any existing change directory, the command prints "no .feature files
found under givn/changes/<id>/specs/" and exits 0 — indistinguishable from
a change that exists but has no spec files. This gives the user no signal
that the change id itself is wrong (typo, never created, already archived).

The characterization scenario "A non-existent change id is not surfaced"
captures this AS-IS (buggy) behaviour.

## Proposed Solution

When `givn lint --change <id>` is given an id for which
`givn/changes/<id>/` does not exist, the command exits non-zero and prints
an error to stderr naming the missing change. This distinguishes a missing
change from a change that exists but has no specs (the latter remains exit 0
with the "no .feature files found" message).

The scenario "A non-existent change id is not surfaced" is modified to
assert exit non-zero and an stderr error mentioning the missing change id.
The scenario "--change with no specs directory reports no files and exits 0"
continues to pass unchanged — an existing change with no specs is still
exit 0.

## Out of Scope

- The severity-ranking inversion (separate change:
  `fix-lint-severity-ranking`).
- The `@givn.removed` over-broad suppression bug (separate change:
  `fix-lint-removed-suppression`).
- The `std::process::exit` architectural note (separate change:
  `fix-lint-exit-handling`).
- The exact exit code for a missing change (1 vs 2) — left to design, but it
  must be non-zero.

## Open Questions

Should a missing change exit 1 (same as a parse error) or a distinct code?
Default assumption: exit 1 (error), since this is an invocation error, not a
lint finding.
