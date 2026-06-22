# Design Review: reverse-engineer-new-command

## Grilling log

| # | Branch | Question | Recommended | Outcome |
|---|--------|----------|-------------|---------|
| Q1 | Testability | Success scenario asserts a single "Next:" line mentioning the instructions command, but `new.rs` prints two separate lines. Single line or loose interpretation? | Option (b) — loose interpretation: stdout contains a line beginning with "Next:" AND stdout mentions "givn instructions proposal --change <id>". | AGREED (option b). Spec loosened to two assertions. |
| Q2 | Risk | The empty-id scenario assumes clap accepts `givn new ""` so `validate_id` runs. If clap rejects it, exit code becomes 2 with a different stderr. Mitigation? | Write the empty-id test as a probing test (assert only "non-zero exit" initially), observe actual stderr/exit, then tighten spec to match AS-IS. | AGREED (probe first, tighten to AS-IS). Spec loosened; design + tasks updated with probing-test strategy. |
| Q3 | Missing scenarios | Generic clap behaviours (no argument, extra args, --help) — in scope? | Out of scope — generic clap behaviours not specific to the `new` command. Document in proposal Out of Scope. | AGREED (out of scope). Documented in proposal.md. |
| Q4 | Tech choices | Should `make_init_root` run `givn init` or create a minimal `givn/` subtree? `givn init` does extra work (banner, config, skills). | (Original recommendation was minimal subtree.) User OVERRIDDEN. | OVERRIDDEN — fixture MUST run `givn init` to avoid duplicating scaffolding logic. Reusable `TestEnv`/`temp_givn_root()` fixture created in design + tasks. |
| Q5 | Risk | Hand-rolled temp dirs leak on panic. Add `tempfile` as a dev-dependency now? | Add `tempfile` as a dev-dependency now; auto-cleanup on panic prevents leaked dirs. | AGREED (add tempfile dev-dependency). design.md + tasks.md updated; fixture builds on `tempfile::TempDir`. |
| Q6 | Risk | Convention-based scenario↔test mapping has no automated enforcement. Drift mitigation? | Accept risk (17 scenarios auditable by hand); add a DONE checklist item to tasks.md for manual confirmation. | AGREED (accept risk, add checklist item). tasks.md Done criteria + design.md drift-mitigation note added. |
| Q7 | Missing scenarios | `validate_id` runs before the givn/-not-found check, so `givn new BadId` in an uninit project yields the kebab error. Worth a scenario? | Out of scope — two existing scenarios independently characterise the behaviours; composition adds no new guarantee. | AGREED (out of scope). Documented in proposal.md. |
| Q8 | Scope | tasks.md "Done when" requires `givn status` to report "characterization complete, awaiting review". Is this state achievable given `givn status` derives state from the lifecycle graph? | Verify by reading `src/cli/status.rs` + `src/state.rs`; broaden Done criteria or narrow the status expectation. Do not leave an unsatisfiable "Done when" clause. | AGREED (verify status logic, fix Done criteria). Verified: `stage_label()` returns `tasks-done` once tasks is the last Done artifact, with `review` as next required. "characterization complete, awaiting review" is NOT a real string. tasks.md fixed. |

## Resolved by codebase exploration

