# Tasks: add-omp-agent-target

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Release note (every revision's message subject):
Givn installs the workflow skills, the workflow commands, and a commit
attribution hook for the omp coding agent when omp is a selected agent target.

Use-case constraints carried into every scenario (from
`specs/initialize-project/usecase.md` and `specs/fragments/fragment.md`):

- Initialization writes generated guidance only to the selected targets and
  reports what was skipped and why (main flow 1).
- The installation report names what was installed, what was skipped, and why,
  and may name only commands that work in the active project (rules).
- The project remains unchanged when an invalid request is rejected, and no
  generated guidance names a command that cannot work (minimal guarantee).
- Attribution stays best-effort: missing context never blocks a commit and is
  never fabricated (fragment `attribute-commits`).

Single-scenario runner (against the change delta only):

```sh
GIVN_FEATURES=givn/changes/add-omp-agent-target/specs cargo test --locked --test features_runner -- --name "<scenario title>"
```

Delta-scope suite: `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh`
E2E scope: `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh --e2e`

## Setup

- [x] Runner and strict mode confirmed. `verify.command` is `./run-tests.sh`
      (unchanged); the cucumber-rs builder sets `.fail_on_skipped()`
      (`tests/features_runner.rs:55,151`). Proof-of-strictness — an undefined
      delta step fails the runner:
      `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs cargo test --locked --test features_runner -- --name 'omp leaves a command that creates no revision unchanged'`
      → exit 1, `1 scenario (1 failed)`, `Step doesn't match any function`.
- [x] `verify.e2e_command` is `./run-tests.sh --e2e` and is a strict subset of
      `verify.command` (`./run-tests.sh`). Proof at delta scope:
      `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh` →
      `10 tests run: 10 passed, 0 skipped`;
      `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh --e2e`
      → `3 tests run: 3 passed, 7 skipped`. 3 < 10.
- [x] Interaction coverage re-checked: `fragments` carries no inventory row
      (skipped by `src/gherkin/interaction.rs`); the `initialize-project`
      selection action keeps its existing single `@e2e`, so the omp selection
      stays a regular scenario. Recorded in `design-review.md`.
- [x] Note: scenarios 1, 2, 3 and 5 are modifications of already-green permanent
      scenarios. Their RED sub-task records the observed pre-change failure of
      the added/changed assertion rather than a whole-scenario failure.

## Scenario 1: The --agents flag accepts "all" as a shorthand for every target

- [x] RED: remove `@wip` from this scenario only; run targeting this scenario →
      failed as expected (exit 1): `1 scenario (1 failed), 7 steps (6 passed, 1
      failed)`; the added assertion `the file ".omp/skills/givn-propose/SKILL.md"
      exists` failed because `omp` resolved to no target.
- [x] GREEN: added `AgentTarget::Omp` (`src/skills/targets.rs`: enum variant,
      `ALL`, `name()`, `dir()`, `from_name()`) and `omp` to
      `KNOWN_AGENT_TARGETS` (`src/config/overlay.rs`). Run → `1 scenario (1
      passed), 8 steps (8 passed)`, exit 0.
- [x] REFACTOR: dropped the not-yet-needed `owns_commands()` helper; re-run →
      `1 scenario (1 passed), 8 steps (8 passed)`.
- [x] COMMIT: `84d57c7b` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 2: givn skills targets add accepts "all" as a shorthand for every target **[regression]**

- [x] RED: remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 8 steps (8 passed)`): the same
      `AgentTarget::ALL` from Scenario 1 already expands `all` to the fourth
      target, and `.omp` is now a known name. Kept as a regression guard; the
      added assertion has no independent RED state.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `fb42e95e` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 3: The --agents flag accepts the dotted directory form for the omp target **[regression]**

- [x] RED: remove `@wip`, run targeting this scenario → observed side-effect
      GREEN (`1 scenario (1 passed), 5 steps (5 passed)`): `AgentTarget::from_name`
      already trims a leading dot, so `.omp` resolves once `omp` is known
      (Scenario 1). Kept as a regression guard.
- [x] GREEN: re-run → passes (same output).
- [x] REFACTOR: re-run → passes (same output).
- [x] COMMIT: `4b0f2100` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 4: The omp target receives its own guidance and attribution integration

- [x] RED: remove `@wip`, run targeting this scenario → failed as expected
      (exit 1): `1 scenario (1 failed), 5 steps (4 passed, 1 failed)`; the added
      assertion `the file ".omp/commands/givn-propose.md" exists` failed because
      command writing was opencode-only.
- [x] GREEN: production changed — `assets/agent-integrations/omp/givn-attribution.js`
      (new), `src/skills/targets.rs` (`owns_commands()`, per-target command loop,
      `write_command`, the omp branch in `sync_agent_integrations`, per-target
      `reconcile_addon_artifacts`, `.omp` entries in `managed_generated_paths`),
      `src/cli/init.rs` (report line + refusal string), `src/cli/skills.rs`
      (report), `src/cli/addons.rs` (`.omp` cleanup), `src/main.rs` (help). Run →
      `1 scenario (1 passed), 9 steps (9 passed)`, exit 0.
- [x] REFACTOR: `cargo fmt` + `cargo clippy --locked --all-targets` clean;
      re-run → `1 scenario (1 passed), 9 steps (9 passed)`.
- [x] COMMIT: `f1222d85` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 5: An omp project keeps the generated omp rules out of the repository

- [x] RED: remove `@wip`, run targeting this scenario → failed as expected
      (exit 1): the ignore block and the rule predicate had no `.omp` entries,
      so the tracked `.omp` rule stayed tracked: `the revision does not track
      ".omp/skills/givn-propose/SKILL.md"` failed.
- [x] GREEN: production changed — `src/skills/rules.rs` (`ignore_entries()`
      gains `.omp/skills/givn-*/`, `.omp/commands/givn-*.md`,
      `.omp/commands/grill-*.md`, `.omp/hooks/pre/givn-attribution.js`;
      `is_rule_path()` gains the `.omp` command prefix, the `.omp` skill prefix,
      and the omp hook path). Run → passed, exit 0.
- [x] REFACTOR: re-run → passes.
- [x] COMMIT: `2fdcd508` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 6: omp leaves a command that creates no revision unchanged **[harness]**

- [x] RED: remove `@wip`, run targeting this scenario → the step definitions did
      not exist yet (`Step doesn't match any function`), exit 1. Recorded as the
      RED for the new `node`-driven step harness.
- [x] GREEN: production changed — the hook's revision-verb gate shipped with the
      asset in Scenario 4; this scenario binds it through the new step
      definitions in `tests/steps/llm_attribution_e2e_steps.rs` plus the world
      state in `tests/steps/mod.rs`. Run → `1 scenario (1 passed), 3 steps
      (3 passed)`, exit 0. The gate is discriminating: without it the hook would
      rewrite `git status` too.
- [x] REFACTOR: `cargo fmt`; re-run → passes.
- [x] COMMIT: `de246cdf` — `feat(attribute-commits): <release note>`; body
      carries the scenario title.

## Scenario 7: omp marks a partial attribution context as unavailable **[harness]**

- [x] RED: remove `@wip`, run targeting this scenario → observed side-effect
      GREEN: the `unavailable` fallback shipped with the asset in Scenario 4 and
      the harness steps landed with Scenario 6. Kept as the second invariant of
      the generated hook.
- [x] GREEN: re-run → `1 scenario (1 passed), 5 steps (5 passed)`.
- [x] REFACTOR: re-run → passes.
- [x] COMMIT: `db258059` — `feat(attribute-commits): <release note>`; body
      carries the scenario title.

## E2E setup

- [ ] E2E harness: `node` on `PATH` (already required by the OpenCode
      integration E2E); the scenario writes `package.json` =
      `{"type":"module"}` beside the project before importing
      `.omp/hooks/pre/givn-attribution.js`. No server, database, container, or
      digital twin is required, so there is no local environment to bring up
      beyond `git` and `node`.
- [ ] E2E steps live in `tests/steps/llm_attribution_e2e_steps.rs`, one file for
      the capability, as `design.md` names.
- [ ] Strict mode for the e2e scope proven (same builder, same
      `.fail_on_skipped()`).
- [ ] Scenario-count proof: `./run-tests.sh` → N scenarios; `./run-tests.sh
      --e2e` → M scenarios; M < N (paste both).

## Scenario 8 (@e2e): Consumer initializes a project and receives shared attribution plumbing

- [x] RED: with the scenario's `--agents` list held at the pre-change
      `opencode,claude`, the added assertion fails: `assertion failed:
      env.path(".omp/hooks/pre/givn-attribution.js").is_file()` →
      `1 scenario (1 failed), 8 steps (7 passed, 1 failed)`, exit 1.
- [x] GREEN: the omp target from Scenario 4 writes the integration; restored
      `--agents opencode,claude,omp` → `1 scenario (1 passed), 8 steps
      (8 passed)`, exit 0.
- [x] REFACTOR: `./run-tests.sh --e2e` scope re-run at the end of the e2e phase.
- [x] COMMIT: `f579c688` — `test(e2e): <release note>`; body carries the
      scenario title.

## Scenario 9 (@e2e): omp attributes a commit using its current model **[harness]**

- [x] RED: the step definitions for the omp hook did not exist before Scenario 6
      (`Step doesn't match any function`, exit 1); the hook's rewrite behaviour
      itself shipped with the asset in Scenario 4.
- [x] GREEN: `node` imports the generated `.omp/hooks/pre/givn-attribution.js`,
      resolves the `tool_call` handler from a fake `pi`, invokes it with
      `{ toolName: "bash", input: { command } }` and
      `{ models: { current: () => model }, model }`, `sh -c` runs the returned
      command, and `git show` reads the commit → `1 scenario (1 passed), 7 steps
      (7 passed)`, exit 0. Observed in the produced commit:
      committer `LLM: deepseek/deepseek-flash via omp`, message trailer
      `Co-Authored-By: deepseek/deepseek-flash/high@omp with Givn 0.8.3`.
- [x] REFACTOR: `./run-tests.sh --e2e` scope re-run at the end of the e2e phase.
- [x] COMMIT: `15008b38` — `test(e2e): <release note>`; body carries the
      scenario title.

## Scenario 10 (@e2e): Consumer sees the commit attribution installation report

- [x] RED: with the scenario's `--agents` list held at the pre-change
      `opencode,claude`, the added assertion fails: `assertion failed:
      stdout.contains(".omp/hooks/pre/givn-attribution.js")` → `1 scenario
      (1 failed), 7 steps (6 passed, 1 failed)`, exit 1.
- [x] GREEN: production changed — `src/cli/init.rs` names the omp integration in
      the installation summary (Scenario 4); this scenario adds the stdout step
      in `tests/steps/llm_attribution_installation_e2e_steps.rs` and extends the
      managed-upgrade fixture with the omp plumbing. Run → `1 scenario (1
      passed), 7 steps (7 passed)`, exit 0; the permanent upgrade scenario
      "Consumer upgrades an existing project with attribution plumbing missing"
      also passes with the extended fixture (`1 scenario (1 passed), 5 steps
      (5 passed)`).
- [x] REFACTOR: `./run-tests.sh --e2e` scope re-run at the end of the e2e phase.
- [x] COMMIT: `4c680535` — `test(e2e): <release note>`; body carries the
      scenario title.

## Live contract check

- [x] Live smoke run against a real omp session (operator decision, R-78):
      a throwaway Git project initialized with the built binary's
      `givn init --agents omp`, then a real `omp -p --no-session` session
      (omp 18.3.0) asked to run `git commit -m 'smoke attribution'`. The hook
      fired through omp's real loader and `tool_call` event; the agent even
      echoed the revised command back as a self-reported "deviation", which is
      the rewritten input omp persisted. Produced commit `dfc97582`:
      committer `LLM: deepseek/deepseek-flash via omp <smoke@example.test>`,
      message:
      `smoke attribution` + `Co-Authored-By: deepseek/deepseek-flash/high@omp with Givn 0.8.3`.
      The contract assumed by D2 is therefore validated against the real
      harness, not only against the faked E2E context.

## Final

- [x] `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh`
      exits 0 → `10 tests run: 10 passed, 0 skipped`.
- [x] `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh --e2e`
      exits 0 → `3 tests run: 3 passed, 7 skipped`.
- [x] `cargo fmt` and `cargo clippy --locked --all-targets` clean.
