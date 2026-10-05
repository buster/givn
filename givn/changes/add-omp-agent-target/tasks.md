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
- [ ] `verify.e2e_command` is `./run-tests.sh --e2e` and is a strict subset of
      `verify.command` (`./run-tests.sh`): the scope counts from both runs are
      pasted below.
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

## Scenario 3: The --agents flag accepts the dotted directory form for the omp target

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT: hash —.

## Scenario 4: The omp target receives its own guidance and attribution integration

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT: hash —.

## Scenario 5: omp leaves a command that creates no revision unchanged

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT (`feat(attribute-commits): <release note>`): hash —.

## Scenario 6: omp marks a partial attribution context as unavailable

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT: hash —.

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

## Scenario 7 (@e2e): Consumer initializes a project and receives shared attribution plumbing

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT (`test(e2e): <release note>`): hash —.

## Scenario 8 (@e2e): omp attributes a commit using its current model

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT: hash —.

## Scenario 9 (@e2e): Consumer sees the commit attribution installation report

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT: hash —.

## Live contract check

- [ ] Live smoke run against a real omp session (operator decision, R-78): paste
      the produced commit's `%cn` and `%B` from a throwaway project.

## Final

- [ ] `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh`
      exits 0.
- [ ] `GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh --e2e`
      exits 0.
