# Proposal: fix-lint-severity-ranking

## Problem / Opportunity

When `givn lint` checks a project that contains both a parse error (in one
file) and a `@wip`-tagged scenario (in another file), the command exits with
code 2 (the `@wip` exit code) instead of code 1 (the parse-error exit code).

The source doc comment on `run()` states "Parse errors outrank wip/PENDING",
documenting the intent that a parse error is the most severe finding and
should determine the exit code when both kinds are present. The code assigns
`ParseError -> 1u8` and `Wip -> 2u8` and takes the maximum, which inverts
that intent — `@wip` (2) outranks parse errors (1).

The characterization scenario "A @wip finding outranks a parse error"
captures this AS-IS behaviour. This change modifies that scenario to assert
the intended behaviour: parse errors outrank `@wip`.

## Proposed Solution

When both parse errors and `@wip`/PENDING findings are present (across one
or more files), the exit code is 1 (parse error), because a parse error is
the most severe finding and must not be masked by a `@wip` finding. When only
`@wip`/PENDING findings are present, the exit code remains 2. When only
parse errors are present, the exit code remains 1. The clean case remains 0.

The scenario "A @wip finding outranks a parse error" is modified to assert
exit code 1 instead of 2.

## Out of Scope

- The `@givn.removed` over-broad suppression bug (separate change:
  `fix-lint-removed-suppression`).
- The `--change <non-existent-id>` silent-exit bug (separate change:
  `fix-lint-missing-change-error`).
- The `std::process::exit` architectural note (separate change:
  `fix-lint-exit-handling`).
- No change to the stdout/stderr message format — only the exit code logic
  changes.

## Open Questions

None. The intended behaviour is documented in the source doc comment and is
unambiguous.
