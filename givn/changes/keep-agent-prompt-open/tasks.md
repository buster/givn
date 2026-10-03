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
- [x] COMMIT: `f0a1c804` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Scenario 2: The interactive prompt keeps asking until a target is marked

- [ ] RED: Remove `@wip`, run targeting this scenario → must fail (the space
      marks opencode under the current code, so exit 0 passes today but the
      error phrase never appears). Evidence: <paste>
- [ ] GREEN: keep the dialog open. Production: `src/cli/init.rs` — build a
      fresh `MultiSelect` with `defaults` inside a `loop`; on an empty
      selection print
      `error: no agent target selected; mark at least one agent target (.opencode, .agents, .claude) and confirm`
      to stderr and re-display; return only a non-empty selection. Finish the
      doc comment (pre-marked default, re-ask on an empty confirmation,
      Ctrl-C aborts). Test side: generalize the PTY helper to take the key
      sequence; replace the old step with the two new steps
      (`… confirms the prompt` → `"\n"`; `… unmarks every target, and confirms
      twice` → `"\x20\n\n"`) and document the `AgentTarget::ALL[0]` invariant.
      Run targeting this scenario → passes. Evidence: <paste>
- [ ] REFACTOR: re-run → passes. Evidence: <paste>
- [ ] COMMIT: `<hash>` — `feat(initialize-a-project): <release note>`; body
      carries the scenario title.

## Final verification

- [ ] Delta scope, both scopes: regular and `--e2e` → exit 0. Evidence: <paste>
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` → clean. Evidence: <paste>
- [ ] `givn lint --change keep-agent-prompt-open` → no `@wip` findings remain;
      the SHAPE/SUBST advisories are dispositioned in review.md. Evidence: <paste>
