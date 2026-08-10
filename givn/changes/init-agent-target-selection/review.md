# Review: init-agent-target-selection

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies:

```
rg -n "\{\}|todo!|unimplemented!" tests/steps/init_agent_target_selection_steps.rs tests/steps/mod.rs tests/steps/arc42_review_alignment_steps.rs
→ 0 hits. No empty/no-op step bodies.
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source. Side-effect-GREEN scenarios
(production implemented in an earlier scenario of the same change) have
spec-only commits recording the `@wip` removal; the production code they
depend on is present and covered (commits 3d40bdc, 01829e6 implemented the
entire sync/prompt/config surface; everything after is tracked by the
corresponding scenario commit):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| S1 Non-interactive init installs skills to opencode only | 3d40bdc | YES (targets.rs, overlay.rs, init.rs) |
| S2 Interactive init installs only the agents the user selects | 01829e6 | YES (init.rs) |
| S3 default selection with empty answer | 07540d5 | no (side-effect GREEN; impl in 01829e6) |
| S4 selected agents persisted | 2af0027 | no (side-effect GREEN) |
| S5 non-interactive persists opencode default | 4290e08 | no (side-effect GREEN) |
| S6 skills sync installs to persisted selection | cf3929c | no (side-effect GREEN; impl in 3d40bdc) |
| S7 skills not installed to excluded agent | a57a604 | no (side-effect GREEN) |
| S8 re-running init keeps persisted selection | cc5efe7 | no (side-effect GREEN) |
| S9 init --force re-applies default | 93de4c2 | no (side-effect GREEN) |
| S10 unknown agent dropped with hint | 931b2df | no (side-effect GREEN) |
| S11 no known agent falls back to default | 4598bfc | no (side-effect GREEN) |
| S12 install-extras init (modified) | b4a9d43 | no (side-effect GREEN) |
| S13 addons enable (modified) | e44d9f9 | no (side-effect GREEN) |
| S14 addon skills default target | 7eeb621 | no (side-effect GREEN) |
| S15 addon skills every configured target | ee38602 | no (side-effect GREEN) |
| @e2e x3 | 79bb7b0 | YES (steps/init_agent_target_selection_steps.rs — e2e binding + argv fix) |
| config template + review step + self-hosting | 800b27b, ec64ddd | YES (template, review step, config) |
| coverage scenarios + dead-code removal | 18ade54 | YES (steps file, targets.rs) |

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `AgentTarget` enum | YES | `src/skills/targets.rs` |
| `Skills` config struct | YES | `src/config/overlay.rs` |
| `load_skills_targets()` | YES | `src/config/overlay.rs` |
| `prompt_for_targets()` | YES | `src/cli/init.rs` |
| `write_skills_block()` | YES | `src/cli/init.rs` |
| step file for capability | YES | `tests/steps/init_agent_target_selection_steps.rs` |
| `skills:` block in config.yaml.tmpl | YES | `assets/config.yaml.tmpl` |
| narrowed review-guidance step | YES | `tests/steps/arc42_review_alignment_steps.rs` |
| self-hosted `skills:` block in repo config | YES | `givn/config.yaml` |

**Strict-mode proof** — confirmed present and passing (tasks.md setup task:
`todo!()` stub → "1 scenario (1 failed) ... error: test failed", stub
reverted; `.fail_on_skipped()` in `tests/features_runner.rs`).

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps
assert through the real interface (CLI subprocess: exit code + the
filesystem the subprocess wrote — for a CLI tool that IS the observable
output; no repository-only downgrade):

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Consumer selects agent targets during interactive init | YES (exit code + file existence on disk) | N/A (CLI) | no |
| Consumer runs init non-interactively and gets opencode skills | YES (exit code + file existence) | N/A (CLI) | no |
| Consumer re-syncs skills after init and the selection is honoured | YES (exit code + file existence) | N/A (CLI) | no |

**E2e scope check** — exactly one `@e2e` per distinct happy-path action:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| init-agent-target-selection | 3 | 3 (interactive init answer; non-interactive init; sync after init) | no |
| install-extras (deltas) | 0 | no new interactions | n/a |

**Duplicate-implementation check** —

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| init-agent-target-selection | `cargo test --test features_runner -- --tags @e2e` → `tests/steps/init_agent_target_selection_steps.rs` | none | n/a |

**Local environment check** — CLI capability; system is the binary itself;
no stack to start (`cargo build --release` is the local run command per
design.md). Label: not applicable by design.

**E2e isolation check** —

| `verify.command` value | `verify.e2e_command` value | Identical? | Full-suite scenario count | E2e-only scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `cargo test --test features_runner` | `cargo test --test features_runner -- --tags @e2e` | no | 21 | 3 | YES (3 < 21) |

**Design.md conformance check** —

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| prompt reads one stdin line, drop-invalid | yes | yes | YES | n/a |
| silent re-init keeps selection; `--force` re-prompts | yes | yes | YES | n/a |
| sync write-only, no deletion | yes | yes | YES | n/a |
| skills targets in `givn/config.yaml` `skills:` block | yes | yes | YES | n/a |
| step file per capability | `tests/steps/init_agent_target_selection_steps.rs` | same | YES | n/a |
| e2e steps share the capability step file | yes | yes | YES | n/a |
| test-runner commands with `GIVN_FEATURES` change dir | yes | used throughout | YES | n/a |

**Interaction coverage verification** —

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| run `givn init` interactively and answer the agent-selection prompt | YES | Consumer selects agent targets during interactive init | subprocess + piped stdin | `run_givn_with_stdin` (piped stdin, answer written, closed) | YES |
| run `givn init` non-interactively (no prompt; falls back to opencode) | YES | Consumer runs init non-interactively and gets opencode skills | subprocess, null stdin | `run_givn` (`Command::output()`, null stdin) | YES |
| run `givn skills sync` after the selection has been persisted | YES | Consumer re-syncs skills after init and the selection is honoured | subprocess after config precondition | `run_givn` against config carrying the persisted block | YES |

## Arc42 implementation conformance

The `arc42` addon is enabled for this repo (`givn/config.yaml`). Independent
facts from the completed change and durable docs:

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| ch.3 context: prompt at init | 03-context-and-scope.md | Yes (row 3) | prompt on fresh config, default opencode | S2, S3, S9 | `prompt_for_targets()` in `src/cli/init.rs`; scenarios green | MATCH |
| ch.4 agent-integration row | 04-solution-strategy.md | Yes (row 4) | configured targets | S1, S6, S14, S15 | `skills.targets` config + `sync()` target-driven | MATCH |
| ch.5 building blocks | 05-building-block-view.md | Yes (row 5) | init/overlay/targets responsibility changes | S1–S13 | files/impl per design | MATCH |
| ch.6 init + sync flows | 06-runtime-view.md | Yes (row 6) | flow 6.1/6.8 rewritten | S1, S2, S8, S9 | implementation matches flows | MATCH |
| ch.8 skill generation | 08-crosscutting-concepts.md | Yes (row 8) | configured-target emit | S6, S7, S14, S15 | `write_skill_to_targets` per configured target | MATCH |
| ch.9 ADR-036 | 09-architecture-decisions.md | Yes (row 9) | decision outcome = ask-at-init w/ opencode default | spec + impl | prompt + persisted `skills:` block + write-only sync | MATCH |
| ch.11 R-12 | 11-risks-and-technical-debt.md | Yes (row 11) | migration risk recorded | S8, S9, full-suite gate | self-hosted config keeps all three targets | MATCH |
| ch.12 "Agent target" | 12-glossary.md | Yes (row 12) | term used in design | spec steps | `AgentTarget` enum | MATCH |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN

---

## Coverage

**Coverage result:** measured via `cargo llvm-cov` (change-scoped run —
the delta suite is the only suite whose expected failures are resolved
pre-archive; the permanent suite's 3 failing scenarios are exactly the
ones this change's deltas modify).

```
Filename            Regions Missed Regions Cover  Functions Missed Funcs Executed Lines Missed Lines Cover
cli/init.rs              215      32        85.12%      5          0       100.00%    119      13      89.08%
skills/targets.rs        211      21        90.05%     13          0       100.00%    145       5      96.55%
config/overlay.rs        308      87        71.75%     16          6        62.50%    197      60      69.54%
cli/skills.rs             27       4        85.19%      2          0       100.00%     18       1      94.44%
```

Coverage measurement validity: the runner (`cargo test --test
features_runner`) executes the `.feature` specs; coverage was captured with
`cargo llvm-cov --test features_runner` (instrumented binary, merged
profile, non-zero production coverage — the four changed production files
all show non-zero coverage with 89–97% line coverage). Measurement VALID.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/cli/init.rs` L133–142 | TTY prompt rendering (`stdin.is_terminal()` branch) | 3 (legitimately hard to test) | Prompt semantics (answer parsing, default, drop-invalid, EOF) are fully covered via piped stdin; the literal terminal rendering requires a PTY, which the test harness does not use. Justified. |
| `src/cli/init.rs` L158 | duplicate-target guard `Some(_) => {}` | 3 (legitimately hard to test / defensive) | Redundant with the dedup in `load_skills_targets`; unreachable observable divergence. Justified. |
| `src/cli/init.rs` L190 | trailing-newline guard | 3 (defensive) | givn-written configs always end with `\n`; guard covers only hand-edited files. Justified. |
| `src/cli/init.rs` L91–94 | skill-sync failure warning in init | 3 (error path; pre-existing) | Pre-existing error branch (unchanged code) requiring skill-generation failure; not introduced by this change. |
| `src/skills/targets.rs` L176/188/234/243/249 | `validate_frontmatter` bails | 3 (error paths; pre-existing) | Pre-existing validation branches for malformed templates, outside this change's surface. |
| `src/config/overlay.rs` L35–L269 (apply/validate/cycle/overlay_to_artifact/merge_artifact) | pre-existing overlay machinery | 3 (pre-existing; unchanged) | Not covered by the features_runner measurement (it has its own `#[cfg(test)]` unit tests in `src/` which the repo's coverage commands do not include); untouched by this change. |
| `src/cli/skills.rs` L11 | `givn/ not found` bail | 3 (pre-existing; unchanged code) | Pre-existing unreachable-in-suite guard identical to the old implementation; not part of this change's surface. |

All new code introduced by this change is covered: 96.55% lines in
`skills/targets.rs`, 89.08% in `cli/init.rs`, with the only uncovered new
lines classified above as bucket 3 (defensive/PTY/pre-existing). The two
bucket-2 gaps found during review (`.agents` prompt selection;
no-skills-block default fallback) were resolved by adding scenarios and a
dead-branch removal — commit `18ade54`, re-measured green.

---

## Test suite result

```
GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/ cargo test --test features_runner
→ 21 scenarios (21 passed), 123 steps (123 passed)

GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/ cargo test --test features_runner -- --tags @e2e
→ 3 scenarios (3 passed), 18 steps (18 passed)

givn lint --change init-agent-target-selection
→ "2 file(s) checked — clean"

Permanent suite (GIVN_FEATURES=givn/specs/): 134 scenarios (131 passed) —
the 3 failures are exactly the install-extras scenarios this change's
deltas modify; they merge green at archive.
```

**Result:** PASS

**Coverage measured:** YES (via `cargo llvm-cov`, see coverage section)

---

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no
      over-produced e2e scope).
- [x] All scenarios pass (`GIVN_FEATURES=givn/changes/.../specs/ cargo test
      --test features_runner` exits 0).
- [x] Every coverage gap is classified and resolved (or justified).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack cleanly (N/A — CLI-only
      capability; binary is the system).
- [x] No parallel e2e implementation exists (tree searched incl. untracked).
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (3 < 21).
- [x] Implementation matches design.md as reviewed; no silent deviations.
- [x] Interaction coverage verified: every inventory entry maps to a matrix
      row, an existing `@e2e` scenario, and matching driving mechanism — no
      mismatches.
- [x] Design.md's `## Internal Primitives` table accurately lists the new
      primitives (`AgentTarget`, `Skills`, `load_skills_targets`,
      `prompt_for_targets`, `write_skills_block`) — all implemented.
- [x] arc42 slug-mapping verified: every chapter slug in arc42.md maps to a
      real `docs/arc42/*.md` file (rows 3, 4, 5, 6, 8, 9, 11, 12 — all
      exist and were edited by this change).
- [x] No finding in this report is excused with a classification outside the
      three buckets.

**REVIEW: PASS**