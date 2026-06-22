# Proposal: fix-lint-exit-handling

## Problem / Opportunity

The source comment in `lint::run()` states "actual process exit is handled
by mapping to a non-zero return in main", but the function calls
`std::process::exit(1)` and `std::process::exit(2)` directly, bypassing
`main`'s `anyhow` error handler. This makes the comment misleading and
prevents `main` from owning the process exit strategy (e.g. for consistent
error reporting, testability, or future hooks like coverage of non-zero
exit paths).

This is an architectural note rather than an observable bug — the exit
codes themselves (0/1/2) are correct and documented. The issue is the
mechanism: `run()` performs a hard process exit instead of returning a value
that `main` maps to an exit code.

## Proposed Solution

`lint::run()` returns a result or exit-code value rather than calling
`std::process::exit` directly. `main` (or the dispatch layer) maps the
returned value to the process exit code (0/1/2). The observable exit codes
remain identical — no scenario's exit-code assertion changes. The source
comment is updated (or removed) to match the new mechanism.

No scenario is modified — the exit codes are already correct. If a new
scenario is desired to protect the refactor, it would duplicate the existing
exit-code scenarios, which is unnecessary. The behaviour-preservation
invariant (all existing scenarios stay GREEN) is the safeguard.

## Out of Scope

- The severity-ranking inversion (separate change:
  `fix-lint-severity-ranking`).
- The `@givn.removed` over-broad suppression bug (separate change:
  `fix-lint-removed-suppression`).
- The `--change <non-existent-id>` silent-exit bug (separate change:
  `fix-lint-missing-change-error`).
- Changing the exit-code contract (0/1/2) — only the mechanism changes.

## Open Questions

Should `run()` return `Result<i32>` (exit code) or a typed enum
(`LintOutcome { Clean, ParseError, Wip }`)? The enum is clearer but
introduces a new public type. Default assumption: `Result<i32>` is
sufficient and minimal.
