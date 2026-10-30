# Design: start-and-proportionate-work

## Overview

The change replaces the `givn-explore` entry point with `givn-start`, changes what the session-start guidance recommends when no change is active, and removes recognizable generated explore output from existing projects.

Three boundaries define the work:

1. **Guidance boundary.** `assets/instructions/start.md` owns the four outcomes, proportionality, direct-work obligations, authorization, and handoff. `givn instructions` reports state separately from request routing.
2. **Generation boundary.** `givn-start` replaces `givn-explore` in the default skill list. Sync removes recognizable generated explore skills and legacy commands, preserves project-owned files, and surfaces a manual transition for unrecognized content. Upgrade can stage the deletion of tracked generated explore output.
3. **Entry-point documentation boundary.** Onboarding, README, reference, workflow, the workflow diagram, the managed project-start block, and the ideation comparison describe the same behavior.

## Visual Design Contract

- Interface: terminal
- Viewports: not applicable
- Design system: not applicable
- Persona: agent--technical-role, developer--primary-user, givn-maintainer--owner

The product surface is served CLI text and Markdown. No browser screen is rendered. The changed `@e2e` startup scenario commits a transcript under `givn/changes/start-and-proportionate-work/evidence/visual/<scenario-slug>/transcript.txt`.

## Use-Case Traceability

