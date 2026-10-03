# Review: require-agent-target

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, `todo!`, `unimplemented!`, no
assertion reference):

```
$ grep -nE "todo!|unimplemented!|^\s*\{\s*\}\s*$|return;\s*$" \
    tests/steps/init_agent_target_selection_steps.rs tests/steps/mod.rs
tests/steps/mod.rs:306,309,312,318,322: `return;` — early exits inside the
fixture helper `ensure_fixture_usecase` (path-shape guards), not step bodies.

0 empty step bodies found across the two step-definition files (19 `#[when]`/
`#[then]`/`#[given]` registrations in init_agent_target_selection_steps.rs,
81 in mod.rs). The new step `init_in_terminal_empty_confirmation` spawns the
real `script -qec` PTY, writes the confirmation key, enforces a 60-second
deadline, and stores the transcript as the command output.
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit. Side-effect-GREEN scenarios share the production commit of
the scenario that introduced the code path; their own revisions carry the
delta change (the scenario body/tag and the task record):

| Scenario | Commit | Touches production code? |
|---|---|---|
| 1 Non-interactive init without an agent target refuses initialization | `34360457` | yes — `src/cli/init.rs` + fixture call sites |
| 2 The interactive prompt refuses a confirmation without a marked target | `a11ae096` | yes — `src/cli/init.rs` dialog branch + PTY step |
| 3 An empty answer refuses initialization | `9d1072e7` | yes — `src/cli/init.rs` piped path |
| 4 An answer without any known agent refuses initialization | `46618bdd` | side-effect of `9d1072e7` |
| 5 An unknown-only --agents value refuses initialization | `a9522198` | yes — `resolve_agents_flag` returns `Result` |
| 6 givn init --force re-applies the explicitly given selection | `12916901` | side-effect of `34360457`/`a9522198` |
| 7 givn init --force without a selection keeps the persisted selection | `78d758bd` | side-effect of `34360457` (resolve-before-write) |
| 8–9 enable-optional-addons migrations | `b477f7e2`, `15272625` | side-effect of the above |
| 10 A fresh project resolves the rules policy to local-only | `a6c360af` | side-effect of the above |
| 11–14 e2e migrations | `0723aefa`, `f2d69c4e`, `d6340e8d`, `2a88b6fc` | side-effect of the above |

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `require_agent_target()` and the refusal sentence | yes | `src/cli/init.rs:205-216` |
| `prompt_for_targets()` returning `Result`, refusing EOF | yes | `src/cli/init.rs:178-201` |
| `resolve_agents_flag()` returning `Result` | yes | `src/cli/init.rs:216-225` |
| target resolution before the first write | yes | `src/cli/init.rs:33-50` |
| `DEFAULT_TARGETS` deleted | yes (absent) | `grep DEFAULT_TARGETS src/` → no hits |
| PTY confirmation step | yes | `tests/steps/init_agent_target_selection_steps.rs:165-214` |
| fixtures naming the selection | yes | 16 `"--agents"` call sites under `tests/` |

**Strict-mode proof** — present in tasks.md's setup task: `.fail_on_skipped()`
at `tests/features_runner.rs:55,129`, plus the live proof
`GIVN_FEATURES=/tmp/givn-strict-proof/ cargo test --locked --test features_runner -- --name "An undefined step must fail the runner"`
→ exit 1 with `Step doesn't match any function`.

**Downgraded e2e scenario scan** — every `@e2e` scenario's primary assertion
is on the CLI's observable result (exit code, stdout/stderr, the files it
wrote), driven by a real subprocess; no browser interface exists:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Consumer selects agent targets during interactive init | yes — exit code + installed files | n/a (CLI) | no |
| Consumer runs init non-interactively and gets opencode skills | yes — `givn init --agents opencode`, exit 0, files exist/absent | n/a (CLI) | no |
| Consumer re-syncs skills after init and the selection is honoured | yes — `givn skills sync` output + files | n/a (CLI) | no |
| Consumer initializes a Fossil checkout and receives backend-aware guidance | yes — exit code + generated guidance text | n/a (CLI) | no |
| Consumer initializes a plain directory and receives a named warning | yes — stdout warning + later refusal | n/a (CLI) | no |

