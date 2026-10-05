# Review: add-omp-agent-target

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
$ grep -rnE '^\s*\{\s*\}\s*$' tests/steps/*.rs | wc -l
0
$ grep -rnE '^\s*(pass|return)\s*;\s*$' tests/steps/*.rs | wc -l
13
```

**0 empty step bodies found across 63 step definition files.** The 13 bare
`return;` matches are guard-clause early exits inside helper functions, each
preceded by real logic and a `let ... else`/`if` condition
(`tests/steps/mod.rs:314,317,320,326,330`,
`fix_uppercase_checkbox_parsing_steps.rs:44`,
`repository_ownership_fixtures_support.rs:99`, `run_execution_e2e_steps.rs:64`,
`run_execution_support.rs:31,192`, `scm_backend_selection_steps.rs:170,199`,
`workflow_enforcement_steps.rs:72`). No step body is a bare `return`. No
`unimplemented!()` / `todo!()` remains anywhere under `tests/steps/`.

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit:

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| 1 The --agents flag accepts "all" | `84d57c7b` | yes — `src/skills/targets.rs`, `src/config/overlay.rs` |
| 2 skills targets add accepts "all" **[regression]** | `fb42e95e` | no new production — the same `AgentTarget::ALL` from Scenario 1; the commit carries the scenario's spec/tasks as the guard |
| 3 dotted directory form **[regression]** | `4b0f2100` | no new production — `AgentTarget::from_name` already trims a leading dot (Scenario 1); spec/tasks guard |
| 4 omp target guidance + integration | `f1222d85` | yes — `assets/agent-integrations/omp/givn-attribution.js`, `src/skills/targets.rs`, `src/cli/init.rs`, `src/cli/skills.rs`, `src/cli/addons.rs`, `src/main.rs` |
| 5 omp rules stay out of the repository | `2fdcd508` | yes — `src/skills/rules.rs` |
| 6 no-revision command unchanged **[harness]** | `de246cdf` | test infrastructure — `tests/steps/llm_attribution_e2e_steps.rs`, `tests/steps/mod.rs`; the gate under test shipped with the asset in Scenario 4 |
| 7 partial context unavailable **[harness]** | `db258059` | no new production — the `unavailable` fallback shipped in Scenario 4's asset; spec/tasks guard |
| 8 e2e shared attribution plumbing | `f579c688` | no new production — the `.omp` integration ships with Scenario 4's target; spec/tasks carry the e2e evidence |
| 9 e2e omp attributes a commit **[harness]** | `15008b38` | no new production — the asset (Scenario 4) plus the harness from Scenario 6 |
| 10 e2e installation report | `4c680535` | test infrastructure — `tests/steps/llm_attribution_installation_e2e_steps.rs`; the report line shipped in Scenario 4 |
| 11 addons disable removes command files (remediation) | `eef5958b` | test infrastructure + spec — `tests/steps/mod.rs`; the `.omp` cleanup loops shipped in Scenario 4 |

Reading of this table: every scenario's implementation exists and is verified,
but the production artefact for six of them is a single file or loop that
shipped with the first scenario that needed it (the generated hook in Scenario
4, the target in Scenario 1, the rule patterns in Scenario 5). Those rows name
the enabling commit explicitly, so the mapping is traceable rather than
implied; the affected scenarios keep their own commit for the spec change and,
where new, the step definitions. No checked task is backed only by a stub, and
no task is unchecked.

**Promised-architecture cross-check** — every file and primitive `design.md`
promised:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| omp attribution asset | yes | `assets/agent-integrations/omp/givn-attribution.js` |
| `AgentTarget::Omp` + `owns_commands` | yes | `src/skills/targets.rs` |
| `write_command` (per-target) | yes | `src/skills/targets.rs` |
| `.omp` entries in `managed_generated_paths` | yes | `src/skills/targets.rs` |
| `.omp` loops in `reconcile_addon_artifacts` | yes | `src/skills/targets.rs` |
| `.omp` patterns in `ignore_entries` / `is_rule_path` | yes | `src/skills/rules.rs` |
| `omp` in `KNOWN_AGENT_TARGETS` | yes | `src/config/overlay.rs` |
| init report + refusal naming omp | yes | `src/cli/init.rs` |
| generalized sync report | yes | `src/cli/skills.rs` |
| `.omp` addon cleanup | yes | `src/cli/addons.rs` |
| `--agents` help names omp | yes | `src/main.rs` |
| `GivnWorld::omp_attribution`, `OmpAttributionState`, `omp_prepared_command`, `assert_prepared_env` | yes | `tests/steps/mod.rs`, `tests/steps/llm_attribution_e2e_steps.rs` |

**Strict-mode proof** — confirmed present and passing in tasks.md's setup task:
an undefined delta step fails the runner (`1 scenario (1 failed)`,
`Step doesn't match any function`, exit 1) under
`GivnWorld::cucumber().fail_on_skipped()` (`tests/features_runner.rs:55,151`).

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Consumer initializes a project and receives shared attribution plumbing | yes — CLI subprocess plus assertions on the generated files and the Git hook launcher | n/a (CLI) | no |
| omp attributes a commit using its current model | yes — the generated hook is executed by `node`, its prepared command is run via `sh -c`, and the resulting Git commit's committer and message are read back | n/a (CLI) | no |
| Consumer sees the commit attribution installation report | yes — the CLI's stdout is the asserted surface | n/a (CLI) | no |

The interface classification was tested in the opposite direction too: the
change renders no HTML/CSS/JS in a browser, produces no page, and exposes no
HTTP route, so it is not a Web UI and no browser driver applies; the real
interface is the CLI subprocess plus the generated artifact, which is what the
steps drive.

**E2e scope check** — one `@e2e` per distinct action, not per variant:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| initialize-project (delta) | 0 new | the omp target selection is a variant of the existing non-interactive selection action | no |
| attribute-commits (delta) | 1 new (omp attributes a commit), 1 modified (shared plumbing) | create a commit while attribution context is available | no |
| enable-commit-attribution (delta) | 1 modified (installation report) | initialize and read the installation report | no |

**Duplicate-implementation check** — the exact files `verify.e2e_command`
invokes and the tree search for a parallel implementation:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| llm-attribution (omp arm) | `tests/features_runner.rs` → `tests/steps/llm_attribution_e2e_steps.rs` and `tests/steps/llm_attribution_installation_e2e_steps.rs` (the runner registers every module in `tests/steps/mod.rs`) | no — `git status` shows only the change's own design.md pending, and the omp steps exist in exactly one module | n/a |

**Local environment check** — `design.md` declares no local stack: the product
is a CLI with no server, database, container, or third-party dependency, and
the E2E needs only `git` and `node`, both already required by the existing
OpenCode/Claude integration E2E. The declared local run command starts the
product cleanly:

```
$ ./target/debug/givn run --print
cargo run -- instructions
$ ./target/debug/givn instructions >/dev/null ; echo $?
0
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/commands.yaml`:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 11 (delta scope) | 3 (delta scope) | yes, 3 < 11 |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `initialize-project` rule "the installation report names what was installed, what was skipped, and why" holds; `fragment.md`'s `attribute-commits` behavior is extended, not altered | yes |
| Capability ownership and typed relationships preserved | `initialize-project` still includes `fragment: support`; no relationship changed; no new capability minted | yes |
| Confirmed Persona-relevant outcomes preserved | `agent--technical-role` gains the `.omp` guidance and hook; `developer--primary-user` gains the omp target and report line | yes |
| Actors remain distinct from Persona biographies | Gherkin actors stay `the consumer` / `the user` / `omp`; no Persona is an actor | yes |
| Migration ledger and coverage evidence complete, if applicable | no migration in this change | n/a |

No use-case scope rejection criterion holds: the goal is a single verb phrase,
the primary actor and trigger are unchanged, no capability received an
observing-goal mutation, no extension was added, no mechanism vocabulary
appeared in the goal, and no capability name is component- or channel-shaped.

```
PERSONAS: agent--technical-role, developer--primary-user
```

## Deferred mandatory checks

None. No mandatory check was deferred, so the DEFERRAL/OPERATOR-DECISION pair
is omitted. The one operator decision taken during the change (validating the
omp contract with a live session) was an added check, not a deferral, and its
result is recorded in tasks.md's live contract check.

**Design.md conformance check** — implementation diffed against what
`design.md` named:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Generated hook contract (D2) | `.omp/hooks/pre/givn-attribution.js`, ESM, no imports, `export default function givnAttribution(pi)`, `pi.on("tool_call", handler)`, returns `{ input: {...} }` or `undefined` | identical; live-validated against omp 18.3.0 | yes | n/a |
| Rewrite gate (D1) | only commands matching `\bcommit\b` | `REVISION_VERB = /\bcommit\b/` | yes | n/a |
| Command ownership | `owns_commands` for `opencode`, `omp`; `write_command` writes `<dir>/commands/` | identical | yes | n/a |
| Managed allowlist | `.omp/skills/**`, `.omp/commands/**`, `.omp/hooks/pre/givn-attribution.js` | identical | yes | n/a |
| Step-definition files | `tests/steps/init_agent_target_selection_steps.rs`, `tests/steps/llm_attribution_e2e_steps.rs`, `tests/steps/mod.rs`, `tests/steps/llm_attribution_installation_e2e_steps.rs` | the omp target-selection assertions reuse the existing generic steps in `tests/steps/mod.rs` instead; the other three files are used as named | yes, with one clarification | The clarification was recorded in `design.md`'s step-definitions section during review and re-confirmed by a fresh-context design-review check (`design-review.md`, "Re-confirmation" block) |
| E2E driver | `node` importing the generated `.js` with a `{"type":"module"}` fixture | identical | yes | n/a |
| Runner / strict mode | `./run-tests.sh`, `fail_on_skipped()` | identical | yes | n/a |

**Interaction coverage verification** — spec inventory cross-referenced:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| initialize-project — initialize with an explicit noninteractive target selection | yes | Consumer runs init non-interactively and gets opencode skills (permanent) | `common::run_givn` subprocess | `common::run_givn` subprocess | yes |
| fragments — no inventory (`src/gherkin/interaction.rs` skips `givn/specs/fragments/`) | yes, recorded as "no inventory" | omp attributes a commit using its current model | `node` drives the generated hook, `sh -c` runs the prepared command, `git show` reads the commit | identical | yes |

No inventory entry is unmapped, and no matrix row lacks a scenario.

## Visual Review

README-IMPACT: updated - agent target selection paragraph; commit attribution paragraph

N/A — `design.md` declares no visual interface (`## Visual Design Contract`:
"Not present … no rendered screen"). The change adds one line to CLI text and
created files, so no screenshot or transcript evidence is required and the
visual review gate stays inactive, consistent with `llm-attribution-installation`
and `keep-agent-prompt-open`.

## Retirements

N/A — this change retires no capability.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Agent target set gains `.omp` | `docs/arc42/03-context-and-scope.md` (Agent target directories row, new omp hook boundary row) | Yes | Production files + architecture impact diagram | Scenario 4 | `AgentTarget::Omp`, `.omp/skills`, `.omp/commands`, `.omp/hooks/pre/` | match |
| omp adapter in the strategy table | `docs/arc42/04-solution-strategy.md` | Yes | Technology decisions | Scenario 4 | `assets/agent-integrations/omp/givn-attribution.js` | match |
| `cli::skills` synchronizes commands + integrations | `docs/arc42/05-building-block-view.md` | Yes | Production files | Scenarios 4, 5 | `sync`, `write_command`, `sync_agent_integrations` | match |
| Commit-attribution ordering per harness | `docs/arc42/06-runtime-view.md` §6.1a, 6.12 | Yes | Runtime behaviour section | Scenarios 6, 7, 9 + live check | asset + live omp 18.3.0 run | match |
| Skill and rule generation paths + ignore prefixes | `docs/arc42/08-crosscutting-concepts.md` §8.7 | Yes | Production files, decisions | Scenarios 4, 5, 11 | `managed_generated_paths`, `ignore_entries`, `is_rule_path` | match |
| ADR-013 amendment (`AMEND_ADR`) | `docs/arc42/09-architecture-decisions.md` register + `adr/ADR-013-agent-integration.md` Consequences | Yes | Decisions/technology section, no new ADR | n/a | ADR-013 Consequences now names `.omp` and per-target commands | match |
| Target-selection quality scenario | `docs/arc42/10-quality-requirements.md` Q-5, new Q-96 | Yes | Test runner + E2E infrastructure | Scenarios 1, 2, 3, 8, 9 | delta scenarios | match |
| Revision-verb limitation and contract assumption | `docs/arc42/11-risks-and-technical-debt.md` R-77, R-78 | Yes | Decisions D1, Black-Box-First | Scenarios 6, 7; live check | R-78 now records the validated run | match |
| Glossary terms | `docs/arc42/12-glossary.md` Agent target / Harness / Slash command | Yes | — | n/a | the three rows name `.omp`/omp | match |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [x] Every Arc42 fact or decision relevant to this change is represented in
      `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; the
      evidence column names the built file, command, test, or observable
      result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment
      (R-78's validated run, the chapter-09 register entry).
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN.

---

## Coverage

No archive verification receipt exists yet for this change — the archive has
not run. Coverage is therefore classified from static inspection of the new
production regions plus the delta scenarios that exercise them; the archive
will carry the measured merged result in
`givn/archive/add-omp-agent-target/verification.json` and the README.

**Coverage result:** not yet measured — no receipt exists for this change.

```
No receipt: givn/archive/add-omp-agent-target/verification.json does not exist
until the archive runs. The archive receipt is the durable measured result;
this review classifies from inspection instead.
```

## Coverage gap classification

Every new production region and the scenario that exercises it:

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/skills/targets.rs` — `AgentTarget::Omp`, `owns_commands`, per-target command loop, `write_command` | new | covered | Scenarios 1, 2, 3, 4 |
| `src/skills/targets.rs` — omp branch of `sync_agent_integrations`, `.omp` entries in `managed_generated_paths`, per-target `reconcile_addon_artifacts` | new | covered | Scenarios 4, 11 |
| `src/skills/rules.rs` — `.omp` ignore entries and rule predicate | new | covered | Scenario 5 (skill, command, and hook paths) |
| `src/config/overlay.rs` — `KNOWN_AGENT_TARGETS` gains `omp` | new | covered | Scenarios 1, 3, 4 (an unknown name would drop the target) |
| `src/cli/init.rs` — installation-summary omp arm, refusal string | new | covered | Scenarios 4, 8, 10; the refusal string is asserted by the permanent "no agent target selected" scenarios, which now list `.omp` |
| `src/cli/init.rs` — `_ => {}` arm for a target without an integration | unchanged | covered by construction | reached by the `agents` target in existing scenarios |
| `src/cli/skills.rs` — omp commands-directory report line | new | **missing coverage → fixed** | Scenario 5 gained `stdout contains ".omp/commands/"` (`eef5958b`) |
| `src/cli/addons.rs` — `.omp` in the skill and command cleanup lists | new | **missing coverage → fixed** | Scenario 11 (`eef5958b`) |
| `src/main.rs` — `--agents` help text | changed doc comment | not executable code | clap renders it into `--help`; no behavior to assert |
| `assets/agent-integrations/omp/givn-attribution.js` — factory, gate, unavailable fallback | new | covered | Scenarios 6, 7, 9 plus the live omp 18.3.0 check |

No dead code was introduced (the one helper written before its first caller was
removed in Scenario 1's REFACTOR). No redundant unit test was added: the
`.feature` files are the sole source of scenario truth, and the two
`validate_frontmatter`-style unit tests in `src/skills/targets.rs` are
pre-existing.

## Scenario execution evidence

Review does not re-run the suite. The change has not been archived yet:

```
unproven candidate — no receipt yet. Delta-scope runs recorded in tasks.md:
  GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh      -> 11 tests run: 11 passed, 0 skipped
  GIVN_FEATURES=givn/changes/add-omp-agent-target/specs ./run-tests.sh --e2e -> 3 tests run: 3 passed, 8 skipped
```

**Receipt status:** not archived yet

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| givn skills targets add installs the new agent without touching anything else | The --agents flag accepts the dotted directory form for the omp target | boundary |
| Interactive init can select the .agents target | The --agents flag accepts the dotted directory form for the omp target | variant |

The remaining advisories reported for this delta are `[SUBST]` subset notes
between the new short scenarios and long unrelated permanent scenarios; no
subset note pairs two of the change's own scenarios in a way that would make
one redundant.

## Split-or-keep

No scenario in the delta exceeds the lint threshold, so the table is empty.

| Scenario | Decision |
|---|---|

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified and
      mapped, all promised components exist, no downgraded e2e scenarios, no
      e2e scenario driven via HTTP/fetch in place of the real interface, no
      over-produced e2e scope). The commit cross-check table discloses, per
      row, which commit carries the production artefact for the six guard and
      e2e scenarios whose implementation shipped with an enabling scenario.
- [x] Scenario execution evidence recorded per scope in tasks.md; the archive
      receipt will carry the merged result (review does not run the suite).
- [x] Every coverage gap classified and resolved: two regions were missing
      coverage and were fixed with real assertions in `eef5958b`; the rest are
      covered or are not executable code.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the product cleanly (no stack or digital twin
      exists to start).
- [x] The exact files `verify.e2e_command` invokes were identified and read,
      and the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting one.
- [x] `verify.e2e_command` is not identical to `verify.command`; the
      per-scope counts prove real isolation (3 of 11 at delta scope).
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; the single clarification (which step file carries
      the target-selection assertions) was recorded in design.md and
      re-confirmed by a fresh-context design-review check.
- [x] Interaction coverage verified: the only applicable inventory entry maps
      to a matrix row, an existing `@e2e` scenario, and step definitions using
      the promised driving mechanism; the fragments directory has no inventory.
- [x] `design.md`'s `## Internal Primitives` section lists every internal data
      structure and abstraction this change introduces, including the world
      slot and test helpers the step definitions rely on.
- [x] arc42 slug-mapping verified: every chapter slug in `arc42.md` maps to a
      real `docs/arc42/<slug>.md` file.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

**REVIEW: PASS**
