# Tasks: mask-rule-names

Owning contract: `initialize-project`, capability `initialize-a-project`.
Confirmed Personas: `developer--primary-user`,
`external-contributor--open-source-consumer`, `givn-maintainer--owner`.

## Setup

- [x] Change scaffolded; proposal, specs, design, design-review, arc42
      complete.
- [x] Runner strict mode: `.fail_on_skipped()` (existing); the modified
      scenario fails RED against the per-path block.
- [x] Planning artifacts committed. COMMIT: `<plan-hash>` -- `docs(mask-rule-names): plan the change`

## Implementation

- [x] RED/GREEN: the ignore block carries prefix patterns, masks the rule
      names, and still ignores the rules; the negative assertion is
      substring- and block-scoped; unit tests prove every rule shape is
      covered and user content is not. Files: `src/skills/rules.rs`,
      `tests/steps/local_only_rules_steps.rs`,
      `specs/initialize-project/initialize-a-project.feature`,
      `docs/arc42/08-crosscutting-concepts.md`.
      Targeted run: 1 scenario passes; lib tests 268 pass.
- [x] COMMIT: `<impl-hash>` -- `feat(initialize-a-project): mask generated rule names in the ignore block`

## Verification

- [x] Delta suite — green. Evidence: `1 test run: 1 passed, 0 skipped`.
- [x] `cargo fmt` and `cargo clippy -D warnings` — clean.
- [x] `cargo nextest run --lib` — `268 tests run: 268 passed, 1 skipped`.
- [x] Review artifact complete and signed off.
- [x] Archive the change; the archive verifies the merged candidate and
      records the receipt.
