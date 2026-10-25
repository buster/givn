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
- **A red test fails for the reason it claims.** Discovery, registration,
  binding, import, fixture, and environment failures are not RED. A green
  first run is a design signal: covered, parameterize, fix the setup, or delete.
- **Name the operative source.** When documents disagree, declare which one
  governs and mark the other stale at its entry point. Never leave two
  unqualified authorities.
- **External boundaries are testable.** Every trust boundary gets adversarial
  cases: wrong identity, malformed shape, timeout, partial response, replay.
  Never classify external-input validation as "defensive."
- **Define the measured population before measuring.** Do not change
  exclusions after seeing the number to hit a threshold.
- **Stop when uncertainty cannot change the decision.** Do not reverse-engineer
  private validators or hidden graders beyond the documented contract.
- **A prior violation is not a justification.** Evaluate against the rule, not
  the archive.
- **Boring when called for.** Prefer established patterns in the repo over new
  abstractions. A second convention beside an existing one is prohibited.
- **No avoidable allocation or computation.** Consider what code compiles to.
- **Fix problems at the source.** Remove obsolete code — no leftover aliases,
  re-exports, or commented-out scaffolding.
- **You are not alone.** Treat unexpected changes as someone else's work and
  adapt; coordinate before editing a file a peer may own.
- **Code is owned by everyone.** Seeing technical debt and leaving it because
  "it was not me" is not acceptable. Leave existing code better than you found
  it whenever you touch it.

## Anti-patterns

- Inventing a generic abstraction to serve one call site.
- Suppressing a warning or special-casing an input instead of fixing the cause.
- Expanding scope "while you're at it" — retries, telemetry, or validation the
  change does not require.
- "Not my code" — leaving a defect or debt in a file because someone else wrote
  it.

## Examples

| Principle | Bad | Good |
|---|---|---|
| Operative source | `law.md` says one sense, `task.md` says two, both current. | `task.md` governs; `law.md` has a dated stale note pointing to it. |
| Causal RED | `scenario is not registered` used as RED. | `expected full, got provisional` used as RED. |
| External boundary | Provider licence-id mismatch called “defensive.” | Fake provider returns another provider's licence id. |
| Measured population | Coverage is 83.9%, so exclude another file. | Exclusions are declared in design.md before the first run. |
| Stop rule | 24 binary-string probes to infer a parser. | One minimal reproducer, then a documented assumption. |
| Precedent | “The archived review did it.” | “The rule requires a production diff; this commit has none.” |
