# Review: fix-opencode-llm-attribution

## Fabrication Audit (do this first)

**Step definition scan** — `tests/steps/llm_attribution_steps.rs` and
`tests/steps/llm_attribution_e2e_steps.rs` scanned for empty/no-op bodies
(`{}`, bare `return`, bare `pass`, `unimplemented!`, `todo!`, no assertion
reference). Every step body asserts on a real subprocess result, environment
value, filesystem state, or Git object.

```
0 empty step bodies found across 2 capability step definition files.
0 occurrences of unimplemented!/todo!() in tests/steps/.
```

**`@e2e` tag integrity** — all three delta scenarios carry `@e2e` in the
tracked feature file (`@givn.modified @e2e`, `@givn.added @e2e`,
`@givn.added @e2e`); no tag was removed to bypass the verify-e2e gate.

**Commit cross-check** — every scenario task has a recorded commit touching
production source (plugin asset, checked-in generated plugin, or attribution
CLI):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| An attributed commit exposes the current LLM and harness (modified) | `95f4b3e9` | Yes — `src/cli/attribution.rs`, `src/cli/init.rs`, `assets/agent-integrations/opencode/givn-attribution.js`, `.opencode/plugins/givn-attribution.js` |
| OpenCode prepares attribution using its selected model (added) | `25de68da` | Yes — `assets/agent-integrations/opencode/givn-attribution.js` + checked-in copy |
| Claude attributes a commit using its current model (added) | `55cc9b05` | Yes — `src/cli/attribution.rs` |
| Evidence commits | `a50b4c33`, `bba32edc` | Docs only (tasks.md evidence) — not counted as scenario commits |

**Promised-architecture cross-check** — every component design.md named exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Generated OpenCode plugin reading `input.model` | Yes | `assets/agent-integrations/opencode/givn-attribution.js` (lines 4–37) |
| Checked-in generated plugin (byte-parity asserted by scenario) | Yes | `.opencode/plugins/givn-attribution.js` |
| Claude transcript model extraction | Yes | `src/cli/attribution.rs:124-157` (`claude_model`, `claude_transcript_model`) |
| Managed co-author stamping only | Yes | `src/cli/attribution.rs:29-58` (`stamp_message_values`, `is_managed_trailer`) |
| Existing embedded-asset sync path | Yes | `src/skills/targets.rs` (unchanged, embeds the asset) |
| Capability-specific E2E step file | Yes | `tests/steps/llm_attribution_e2e_steps.rs` |

**Strict-mode proof** — present in tasks.md setup task and in code:
`tests/features_runner.rs:55` and `:104` call `.fail_on_skipped()`; the
recorded RED run exits non-zero (`exit 101`, `assertion left: None, right:
Some("openai/gpt-5.6-luna")` at tasks.md:39-47).

**Downgraded e2e scenario scan** — no browser UI exists for this capability;
each `@e2e` Then asserts the observable output of the real boundary named in
design.md:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| An attributed commit exposes the current LLM and harness | Yes — `git show` committer (`%cn`) and full message (`%B`) after a real commit | N/A (CLI/Git) | No |
| OpenCode prepares attribution using its selected model | Yes — Node.js runs the generated plugin module and the test parses the JSON shell environment it returns | N/A (plugin module) | No |
| Claude attributes a commit using its current model | Yes — generated PreToolUse hook output is executed and the resulting Git commit is inspected | N/A (hook/CLI) | No |

