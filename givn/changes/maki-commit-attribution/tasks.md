# Tasks: maki-commit-attribution

Release note for every batch commit: `Maki commits now carry the same best-effort model, reasoning, and harness attribution as the other supported harnesses.`

## Setup

- [x] **Runner, strict mode, single-scenario command, and pinned Maki harness**
  - Design constraints: `./run-tests.sh --non-e2e` is `verify.command`; `./run-tests.sh --e2e` is `verify.e2e_command`; `tests/features_runner.rs` builds with `.fail_on_skipped()` and exits nonzero on failed or skipped scenarios; RED bodies use `unimplemented!()`. Single scenario: `GIVN_FEATURES=givn/changes/maki-commit-attribution/specs cargo test --locked --test features_runner -- --name "Maki prepares attribution for a revision command"`.
  - Strictness proof: `GIVN_FEATURES=/tmp/strict-probe.feature cargo test --locked --test features_runner` on a feature with `Given an undefined strictness probe step` exited 1 with `Step doesn't match any function` and `1 scenario (1 failed)`.
  - Harness: `maki --version` prints `maki 0.7.0`; the suite resolves it through `maki_binary()` (`GIVN_E2E_BIN`, `$HOME/.cache/givn-e2e/bin/maki`, or `PATH`). `git` and the compiled `givn` shim are on `PATH` for the commit hook.
  - E2E setup: the delta has 4 `@e2e` scenarios; the full corpus is larger, so the e2e count is strictly less. No new browser, container, or service is introduced; the real Maki binary and Git are the infrastructure.

## Batch: maki-attribution

Contract: `CommitAttribution` — `src/cli/attribution.rs` owns the managed trailer; the generated Maki Lua asset is the adapter that contributes model, reasoning, and harness facts. It does not format trailers.

- [x] **Maki prepares attribution for a revision command**
  - Design constraints: `assets/agent-integrations/maki/givn-attribution.lua` reads `maki.model.get()` once; identity is `spec`, then `provider/id`, then `id`; reasoning is `thinking_shown`, then `thinking`. The prefix is `export GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM='<identity>' GIVN_ATTRIBUTION_REASONING='<reasoning>' GIVN_ATTRIBUTION_HARNESS='maki' GIT_COMMITTER_NAME='LLM: <identity> via maki'; <command>`, with single-quote escaping. Only a command matching `%f[%w]commit%f[%W]` is rewritten. The layer calls `prev` first, propagates stop/ask, shallow-copies before replacing `command`, and is idempotent in `setup()`. The driver runs the real `maki --trust prompt`, overrides `maki.model.get`, calls `prepare`, and fails unless `maki.api.get_slots()` lists the `project/init.lua` filler for `tool.bash.input`.
  - [x] COMMIT: `a022d6bf` -- Batch: maki-attribution

- [x] **Maki leaves a command that creates no revision unchanged**
  - Design constraints: `M.prepare("git status")` returns the exact input string; the slot layer returns the value unchanged; no environment export, no copy, no committer.
  - [x] COMMIT: `a022d6bf` -- Batch: maki-attribution

- [x] **Maki marks a partial attribution context as unavailable**
  - Design constraints: with `maki.model.get()` returning `nil`, the prefix uses `LLM='unavailable'`, `REASONING='unavailable'`, `HARNESS='maki'`, and omits `GIT_COMMITTER_NAME`; the command is still rewritten and a commit is never blocked.
  - [x] COMMIT: `a022d6bf` -- Batch: maki-attribution

- [x] **Maki marks a missing reasoning mode as unavailable**
  - Design constraints: with a model `spec` and no `thinking_shown`/`thinking`, `LLM` is the identity, `REASONING='unavailable'`, and `GIT_COMMITTER_NAME` is present; this is distinct from the no-model boundary.
  - [x] COMMIT: `a022d6bf` -- Batch: maki-attribution

## Batch: maki-install

Contract: `InitializeProject` — `src/skills/targets.rs` installs the generated assets and merges user-owned Maki configuration; `src/cli/init.rs` is the terminal adapter over it. `.maki/plugin.toml` and `.maki/init.lua` stay user-owned.

- [x] **The generated Maki permission manifest names the session read**
  - Design constraints: `assets/agent-integrations/maki/plugin.toml` grants `fs_read`, `fs_write`, `run`, `session_read`, `session_write`, and revokes `net`, `env`, `agent`, `session_settings`, `input`. The file comment states that the attribution layer needs `session_write` and never writes session content. Update `assert_maki_merged_files` and the manifest assertions to the new grant.
  - [x] COMMIT: `b44ef4ee` -- Batch: maki-install

