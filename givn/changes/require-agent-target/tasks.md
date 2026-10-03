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

- [x] Runner and strict mode confirmed. `verify.command` is `./run-tests.sh`
      (unchanged); the cucumber-rs builder sets `.fail_on_skipped()`
      (`tests/features_runner.rs:55,129`). Proof-of-strictness:
      `GIVN_FEATURES=/tmp/givn-strict-proof/ cargo test --locked --test features_runner -- --name "An undefined step must fail the runner"`
      → exit 1, `Step doesn't match any function`.
- [x] Interaction Coverage re-checked: five inventory entries, five matrix rows,
      five `@e2e` titles, all CLI subprocess mechanisms (recorded in
      design-review.md).
- [x] Note: scenarios marked **[regression]** are already green before the
      change (no distinguishable RED state). They are kept as guards; their RED
      sub-task records the observed side-effect GREEN instead of failing.

## Scenario 1: Non-interactive init without an agent target refuses initialization

- [x] RED: Remove `@wip`, run targeting this scenario → failed as expected
      (exit 1; current code fell back to `.opencode` on EOF, exit 0):
      `1 scenario (1 failed), 3 steps (2 passed, 1 failed)`.
- [x] GREEN: refuse before the first write. Production changed:
      `src/cli/init.rs` (`require_agent_target()`, `prompt_for_targets() ->
      Result`, EOF refusal, target resolution moved before scaffolding) and the
      fixture call sites now name the selection (`tests/common/mod.rs`,
      `tests/steps/mod.rs`, `archive_proof_steps.rs`, `local_only_rules_steps.rs`,
      `release_0_3_0_e2e_steps.rs`, `repository_ownership_fixtures_support.rs`,
      `run_execution_e2e_steps.rs`, `scm_backend_selection_steps.rs`).
      Run → `1 scenario (1 passed), 8 steps (8 passed)`, exit 0.
- [x] REFACTOR: `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings`
      clean (exit 0); scenario re-run → `1 scenario (1 passed), 8 steps (8 passed)`,
      exit 0.
- [x] COMMIT: `34360457` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 2: The interactive prompt refuses a confirmation without a marked target

- [x] RED: Remove `@wip`, add the PTY step
      (`tests/steps/init_agent_target_selection_steps.rs`), run targeting this
      scenario → failed as expected (exit 1; transcript showed
      `COMMAND_EXIT_CODE="0"` — the pre-marked `.opencode` was installed).
- [x] GREEN: drop the dialog default and refuse an empty selection.
      Production: `src/cli/init.rs` (`MultiSelect` without `defaults`, empty
      selection → `require_agent_target`). Run → `1 scenario (1 passed), 5
      steps (5 passed)`, exit 0 — the real dialoguer path is exercised through
      a `script` PTY.
- [x] REFACTOR: `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings`
      clean; scenario re-run → `1 scenario (1 passed), 5 steps (5 passed)`.
- [x] COMMIT: `a11ae096` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 3: An empty answer refuses initialization

- [x] RED: Remove `@wip`, run targeting this scenario → failed as expected
      (exit 1). The answer step now writes one newline-terminated line, so the
      empty answer is the empty-line path, distinct from the null-stdin EOF
      path of Scenario 1.
- [x] GREEN: route the piped answer through `require_agent_target`; remove the
      `DEFAULT_TARGETS` fallback from `resolve_target_names` and delete the
      constant. Run → `1 scenario (1 passed), 5 steps (5 passed)`, exit 0.
- [x] REFACTOR: `cargo fmt` + clippy clean; scenario re-run → `1 scenario
      (1 passed), 5 steps (5 passed)`.
- [x] COMMIT: `9d1072e7` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 4: An answer without any known agent refuses initialization

- [ ] RED: Remove `@wip`, run targeting this scenario → side-effect GREEN
      expected (Scenario 3's resolution path covers the unknown-only answer).
      Evidence: <pending>
- [ ] GREEN: re-run → passes. Evidence: <pending>
- [ ] REFACTOR: re-run → passes. Evidence: <pending>
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