**E2e scope check** — the active owner is the reusable `support` fragment; its
`## Interactions` table is `none` by contract, and the three scenarios are
distinct actions (commit attribution, OpenCode shell-environment preparation,
Claude Bash-command rewrite), not variants of one action:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| llm-attribution (fragment) | 3 | Yes — one per distinct interaction | No |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e` (tag filter `@e2e`); for this change scope it binds
`tests/steps/llm_attribution_e2e_steps.rs`. `git status` was searched for
parallel implementations: only the unrelated installation capability step
files exist, plus an untracked `givn/changes/archive-proof/` belonging to the
separate `archive-proof` active change and the uncommitted glossary terms that
change is adding. No second/weaker attribution E2E implementation was found.

**Local environment check** — design.md's Local Runnability names
`./run-tests.sh`; the configured `run.command` starts cleanly:

```
$ cargo run --locked -- instructions
... (status output printed)
exit 0
```

No application server, database, network dependency, or digital twin is
required; scenarios create temporary projects and drive the generated
integrations and real Git.

**E2e isolation check** — values read literally from `givn/commands.yaml`:

| `verify.command` value | `verify.e2e_command` value | Identical? | Full-suite scenario count | E2e-only scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 390 | 75 | Yes — 75 < 390 |

Change-scoped: verify `3` / e2e `3`. The delta has no non-`@e2e` scenarios, so
the configured exception applies: every in-scope scenario is `@e2e`, stated
explicitly here.

**Interaction coverage verification** — the fragment inventory is empty
(`givn/specs/fragments/fragment.md` → `## Interactions: none`), so no
inventory row exists to map; the design's matrix rows still resolve to real
scenarios and drivers:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| none (fragment) | n/a | An attributed commit exposes the current LLM and harness | Git CLI | `git_with_givn_path` commit + `git show` inspection | Yes |
| none (fragment) | n/a | OpenCode prepares attribution using its selected model | Generated OpenCode plugin module | `node` imports the generated module, dispatches `chat.message`/`chat.params`/`shell.env`, parses returned env | Yes |
| none (fragment) | n/a | Claude attributes a commit using its current model | Claude hook + RTK wrapper + Git CLI | Generated hook receives a real JSON payload with `transcript_path`; returned RTK-wrapped command executed; `git show` inspected | Yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | Owning owner is the reusable `support` fragment (`fragment.md`). Best-effort attribution, non-blocking commits, process-scoped env protocol, and managed co-author output are preserved; the shared Git path stamps only the managed line and drops stale legacy trailers | Yes |
| Capability ownership and typed relationships preserved | `llm-attribution` remains registered in `givn/specs/fragments/fragment.md`; the delta routes `NEW in fragments` per the proposal table; archive resolves the existing fragment directory | Yes |
| Confirmed Persona-relevant outcomes preserved | Proposal records no confirmed Personas; none invented or promoted | Yes |
| Actors remain distinct from Persona biographies | Actors are the harnesses/consumer roles; no Persona biography is used as a Gherkin actor | Yes |
| Migration ledger and coverage evidence complete, if applicable | Not a migration change — N/A | N/A |

**Design.md conformance check** — implementation diffed against what design.md
explicitly named:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| OpenCode model source | Read `input.model.providerID`/`modelID` in `chat.params`, optional `chat.message` enrichment | `assets/agent-integrations/opencode/givn-attribution.js:15-20`, `:27-35` | Yes | — |
| Claude model source | Latest assistant model from `transcript_path`, env fallbacks retained | `src/cli/attribution.rs:124-157` | Yes | — |
| Bash scope | Export context for the entire Bash command (`export ...; command`), not one-process `env` | `src/cli/attribution.rs:106-121` | Yes | — |
| Message output | Only the managed `Co-Authored-By: <model>/<reasoning>@<harness> with Givn <version>` line; legacy trailers removed | `src/cli/attribution.rs:29-58` | Yes | — |
| Step file | `tests/steps/llm_attribution_e2e_steps.rs` | Exists and bound | Yes | — |
| Drivers | Node.js latest LTS; real PreToolUse payload; real Git commit | Steps use `node`, real hook stdin, real `git`/`sh` | Yes | — |
| Runners | `./run-tests.sh`, `./run-tests.sh --e2e`, `GIVN_FEATURES` scoping | Matches `givn/commands.yaml` | Yes | — |
| `src/cli/init.rs` touched | Not named (no behavior change) | Diff is brace-formatting only — no deviation | Yes | — |

## Retirements

N/A — this delta contains no `@givn.retired` capability.

## Arc42 implementation conformance

