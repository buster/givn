# Tasks: start-and-proportionate-work

Owning use case: `obtain-effective-guidance`
(`givn/specs/obtain-effective-guidance/usecase.md`). Supporting owners:
`initialize-project`, `inspect-change-state`, and the reusable `fragments`
document. Personas: `agent--technical-role`, `developer--primary-user`,
`givn-maintainer--owner`.

Release note: New and unclassified work now starts with a proportionate recommendation, while an existing change remains an explicit continuation rather than an assignment.

## Setup

- [x] Confirm the existing runner and strict mode; no new runner is installed.
      Evidence: `tests/features_runner.rs` calls
      `GivnWorld::cucumber().fail_on_skipped()` and exits non-zero on failed or
      skipped scenarios; `givn/commands.yaml` sets
      `verify.command = ./run-tests.sh --non-e2e` and
      `verify.e2e_command = ./run-tests.sh --e2e`.
- [x] Confirm the step definition files named by `design.md` exist and remain
      the homes for this change:
      `tests/steps/serve_extended_rules_steps.rs`,
      `tests/steps/prose_consolidation_steps.rs`,
      `tests/steps/initialize_a_project_e2e_steps.rs`,
      `tests/steps/instruction_integrity_steps.rs`.
- [x] No new E2E action is added. The modified startup scenario keeps the
      existing `givn instructions` subprocess and its terminal transcript.

## Batch A — start guidance and ideation comparison

Design constraints for every scenario in this batch:

- Add `assets/instructions/start.md`; delete `assets/instructions/explore.md`.
  The start instruction must contain the four outcome labels, the
  proportionality judgment, the direct-work obligations, the authorization
  boundary, and the no-artifact rule.
- Required exact phrases: `Answer and stop`, `Direct work without a Givn change`,
  `givn-ideation`, `givn-propose`, `proportionality`, `creates no workflow
  artifacts`, `do not assume every modification requires a Givn change`,
  `Read the relevant code`, `focused correction`, `regression coverage`,
  `Report the modification and verification evidence`, `Do not create a
  proposal, design, task list, or archive ceremony`, `authorization to edit is
  not authorization to commit`, `wait unless continuation is already
  authorized`, `An ordinary request to discuss is not authorization`, and
  `Direct entry remains available`.
- Add `start` to `POLICY_INSTRUCTION_IDS` in `src/cli/instructions.rs`.
- Delete the served explore instruction asset; `givn instructions explore` must
  fail before any project override is consulted.
- Update `assets/instructions/ideation.md` so the comparison names
  `givn-start`, not `givn-explore`, and the start instruction names
  `givn-ideation`.
- Contract: Effective guidance — implemented only by `src/cli/instructions.rs`
  plus the embedded instruction assets.

### Scenario: The start guidance recommends a proportionate outcome

- [x] Scenario: The start guidance recommends a proportionate outcome
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: The start guidance preserves direct-work obligations

- [x] Scenario: The start guidance preserves direct-work obligations
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: The start guidance keeps the handoff with the user

- [x] Scenario: The start guidance keeps the handoff with the user
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: The start guidance is available without optional addons

- [x] Scenario: The start guidance is available without optional addons
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: The retired explore instruction has no callable alias

- [x] Scenario: The retired explore instruction has no callable alias
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: A project-owned explore override cannot restore a callable alias

- [x] Scenario: A project-owned explore override cannot restore a callable alias
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: The start guidance points to ideation for structured domain work

- [x] Scenario: The start guidance points to ideation for structured domain work
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: The ideation guidance compares start rather than explore

- [x] Scenario: The ideation guidance compares start rather than explore
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

## Batch B — no-active and active startup routing

Design constraints for every scenario in this batch:

- `src/cli/instructions.rs`:
  - `generic_overview` adds: for new or unclassified work, load `givn-start`;
    a reported active-change step applies only when continuing that change and
    does not authorize unrelated work; direct entry to ideation/propose remains.
  - `report_no_active_changes`: with no unproven candidate, text recommends
    loading `givn-start` and names `givn new <change-id>` as the direct
    alternative. JSON carries `next_skill: "givn-start"` and keeps
    `next_command: "givn new <change-id>"`.
  - With an unproven candidate, keep `givn archive --change <id>` as the next
    command and set `next_skill: "givn-archive"`.
  - `print_json` accepts an optional `next_skill`.
  - `report_change` text adds: `This step applies when continuing this change;
    it does not authorize unrelated work.`
- Keep the active-change JSON `next_command` / `next_skill` pair unchanged.
- Required JSON fields: `status`, `summary`, `next_command`, `next_skill`,
  `overview`; the direct command must be described as optional in the summary.
- Contract: Effective guidance — implemented only by `src/cli/instructions.rs`.

### Scenario: Machine-readable startup guidance retains the dialog obligation while reporting an active change

- [x] Scenario: Machine-readable startup guidance retains the dialog obligation while reporting an active change
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: Machine-readable no-active startup guidance carries the start route