**E2e scope check** — one `@e2e` scenario per distinct consumer action; the
refusal cases and migrations are regular scenarios:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| initialize-a-project | 4 (select targets, init non-interactively, resync, plus the two backend variants for Fossil/no-SCM) | yes — one per inventory action | no |
| enable-optional-addons | 0 | n/a (its initialization variants are regular) | no |
| declare-the-product-start | 1 | yes (inventory row added by its own change) | no |
| measure-coverage-per-project | 1 | yes | no |
| readme-enforcement | 1 | yes | no |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`, which runs `cargo nextest run --locked --test
features_runner` with the `@e2e` tag expression; the invoked file is
`tests/features_runner.rs` with step definitions in `tests/steps/*.rs`.
`git status` showed no untracked step implementation; a tree search found no
parallel e2e implementation for these capabilities.

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| initialize-a-project / the migrated capabilities | `tests/features_runner.rs` + `tests/steps/{init_agent_target_selection_steps,mod}.rs` | no | n/a |

**Local environment check** — the CLI has no server, database, or digital
twin; the local run command is
`cargo build --release && ./target/release/givn init --agents opencode`
(interactive: `./target/release/givn init` from a real terminal). The PTY
scenario additionally proves the terminal path via `script` (util-linux),
now documented in `AGENTS.md` as a suite prerequisite.

**E2e isolation check** — read literally from `givn/commands.yaml`; the
change is not archived yet, so no receipt exists. The delta-scope runs during
implementation measured the counts:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 19 (delta scope) | 4 (delta scope) | yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `specs/initialize-project/usecase.md`: main flow 1 requires a selection; the minimal guarantee ("project remains unchanged when a request is rejected") is asserted by the refusal scenarios (`givn` directory absent, persisted selection unchanged) | yes |
| Capability ownership and typed relationships preserved | `initialize-a-project` extended in place; `Includes: fragment: support`, `Extends: none` untouched | yes |
| Confirmed Persona-relevant outcomes preserved | `developer--primary-user` sees the refusal in its own vocabulary (targets + `--agents`) | yes |
| Actors remain distinct from Persona biographies | Gherkin actors stay `the user` / `the consumer` | yes |
| Migration ledger and coverage evidence complete, if applicable | not a migration change | yes |

PERSONAS: developer--primary-user

## Deferred mandatory checks

None. The interactive path is covered by a real PTY driver rather than
deferred; no `DEFERRAL:`/`OPERATOR-DECISION:` pair is recorded.

**Design.md conformance check** — implementation diffed against what design.md
named:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Runner and e2e command | `verify.command = ./run-tests.sh`, `verify.e2e_command = ./run-tests.sh --e2e` | unchanged | yes | — |
| Step-definition layout | init-specific steps in `tests/steps/init_agent_target_selection_steps.rs`, generic steps in `tests/steps/mod.rs` | exactly that; one new PTY step | yes | — |
| Refusal error | `no agent target selected; … --agents … (valid targets: …)` | identical text | yes | — |
| Resolution order | targets resolved before the first write | `src/cli/init.rs:33-50` | yes | — |
| Dialog default | removed; empty confirmation refused | no `.defaults(...)`, `require_agent_target` | yes | — |
| PTY obstacle fix | `script -qec`, 60-second deadline | step as designed | yes | — |
| Step Definitions table format | one backticked path per row | the table was reformatted after design-review to satisfy the integrity gate's one-path-per-row parse; no decision changed, and the promised files are unchanged | yes | formatting-only fix recorded here; no design decision changed |

**Interaction coverage verification** — spec inventory (delta usecase.md
Interactions) against design matrix, `.feature` file, and step definitions:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| select agent targets | yes | Consumer selects agent targets during interactive init | CLI subprocess with piped stdin | `run_givn_with_stdin` writes `opencode, .claude\n` | yes |
| initialize non-interactively | yes | Consumer runs init non-interactively and gets opencode skills | CLI subprocess with `--agents opencode` | `consumer_runs` → `run_givn` with the flag | yes |
| resynchronize selected targets | yes | Consumer re-syncs skills after init and the selection is honoured | CLI subprocess `givn skills sync` | `run_givn(env, &["skills","sync"])` | yes |
| initialize a Fossil project | yes | Consumer initializes a Fossil checkout and receives backend-aware guidance | CLI subprocess in a Fossil checkout | `llm_attribution_steps.rs:172` spawns `init --agents opencode,claude` | yes |
| initialize without SCM | yes | Consumer initializes a plain directory and receives a named warning | CLI subprocess in a plain directory | `init` via the recorded step + stdout assertions | yes |

## Visual Review

N/A — the design declares no visual interface (`## Visual Design Contract`
section: "Not present: user-visible surface is CLI text only"). The opposite
classification was tested: nothing in this change renders HTML, CSS, or
JavaScript; the only surface is stderr/stdout text from the CLI binary, so a
Web UI classification does not hold.

## Retirements

N/A — this change retires no capability.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| `givn init` resolves the target selection before the first write and refuses an empty selection | `docs/arc42/06-runtime-view.md` §6.1 | affected, updated | "resolution order in `run()`" decision | scenario 1 (resolve-before-write), scenario 7 (--force keeps selection) | `src/cli/init.rs:33-50`; delta scenarios pass | yes |
| Initialization requires a selection; sync keeps the `opencode` fallback | `docs/arc42/08-crosscutting-concepts.md` §8.7 | affected, updated | "only the initialization precondition changes" | scenarios 1–5; final verification | `require_agent_target`; `load_skills_targets` unchanged in `src/config/overlay.rs` | yes |
| ADR-013 target-selection contract refined | `docs/arc42/adr/ADR-013-agent-integration.md`; register `docs/arc42/09-architecture-decisions.md`; `adr/README.md` | affected, updated (`AMEND_ADR`) | `## Decisions` D1/D2 | scenario 1–2 commits | ADR + register rows present | yes |
| Q-5 agent-ergonomics precondition enforced at initialization | `docs/arc42/10-quality-requirements.md` | affected, updated | refusal before write | scenarios 1–5 | delta refusal scenarios | yes |
| Scripted init compatibility consequence tracked | `docs/arc42/11-risks-and-technical-debt.md` R-72 | affected, updated | risk noted in proposal/design | final verification | delta scenario 1 asserts the refusal message names `--agents` | yes |
| Glossary terms | `docs/arc42/12-glossary.md` | not affected | design uses "Agent target"/"Harness" | — | wording matches glossary; no new term | yes |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; evidence
      names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN

## Coverage

Coverage is measured once by the archive against the committed candidate. No
receipt exists yet for this change (unproven candidate), so this review
classifies from static inspection of the delta and the touched code.

**Coverage result:** no archive receipt yet — classification from inspection.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/cli/init.rs` refusal path and both branches | `require_agent_target`, EOF branch, dialog branch, piped branch, flag branch | no gap — every branch is exercised by a delta scenario (EOF, PTY empty confirmation, empty line, unknown-only answer, unknown-only flag, valid flag, `--force` with and without a value) | — |
| `DEFAULT_TARGETS` | deleted | dead code (YAGNI) — no fallback target set exists | deleted in `9d1072e7` |
| Removed step `consumer_init_silent` | deleted | dead code | deleted in `46618bdd` |
| No other uncovered regions identified | — | — | — |

## Scenario execution evidence

Review does not re-run the suite. The change is not archived yet, so no
receipt exists; the delta-scope runs recorded in tasks.md are the execution
evidence: regular scope `19 tests run: 19 passed, 0 skipped`; e2e scope
`4 tests run: 4 passed, 15 skipped`.

**Receipt status:** not archived yet — unproven candidate

## Overlap dispositions

| Scenario A | Scenario B | Disposition | Rationale |
|---|---|---|---|
| The interactive prompt refuses a confirmation without a marked target | An empty answer refuses initialization | variant | Same refusal rule; different input path (PTY empty confirmation vs piped empty line). |
| The interactive prompt refuses a confirmation without a marked target | An unknown-only --agents value refuses initialization | variant | Same rule; PTY confirmation vs unknown-only flag value. |
| An empty answer refuses initialization | An unknown-only --agents value refuses initialization | variant | Same rule; empty piped line vs unknown-only flag value. |
| The selected agents are persisted in givn/config.yaml | givn init --force re-applies the explicitly given selection | variant | Same action (init persists the selection); different precondition (fresh config vs `--force` re-run with an explicit value). |
| Non-interactive init persists opencode as the default target | givn init --force re-applies the explicitly given selection | variant | The removed default-persistence scenario is replaced by explicit-flag persistence under `--force`. |
| Re-running init keeps the persisted selection | givn init --force re-applies the explicitly given selection | variant | Both re-initialize an existing project; no `--force` (keep) vs `--force` with an explicit value (re-apply). |
| givn init --force re-applies the default selection | givn init --force re-applies the explicitly given selection | duplicate | Same scenario: the delta removes the default form and adds the explicit-selection form. |
| An answer without any known agent falls back to the default | givn init --force re-applies the explicitly given selection | boundary | Different rules and inputs (unknown-only answer fallback vs explicit flag under `--force`); shape coincidence from shared steps. |
| givn skills sync defaults to opencode when the config has no skills block | Consumer runs init non-interactively and gets opencode skills | boundary | Different commands and guarantees (sync legacy fallback vs init's explicit selection). |
| The --agents flag accepts the dotted directory form | givn init --force re-applies the explicitly given selection | boundary | Flag-name parsing vs `--force` re-selection semantics; no shared rule. |

Removed+added pairs: the five removed scenarios are superseded by the added
refusal/`--force` scenarios (`Non-interactive init installs skills to opencode
only`, `Interactive init accepts the default selection with an empty answer`,
`Non-interactive init persists opencode as the default target`, `An answer
without any known agent falls back to the default`, `givn init --force
re-applies the default selection`), except `An answer without any known agent
falls back to the default`, whose replacement is `An answer without any known
agent refuses initialization` (added). The removed `@e2e` title `Consumer runs
init non-interactively and gets opencode skills` was not deleted to bypass the
gate: the same title is carried by the `@givn.modified @e2e` scenario, which
keeps its `@e2e` tag.

## Split-or-keep

No delta scenario exceeded the lint's long-scenario threshold; the gate
reported none.

| Scenario | Decision |
|---|---|

## Post-merge verification repair

The archive's merged-corpus regular verification failed one scenario:
`fragments/detect-the-project-repository.feature::A Fossil checkout nested
inside an ancestor Git worktree selects Fossil` — its Given calls
`run_givn_with_envs_in(..., &["init"], ...)` in
`tests/steps/scm_backend_selection_steps.rs`, a call site the fixture sweep
missed because it uses `let scaffold =` instead of `let out =`. The Given now
passes `--agents opencode`; the scenario passes. No spec, design, or review
decision changed — this is a test-fixture conformance repair.

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope (delta-scope runs; the
      archive receipt will carry the merged result; review does not re-run the
      suite).
- [x] Every coverage gap is classified and resolved (or justified) from
      inspection; no gap remains unclassified.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Canonical E2E policy read from `givn instructions specs`; every
      inventory action has exactly one `@e2e` scenario; error cases are
      regular scenarios.
- [x] Every E2E scenario asserts through the real interface (real CLI
      subprocess; the interactive path through a real PTY).
- [x] `verify.e2e_command` is not identical to `verify.command`, and the
      delta-scope counts prove the e2e scope is strictly smaller (4 < 19).
- [x] Implementation matches what design.md named; the one post-review edit
      (Step Definitions table formatting for the integrity gate) is recorded
      above and changes no decision.
- [x] Interaction coverage verified row by row above — no mismatches.
- [x] No finding was excused with a classification outside the three buckets.

README-IMPACT: updated - Quickstart

REVIEW: PASS