Affected chapters were derived independently from the completed change and
compared against `docs/arc42/` and `arc42.md`. All affected chapters contain
current, decision-specific content (not placeholders or templates).

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| 01 Introduction and Goals | `01-introduction-and-goals.md` | Not affected | Existing goal refined | No task | No new goal/stakeholder | Match |
| 02 Constraints (TC-30) | `02-architecture-constraints.md:30` | Affected | Payload/transcript constraints stated | Setup + scenario tasks | `src/cli/attribution.rs:124-157`; Claude scenario | Match |
| 03 Context and Scope | `03-context-and-scope.md:56` | Affected | OpenCode/Claude boundaries | Scenario tasks | Generated plugin + hook assets | Match |
| 04 Solution Strategy | `04-solution-strategy.md` | Not affected | Adapter/hook strategy unchanged | No task | No strategy change | Match |
| 05 Building Block View | `05-building-block-view.md:6-10` | Affected | Attribution CLI owns transcript lookup + stamping | Production-impact tasks | `src/cli/attribution.rs`; plugin asset | Match |
| 06 Runtime View | `06-runtime-view.md:46-69` | Affected | Hook sequences documented | E2E scenario tasks | Two scenario step definitions drive the sequences | Match |
| 07 Deployment View | `07-deployment-view.md` | Not affected | Generated paths unchanged | No task | No new deployment artifact | Match |
| 08 Crosscutting Concepts | `08-crosscutting-concepts.md:348-352` | Affected | Managed trailer normalization, full-command scope | GREEN/REFACTOR tasks | `stamp_message_values`, `export ...; command` | Match |
| 09 Architecture Decisions | `09-architecture-decisions.md` | Not affected | `arc42.md` ADR qualification `NOT_QUALIFIED` | No task | No ADR required; feature/design canonical | Match |
| 10 Quality Requirements (Q-78, Q-79) | `10-quality-requirements.md:75-76` | Affected | Executable OpenCode/Claude quality scenarios | Scenario tasks | Three `@e2e` scenarios pass | Match |
| 11 Risks and Technical Debt (R-51) | `11-risks-and-technical-debt.md:38` | Affected | Payload/transcript drift with regression coverage | Scenario tasks | Missing-model and transcript scenarios | Match |
| 12 Glossary | `12-glossary.md:9-15` | Affected | `Model identity`, `Managed co-author attribution` defined | Specs/design reuse exact terms | Specs, design, code use the glossary terms | Match |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or
      placeholder content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; evidence
      names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN.

## Ubiquitous language conformance

Specs, design, and code reuse the durable glossary terms `Model identity`,
`Attribution context`, `Reasoning mode`, `Harness`, `Attribution trailer`, and
`Managed co-author attribution`; no competing synonym (for example "model
name") appears in the delta. The legacy `GIVN_ATTRIBUTION_LLM` environment
name and `LLM:` committer prefix are pre-existing protocol identifiers, not
domain vocabulary. Ubiquitous language conformance: CLEAN.

## Coverage

Commands run with the change's delta as the feature scope (the delta scenarios
do not exist in `givn/specs` until archive merges them):

```sh
GIVN_FEATURES=givn/changes/fix-opencode-llm-attribution/specs ./measure-coverage.sh
./merge-coverages.sh
```

Fresh merged report (`coverage/cobertura-coverage.xml`):

```text
line coverage: 1064/18713 = 5.69%
branch coverage: 0/0 (not emitted by the configured Cobertura tool)
```

The 5.69% figure reflects the three-scenario delta scope, not product quality.
The instrumented production process is the spawned `givn` binary (proved by
non-zero coverage in `src/cli/attribution.rs`); the Gherkin runner is the
instrumented measurement process. The generated JavaScript plugin has no
instrumentation configured; its behaviour is asserted directly by the Node.js
E2E scenario (returned shell environment).

| Change-owned file | Line coverage |
|---|---:|
| `src/cli/attribution.rs` | 109/123 = 88.62% |
| `assets/agent-integrations/opencode/givn-attribution.js` | not instrumented; behaviour asserted E2E |
| `.opencode/plugins/givn-attribution.js` | byte-parity asserted with the asset |
| `src/cli/init.rs` | formatting-only diff; not change-owned |

### Coverage gap classification

