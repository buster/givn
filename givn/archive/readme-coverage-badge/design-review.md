# Design Review: readme-coverage-badge

## Review

- **Scope**: Proposal/spec/design agree — badge moves to the top of the README in templates and instructions.
- **Spec observability**: Scenario runs `givn skills sync` in a real temp project and inspects the generated archive skill for the leading-badge guidance. RED achievable by failing the assertion when the guidance is absent.
- **E2E validity**: CLI interface via real subprocess; matches the interaction matrix. No browser/API/database.
- **Strict mode**: `.fail_on_skipped()` proven in the existing runner; new steps assert real content.
- **Source of truth**: `assets/skills/givn-archive/SKILL.md.tmpl` is canonical; sync regenerates all three skill targets.
- **Coverage markers**: The managed `<!-- givn:begin:coverage -->` fence handles the runtime summary; the badge guidance sits in the illustrative example.
- **arc42**: No chapter affected (documentation/guidance change).
- **Missing scenarios**: Exactly one observable action (skills sync); one scenario covers it.

## Result

DESIGN-REVIEW: PASS
