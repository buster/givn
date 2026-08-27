# Migration Design: 0.3.0 to 0.4.0

This bundle is the release-owned migration phase for projects upgrading to
Givn 0.4.0. Givn 0.4.0 is delivery tooling: it adds the CI, security,
prepare-release, and release workflows, the git-cliff generated changelog, and
corrected repository metadata. It changes no consumer-facing file format and no
Cargo feature contract, so this phase verifies rather than migrates.

When this bundle is materialized, `{{SOURCE_VERSION}}` is the project's current
config version and `{{TARGET_VERSION}}` is `0.4.0`. The managed preparation
commit is `{{UPGRADE_COMMIT}}`.

## Prompt 1: Inventory the managed 0.4.0 upgrade

Inspect the consumer project before editing:

- `givn/config.yaml` `givn_config_version:` — confirm it identifies
  `{{TARGET_VERSION}}`.
- `givn/commands.yaml` and the generated agent integrations — confirm they
  match the current Givn contract.
- The managed upgrade commit `{{UPGRADE_COMMIT}}` — confirm it stages only the
  allowed managed paths (config scaffold, commands, generated skills/commands)
  and nothing user-owned.
- `git status --porcelain` — confirm the project worktree is otherwise clean
  before the migration-change commit.

Record what you inspected and what you found. Do not edit anything yet.

## Prompt 2: Verify no consumer migration is required

Givn 0.4.0 introduces no domain, configuration, specification, or architecture
migration for consumer projects. Verify, using project evidence, that:

- No project specification, step definition, ADR, Arc42 fact, or user-authored
  document needs a 0.4.0 edit.
- No Cargo feature or installation-mode contract changed for consumers.
- The 0.4.0 delivery pipeline (`.github/workflows/`, `cliff.toml`,
  `deny.toml`, `dependabot.yml`) is repository tooling, not consumer project
  content — do not copy it into the consumer project.

If any project-owned surface genuinely needs attention, record it as a
project-specific task with evidence; do not invent work that the project does
not have.

## Prompt 3: Verify and close

- Run `givn status` and confirm the project is healthy.
- If `{{SOURCE_VERSION}}` differs from a previous line, confirm the 0.4.0
  managed scaffold supersedes it without leftover version markers.
- Derive project-specific tasks for this maintenance change, pass its
  design-review and review gates, and archive it when complete.
- Do not add `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence
  to the generated plan; those are earned through the normal Givn gates.

## Required Boundaries

- Do not automatically edit domain specifications or architecture decisions.
- Do not add a product feature spec for this maintenance phase.
- Do not remove `@e2e`, semantic evidence, or review gates to obtain a pass.
- This phase is ordered after the 0.2.0-to-0.3.0 phase in an aggregate plan.