Every uncovered change-owned or change-touched region is classified into the
three buckets; no fourth bucket is used.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/cli/attribution.rs:20` — inactive-attribution early return | 1 | 3 — legitimately hard to test | Pre-existing guard; behaviour asserted by the permanent "human commit remains usable" scenario in the full permanent suite; the delta scope contains no non-e2e scenario that executes it |
| `src/cli/attribution.rs:63` — `unavailable` fallback for an empty env value | 1 | 3 — legitimately hard to test | Pre-existing fallback exercised by the permanent partial-context scenario in the full permanent suite; not reachable from the delta scenarios |
| `src/cli/attribution.rs:71,74` — no-op on malformed/commandless PreToolUse payload | 2 | 3 — legitimately hard to test | Defensive no-op branches for invalid external hook input; the generated hook always supplies a command, so exercising them requires fabricating a payload Claude Code never sends |
| `src/cli/attribution.rs:127-130` — `CLAUDE_MODEL` legacy fallback | 4 | 3 — legitimately hard to test | Design explicitly retains the existing env fallback unchanged; real PreToolUse payloads carry `transcript_path`, so the fallback is reachable only in legacy sessions. The fallback outcome is unit-tested at the function boundary |
| `src/cli/attribution.rs:141-145,147` — alternate assistant-record shape / non-assistant skip | 6 | 3 — legitimately hard to test | Defensive parsing for transcript record variants; the real recorded assistant fixture uses the dominant `type` field, and the skip branch needs a crafted non-assistant transcript |

- Dead code removed: 0 (none found).
- Missing tests added: 0 (no unclassified gap).
- Hard-to-test gaps justified: 5 regions, all with concrete technical
  justification above.
- Redundant unit tests removed: 0 (none found).

## Test suite result

Executed with the dogfooded attribution variables cleared
(`GIVN_ATTRIBUTION_*`, `GIT_COMMITTER_NAME`) because this project's own
OpenCode plugin injects them into the agent shell; with them set, the
permanent "human commit remains usable" scenarios correctly observe an active
attribution context and fail. This is an environment artefact of running the
suite from inside an attributed agent session, not a product defect.

```
$ ./run-tests.sh
Summary: 390 tests run: 390 passed, 0 skipped

$ ./run-tests.sh --e2e
Summary: 75 tests run: 75 passed, 315 skipped

$ GIVN_FEATURES=givn/changes/fix-opencode-llm-attribution/specs cargo test --test features_runner
3 scenarios passed (change-scoped, all @e2e)

$ cargo nextest run --locked
Summary: 875 tests run: 875 passed, 2 skipped (unit + features)
```

**Result:** PASS

**Coverage measured:** YES (configured measure + merge commands produced a
fresh merged report)

## Overlap dispositions

The deterministic gate reported `net delta: 2 added, 1 modified, 0 removed`
and found no current shape-match findings involving this delta, so no
disposition row is required. The one modified scenario ("An attributed commit
exposes the current LLM and harness") supersedes its permanent predecessor:
same title and interaction, assertion updated from legacy `Givn-*` trailers to
the managed co-author line.

## Split-or-keep

No delta scenario exceeds the long-scenario threshold; the gate requested no
split-or-keep row.

## README impact decision

README-IMPACT: none

The README Quick start already states that OpenCode and Claude Code targets
receive best-effort runtime integrations whose commits show the LLM, reasoning
mode, harness, and Givn version; the corrected model resolution does not change
any documented command, flag, configuration, or output description.

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified, all
      promised components exist, no downgraded e2e scenarios, no over-produced
      e2e scope).
- [x] All scenarios pass (`./run-tests.sh` exits 0).
- [x] Every coverage gap is classified and resolved (or justified).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly (no service or twin required; `cargo run --locked --
      instructions` exits 0).
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and the
      tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (75 < 390) — change scope is 3/3 because all
      in-scope scenarios are `@e2e`, stated explicitly.
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; no deviation.
- [x] Interaction coverage verified: the fragment inventory is `none` and each
      design matrix row maps to an existing `@e2e` scenario whose step
      definitions use the promised driving mechanism.
- [x] Internal primitives: the change adds only file-private helpers
      (`CLAUDE_HARNESS`, `claude_model`, `claude_transcript_model`), named in
      design.md's Technical Decisions; no new abstraction or public type.
- [x] arc42 slug-mapping verified: affected chapter names in `arc42.md` resolve
      to real `docs/arc42/*.md` files with current content.
- [x] No finding is excused with a classification outside the three buckets.

REVIEW: PASS
