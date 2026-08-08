# Design Review: add-coverage-readme-example

## Review

- **Scope**: Proposal/spec/design all agree — add an illustrative example to the archive guidance.
- **Spec observability**: The scenario runs `givn skills sync` in a real temp project and inspects the generated skill for the expected example text. RED is achievable: fail the assertion if the example is missing.
- **E2E validity**: CLI interface via real subprocess, matches interaction matrix. No browser/API/database needed.
- **Strict mode**: `.fail_on_skipped()` is proven in the existing runner; new steps must contain real assertions.
- **Source-of-truth**: `assets/skills/givn-archive/SKILL.md.tmpl` is the canonical template; `givn skills sync` propagates to all three skill targets. Design references all four paths.
- **Coverage markers**: The example sits under the existing `## Post-archive: README update` rule text. The `<!-- givn:begin:coverage -->` markers remain the mechanism for generated runtime coverage output — the illustrative example is not wrapped in those markers and is labelled "Example".
- **arc42**: No chapter is affected (pure documentation/guidance change to skill template).
- **Missing scenarios**: No uncovered behaviours — the capability has exactly one observable action (skills sync) and one scenario.

## Result

DESIGN-REVIEW: PASS
