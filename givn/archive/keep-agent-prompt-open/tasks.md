# Tasks: keep-agent-prompt-open

RED→GREEN→REFACTOR→COMMIT per scenario.

Release note (every revision's message subject):
`givn init` keeps its interactive agent-target prompt open with OpenCode
pre-selected and reports an error when the confirmation marks no target,
instead of closing the dialog.

Use-case constraints carried into every scenario (from
`specs/initialize-project/usecase.md`):

- Main flow 1: the interactive list opens with `.opencode` marked and re-asks
  when nothing is marked.
- Extension 1 (interactive): the system states that at least one agent target
  must be marked, shows the selection again, and proceeds only after a target
  is marked.
- Extension 1 (non-interactive): an empty or unknown-only answer/flag refuses
  with a non-zero exit and writes nothing — unchanged by this change.
- Minimal guarantee: nothing is written before a target is marked; Ctrl-C
  aborts.

Single-scenario runner:

```sh
GIVN_FEATURES=givn/changes/keep-agent-prompt-open/specs/ cargo test --locked --test features_runner -- --name "<scenario title>"
```

Delta scope: `GIVN_FEATURES=givn/changes/keep-agent-prompt-open/specs/ ./run-tests.sh` and `… --e2e`.

## Setup

- [x] Runner and strict mode confirmed: `verify.command` is `./run-tests.sh`
      (unchanged), `.fail_on_skipped()` at `tests/features_runner.rs:55,129`;
      the proof-of-strictness run from the previous change still holds
      (`… --name "An undefined step must fail the runner"` → exit 1).
- [x] `script` (util-linux) present on PATH (the PTY steps) — documented in
      `AGENTS.md`.
- [x] Both `@wip` tags are removed in their own scenario task before the
      tasks/review/archive gates run.

## Scenario 1: The interactive prompt accepts the pre-marked OpenCode default

- [x] RED: Remove `@wip`, run targeting this scenario → failed as expected
      (exit 1; transcript showed the unmarked list and the scripted refusal
      `no agent target selected; pass --agents …`).
- [x] GREEN: pre-mark the default. Production: `src/cli/init.rs` —
      `DEFAULT_PROMPT_TARGET`, the `defaults` vector, `.defaults(&defaults)`;
      doc comment adjusted. Test side: the PTY helper now takes the key
      sequence (`tests/steps/init_agent_target_selection_steps.rs`). Run →
      `1 scenario (1 passed), 6 steps (6 passed)`, exit 0.
- [x] REFACTOR: `cargo fmt` + clippy clean; scenario re-run → `1 scenario
      (1 passed), 6 steps (6 passed)`.
- [x] COMMIT: `5e1702b5` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 2: The interactive prompt keeps asking until a target is marked

- [x] RED: Remove `@wip`, run targeting this scenario → failed as expected
      (exit 1; unmarking every entry hit the scripted refusal
      `no agent target selected; pass --agents …`, no re-ask).
- [x] GREEN: keep the dialog open. Production: `src/cli/init.rs` — fresh
      `MultiSelect` with `defaults` inside a `loop`; an empty selection prints
      the interactive error to stderr and re-displays; only a non-empty
      selection returns. Doc comment finished. Test side: the two key-sequence
      steps plus the `AgentTarget::ALL[0]` invariant comment. Run → `1 scenario
      (1 passed), 6 steps (6 passed)`, exit 0.
- [x] REFACTOR: `cargo fmt` + clippy clean; name-filtered re-run → `3
      scenarios (3 passed)` (the two new plus the removed placeholder).
- [x] COMMIT: `ee00e05d` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Final verification

- [x] Delta scope, both scopes:
      `GIVN_FEATURES=givn/changes/keep-agent-prompt-open/specs/ ./run-tests.sh`
      → `3 tests run: 3 passed, 0 skipped`, exit 0; `… --e2e` → `0 tests run`,
      exit 0 (this delta adds no `@e2e` scenario; the merged-corpus e2e scope
      is verified by the archive).
- [x] `cargo clippy --workspace --all-targets -- -D warnings` → clean (exit 0).
- [x] `givn lint --change keep-agent-prompt-open` → exit 0, no `@wip` findings;
      the SHAPE/SUBST advisories are dispositioned in review.md.