Owning use case: `obtain-effective-guidance` (`givn/specs/obtain-effective-guidance/usecase.md`). Supporting owners: `initialize-project`, `inspect-change-state`, and the reusable `fragments` document.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| The served content is the effective guidance resolved through the cascade | `start` is a policy instruction resolved before change context; project overrides still win | `serve-extended-rules`, `The start guidance recommends a proportionate outcome` |
| The built-in rules stay technology-agnostic | Start contains no runtime classifier, score, or project-specific rule | `serve-extended-rules`, `The start guidance preserves direct-work obligations` |
| A request never changes project state | Start creates no artifacts; `givn instructions` resolves and prints | `serve-extended-rules`, `The start guidance keeps the handoff with the user` |
| The next required action remains truthful | Active-change output keeps its `next_command` / `next_skill` pair and adds the request-scope qualification | `instruction-integrity`, `The active-change startup guidance qualifies its next step` |
| Session-start guidance separates state from request routing | No-active text recommends start and names the direct `givn new` alternative; JSON carries `next_skill` | `instruction-integrity`, `Machine-readable no-active startup guidance carries the start route` |
| Skills are the only generated invocation surface | `givn-start` is a thin skill pointing to `givn instructions start`; no explore alias | `refresh-generated-guidance`, `Fresh initialization generates start instead of explore` |
| Generated guidance remains recognizable and replaceable | Retired explore output is removed only when recognizable; project content is preserved | `refresh-generated-guidance`, `Synchronization preserves a custom explore skill and reports a manual transition` |
| Initialization reports the entry point truthfully | Onboarding names start and the direct alternative | `initialize-a-project`, `Initialization onboarding names the proportionate entry point` |
| Ideation remains a direct entry | The comparison names start; ideation does not depend on start | `generate-ideation-guidance`, `The ideation guidance compares start rather than explore` |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock`; no new dependency |
| cucumber-rs | existing workspace pin | `.fail_on_skipped()` remains the strict-mode mechanism |
| Git/Fossil | existing backend seam | Upgrade staging uses the existing `Backend::revision_file` and `stage_paths`; no new SCM command |
| Markdown templates | n/a | Existing `rust-embed` asset tree; one template added, one removed |

## Decisions

- Decides D1: no-active text recommends `givn-start` and names `givn new <change-id>` as the direct alternative; JSON carries `next_skill: "givn-start"` and keeps `next_command` — realized in `src/cli/instructions.rs` and the start instruction.
- Decides D2: sync removes only recognizable generated explore skills and legacy commands; project content is preserved with a manual transition notice — realized in `src/skills/targets.rs`.
- Decides D3: no automatic conversion of explore overrides; bytes are preserved and a manual notice points to start — realized in the retirement reconciliation and its tests.
- Decides D4: start is a default skill and policy instruction, independent of optional addons — realized in `default_skill_names` and `POLICY_INSTRUCTION_IDS`.

## Architecture Impact

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Effective guidance | obtain-effective-guidance | `givn instructions`, generated skills | `src/cli/instructions.rs` plus embedded instruction assets | A second content source or project-specific entry policy appears |
| Generated guidance | obtain-effective-guidance, initialize-project | `givn skills sync` | `src/skills/targets.rs` and `src/skills/render.rs`; this includes recognizable retired-skill removal | Skill generation needs a second source of managed-path policy |
| Project initialization | initialize-project | `givn init`, onboarding output | `src/cli/init.rs` | Onboarding needs a separate runtime router or persisted state |
| Managed upgrade | upgrade-a-project | `givn upgrade` | `src/cli/migration.rs`; this includes staging retired tracked paths | Upgrade needs a second managed-path policy source |

### Module changes

- `assets/skills/givn-start/SKILL.md.tmpl` (new) and `assets/instructions/start.md` (new). Delete `assets/skills/givn-explore/SKILL.md.tmpl` and `assets/instructions/explore.md`.
- `src/skills/render.rs`: replace `givn-explore` with `givn-start` in `default_skill_names`.
- `src/skills/targets.rs`:
  - `RETIRED_MANAGED_SKILLS: &[&str] = &["givn-explore"]`.
  - `is_generated_skill(content, name)` recognizes the frontmatter name and the `givn instructions <name>` pointer.
  - `reconcile_retired_skills` removes a target skill directory only when its `SKILL.md` is recognizable generated output; otherwise it prints `manual transition: preserved <path>`. Empty parent directories are removed.
  - `legacy_command_names` explicitly includes `givn-explore`, so old generated command files are still recognized after the default list changes.
  - `retired_managed_generated_paths(project_root)` returns the configured targets' retired skill and command paths for upgrade staging.
  - `report_retired_explore_overrides` prints a manual transition when `givn/skills/givn-explore/SKILL.md.tmpl` or `givn/artifacts/explore/instructions.md` exists; it never edits either file.
- `src/cli/instructions.rs`:
  - `POLICY_INSTRUCTION_IDS` adds `start`.
  - `RETIRED_INSTRUCTION_IDS` contains `explore`; a request for it is refused before the cascade with a message that names `givn instructions start`. A project-owned `givn/artifacts/explore/instructions.md` therefore cannot restore a callable alias.
  - `generic_overview` adds the request-scope qualification and the start/direct-work routing sentence.
  - `report_no_active_changes` text recommends start, names `givn new` as the direct alternative, and its JSON carries `next_skill: "givn-start"` plus `next_command: "givn new <change-id>"`.
  - `print_json` accepts an optional `next_skill`.
  - `report_change` text adds `This step applies when continuing this change; it does not authorize unrelated work.`
- `src/cli/init.rs`: onboarding names `givn-start` and the four outcomes, and no longer names `givn-explore`.
- `assets/agents-project-hint.md`: separates state discovery from request routing, names `givn-start`, and allows direct work.
- `src/cli/migration.rs`: snapshot includes retired managed paths; for a versioned rules policy, tracked retired paths are staged even when sync removed them from disk. Local-only policy leaves them ignored.
- Documentation: `README.md`, `docs/reference.md`, `docs/workflow.md`, `docs/workflow.svg`, `docs/glossary.md`, `docs/arc42/05-building-block-view.md`, `docs/arc42/06-runtime-view.md`, `docs/arc42/08-crosscutting-concepts.md`, `docs/arc42/11-risks-and-technical-debt.md`.

No persistent schema, manifest field, or command surface changes. The CLI still has no `givn start` subcommand.

## Data Model Changes

None. The only new file kinds are the start instruction and skill template.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `serve-extended-rules` | `tests/steps/serve_extended_rules_steps.rs` | same file |
| `refresh-generated-guidance` | `tests/steps/prose_consolidation_steps.rs` | same file |
| `initialize-a-project` | `tests/steps/initialize_a_project_e2e_steps.rs` | same file |
| `generate-ideation-guidance` | `tests/steps/serve_extended_rules_steps.rs` | same file |
| `instruction-integrity` | `tests/steps/instruction_integrity_steps.rs` | same file |

Test runner command:

```text
./run-tests.sh --non-e2e
```

Single-scenario command:

```text
GIVN_FEATURES=givn/changes/start-and-proportionate-work/specs cargo test --locked --test features_runner -- --name "<scenario title>"
```

Strict mode is unchanged: `tests/features_runner.rs` calls `.fail_on_skipped()`.

## Interaction Coverage Matrix

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `serve-extended-rules` — request effective instructions by id | Present | An agent receives the extended rules through the Givn CLI | Real `givn instructions` subprocess |
| `instruction-integrity` — request normal session-start instructions | Present | Author starts a session with the universal dialog obligation | Real `givn instructions` subprocess |
| `refresh-generated-guidance` — synchronize selected skills | Present | Synchronized guidance explains how to resolve the deterministic gates | Real `givn skills sync` subprocess; the retirement scenarios are regular variants |
| `initialize-a-project` — initialize with explicit noninteractive selection | Present | Consumer runs init non-interactively and gets opencode skills | Real `givn init --no-addons --agents opencode` subprocess |

No new E2E action is introduced. The startup transcript is the visual evidence.

## External Boundary Cases

| Boundary | Adversarial cases | Handling |
|---|---|---|
| Generated skill cleanup | Missing target; recognizable generated skill; custom skill; empty parent | Missing is fine; recognizable is removed; custom is preserved with a manual notice; empty parent removed |
| Legacy command cleanup | Generated command bytes; custom command bytes; missing command directory | Generated removed; custom preserved and reported; missing directory ignored |
| Upgrade staging | Local-only rules; versioned rules; tracked retired skill removed from disk; untracked retired skill | Local-only leaves the path ignored; versioned stages a tracked deletion; untracked missing path is skipped. The upgrade scenario exercises the versioned path with a committed explore skill. |
| No-active guidance | No changes ever; archived changes only; unproven archive candidate | Start/direct guidance for the first two; archive resume remains the truthful next command for a candidate |
| Active guidance | Active change plus unrelated request | The lifecycle step is reported with an explicit scope qualification |
| Retired instruction | `givn instructions explore`; project-owned explore override | Refused before the cascade; no alias even when an override exists |

## Local Runnability

No server, container, database, or external service is involved. The CLI is driven through the existing test harness.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Feature runner | `./run-tests.sh --non-e2e` / `--e2e` | `features_runner` and the `givn` binary | `target/llvm-cov-target` profraw | `merge-coverages.sh` | `src/cli/instructions.rs` no-active routing and `src/skills/targets.rs` retirement cleanup |
| Ready fixture | `run-tests.sh` before the suite | `ready_fixture` | same profraw set, discarded before measurement | n/a | fixture startup only |

Measured population: production source under `src/`. Excluded: tests, generated code, coverage scripts, and embedded Markdown assets.

## Out-of-Scope Technical Decisions

- No runtime intent classifier or routing score.
- No compatibility alias for `givn-explore` or automatic override conversion.
- No new shell command.
- No change to the ideation artifact model or proposal's downstream lifecycle.