| Branch | Finding |
|--------|---------|
| Scope (SMELL-1) | `src/cli/new.rs:9-16` confirms the "already exists" check is ordered before the "givn/ not found" check. The "already exists" branch is unreachable when `givn/` is absent. No observable scenario exposes this ordering — no missing scenario. |
| Tech choices (seam) | `src/cli/new.rs:6` derives `project_root` from `std::env::current_dir()`; all filesystem operations are relative to it. The subprocess + temp-dir-current-directory seam is sound; no production code change needed. |
| Testability (exit codes) | `src/main.rs:147` calls `std::process::exit(1)` on any `Err` from dispatch. Only the clap argument-parsing path exits 2. Spec's "exits non-zero" wording for validator/init/already-exists scenarios is correct; the explicit "exit code 2" for the start-hyphen clap-intercepted scenario is also correct. |
| Testability (absolute path) | `new.rs:6-7` uses `current_dir().join("givn/changes").join(id)`, line 23 joins `proposal.md`. When the binary runs with the temp dir as CWD, `proposal_path.display()` is `<temp_root>/givn/changes/<id>/proposal.md` — an absolute path. The absolute-path assertion is satisfiable. |
| Testability (include_str!) | `tests/new_command.rs` lives at the project root, so `include_str!("../assets/templates/proposal.md")` points at `<project_root>/assets/templates/proposal.md` — the same file rust-embed embeds. Byte-for-byte equality is achievable. |
| Missing scenarios (SMELL-3) | `src/cli/init.rs:26-30` scaffolds only `givn/specs`, `givn/changes`, `givn/archive` — NOT `givn/artifacts/`. The override scenario's `write_override` helper must `create_dir_all` the `givn/artifacts/proposal/` subtree itself. |
| Tech choices (config) | `make_init_root` running `givn init` writes the sentinel `verify.command: "givn missing-testrunner"` into the temp dir's `givn/config.yaml`, NOT the project's `cargo test`. Harmless because `src/cli/new.rs` does not read `givn/config.yaml`. |
| Risk (parallelism) | Each test uses a unique temp dir as CWD and `src/cli/new.rs` reads no shared global state. `cargo test`'s default parallel runner will not cause interference. |

## Open questions

| # | Question | Blocking? |
|---|----------|-----------|
| none | All questions resolved during grilling. | — |

## Changes made during hardening

| Artifact | Change summary |
|----------|----------------|
| `specs/new-command/new-command.feature` | Q1: split the success scenario's single "Next:" assertion into two — "stdout contains a line beginning with `Next:`" AND "stdout mentions `givn instructions proposal --change add-user-auth`". Q2: loosened the empty-id scenario to assert only "non-zero exit" + "no change directory created"; removed the specific stderr message assertion; added a `#` comment explaining the tighten-to-AS-IS strategy. |
| `proposal.md` | Q3: added Out-of-Scope entry for generic clap behaviours (no argument → exit 2, extra args → exit 2, `--help` → exit 0). Q7: added Out-of-Scope entry for the invalid-id + uninitialised-project composition (validate_id runs before the givn/-not-found check). No other proposal text changed. |
| `design.md` | Q2: empty-id seam note rewritten as a probing-test strategy (assert non-zero exit first, observe, then tighten to AS-IS). Q4: Test harness section rewritten — reusable `TestEnv`/`temp_givn_root()` fixture MUST run `givn init` in a `tempfile::TempDir`; helpers `make_bare_root`/`make_init_root`/`run_givn`/`write_override` now take the fixture handle; `write_override` must `create_dir_all` the `givn/artifacts/proposal/` subtree; noted that `givn init` writes the sentinel `verify.command` into the temp config (harmless). Q5: `tempfile` declared a dev-dependency (not deferred); `tempfile::TempDir` used for auto-cleanup on panic. Q6: added a drift-mitigation paragraph under the binding-approach section referencing the tasks.md manual-confirmation checklist item. Seam strategy "givn/ initialised" bullet updated to mandate `givn init`. |
| `tasks.md` | Q4: Task 0 rewritten to describe the reusable `TestEnv`/`temp_givn_root()` fixture running `givn init` in a `tempfile::TempDir`-managed temp dir. Q5: Task 0 adds the `tempfile` dev-dependency step. Q2: Task 4 rewritten as a probing-test cycle (assert non-zero only → observe → tighten spec → re-run). Q6: Done-when checklist adds a manual confirmation that each `// Scenario:` comment maps to exactly one `.feature` scenario. Q8: Done-when `givn status` criterion fixed — now requires stage `tasks-done` with `review` as next required step (verified against `src/cli/status.rs` + `src/state.rs`); the non-existent "characterization complete, awaiting review" string removed. |

## Sign-off

- [x] All branches walked.
- [x] All open questions resolved or flagged as non-blocking.
- [x] design.md reflects decisions reached.
- [x] specs/*.feature updated for any missing scenarios.
- [x] givn lint exits 0 or 2.

DESIGN-REVIEW: PASS