- [x] Scenario: Machine-readable no-active startup guidance carries the start route
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

### Scenario: The active-change startup guidance qualifies its next step

- [x] Scenario: The active-change startup guidance qualifies its next step
  - [x] COMMIT: 324ccc15 -- Batch: start-entry-routing

## Batch C — skill generation, retirement, and onboarding

Design constraints for every scenario in this batch:

- Add `assets/skills/givn-start/SKILL.md.tmpl`; delete the explore skill
  template. The skill is thin and points to `givn instructions start`.
- `src/skills/render.rs::default_skill_names` replaces `givn-explore` with
  `givn-start`.
- `src/skills/targets.rs`:
  - `RETIRED_MANAGED_SKILLS = ["givn-explore"]`.
  - Recognition requires the generated frontmatter name and the body pointer
    `givn instructions givn-explore` or `givn instructions explore`; a custom
    file is preserved and reported.
  - `reconcile_retired_skills` removes recognizable target skill directories
    and empty parents; prints `manual transition: preserved <path>` otherwise.
  - `legacy_command_names` explicitly includes `givn-explore`.
  - `retired_managed_generated_paths` returns configured target skill/command
    paths for upgrade staging.
  - `report_retired_explore_overrides` prints a manual transition for
    `givn/skills/givn-explore/SKILL.md.tmpl` and
    `givn/artifacts/explore/instructions.md`, without editing them.
- `src/cli/migration.rs`: snapshot includes retired paths; under the versioned
  rules policy, stage a tracked retired path even when it is missing from disk.
  Local-only policy leaves it ignored.
- `src/cli/init.rs` onboarding: name `givn-start`, the four outcomes, and
  `givn new` as the direct alternative; remove `givn-explore`.
- Contract: Generated guidance — `src/skills/targets.rs` and
  `src/skills/render.rs`; Managed upgrade — `src/cli/migration.rs`.

### Scenario: Fresh initialization generates start instead of explore

- [x] Scenario: Fresh initialization generates start instead of explore
  - [x] COMMIT: 2df5429a -- Batch: skill-retirement

### Scenario: Synchronization removes a recognizable generated explore skill

- [x] Scenario: Synchronization removes a recognizable generated explore skill
  - [x] COMMIT: 2df5429a -- Batch: skill-retirement

### Scenario: Synchronization preserves a custom explore skill and reports a manual transition

- [x] Scenario: Synchronization preserves a custom explore skill and reports a manual transition
  - [x] COMMIT: 2df5429a -- Batch: skill-retirement

### Scenario: Synchronization still removes a recognizable generated explore command

- [x] Scenario: Synchronization still removes a recognizable generated explore command
  - [x] COMMIT: 2df5429a -- Batch: skill-retirement

### Scenario: Synchronization reports a project-owned explore override without changing it

- [x] Scenario: Synchronization reports a project-owned explore override without changing it
  - [x] COMMIT: 2df5429a -- Batch: skill-retirement

### Scenario: Upgrade removes a tracked generated explore skill under the versioned rules policy

- [x] Scenario: Upgrade removes a tracked generated explore skill under the versioned rules policy
  - [x] COMMIT: 2df5429a -- Batch: skill-retirement

### Scenario: Initialization onboarding names the proportionate entry point

- [x] Scenario: Initialization onboarding names the proportionate entry point
  - [x] COMMIT: 2df5429a -- Batch: skill-retirement

## Batch D — documentation closeout

- [ ] Update `assets/agents-project-hint.md`, `README.md`,
      `docs/reference.md`, `docs/workflow.md`, `docs/workflow.svg`,
      `docs/arc42/03-context-and-scope.md`,
      `docs/arc42/04-solution-strategy.md`,
      `docs/arc42/05-building-block-view.md`,
      `docs/arc42/06-runtime-view.md`,
      `docs/arc42/08-crosscutting-concepts.md`,
      `docs/arc42/11-risks-and-technical-debt.md`, and `docs/glossary.md`.
- [ ] Confirm every delta step phrase has a permanent binding and no
      unimplemented stub remains.
- [ ] COMMIT: <hash> -- Batch: start-documentation-closeout

## Batch E — @e2e

Design constraints: the modified startup scenario keeps the existing real
`givn instructions` subprocess. It asserts the dialog obligation, the start
route for new/unclassified work, the direct `givn new` alternative, and the
terminal transcript. Do not replace the subprocess with an in-process call.

### Scenario: Author starts a session with the universal dialog obligation

- [ ] Scenario: Author starts a session with the universal dialog obligation
  - [ ] COMMIT: <hash> -- Batch: start-entry-e2e

### Scenario: Consumer runs init non-interactively and gets opencode skills

- [ ] Scenario: Consumer runs init non-interactively and gets opencode skills
  - [ ] COMMIT: <hash> -- Batch: start-entry-e2e
