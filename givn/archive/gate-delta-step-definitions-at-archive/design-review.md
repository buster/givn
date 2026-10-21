# Design Review: gate-delta-step-definitions-at-archive

## Grilling log

A fresh-context grilling subagent read the proposal, the delta feature, the
use-case delta, `design.md`, `arc42.md`, the archive and integrity-hook
sources, the archive-proof test fixtures, and the motivating givn-harness step
file. It reproduced the extractor failures against real files (52 bindings vs
44 single-line, 209 multi-line Rust attributes, 15 truncated-phrase
collisions). Every finding below is resolved in the artifacts; the reproduced
evidence is the basis for the disposition.

| ID | Branch | Finding | Disposition |
|---|---|---|---|
| B1 | Tech choices / Testability | The line-based extractor missed 8 of 52 bindings in the motivating file and truncated raw-regex phrases at the first inner quote, so a delta definition could be declared permanent by a shared truncated prefix. | Replaced with a binding-expression scanner: supported tokens are found anywhere in the file; each expression runs from `(` to its matching `)` with a string/raw-string/regex/comment-aware counter; the first escaped string, raw string (any `#`), or `/regex/` literal is the phrase; adjacent Python literals are joined; whitespace is normalized. Recorded in `design.md` §Internal Primitives, §Technology Decisions, and arc42 §8.3, with unit expectations that the motivating file yields 52 phrases and the repo's `#[given(regex = r#"^a givn project …"#)]` yields the full regex text. |
| B2 | Contracts and components | Both documented arguments were on the delta side; the permanent root was never passed or defined, and deriving it from `current_dir()` or `features_path` misses `tests/steps`. | Signature is now `ensure_step_definitions_merged(project_root: &Path, change_root: &Path, phase: Phase)`. Delta tree = `change_root/specs`; permanent tree = `project_root` minus the prunes. Both call sites (fresh `change_dir`, pending/resume `archive_target`) pass the same `project_root`. Stated in `design.md` §Internal Primitives and §Architecture Impact. |
| B3 | Scope / Testability | "Named directory components" cannot express `givn/changes` or `givn/archive`; the delta copy would satisfy itself and the gate would be a no-op in the failing case. | Pruning is defined as component names plus project-relative prefixes. Permanent walk prunes `.git`, `.hg`, `.svn`, `target`, `node_modules`, `.venv`, `venv`, `coverage`, `dist`, `build`, `vendor`, `__pycache__`, and the prefixes `givn/changes`, `givn/archive`, `givn/history` (component-wise `Path::starts_with`). The delta walk prunes only component names. A unit-level case requires a phrase present only under `givn/changes/<id>/specs/steps/` or `givn/archive/<id>/specs/steps/` to fail the gate. Recorded in `design.md` and arc42 §8.3. |
| S1 | Tech choices | Cucumber-JVM (`@Given(`, `@And(`, `@But(`, `@Step(`), Cucumber-JS `And(`/`But(`, Behave `@step(`, and object-method bindings were silently skipped. | Full token set added to `design.md`, proposal D2, arc42 §8.3, and `assets/instructions/steps.md`: `#[given(`/`#[when(`/`#[then(`; `@given(`/`@when(`/`@then(`/`@step(`/`@Given(`/`@When(`/`@Then(`/`@And(`/`@But(`/`@Step(`; `Given(`/`When(`/`Then(`/`And(`/`But(`/`Step(`; `.Given(`/`.When(`/`.Then(`/`.Step(`. |
| S2 | Testability | The pending scenario's "no result" assertion passed even if the scope ran, because the `fail` runner exits before writing a result file. | Scenario now builds an unproven candidate with the `fail` runner, adds the candidate's `specs/steps/catalog_steps.py` phrase, then switches the existing `the runner records every invocation with its scope and features path` Given to `record` and commits. Assertions: non-zero exit, stderr names the phrase, candidate still unproven, no `givn/archive/<id>/results/regular.json` (`record` would write it). The vacuous "verification scopes have not been run again" Given and "no verification scope result" Then are removed. |
| S3 | Testability | The success fixture's permanent file had no pinned path, so it could be written wherever the implementation scans (circular proof). | The permanent fixture is pinned at `tests/steps/catalog_steps.py`, outside `givn/specs`; the feature Given and the design's Local Runnability and E2E sections say so. The usecase example notes both layouts are valid. |
| S4 | Testability | The bounded phrase list (`MAX_REPORTED_PHRASES = 20`, `+N more`) had no executable coverage. | Added a fourth `@givn.added @wip` scenario: 21 distinct missing phrases; stderr names the alphabetically first 20, does not name the 21st, says `+1 more`, and still names the delta file. Proposal, design, and usecase changed to "bounded with a remainder count". |
| S5 | arc42 | `docs/arc42/README.md` was missing from the planned documentation updates. | A paragraph is added naming the change, the canonical artifacts, the no-ADR disposition, and chapters 5, 6, 8, 11, 12; `design.md` and `arc42.md` list it. |
| S6 | Contracts and components / Testability | The design-instruction addition conflicted with `src/integrity_hook.rs`, which validates column 1 only and resolves it as one path. | The `## Step Definitions` table keeps column 1 as the hook-validated `Step definition file` and adds a third `Permanent merge target` column. `assets/templates/design.md` and `assets/instructions/design.md` are updated together. This change's table uses two files for the capability (non-e2e and e2e) with `n/a` in column 3 because both step files are repository tests, not a delta mirror. |
| Q1 | Risk | `__pycache__/*.pyc` under a `steps` directory is classified as a step file and aborts `read_to_string` with a UTF-8 error. | Both walks prune the `__pycache__` component and skip unreadable/non-UTF-8 files without failing. Recorded in `design.md` §Internal Primitives, arc42 §8.3, and R-122. |
| Q2 | Scope | The printed delta path had no defined base; the feature expects `specs/steps/catalog_steps.py`, the usecase said the same, but existing archive errors print absolute paths. | `delta_file` is project-relative (`givn/changes/<id>/specs/steps/catalog_steps.py` or the archive equivalent), which always contains the asserted `specs/steps/catalog_steps.py` suffix. Stated in `design.md`; feature asserts the substring. |
| Q3 | Scope | The release note promised "every unmerged step phrase" while the design caps the list at 20. | Release note now says "the missing step phrases (bounded with a remainder count)". |
| Q4 | Testability | The Mermaid routed pending/all-present through the fresh write-receipt-and-move box. | Diagram split: fresh/all-present → write receipt and move → merge and publish; pending/all-present → merge-if-needed and publish-if-needed; both reach verification. |
| Q5 | Use-case context | The resume comparison is numbered `- 5:` but precedes main-flow step 6. | Extension label changed to `- 6:` in the usecase delta; the traceability table matches. |
| Q6 | Ubiquitous language | "Step phrase" had no glossary entry; "delta step" was an anti-term while artifacts say delta step file/copy; the path rule drifted between `_steps` and `_steps.<ext>`. | Added **Step phrase** to the glossary, reconciled the **Step definition** entry to the delta-step-file wording, and stated one path rule everywhere: a file under a directory named `steps` or ending in `_steps`, or a file whose name ends in `_steps.<ext>`. |
| Q7 | Contracts and components | The "Archive command contract" row described the external CLI, not a component contract. | The contract table is deleted. `design.md` states: no new contract; the preflight is an internal helper inside `cli::archive`; the external CLI surface is unchanged. |
| Q8 | Testability | Two step files for one capability left shared-Given ownership unspecified, risking a duplicate same-kind definition. | Shared Givens live in `tests/steps/require_merged_step_definitions_steps.rs`; the e2e file defines only e2e-specific steps. Recorded in `design.md` §Step Definitions and §E2E. |
| Q9 | Missing scenarios | The feature proves the resume gate refuses but not that an all-present resumed candidate is verified normally. | No new scenario: the `record`-runner fixture in the pending scenario is the discriminating ordering proof (a scope run would write `results/regular.json`). A positive resume path is already covered by the permanent archive-proof resume scenarios. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Scope vs proposal | The four scenarios cover the fresh refusal, the fresh success, the bounded list, and the resume refusal; "the change stays active", "no candidate", "delta copy not moved", and "archived change keeps the step file" are asserted. Scope matches the proposal with no additions beyond the bounded-list refinement. |
| Run order | The pending path before the proposed site is `read_receipt` → clean witness → `ensure_e2e_configured` (`src/cli/archive.rs`); none runs a subprocess or writes a scope result. `run_scope` begins after the candidate publication, so placing the gate before `ensure_e2e_configured`/`apply_merge_plan` proves the ordering. |
| `the user invokes` step | Exists in `tests/steps/mod.rs:553`, delegates to `user_runs`, and spawns `CARGO_BIN_EXE_givn`; no new step is needed for the e2e scenario. The e2e Given `a givn project with a completed change … ready to archive` exists in `archive_proof_steps.rs:19`. |
| Archived change keeps the step file | `move_change_to_archive` is one `rename` of the whole change directory; `apply_merge_plan` writes only features and narrative documents. The fixture's new files are committed by `common::commit_fixture` inside `user_runs`. |
| Usecase merge | `sha256sum givn/specs/archive-change/usecase.md` = `767ff897…`, matching the delta's single `<!-- givn:base-sha256:767ff897… -->` marker. The delta adds flow, extension, rule, example, capability, and interaction text; nothing except the old trailing marker is removed. |
| ADR qualification | `givn instructions arc42-docs` verdict: `NOT_QUALIFIED`, `routing: CANONICAL_ARTIFACT`; product behaviour owned by the usecase, capability, `design.md`, and chapter 8.3. No ADR. |
| Self-archive | The change's own `specs/` contains no step-definition file, so its delta phrase set is empty and the gate short-circuits; the change's own archive is unaffected. |
| Permanent-corpus pins | No ADR, agent-target, or scenario-count pin is staled: no ADR is added, target lists are untouched, and instruction-integrity scenarios do not pin the edited `steps.md`/`archive.md`/`tasks.md` phrases. The arc42 index is the only stale artifact and is updated. |
| Generated skills | `assets/skills/givn-archive` and `givn-steps` carry no rule text ("served by `givn instructions …`"), so no skill regeneration follows from the instruction edits. |
| Decision ledger | D1 and D2 resolve in `design.md` §Decisions with realization sites; no carried questions. |
| Contracts | No new contract; one internal helper with one consumer. |
| Personas | Three confirmed Personas are declared and remain Gherkin-external; actor/Persona separation holds. |
| Visual contract | `Interface: none` with viewports and states; the opposite classification (Web UI) does not hold because the output is CLI text. |
| Interaction coverage | Four matrix rows cover the inventory entry and its blocking, bounded-list, and resume variants; every row names the CLI subprocess driver. |
| Strict mode | `.fail_on_skipped()` is present on both Cucumber builders; the `todo!` stub and both single-scenario commands are concrete and match the runner's `--exact` parsing. |
| Deferrals | None. |

