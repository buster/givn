# Tasks: init-agent-target-selection

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.
Runner: `cargo test --test features_runner` with `GIVN_FEATURES` pointing at
the change's delta specs (see design.md "Test runner command").

Legend: `[ ]` pending · `[x]` done only when evidence + commit hash are filled in. Check boxes immediately after each scenario.

---

## Strict-mode proof (do first)

The runner (`tests/features_runner.rs`) already fails on skipped/undefined
steps via `.fail_on_skipped()`. Prove gate-to-gate before any scenario work.

- [ ] **Proof-of-strictness**: temporarily add an undefined step (or a
      `todo!()` body) in the new step file (see first task), run
      `GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/ cargo test --test features_runner`, confirm non-zero exit with a skippable/undefined-step error, then remove the stub.
      ```
      <!-- paste command + output -->
      ```

---

## Setup

- [x] Register `tests/steps/init_agent_target_selection_steps.rs` in
      `tests/steps/mod.rs`. Add a stdin-capable invocation helper to
      `tests/common/mod.rs` (spawn with piped stdin, write `<answer>\n`,
      close pipe, wait) alongside the existing `run_givn`.
      New steps (all with real bodies — never empty; `todo!()` until GREEN):
      - `#[when] the user runs `givn init` and answers the agent prompt with "<answer>"` and its e2e twin `the consumer runs ...` — piped stdin.
      - `#[when] the consumer invokes `givn init` without answering a prompt` — same as existing `the user runs` (null stdin).
      - `#[given] / #[then] givn skills targets in givn/config.yaml equal "<a, b, c>"` — given writes the `skills:` block, then asserts the parsed targets.
      - `#[given] a givn/config.yaml with skills targets "<a, b, c>"` — scaffold `givn/` + minimal `givn/config.yaml` with the `skills:` block (no init run).
      Proof-of-strictness (section above) must pass first.
      Ran: `GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/ cargo test --test features_runner -- --name "Interactive init installs only the agents the user selects"` with `init_with_answer` stubbed `todo!()` -> "1 scenario (1 failed) ... error: test failed". Stub reverted.
      ```
      evidence: strict-mode proof above (panic -> scenario failed -> non-zero exit)
      ```

---

## Capability: init-agent-target-selection

### Scenario 1: Non-interactive init installs skills to opencode only

- [x] RED: Remove `@wip`. Run targeting this scenario only:
      `GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/ cargo test --test features_runner -- --name "Non-interactive init installs skills to opencode only"`.
      Must fail — currently `sync` writes to all three targets.
      ```
      evidence: "expected directory .agents/skills/givn-propose to not exist ... 1 scenario (1 failed) ... error: test failed"
      ```
- [x] GREEN: implement target-driven sync in `src/skills/targets.rs`
      (`configured_targets` default `[Opencode]`), rewire `init`'s sync call.
      Files: `src/skills/targets.rs`, `src/config/overlay.rs` (Skills +
      `load_skills_targets`), `src/cli/init.rs`, `src/cli/skills.rs`.
      Must pass.
      ```
      evidence: 1 scenario (1 passed), 7 steps (7 passed)
      ```
- [x] REFACTOR: `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`. Re-run, must still pass.
      ```
      evidence: clippy clean; scenario passed
      ```
- [x] COMMIT: `3d40bdc` — `feat(init-agent-target-selection): Non-interactive init installs skills to opencode only`

### Scenario 2: Interactive init installs only the agents the user selects

- [x] RED: Remove `@wip`. The `answers the agent prompt` step is unimplemented (`todo!()`) → must fail.
      ```
      evidence: "1 step failed ... error: test failed" (answer ignored -> .claude missing)
      ```
- [x] GREEN: implement `prompt_for_targets()` in `src/cli/init.rs` (read one
      stdin line, normalize names, prompt on TTY) and `write_skills_block`
      (line-based append to `givn/config.yaml`, pattern of `set_addon`).
      Interactive answer `opencode, .claude` → both targets installed,
      `.agents` untouched. Must pass.
      ```
      evidence: 1 scenario (1 passed), 6 steps (6 passed)
      ```
- [x] REFACTOR: re-run, must still pass.
      ```
      evidence: scenario passed after fmt/clippy
      ```
- [x] COMMIT: `01829e6` — `feat(init-agent-target-selection): Interactive init installs only the agents the user selects`

### Scenario 3: Interactive init accepts the default selection with an empty answer

- [x] RED: Remove `@wip`. Must fail (prompt not yet accepting answers).
      ```
      evidence: side-effect GREEN from Scenario 2 — `prompt_for_targets` EOF/empty fallback was implemented there; no separate RED state existed for this scenario. Recorded as reuse-GREEN.
      ```
