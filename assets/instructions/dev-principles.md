# Instruction: dev-principles

Apply the team's engineering principles whenever code is read or written
during a givn change.

For every user-facing question, follow the effective policy from
`givn instructions dialog`; load it once per context before the first question.

## Principles

- **Correctness first, then the next maintainer.** Optimise for the reader six
  months out, not for cleverness today.
- **Match the TDD loop.** RED (real failing test) -> GREEN (minimum code) ->
  REFACTOR (no behaviour change). Never skip RED; never suppress a test to pass.
- **Boring when called for.** Prefer established patterns in the repo over new
  abstractions. A second convention beside an existing one is prohibited.
- **No avoidable allocation or computation.** Consider what code compiles to.
- **Fix problems at the source.** Remove obsolete code — no leftover aliases,
  re-exports, or commented-out scaffolding.
- **You are not alone.** Treat unexpected changes as someone else's work and
  adapt; coordinate before editing a file a peer may own.

## Anti-patterns

- Inventing a generic abstraction to serve one call site.
- Suppressing a warning or special-casing an input instead of fixing the cause.
- Expanding scope "while you're at it" — retries, telemetry, or validation the
  change does not require.
