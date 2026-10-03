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
- [x] COMMIT: `4c1a96ce` — `feat(initialize-a-project): <release note>`; body
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
- [x] COMMIT: `92be3e2c` — `feat(initialize-a-project): <release note>`; body
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
- [x] COMMIT: `ff4073a2` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 4: An answer without any known agent refuses initialization

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN: `1 scenario (1 passed), 6 steps (6 passed)` — Scenario 3's
      resolution path already refuses an unknown-only answer. Recorded as
      side-effect GREEN, not a failing RED.
- [x] GREEN: same run → passes (no production change beyond Scenario 3).
- [x] REFACTOR: re-run → `1 scenario (1 passed), 6 steps (6 passed)`.
- [x] COMMIT: `214e77e9` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 5: An unknown-only --agents value refuses initialization

- [x] RED: Remove `@wip`, run targeting this scenario → failed as expected
      (exit 1; the flag path fell through to a write with an empty target list
      because the fallback constant was already gone).
- [x] GREEN: `resolve_agents_flag()` returns `Result` and routes through
      `require_agent_target`. Run → `1 scenario (1 passed), 5 steps (5 passed)`,
      exit 0.
- [x] REFACTOR: `cargo fmt` + clippy clean; scenario re-run → `1 scenario
      (1 passed), 5 steps (5 passed)`.
- [x] COMMIT: `baa69b27` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 6: givn init --force re-applies the explicitly given selection **[regression]**

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 5 steps (5 passed)`): the current code
      already honours `--agents opencode` under `--force`. Kept as a regression
      guard, no distinguishable RED state.
- [x] GREEN: re-run after the code changes → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `cee2ba82` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 7: givn init --force without a selection keeps the persisted selection

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 6 steps (6 passed)`): the resolution-before-
      write ordering from Scenario 1 already refuses the EOF before `--force`
      can overwrite the config. Recorded as side-effect GREEN.
- [x] GREEN: re-run → passes (same output); the persisted `opencode, claude`
      selection survives the refused `--force`.
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `69f6e0ed` — `feat(initialize-a-project): <release note>`;
      body carries the scenario title.

## Scenario 8: givn init defaults to all addons enabled and generates their files (modified) **[regression]**

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 8 steps (8 passed)`): the modified `When`
      names `--agents opencode`. Migration + regression guard.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `96205583` — `feat(enable-optional-addons): <release note>`; body
      carries the scenario title.

## Scenario 9: givn init --no-addons produces no skill or command addon files (modified) **[regression]**

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 7 steps (7 passed)`). Migration +
      regression guard.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `4f805360` — `feat(enable-optional-addons): <release note>`; body
      carries the scenario title.

## Scenario 10: A fresh project resolves the rules policy to local-only (modified) **[regression]**

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 5 steps (5 passed)`). Migration +
      regression guard.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `07027cf5` — `feat(rules-policy): <release note>`; body carries
      the scenario title.

## E2E setup

- [x] Real interface and e2e runner confirmed: `./run-tests.sh --e2e` with the
      delta scope; the e2e step definitions live in the capability files named
      in design.md; local infrastructure is the CLI subprocess plus the
      `script` PTY for the interactive scenario — nothing to start. Single
      scenarios run through the same cucumber binary with
      `cargo test --locked --test features_runner -- --name "<title>"`.

## Scenario 11: Scaffolding ships an unconfigured run declaration (modified, @e2e) **[regression]**

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 6 steps (6 passed)`). Migration +
      regression guard.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `fa56ce37` — `test(e2e): <release note>`; body carries the
      scenario title.

## Scenario 12: An initialized project documents reusable test and coverage scripts (modified, @e2e) **[regression]**

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 5 steps (5 passed)`). Migration +
      regression guard.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `51727e5f` — `test(e2e): <release note>`; body carries the
      scenario title. (First landed bundled with Scenario 13; `git reset
      --soft` + separate commits produced this scenario's own revision.)

## Scenario 13: Scaffolding marks the unfilled sections (modified, @e2e) **[regression]**

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 6 steps (6 passed)`). Migration +
      regression guard.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `e32e1e30` — `test(e2e): <release note>`; body carries the
      scenario title.

## Scenario 14: Consumer runs init non-interactively and gets opencode skills (modified, @e2e) **[regression]**

- [x] RED: Remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 5 steps (5 passed)`): `--agents opencode`
      already worked. Kept as the non-interactive e2e guard.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `37b4e019` — `test(e2e): <release note>`; body carries the
      scenario title.

## Final verification

- [x] Delta scope, both scopes:
      `GIVN_FEATURES=givn/changes/require-agent-target/specs/ ./run-tests.sh`
      → `19 tests run: 19 passed, 0 skipped`, exit 0; `... --e2e` →
      `4 tests run: 4 passed, 15 skipped`, exit 0.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` → clean (exit 0).
- [x] `givn lint --change require-agent-target` → exit 0, no `@wip` findings;
      advisory shape/subset notes are dispositioned in review.md.