- [x] GREEN: empty answer → default `[opencode]`. No new production code
      beyond Scenario 2 if the fallback is implemented there; verify.
      Files: —
      ```
      evidence: 1 scenario (1 passed), 6 steps (6 passed)
      ```
- [x] REFACTOR: re-run, must still pass.
      ```
      evidence: scenario passed after fmt
      ```
- [x] COMMIT: `07540d5` — `feat(init-agent-target-selection): Interactive init accepts the default selection with an empty answer`

### Scenario 4: The selected agents are persisted in givn/config.yaml

- [x] RED: Remove `@wip`. The `givn skills targets ... equal` Then-step is
      unimplemented → must fail.
      ```
      evidence: side-effect GREEN — the assert step existed (setup) and `write_skills_block` persists targets (Scenario 2). No separate RED state.
      ```
- [x] GREEN: implement the Then-step assertion (parse `targets:` list from
      `givn/config.yaml`, normalize). Assert interactively-selected targets
      are written. Must pass.
      ```
      evidence: 1 scenario (1 passed), 4 steps (4 passed)
      ```
- [x] REFACTOR: re-run, must still pass.
      ```
      evidence: scenario passed
      ```
- [x] COMMIT: `2af0027` — `feat(init-agent-target-selection): The selected agents are persisted in givn/config.yaml`

### Scenario 5: Non-interactive init persists opencode as the default target

- [ ] RED: Remove `@wip`. Must fail — config has no `skills:` block yet.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: `init` writes the default `[opencode]` block on fresh config.
      Files: `src/cli/init.rs`.
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): Non-interactive init persists opencode as the default target`

### Scenario 6: givn skills sync installs to the persisted selection only

- [ ] RED: Remove `@wip`. Must fail — sync ignores the config block.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: `sync` reads `load_skills_targets()` from config. Presets can
      be covered by earlier scenarios; ensure the Given-step (write block)
      works. Must pass.
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): givn skills sync installs to the persisted selection only`

### Scenario 7: Skills and commands are not installed to an agent excluded from the selection

- [ ] RED: Remove `@wip`. Implement the clean-state Given step
      (`givn/config.yaml` with targets `claude`). Must fail — sync still
      writes everywhere.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: assert-only path — sync writes to `claude` only, never touches
      `.opencode`. Exercises `AgentTarget` dir mapping
      (`claude` → `.claude`). Must pass. Note: sync is intentionally
      write-only — no deletion of stale targets (ADR-036/R-12).
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): Skills and commands are not installed to an agent excluded from the selection`

### Scenario 8: Re-running init keeps the persisted selection

- [ ] RED: Remove `@wip`. Must fail — init currently forces the default on
      every run (or writes all targets).
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: `init` on an existing project skips the prompt and keeps the
      persisted `claude` selection (config untouched, sync to `claude`).
      Files: `src/cli/init.rs`.
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): Re-running init keeps the persisted selection`

### Scenario 9: givn init --force re-applies the default selection

- [ ] RED: Remove `@wip`. Must fail.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: `--force` replaces config → prompt runs → EOF → default
      `[opencode]`. Ensure this cannot clobber Scenario 8's skip path.
      Must pass.
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): givn init --force re-applies the default selection`

### Scenario 10: An unknown agent name in the answer is dropped with a hint listing the valid agents

- [ ] RED: Remove `@wip`. Must fail.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: drop-invalid semantics — answer `claude, nope` → targets
      `claude`, warning printed to stdout listing `opencode`, `.agents`,
      `.claude`. Files: `src/cli/init.rs` (`prompt_for_targets`).
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): An unknown agent name in the answer is dropped with a hint listing the valid agents`

### Scenario 11: An answer without any known agent falls back to the default

- [ ] RED: Remove `@wip`. Must fail.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: `nope`-only answer → zero known agents → default `[opencode]`.
      Must pass.
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): An answer without any known agent falls back to the default`

---

## Capability: install-extras (delta deltas)

The `@givn.removed` scenario ("addon skills are written to all target
directories") is spec-only: no implementation — the merge on archive removes
it. Do not write a TDD loop for it.

### Scenario 12: givn init defaults to all addons enabled and generates their files (modified)

- [ ] RED: Remove `@wip`. Must fail — init writes addon skills to `.agents`
      (spec now asserts `.opencode` only).
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: fix from Scenario 1 (default `[opencode]`) makes this pass.
      Verify: `.opencode` skills + commands exist, `.agents/skills/givn-domain-modeling` absent. Files: —
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: full-suite re-run (`cargo test --test features_runner`) —
      all permanent hooks (`givn check tasks` path) must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): givn init defaults to all addons enabled and generates their files`

