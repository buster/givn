# Tasks: make-usecases-canonical

RED→GREEN→REFACTOR→COMMIT per scenario.

## Domain Constraints

| Scenario / capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| canonical-usecase-contract | A capability has one explicit use-case owner; reusable behavior is a fragment. | none |

## Tasks

- [x] Setup: strict Cucumber execution is configured by `.fail_on_skipped()`;
      existing feature runner evidence is recorded by the repository suite.
- [x] Scenario: `Use-case tree reports explicit use-case ownership`
  - RED/GREEN/REFACTOR: implemented use-case parser, tree output, and migrated
    corpus; `cargo test --test features_runner` passed.
  - Files: `src/gherkin/usecase_md.rs`, `src/spec/tree.rs`, corpus documents.
  - [x] COMMIT: 92975e3e694e04475e84ae9ab71b4fb522e4311f — feat: promote use-case specifications and ideation

## Follow-up implementation sessions

- [x] Strict parser and fragment schema.
- [x] Discovery, routing, graph, interaction, and lint migration.
- [x] Atomic archive narrative replacement and stale-base rejection.
- [x] Coverage inventory and migration comparison commands.
- [x] Ideation, Persona, workflow, template, and generated guidance promotion.

## Evidence

- `cargo test --lib`
- `cargo test --test features_runner`
- `cargo run -- lint --json`
- `cargo run -- spec coverage --format json`