## Architecture documentation (arc42) check

The reviewer re-derived the chapter set from proposal/spec/design before
opening `arc42.md`, then diffed.

| # | Chapter | Reviewer (Yes/No) | `arc42.md` (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | N/A |
| 2 | 02 architecture-constraints | No | No | Yes | N/A |
| 3 | 03 context-and-scope | No | No | Yes | N/A |
| 4 | 04 solution-strategy | No | No | Yes | N/A |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — the archive row names the preflight, project-root scope, and bounded refusal |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — sequence diagram and stage 3 place the gate before hooks and re-run it on resume |
| 7 | 07 deployment-view | No | No | Yes | N/A |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — §8.3 records the path rule, scanner, tokens, prune sets, unreadable files, bound, and repair |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — verdict routes to the canonical artifact; no MADR |
| 10 | 10 quality-requirements | No | No | Yes | N/A |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-122 matches the scanner and its remaining limits |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — Step definition and Step phrase match the artifacts |

- [x] All 12 chapter rows independently assessed before reading `arc42.md`.
- [x] No reviewer-Yes / claim-No or omitted row.
- [x] Every qualified ADR candidate has a MADR entry; none is proposed here.
- [x] No chapter claimed as updated is placeholder text.
- [x] No ASCII-art diagrams in any chapter file; the only diagrams are Mermaid.

