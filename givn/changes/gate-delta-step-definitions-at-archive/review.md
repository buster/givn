# Review: gate-delta-step-definitions-at-archive

## Fabrication audit

### Step-definition scan

Two step-definition files were added; both have substantive bodies and no
fabricated stubs.

| File | Step functions | Empty/trivial bodies | Assertions |
|---|---|---|---|
| `tests/steps/require_merged_step_definitions_steps.rs` | 12 | 0 | 17 `assert!`/`assert_eq!`/`expect` calls |
| `tests/steps/require_merged_step_definitions_e2e_steps.rs` | 1 | 0 | 2 (`assert!` plus the path `expect`) |

**0 empty step bodies found across 2 files.** Every Given writes or inspects the
fixture (delta step file, permanent step file, alternate candidate tree); every
Then asserts on captured stderr, the candidate receipt, the absence of a scope
result file, or the archived tree. The only bare `return;` in the non-e2e file
is the unreadable-directory guard inside the fixture's own walker, not a step
body. Both modules are registered in `tests/steps/mod.rs:69-70`.

### Commit cross-check

Every checked `[x]` scenario has one unique, reachable commit whose message
names the scenario. Scenario 1 carries the production gate; scenarios 2-4 carry
the regression tests for the same gate. The tasks.md sequencing note states
that these functions were written as `todo!()` stubs before scenario 1's GREEN
and filled with real assertions after it, so each later commit is a regression
test, which the repository commit contract allows ("the scenario's
implementation or its regression test"). This is not an invented exception.

| Scenario | Commit | Reachable / unique / names scenario | Production source in diff |
|---|---|---|---|
| Archive is blocked when a delta step definition is missing from the permanent test tree | `35632900` | Yes / Yes / Yes | Yes — `src/cli/archive.rs` (+589 lines, both call sites, scanner, unit tests), `tests/steps/**`, guidance, arc42 chapters |
| The refusal lists a bounded number of missing step phrases and a remainder count | `b27ff3cf` | Yes / Yes / Yes | No — regression test in `tests/steps/require_merged_step_definitions_steps.rs` (21-phrase fixture and bounded-list Thens) |
| An unproven archive candidate with a missing step definition is not verified again | `90a3b4bf` | Yes / Yes / Yes | No — regression test in the same file; the resume call site shipped with the shared gate in `35632900` (design's two call sites landed together) |
| Consumer archives a change whose delta step definitions are already in the permanent test tree | `da81ffa5` | Yes / Yes / Yes | No — `@e2e` regression test in the two step files |

No COMMIT line is a placeholder, reused, unreachable, or message-mismatched.
Tasks 0f6cd0ff and 86125778 update the evidence record only; b2c8a0fe updates
the README; neither is presented as a scenario commit.

### Promised components and files

| Promised by design.md / tasks.md | Exists | Evidence |
|---|---|---|
| `ensure_step_definitions_merged(project_root, change_root, phase)` | Yes | `src/cli/archive.rs:751`; fresh call at `:135`, resume call at `:235` |
| `is_step_definition_path` | Yes | `src/cli/archive.rs:454` |
| `step_binding_expressions` | Yes | `src/cli/archive.rs:473` |
| `binding_phrase` | Yes | `src/cli/archive.rs:658` |
| `collect_step_phrases` | Yes | `src/cli/archive.rs:694` |
| `collect_step_phrases_in_tree` | Yes, under the private name `collect_phrases_in_tree` | `src/cli/archive.rs:738` — same signature and pruning behavior |
| `PruneRules` with component set and project-relative prefixes | Yes | `src/cli/archive.rs:391-422` (`STEP_COMPONENT_PRUNES`, `PERMANENT_PREFIX_PRUNES`, `DELTA_PRUNE`, `PERMANENT_PRUNE`) |
| `StepDefinitionGap` | Realized inline as the `BTreeMap<PathBuf, Vec<String>>` gap map | inside `ensure_step_definitions_merged`; behavior (delta file + missing phrases) is unchanged |
| `MAX_REPORTED_PHRASES = 20` | Yes | `src/cli/archive.rs:425` |
| `tests/steps/require_merged_step_definitions_steps.rs` | Yes, non-empty | 246 lines |
| `tests/steps/require_merged_step_definitions_e2e_steps.rs` | Yes, non-empty | 21 lines |
| `assets/instructions/{steps,archive,design,tasks}.md`, `assets/templates/design.md` | Yes, edited | commit `35632900` |
| `docs/arc42/{README,05,06,08,11,12}.md` | Yes, edited with decision-specific content | commit `35632900` |

The two private-name differences are internal identifiers, not component,
contract, file-layout, command, or driver omissions; the design itself records
that this change adds no module, crate, contract, or external surface.

### Strict-mode proof

`tests/features_runner.rs` carries `.fail_on_skipped()` on both Cucumber
builders (lines 55 and 151), and the runner exits non-zero when
`stats.failed > 0 || stats.skipped > 0`. tasks.md records the strictness probe
and every scenario's targeted RED exit:
`not yet implemented: TODO: ...` at lines 139, 232, 277, and 324. The four
delta scenarios are green without `@wip` (`givn lint` clean).

### @e2e tag integrity

The delta feature has never lost an `@e2e` tag. Its history is:

| Revision | Tag change |
|---|---|
| `836f372d` (planning) | Adds four scenarios; the success scenario is `@givn.added @e2e @wip` |
| `35632900` | Removes `@wip` from scenario 1 only |
| `b27ff3cf` | Removes `@wip` from scenario 2 only |
| `90a3b4bf` | Removes `@wip` from scenario 3 only |
| `da81ffa5` | Removes `@wip` from the `@e2e` scenario only; **`@e2e` preserved** |

No `@wip` remains (`givn lint --change ...` exits 0).

### Downgraded-e2e scan

| `@e2e` Scenario | Primary Then assertion | Driven interface | Downgraded? |
|---|---|---|---|
| Consumer archives a change whose delta step definitions are already in the permanent test tree | The real subprocess exits 0 (`the command exits 0`); the change moved to `givn/archive/`; the archived change still contains `specs/steps/catalog_steps.py` | Real compiled `givn` binary via `tests/common::run_givn_with_envs`, reached through the generic `the user invokes` step (`tests/steps/mod.rs:555-561`); no HTTP/fetch | No |

The exit status is an assertion on the real interface, not a repository-only
assertion; the tree assertions are secondary evidence. The capability renders
nothing, so no browser driver applies.

### Exact `verify.e2e_command` target and parallel-implementation search

`givn/commands.yaml` binds `verify.e2e_command = "./run-tests.sh --e2e"`.
`run-tests.sh` runs `cargo ... --test features_runner` with the tag expression
`@e2e and not @givn.removed`. The exact invoked file is therefore
`tests/features_runner.rs`, with the bound step definitions in
`tests/steps/require_merged_step_definitions_e2e_steps.rs` (e2e-specific) and
`tests/steps/require_merged_step_definitions_steps.rs` (shared Givens/Then).

The tree, including untracked files, contains no second/parallel e2e
implementation for this capability: the only implementation of the
archived-step-file step is the e2e file above. The only untracked files are the
two verification receipts under the change's `verification/` directory.

### E2E scope check

One normalized inventory action exists for this capability; the other three
scenarios are variants (fresh refusal, bounded refusal, resume refusal) and are
regular scenarios.

| Capability | Inventory entries | @e2e scenarios | Over-produced? |
|---|---|---|---|
| require-merged-step-definitions | 1 — archive a change whose delta step definitions are already in the permanent test tree | 1 | No |

The delta e2e scope is 1 scenario; the delta non-e2e scope is 3 scenarios. The
archive will run both scopes against the merged corpus.

### `verify.command` versus `verify.e2e_command`

| `verify.command` | `verify.e2e_command` | Identical? | Change-scoped receipt | E2E scope count | Strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e` | No | regular scope: total 3, passed 3, failed 0, skipped 0 (receipt at `86125778`) | 1 (tasks.md final delta run: 1 passed, 3 skipped; full project: 111 vs 686) | Yes — 1 < 3 and 111 < 686 |

The change-scoped verification receipt records only the regular scope because
`verify.command` is `--non-e2e`; the e2e scope count comes from the delta run
recorded in tasks.md, and the archive receipt will carry both measured scopes.
The tag filter is real: the e2e run skips the 3 regular scenarios and the
regular run skips the 1 e2e scenario.

### Implementation versus design

| Decision named by design.md | As built | Match? |
|---|---|---|
| `verify.command` = `./run-tests.sh --non-e2e`; `verify.e2e_command` = `./run-tests.sh --e2e` | Literal values in `givn/commands.yaml` | Yes |
| Fresh call site after `ensure_artifact_gates`, before `run_archive_gate_hooks` | `src/cli/archive.rs:135` exactly there; before receipt write and rename | Yes |
| Resume call site before `ensure_e2e_configured` and `apply_merge_plan` | `src/cli/archive.rs:235`, after clean-state witness and before both | Yes |
| Delta tree = `change_root/specs`; permanent tree = project root minus component names and `givn/changes`, `givn/archive`, `givn/history` prefixes | `DELTA_PRUNE`/`PERMANENT_PRUNE` and `PruneRules::prunes_dir` | Yes |
| Step-definition path rule (`steps`/`_steps` directories, `*_steps.<ext>` files) | `is_step_definition_path`; unit test covers the rule | Yes |
| Separate non-e2e and e2e step-definition files | Two files, registered separately; only one same-kind definition exists per step | Yes |
| Framework/driver: cucumber-rs runner, real CLI subprocess | `tests/features_runner.rs`, `CARGO_BIN_EXE_givn` via `the user invokes` | Yes |
| Bounded list: 20 phrases then `+N more`, project-relative delta file | `MAX_REPORTED_PHRASES`, BTreeMap ordering, `strip_prefix(project_root)` | Yes |

No command, path, branch-placement, framework, or driver deviation exists.

### Interaction coverage

Owning use case: `archive-change`. The inventory lives in the use-case
document (`## Interactions`) and gains one row in the delta. The design matrix
has four rows: the inventory action plus three variants.

| Inventory entry (delta usecase.md) | Design matrix row | `@e2e` scenario | Promised mechanism | Actual step definitions | Match? |
|---|---|---|---|---|---|
| require-merged-step-definitions: archive a change whose delta step definitions are already in the permanent test tree | Present | Consumer archives a change whose delta step definitions are already in the permanent test tree | cucumber-rs spawns `CARGO_BIN_EXE_givn archive --change merged-steps`, reads exit code/stdout/stderr and the archived tree | `the user invokes` → real subprocess; `the command exits 0`; `the change directory has been moved...`; `the archived change still contains...` | Yes |
| Same action — fresh-refusal variant | Present as a blocking variant (not an inventory entry) | `@e2e` action; refusal is its failure path | Same subprocess driver | Blocked scenario's Thens assert exit, stderr, active change, no candidate | Yes |
| Same action — bounded-list variant | Present as a refusal variant (not an inventory entry) | `@e2e` action; bounded refusal is a failure variant | Same subprocess driver, 21 missing phrases | Bounded-list Thens assert the first 20, the omitted 21st, `+1 more`, delta path | Yes |
| Same action — resume variant | Present as a resume variant (not an inventory entry) | `@e2e` action; resume refusal is a failure variant | Same subprocess driver against an unproven candidate | Pending scenario asserts non-zero, named phrase/file, pending receipt, no `results/regular.json` | Yes |

Every inventory entry has a matrix row, has exactly one `@e2e` scenario, and
the step definitions use the promised driver. No unmapped interaction exists;
no additional `@e2e` scenario is produced for a variant.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 05 building-block-view: `cli::archive` compares delta step phrases with the permanent tree and refuses before mutation (fresh and resume) | `docs/arc42/05-building-block-view.md:63` | Affected; archive row gains the preflight | §Architecture Impact and §Internal Primitives | Scenario 1 files list `src/cli/archive.rs` and docs | `ensure_step_definitions_merged` + both call sites; archive row text matches | Yes |
| 06 runtime-view: stage 3 lists the comparison before hooks; resume repeats it before verification | `docs/arc42/06-runtime-view.md:311,319,348-359` | Affected; sequence diagram and stage 3 | §Overview diagram and call sites | Scenario 1 and scenario 3 | Fresh call before `run_archive_gate_hooks`; resume call before `ensure_e2e_configured`/`apply_merge_plan` | Yes |
| 08 crosscutting-concepts §8.3: durable-source rule, one path rule, binding scanner/tokens, two prune sets, unreadable-file skip, bounded refusal and repair | `docs/arc42/08-crosscutting-concepts.md:66-101` | Affected; §8.3 records all of it | §Technology Decisions, §Internal Primitives, §Pruning | Scenarios 1-3 and the scanner unit test | §8.3 text matches the implemented tokens, path rule, prunes, cap, and repair | Yes |
| 11 risks-and-technical-debt: R-122 records dynamic-binding skip, literal comparison, unreadable-file skip | `docs/arc42/11-risks-and-technical-debt.md:109` | Affected; R-122 updated | §Technology Decisions and R-122 reference | Scenario 1 docs list | R-122 entries match the extractor's documented limits | Yes |
| 12 glossary: **Step definition** durable source and path rule; **Step phrase** comparison unit | `docs/arc42/12-glossary.md:72-73` | Affected; both entries | §Decisions D2 and ubiquitous terms | Scenario 1 docs list | Glossary text matches the feature, design, and code vocabulary | Yes |
| 09 architecture-decisions: no ADR for this change | `docs/arc42/09-architecture-decisions.md` (no new MADR) | Not affected; verdict `NOT_QUALIFIED` → `CANONICAL_ARTIFACT` | §Decisions D1/D2 | No ADR task | No `docs/arc42/adr/` entry added; usecase + Gherkin + §8.3 own the behavior | Yes |
| Arc42 index records the change, affected chapters, and no-ADR disposition | `docs/arc42/README.md:130-136` | Index updated | §Instruction and Documentation Updates | Scenario 1 docs list | Index paragraph names chapters 5, 6, 8, 11, 12 and the canonical artifact | Yes |

- [x] Every chapter marked affected by the reviewer and by `arc42.md` was opened and checked for current decision-specific content.
- [x] Every relevant Arc42 fact is represented in `design.md` and mapped to a concrete task.
- [x] Implementation follows those facts; the evidence names the files and lines.
- [x] No architecture change was discovered that the artifacts fail to record.
- [x] No mismatch, stale chapter, or placeholder remains.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The durable glossary (`docs/arc42/12-glossary.md:72-73`) defines **Step
definition** and **Step phrase**. The spec, proposal, usecase delta, design,
tasks, arc42 chapters, guidance, and implementation reuse those exact terms:

| Term | Glossary entry | Reused consistently | Inconsistent usage found |
|---|---|---|---|
| Step definition | Durable project source; one path rule; archive never merges/copies/deletes it | Feature ("delta step definition"), usecase rules, design, `assets/instructions/steps.md`, code (`is_step_definition_path`, `ensure_step_definitions_merged`) | None — the anti-term "step glue" and bare "delta step" do not appear; compounds say delta step file/copy |
| Step phrase | The literal bound by a binding, whitespace-normalized; the comparison unit | Feature ("missing step phrases"), usecase rules, design (`binding_phrase`, `collect_step_phrases`), arc42 §8.3, R-122 | None — "step body" (anti-term) is not used for the compared unit |

Path-rule wording is identical everywhere: a file under a directory named
`steps` or ending in `_steps`, or a file whose name ends in `_steps.<ext>`.

## Coverage

**Measured coverage for this change:** none exists yet — the change is not
archived, so no archive receipt or merged coverage report carries it. This
review classifies from static inspection; the archive receipt will carry the
measured result.

**Changed executable surface:** `src/cli/archive.rs` adds the gate
(`is_step_definition_path`, `step_binding_expressions`, `binding_phrase`,
`collect_step_phrases`, `collect_phrases_in_tree`, `PruneRules`,
`ensure_step_definitions_merged`, `MAX_REPORTED_PHRASES`) and two call sites.
All other changes are tests, embedded guidance, templates, README, and durable
docs.

## Coverage gap classification

| Location | Classification | Action taken |
|---|---|---|
| Fresh refusal, bounded list, resume refusal, success/archived-copy path | Covered by the four delta scenarios through the real CLI and by the fixture step definitions | None — no gap |
| Scanner edge cases: multi-line/adjacent literals, raw-string and regex literals, path rule, archive/delta self-satisfaction, 21-phrase remainder, pruned `__pycache__` | Covered by unit tests in `src/cli/archive.rs` (the scanner and pruning primitives are not reachable as separate black-box actions; the scenarios assert their outcomes at the archive boundary) | None — no gap |
| Defensive skips (unreadable/non-UTF-8 step file; binding with no resolvable literal) | Legitimately hard to test as a distinct observable: skipping produces the same behavior as a file that binds no phrase, and dynamic bindings are the documented R-122 limitation rather than a promised phrase set | Documented as R-122; no scenario is claimed for an unobservable difference |
| Dead code | None — every added helper is called by a call site or a unit test | None |

No uncovered production line was identified by inspection, no dead code was
added, and no redundant unit test duplicates a scenario at the black-box
boundary. No classification outside the three buckets is used.

## E2E coverage and interface classification

**Interface type: CLI.** The consumer invokes the compiled `givn` binary as a
subprocess. The `@e2e` scenario's primary assertion is the real subprocess exit
status, and it additionally observes the moved archive directory and the
retained delta step file. The scenario uses the real binary; the archive's
configured verification runner is a fixture runner, which is the system under
test's dependency, not a substitute for the interface.

**Opposite classification check:** a capability that renders HTML/CSS/JS in a
browser is a Web UI and requires a real browser driver. This capability renders
no page, serves no route, and exposes no DOM; its interface is CLI text and an
exit code. The opposite classification (Web UI) therefore does not hold, and an
HTTP client or browser driver would not be the real interface.

**Real-interface statement:** the scenario drives the real interface; no
HTTP/fetch shortcut exists.

## Deferrals

None. No mandatory check is deferred, and no operator decision is required.

## Use-case and Persona conformance

| Contract element | Evidence | Pass? |
|---|---|---|
| Goal and success guarantee preserved | The use case still archives a completed change with proven publication; the gate runs before mutation and leaves a failed archive repairable | Yes |
| Rules and typed relationships preserved | Main flow gains step 1's comparison; extensions 1 and 6 name the refusals; `Includes: fragment: support` and `Extends: none` are unchanged; the capability is added to `## Capabilities` and to the inventory | Yes |
| Capability ownership and placement | `require-merged-step-definitions` is `NEW in archive-change` per the proposal routing table, goal-shaped (archive the change), and advances "archive with proven publication" — not a component/channel name | Yes |
| Extension references its main-flow step | Extensions are labelled `1:` and `6:` against main-flow steps 1 and 6 | Yes |
| Actors remain interaction roles; Personas remain review context | The actor is Developer; the three Personas are declared here and in design, never embedded in Gherkin | Yes |
| Confirmed Persona-relevant outcomes | Developer/agent read the pre-mutation refusal and named repair; the maintainer's permanent corpus cannot be published with undefined steps | Yes |
| Migration ledger | N/A — no migration | Yes |

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

Confirmed Personas: developer--primary-user, agent--technical-role,
givn-maintainer--owner. All three are declared on the owning use case and were
confirmed by the operator for this change.

## Visual review

`design.md` declares `Interface: none`: no rendered screen, no viewports, no
states; the consumer reads CLI text. `Interface: none` requires a confirmed
Persona disposition (recorded above) but no visual verdict, no screenshots, and
no transcript evidence. The opposite classification does not hold: nothing
renders a page. No `VISUAL-REVIEW` verdict is required.

## Scenario execution evidence

The change-scoped verification receipt
(`verification/tasks-verification.json`, schema 2, status `passed`) records the
regular scope at `86125778`: command `./run-tests.sh --non-e2e`, total 3,
passed 3, failed 0, skipped 0, runner exit 0. The e2e delta run recorded in
tasks.md reports 1 passed, 3 skipped; the full-project runs report 686 regular
and 111 e2e passed. Review does not re-run the suite. The archive receipt will
carry both measured scopes against the candidate.

## Outline review

The gate reports base `e41e014a` (parent of `836f372d`), 6 touched units, 2
outlines, and 1 metric hint. Both outlines are readable for their
responsibility. The hint (the module grew from 1201 to 1558 code lines; `run`
is 253 code lines with complexity 69) is advisory, not the gate; the printed
call order follows the archive contract: gates precede mutation, the
step-definition comparison precedes hooks and resume scopes, the candidate is
published before verification, and proof precedes finalization.

OUTLINE: AUTHOR deepseek/deepseek-flash
OUTLINE: REVIEWER omp-fresh-context-reviewer
OUTLINE: PASS module:src/cli/archive.rs | Owns the archive lifecycle: validate provenance and clean state, run the artifact gates, compare delta step phrases with the permanent tree, run hooks, run declaration, README and merge preconditions and e2e configuration, move, merge and publish the candidate, then run each scope once and finalize only on matched evidence; the call order is the contract because every gate precedes the mutation it protects and the candidate is published before it is verified.
OUTLINE: PASS type:src/cli/archive.rs::PruneRules | Carries the two prune sets for the step-definition walks; prunes_dir checks component names first and then the permanent-walk project-relative prefixes, so the delta's own step files are read while the change, archive, and history trees cannot satisfy their own phrases.
OUTLINE: PASS

## Overlap dispositions

None — `givn check review` reports no shape match involving this delta's
scenarios (net delta: 4 added, 0 modified, 0 removed). No finding pair exists,
so no disposition row is owed. The three refusal scenarios are regular
variants of the `@e2e` action, but the gate did not report a shape match
between them.

## Split-or-keep

None — the overlap gate reports no long-scenario finding for this delta; no
scenario exceeds the threshold (longest: 10 steps).

## README impact

README.md names the pre-mutation step-definition gate in the "What it is"
section's workflow summary (commit `b2c8a0fe`).

README-IMPACT: updated - What it is

## Sign-off

- [x] Fabrication audit: clean — 0 empty step bodies across 2 files, all four checked scenarios have unique reachable commits carrying the implementation or its regression test, every promised component/file exists, strict-mode proof is present, no `@e2e` tag was removed, and no downgraded `@e2e` scenario exists.
- [x] Every checked scenario has a verified, reachable, unique commit; scenario 1 carries the shared production gate and scenarios 2-4 carry its regression tests.
- [x] Every promised component and file exists.
- [x] Every consumer uses only the declared contract (no new contract; one internal helper with one command caller).
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence is recorded per scope in the change verification receipt and tasks.md; review does not re-run the suite.
- [x] Coverage classification uses the most recent available evidence (none measured yet; inspection-based) and the archive receipt will carry the merged result.
- [x] Every coverage gap is classified and resolved; dead code deleted; missing tests added; no hard-to-test gap is claimed without justification.
- [x] No redundant unit test duplicates a black-box scenario.
- [x] No `@wip` tags remain; the delta spec contains no implementation-layer detail (only observable CLI behavior, exit codes, stderr text, and paths).
- [x] Canonical E2E policy read from `givn instructions specs`: one normalized inventory action has one `@e2e` scenario and variants are regular scenarios.
- [x] Every E2E scenario has a real-interface primary assertion and uses the reviewed driver; no browser UI exists and no HTTP/fetch substitute is used.
- [x] The local run command is the repository's canonical build/test path; there is no stack or digital twin to start, and the fixtures use the local git/fossil executables.
- [x] The exact file `verify.e2e_command` invokes (`tests/features_runner.rs`) was identified and read; the tree (including untracked files) was searched for a parallel e2e implementation and none exists.
- [x] `verify.e2e_command` is not identical to `verify.command`; the counts prove the e2e scope is strictly smaller (1 vs 3 in the delta; 111 vs 686 project-wide).
- [x] Implementation (commands, file layout, framework/driver, call-site placement) matches what design.md named; no silent shortcut exists.
- [x] Interaction coverage verified: every use-case inventory entry maps to a design matrix row, an existing `@e2e` scenario, and step definitions using the promised CLI subprocess driver.
- [x] No finding is excused with a classification outside the three buckets.

REVIEW: PASS