- [x] **Initializing a Maki target installs the attribution integration**
  - Design constraints: add `.maki/lua/givn-attribution.lua` to `MAKI_INTEGRATIONS`; copy the asset verbatim; add the `-- givn:begin:attribution` fence to `MAKI_FENCE_LINES`; merge both fences in memory and write `init.lua` once. `src/cli/init.rs` prints `attribution integration: .maki/lua/givn-attribution.lua` beside the reporter. Add the module to `ignore_entries()`, `is_rule_path()`, and the rule-shape test.
  - [x] COMMIT: `b44ef4ee` -- Batch: maki-install

- [x] **Synchronizing an existing Maki manifest adds the reporter and attribution grants**
  - Design constraints: `MAKI_REQUIRED_GRANTS` is `fs_read`, `fs_write`, `run`, `session_read`, `session_write`. For the legacy manifest, add missing grants inside `[permissions]` only; never remove or flip an existing key; add no new `= false` line. The retired `Synchronizing an existing Maki manifest adds only the session read grant` scenario is removed by the archive merge and replaced by this scenario.
  - [x] COMMIT: `b44ef4ee` -- Batch: maki-install

- [x] **Synchronizing an unmodified generated Maki manifest grants attribution**
  - Design constraints: ship `assets/agent-integrations/maki/plugin-before-attribution.toml` as the exact prior asset; compare the existing file after CRLF normalization; on equality replace it with the current `plugin.toml` and reinstall the module. This is the only automatic `session_write = false` -> `true` path. The old asset's sha256 is `755cf4a1a22f1c878b10e3afdf8accd8aacef7b7edd9f1ee6b1601723e61681b`.
  - [x] COMMIT: `b44ef4ee` -- Batch: maki-install

- [x] **Synchronizing an edited Maki manifest preserves its attribution revocation and names the needed grant**
  - Design constraints: a manifest that is not the exact prior asset keeps its explicit `session_write = false`; the merge still adds other missing grants; stderr prints `warning: .maki/plugin.toml sets session_write = false; Maki commit attribution requires session_write = true`; the file is never widened.
  - [x] COMMIT: `b44ef4ee` -- Batch: maki-install

## Batch: maki-e2e

Contract: `CommitAttribution` and `InitializeProject` through the real interfaces: `givn init`/`givn upgrade`, the real Maki 0.7.0 binary, a real shell, and Git's commit-msg hook. Assertions are on the CLI output and the commit metadata Git produced.

- [x] **Consumer initializes a project and receives shared attribution plumbing** (@e2e)
  - Design constraints: add `maki` to `givn init --agents opencode,claude,omp,maki`; the shared integration-exists and report lists include `.maki/lua/givn-attribution.lua`; the human commit path and hook preservation are unchanged.
  - [x] COMMIT: `e5d7601b` -- Batch: maki-e2e

- [x] **Consumer sees the commit attribution installation report** (@e2e)
  - Design constraints: add `maki` to the agent list and `.maki/lua/givn-attribution.lua` to the stdout assertions; no new report code beyond the init summary and the existing integration row.
  - [x] COMMIT: `e5d7601b` -- Batch: maki-e2e

- [x] **Consumer upgrades an existing project with attribution plumbing missing** (@e2e)
  - Design constraints: initialize the upgrade fixture with `opencode,claude,omp,maki`, remove `.maki/lua/givn-attribution.lua` before the upgrade, and add it to the managed-revision path assertions. The Maki manifest and fence refresh are merged files, not new rules.
  - [x] COMMIT: `e5d7601b` -- Batch: maki-e2e

- [x] **Maki attributes a commit using its current model** (@e2e)
  - Design constraints: the Maki driver prepares `git commit -m 'maki attribution'` with model `deepseek/deepseek-flash` and reasoning `high`; the step writes and stages `fixture.txt`, runs the prepared command through `sh -c` with the compiled `givn` shim on `PATH`, and asserts the Git committer `LLM: deepseek/deepseek-flash via maki` and `Co-Authored-By: deepseek/deepseek-flash/high@maki with Givn <version>`.
  - [x] COMMIT: `e5d7601b` -- Batch: maki-e2e

## Closeout

- [ ] **Delta step phrases are bound in the permanent tree**
  - Design constraints: the change adds no delta step copies; every phrase bound by `givn/changes/maki-commit-attribution/specs/` must resolve in `tests/steps/` before archive. Run `givn lint --change maki-commit-attribution`, `./run-tests.sh --non-e2e`, and `./run-tests.sh --e2e`; the archive gate is not a substitute for this check.
  - [x] COMMIT: `e5d7601b` -- Batch: maki-e2e