## Changes made during hardening (2026-10-21)

| Artifact | Change summary |
|---|---|
| `specs/archive-change/require-merged-step-definitions.feature` | Phrases are now `an imported catalog` (no `Given ` inside the literal); success fixture pinned at `tests/steps/catalog_steps.py`; pending scenario rebuilt around the `record` runner and the missing `results/regular.json`; fourth bounded-list scenario added. |
| `specs/archive-change/usecase.md` | Main-flow step 1 names the project-root permanent tree; extension 1 names the project-relative delta file and bounded list; extension 5 → 6; rules and examples aligned; one base-sha256 marker, still `767ff897…`. |
| `proposal.md` | Release note bounded; solution describes the expression scanner and project-root permanent tree; D2 option 1 lists the full token set. |
| `design.md` | Overview/diagram, call-site contract, scanner primitives, two prune sets, unreadable-file rule, acceptance expectations, Step Definitions table with a permanent-merge-target column, no-contract statement, Local Runnability two-phase fixture, instruction/doc list. |
| `arc42.md` | Chapter-8, chapter-11, and chapter-12 summaries and affected-chapter evidence reflect the scanner, prune sets, and index paragraph. |
| `docs/arc42/README.md` | Change paragraph added (no ADR; chapters 5, 6, 8, 11, 12). |
| `docs/arc42/05`, `06`, `08`, `11`, `12` | Archive row, stage 3, §8.3, R-122, and the Step definition/Step phrase entries updated. |
| `assets/instructions/steps.md` | Durable-source rule, permanent-tree definition, bounded refusal, binding tokens, literal-phrase limitation. |
| `assets/instructions/archive.md` | Gate item 5 and the resume rule. |
| `assets/instructions/design.md` | Step-definition bullet requires the three-column table; durable-source section added. |
| `assets/instructions/tasks.md` | Closeout check for merged step definitions. |
| `assets/templates/design.md` | Three-column Step Definitions table and the column-1 rule. |

