# Tasks: require-agent-target

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Release note (every revision's message subject):
`givn init` refuses to initialize a project unless at least one agent target is
selected, instead of silently installing OpenCode skills.

Use-case constraints carried into every scenario (from
`specs/initialize-project/usecase.md`):

- Initialization requires at least one selected agent target (rule).
- An empty or unknown-only selection refuses with an error naming the valid
  agent targets, exits non-zero, and writes nothing (extension 1).
- The project remains unchanged when a request is rejected (minimal guarantee).

Single-scenario runner (against the change delta only):

```sh
GIVN_FEATURES=givn/changes/require-agent-target/specs/ cargo test --locked --test features_runner -- --name "<scenario title>"
```

Delta-scope suite: `GIVN_FEATURES=givn/changes/require-agent-target/specs/ ./run-tests.sh`
E2E scope: `GIVN_FEATURES=givn/changes/require-agent-target/specs/ ./run-tests.sh --e2e`

## Setup

- [ ] Runner and strict mode confirmed. `verify.command` is `./run-tests.sh`
      (unchanged); the cucumber-rs builder sets `.fail_on_skipped()`
      (`tests/features_runner.rs:55,129`). Proof-of-strictness:
      `GIVN_FEATURES=/tmp/givn-strict-proof/ cargo test --locked --test features_runner -- --name "An undefined step must fail the runner"`
      → exit 1, `Step doesn't match any function`.
- [ ] Interaction Coverage re-checked: five inventory entries, five matrix rows,
      five `@e2e` titles, all CLI subprocess mechanisms (recorded in
      design-review.md).
- [ ] Note: scenarios marked **[regression]** are already green before the
      change (no distinguishable RED state). They are kept as guards; their RED
      sub-task records the observed side-effect GREEN instead of failing.

## Scenario 1: Non-interactive init without an agent target refuses initialization

- [ ] RED: Remove `@wip`, run targeting this scenario → must fail (current code
      falls back to `.opencode` on EOF, exit 0). Evidence: <paste>
- [ ] GREEN: refuse before the first write. Production changes:
      `src/cli/init.rs` — add `require_agent_target()`, make
      `prompt_for_targets()` return `Result`, refuse the EOF branch, resolve the
      targets before scaffolding; test fixtures name the selection
      (`tests/common/mod.rs`, `tests/steps/mod.rs`,
      `tests/steps/archive_proof_steps.rs`, `tests/steps/local_only_rules_steps.rs`,
      `tests/steps/release_0_3_0_e2e_steps.rs`,
      `tests/steps/repository_ownership_fixtures_support.rs`,
      `tests/steps/run_execution_e2e_steps.rs`,
      `tests/steps/scm_backend_selection_steps.rs`).
      Run targeting this scenario → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 2: The interactive prompt refuses a confirmation without a marked target

- [ ] RED: Remove `@wip`, add the PTY step
      (`tests/steps/init_agent_target_selection_steps.rs`), run targeting this
      scenario → must fail (the pre-marked `.opencode` is installed, exit 0).
      Evidence: <paste>
- [ ] GREEN: drop the dialog default and refuse an empty selection.
      Production: `src/cli/init.rs` (`MultiSelect` without `defaults`,
      empty→`require_agent_target`). Run targeting this scenario → passes.
      Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 3: An empty answer refuses initialization

- [ ] RED: Remove `@wip`, run targeting this scenario → must fail (empty answer
      falls back to `.opencode`). Evidence: <paste>
- [ ] GREEN: route the piped answer through `require_agent_target`; remove the
      `DEFAULT_TARGETS` fallback from `resolve_target_names` and delete the
      constant. Run targeting this scenario → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 4: An answer without any known agent refuses initialization

- [ ] RED: Remove `@wip`, run targeting this scenario → must fail (unknown-only
      answer falls back to `.opencode`). Evidence: <paste>
- [ ] GREEN: covered by Scenario 3's resolution path (side-effect GREEN); record
      the command output. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 5: An unknown-only --agents value refuses initialization

- [ ] RED: Remove `@wip`, run targeting this scenario → must fail (`--agents
      nope` falls back to `.opencode`). Evidence: <paste>
- [ ] GREEN: `resolve_agents_flag()` returns `Result` and routes through
      `require_agent_target`. Run targeting this scenario → passes.
      Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 6: givn init --force re-applies the explicitly given selection **[regression]**

- [ ] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (the current code honours `--agents opencode` under `--force`).
      Evidence: <paste>
- [ ] GREEN: re-run after the code changes → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 7: givn init --force without a selection keeps the persisted selection

- [ ] RED: Remove `@wip`, run targeting this scenario → must fail (`--force`
      with EOF currently overwrites the config with `.opencode`). Evidence: <paste>
- [ ] GREEN: resolution precedes the config overwrite (Scenario 1 ordering);
      verify no additional production change is needed. Run targeting this
      scenario → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 8: givn init defaults to all addons enabled and generates their files (modified) **[regression]**

- [ ] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (the modified `When` names `--agents opencode`). Evidence: <paste>
- [ ] GREEN: re-run → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(enable-optional-addons): <release note>`; body
      carries the scenario title.

## Scenario 9: givn init --no-addons produces no skill or command addon files (modified) **[regression]**

- [ ] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN. Evidence: <paste>
- [ ] GREEN: re-run → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(enable-optional-addons): <release note>`; body
      carries the scenario title.

## Scenario 10: A fresh project resolves the rules policy to local-only (modified) **[regression]**

- [ ] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN. Evidence: <paste>
- [ ] GREEN: re-run → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(rules-policy): <release note>`; body carries the
      scenario title.

## E2E setup

- [ ] Real interface and e2e runner confirmed: `./run-tests.sh --e2e` with the
      delta scope; the e2e step definitions live in the capability files named
      in design.md; local infrastructure is the CLI subprocess plus the
      `script` PTY for the interactive scenario — nothing to start.

## Scenario 11: Scaffolding ships an unconfigured run declaration (modified, @e2e) **[regression]**

- [ ] RED: Remove `@wip`, run `./run-tests.sh --e2e` targeting this scenario →
      observed side-effect GREEN. Evidence: <paste>
- [ ] GREEN: re-run → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `test(e2e): <release note>`; body carries the scenario
      title.

## Scenario 12: An initialized project documents reusable test and coverage scripts (modified, @e2e) **[regression]**

- [ ] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN. Evidence: <paste>
- [ ] GREEN: re-run → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `test(e2e): <release note>`; body carries the scenario
      title.

## Scenario 13: Scaffolding marks the unfilled sections (modified, @e2e) **[regression]**

- [ ] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN. Evidence: <paste>
- [ ] GREEN: re-run → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `test(e2e): <release note>`; body carries the scenario
      title.

## Scenario 14: Consumer runs init non-interactively and gets opencode skills (modified, @e2e) **[regression]**

- [ ] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN. Evidence: <paste>
- [ ] GREEN: re-run → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `test(e2e): <release note>`; body carries the scenario
      title.

## Final verification

- [ ] Delta scope, both scopes: `GIVN_FEATURES=givn/changes/require-agent-target/specs/ ./run-tests.sh`
      and `... ./run-tests.sh --e2e` → exit 0. Evidence: <paste>
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` → clean.
      Evidence: <paste>
- [ ] `givn lint --change require-agent-target` → no `@wip` findings remain
      (advisory shape/subset notes are dispositioned in review.md).
      Evidence: <paste>