### Scenario 13: givn addons enable generates skill and command files (modified)

- [ ] RED: Remove `@wip`. Must fail — `addons enable` sync writes to
      `.agents`.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: must pass once `sync` is target-driven (Scenario 1); verify
      `.opencode/skills/givn-domain-modeling` exists, `.agents/...` absent.
      Files: —
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): givn addons enable generates skill and command files`

### Scenario 14: addon skills are written to the default agent target after sync

- [ ] RED: Remove `@wip`. Must fail — sync writes addon skills to all
      targets.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: default config → addon skill in `.opencode` only; `.agents`/
      `.claude` absent. Must pass.
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): addon skills are written to the default agent target after sync`

### Scenario 15: addon skills are written to every configured agent target after sync

- [ ] RED: Remove `@wip`. Must fail — sync cannot yet write all three when
      configured.
      ```
      <!-- paste RED evidence -->
      ```
- [ ] GREEN: config targets `opencode, agents, claude` → all three addon
      skill dirs written. Must pass.
      ```
      <!-- paste GREEN evidence -->
      ```
- [ ] REFACTOR: re-run, must still pass.
      ```
      <!-- paste REFACTOR evidence -->
      ```
- [ ] COMMIT: `<hash>` — `feat(init-agent-target-selection): addon skills are written to every configured agent target after sync`

---

## Arc42/spec alignment

- [ ] `assets/config.yaml.tmpl` documents the `skills:` block (commented
      scaffold section).
- [ ] Narrow `the generated review guidance contains` step
      (`tests/steps/arc42_review_alignment_steps.rs`) from the three fixed
      targets to the configured targets (read `.opencode` for default
      projects). Re-run `cargo test --test features_runner` — permanent
      suite stays green.
      ```
      <!-- paste evidence -->
      ```
- [ ] Self-hosting: append `skills:` block with
      `targets: [opencode, agents, claude]` to this repo's
      `givn/config.yaml`; run `givn skills sync`; confirm `.agents`,
      `.claude`, `.opencode` all regenerated.
      ```
      <!-- paste evidence -->
      ```

---

## E2E Smoke Tests

E2e setup: no external infra (CLI-only capability). e2e steps share
`tests/steps/init_agent_target_selection_steps.rs` with the unit scenarios
(design.md). `verify.e2e_command` is already configured
(`cargo test --test features_runner -- --tags @e2e`); prove the filter
separates e2e from the full suite (full count > e2e count) once e2e
scenarios exist.

- [ ] Set up e2e infrastructure: confirm `verify.e2e_command` runs and
      filters: full-run scenario count vs `--tags @e2e` count must differ.
      ```
      <!-- paste evidence -->
      ```
- [ ] @e2e Scenario: Consumer selects agent targets during interactive init
  - [ ] RED: Remove `@wip`. Must fail (piped-stdin step unimplemented).
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: piped-stdin answer `opencode, .claude` → files in both
        targets, `.agents` absent. Files: `tests/steps/init_agent_target_selection_steps.rs`.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `test(e2e): Consumer selects agent targets during interactive init`
- [ ] @e2e Scenario: Consumer runs init non-interactively and gets opencode skills
  - [ ] RED: Remove `@wip`. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: null-stdin init → opencode skills written, `.claude` absent.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `test(e2e): Consumer runs init non-interactively and gets opencode skills`
- [ ] @e2e Scenario: Consumer re-syncs skills after init and the selection is honoured
  - [ ] RED: Remove `@wip`. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: init'd project with persisted `opencode, claude` → sync
        writes both, `.agents` untouched.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `test(e2e): Consumer re-syncs skills after init and the selection is honoured`

---

## Full-suite gate

- [ ] Run the full change-scoped suite
      (`GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/ cargo test --test features_runner` and the same with `-- --tags @e2e`) — both exit 0.
- [ ] Run the permanent suite (`cargo test --test features_runner`):
      the ONLY failing scenarios must be the three permanent install-extras
      scenarios this change modifies (their deltas merge at archive:
      "givn init defaults to all addons enabled and generates their files",
      "givn addons enable generates skill and command files",
      "addon skills are written to all target directories"). Any other
      failure is unexpected and blocks archive.
      ```
      <!-- paste evidence -->
      ```

**Done when:** every box checked with evidence and commit hash, full suite
and e2e runner exit 0.