## Re-pass (2026-10-21)

After the edits above, every grilling item was re-checked against the artifacts
in a second pass rather than accepted from the edit:

- B1/S1: `design.md`, `proposal.md` D2, arc42 §8.3, and `steps.md` carry the
  same token set and expression-scanning rule; no artifact still says
  "first quoted literal on a line".
- B2/B3/Q1/Q2: `design.md` states the two-argument-plus-phase contract, both
  call sites, project-relative delta paths, the component and prefix prune
  sets, and the unreadable-file rule; arc42 §8.3 matches it.
- S2/S3/S4: the feature has four scenarios; the pending fixture is the
  two-phase `fail` → `record` construction and asserts the missing
  `results/regular.json`; the success fixture pins
  `tests/steps/catalog_steps.py`; the bounded scenario covers 21 phrases and
  `+1 more`.
- S5/S6/Q6/Q7/Q8: the README paragraph exists, the Step Definitions template
  and instruction carry the third column with column 1 hook-validated, the
  glossary has Step phrase and the one path rule, the contract table is
  deleted, and the shared-Given owner is named.
- Q3/Q4/Q5: release note bounded; the Mermaid separates fresh mutation from
  pending merge/publish; the usecase extension is `- 6:`.
- The usecase delta has exactly one `givn:base-sha256` marker; `sha256sum
  givn/specs/archive-change/usecase.md` still equals `767ff897…`.
- `givn lint --change gate-delta-step-definitions-at-archive` exits 2 with the
  four `@wip` findings (one per added scenario).

## Sign-off

- [x] Every grilling finding and question is resolved above.
- [x] `design.md` reflects every decision reached during grilling.
- [x] `specs/**/*.feature` updated where grilling found missing or vacuous scenarios.
- [x] No `@e2e` tag was removed.
- [x] `givn lint --change gate-delta-step-definitions-at-archive` exits 2 with exactly the four expected `@wip` findings (one per added scenario); the remaining `[SUBST] advisory` lines are pre-existing subset notices, not findings.
- [x] arc42 chapter check completed above; index and glossary updated.
- [x] No `tasks.md` or `review.md` was written.

**DESIGN-REVIEW: PASS**
