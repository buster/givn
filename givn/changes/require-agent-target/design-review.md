# Design Review: require-agent-target

## Grilling log

Fresh-context grilling subagent (`PlanGrill`, task agent, read-only) read the
proposal, both delta spec files, design, arc42 assessment, the permanent
contracts, the instructions for specs/design/design-review/arc42-docs, and the
implementation and test fixtures. It walked all fifteen required branches and
returned six findings; every one is resolved below. No question needed operator
input: each was settled by codebase exploration, by the canonical instructions,
or by the hardening edits recorded in this file.

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Five delta files the design declares are missing; six permanent scenarios would fail after the change | Closed — five delta features authored |
| 2 | Testability / Deferrals | The interactive (PTY) refusal path had no scenario and the substitution was unrecorded | Closed — real PTY scenario added via `script -qec` |
| 3 | Testability | Two delta scenarios cannot fail RED (already GREEN before the change) | Closed — kept as regression guards, recorded in tasks.md |
| 4 | Missing scenarios | The error's target/flag naming and the empty `--agents` value were unasserted | Closed — assertions extended; same-path reasoning recorded |
| 5 | Fixture fallout | The deleted step's rationale was wrong, and deletion timing matters | Closed — design corrected; deletion ships with the replacing delta |
| 6 | ADR qualification | The arc42 verdict block deviated from the canonical schema | Closed — verdict normalized with evidence arrays |

### Q1 — Six permanent scenarios break when init requires a selection

- Status: closed 2026-10-03 (orchestrator, from the grill's evidence)
- Blocks: archive verification of the merged permanent corpus
- Context: the design promised delta files for `enable-optional-addons`,
  `declare-the-product-start`, `rules-policy`, `measure-coverage-per-project`,
  and `readme-enforcement`, but only the initialize-project delta existed. Six
  permanent scenarios invoke `givn init` with null stdin and no `--agents`, so
  under the new precondition they refuse and fail. The change-scoped runner
  would not notice, but archive verification runs the merged corpus
  (`GIVN_FEATURES=givn/specs`).
- Question: are those five capability deltas in scope, or is the design's file
  list wrong?
- Recommended: they are in scope — an initialization precondition change must
  migrate every scenario that initializes.
- Options:
  1. **Author the five deltas** — each affected permanent scenario becomes a
     `@givn.modified` delta with the same title and assertions, and the `When`
     line naming the selection.
     In practice: `When the user runs \`givn init --agents opencode\`` replaces
     the bare invocation in the rules-policy scenario.
     Gain: the merged corpus stays green and every initialization states its
     selection. Cost: five small delta files and their `@wip` bookkeeping.
  2. **Drop `--agents` from the design's file list** — not viable: the
     scenarios do initialize, so they would fail at archive.
- Resolution: **option 1** because archive runs the merged permanent corpus and
  a precondition change owns its call sites. Rejected: option 2 because it
  leaves six failing permanent scenarios.

### Q2 — The interactive refusal has no exercised scenario

- Status: closed 2026-10-03 (orchestrator, guided by the interface-obstacle rule)
- Blocks: the claim that the reported dialog behaviour is fixed
- Context: `prompt_for_targets` has two branches: a `dialoguer::MultiSelect`
  branch when stdin is a terminal (`src/cli/init.rs:175-196`) and a piped-line
  branch otherwise (`:198-205`). The cucumber harness only reached the piped
  branch, so the unmarked prompt and the empty-confirmation refusal — the
  report's exact case — were unproven.
- Question: is the piped path sufficient evidence, or must the terminal branch
  be driven directly?
- Recommended: drive the terminal branch through a real PTY; the instructions
  treat an interface obstacle as a design gap to fix, not to defer.
- Options:
  1. **PTY scenario** — spawn `script -qec "<givn> init" <transcript>` with a
     piped newline, assert non-zero exit and the refusal text.
     In practice: the child sees a terminal, `dialoguer` draws the list, the
     newline confirms nothing, `givn` exits 1 with
     `no agent target selected`. Gain: the reported case is proven end to end
     and a future pre-marked default is caught. Cost: `script` (util-linux)
     becomes a suite prerequisite; the step needs a hang deadline.
  2. **Operator-recorded deferral** — accept the piped path as the evidence.
     Gain: no new dependency. Cost: the dialog path stays unproven; the
     substitution is an inference, not a test.
  3. **Pure predicate unit test** — extract selection→targets and unit-test the
     empty case. Gain: cheap. Cost: does not exercise `dialoguer` and adds a
     parallel test seam the executable-spec invariant discourages.
- Resolution: **option 1** (prototyped against the current binary: five runs,
  deterministic transcript, prompt rendered, exit status propagated).
  Rejected: option 2 because the fix's headline surface would stay unproven;
  option 3 because it proves a helper, not the interface.

### Q3 — Two delta scenarios pass before the change

- Status: closed 2026-10-03 (orchestrator, from the grill's evidence)
- Blocks: the RED claim in tasks.md
- Context: `givn init --force re-applies the explicitly given selection` and
  the modified e2e scenario pass against the current code, because the current
  code already honours a given flag.
- Question: should they be dropped, or kept as regression guards?
- Recommended: keep both; they pin the paths the change must not break.
- Options:
  1. **Keep and label** — record in tasks.md that they are GREEN-before
     regression guards, not RED evidence.
     In practice: the task entry says "no RED step — already green; re-run after
     the code change". Gain: the `--force` and explicit-flag contracts stay
     pinned. Cost: two scenarios without a discriminating RED phase.
  2. **Drop them** — removes the guard on `--force --agents` and on the
     non-interactive explicit-selection e2e path. Not viable.
- Resolution: **option 1**; the discriminating RED evidence stays with the five
  refusal scenarios and the `--force` no-selection scenario.

### Q4 — The refusal's full text and the empty `--agents` value

- Status: closed 2026-10-03 (orchestrator, from the grill's evidence)
- Blocks: the error-contract pinning
- Context: the proposal requires the error to name the valid targets and the
  `--agents` escape hatch; the delta asserted only the stable fragment (plus
  `.agents` in one scenario). An explicit empty value (`--agents ""`) is a real
  path (clap yields one empty string).
- Question: assert the full contract, and add a dedicated scenario for the
  empty flag value?
- Recommended: pin the names and the flag in one refusal scenario; no separate
  scenario for `--agents ""`, because it resolves through the same
  `resolve_target_names` → refusal path.
- Options:
  1. **Extend assertions, no new scenario** — the no-target scenario asserts
     `.opencode`, `.claude`, and `--agents`; the empty-flag path is covered by
     the same resolution code as the covered `--agents nope` case.
     Gain: the contract is pinned without a near-duplicate scenario. Cost: the
     empty-flag *input* itself has no scenario (same path, different literal).
  2. **Add an `--agents ""` scenario** — adds another shape-duplicate refusal
     variant for a literal that reaches the same code.
- Resolution: **option 1**; the added assertions are in the delta and the
  reasoning is recorded in design.md's coverage boundaries.

### Q5 — Deleted step: wrong rationale and timing

- Status: closed 2026-10-03 (orchestrator, from the grill's evidence)
- Blocks: the correctness of the test-harness edit
- Context: design.md justified deleting `consumer_init_silent` with a clippy
  dead-code argument, but the repo has no clippy gate in the suite and the
  registration macro references the function. The real constraint is ordering:
  the permanent spec still references the step text until the delta merges.
- Question: when and why delete the step?
- Recommended: delete it in the same change that replaces its scenario, and say
  so without the clippy claim.
- Options:
  1. **Keep the deletion, fix the text** — the delta replaces the only
     scenario; change-scoped runs load the delta, archive runs the merged
     corpus. Gain: no dead step after merge. Cost: none beyond the corrected
     rationale.
  2. **Keep the step until after archive** — an unused step registration with
     no scenario; harmless but stale.
- Resolution: **option 1**; design.md now states the merge-order constraint and
  drops the clippy rationale.

### Q6 — arc42 verdict schema

- Status: closed 2026-10-03 (orchestrator, from the canonical instruction)
- Blocks: the ADR-qualification conformance check
- Context: the arc42-docs instruction requires the structured verdict with
  `qualification: QUALIFIED` and an `evidence` object per dimension; the first
  draft wrote `QUALIFIED (refinement)` and omitted the evidence arrays.
- Recommended: normalize the verdict; the AMEND_ADR route itself is correct
  (ADR-013 owns the selection contract; a refinement amends, never re-mints).
- Resolution: **normalized** in `arc42.md` with per-dimension evidence; the
  route, target, and register records are unchanged.

## Resolved by codebase exploration

Items answered without user input by exploring the existing codebase.

| Branch | Finding |
|---|---|
| Scope | The proposal's done-criteria map one-to-one onto delta scenarios after the five missing capability deltas are added; nothing extra is specified. |
| Tech choices | `anyhow` error → `error: {e:#}` + exit 1 is the established refusal path (`src/main.rs`); `DEFAULT_TARGETS` is referenced only inside `src/cli/init.rs`, so deleting it cannot affect `givn skills sync`, whose `[opencode]` fallback lives in `src/config/overlay.rs`. |
| Missing scenarios | EOF, `--force` with and without a selection, mixed unknown+known answers, and `--agents all` are covered by delta or unchanged permanent scenarios; the naming contract is pinned in the delta; the empty `--agents` value shares the covered resolution path. |
| Testability | Every added/modified scenario binds to existing step definitions; the five refusal scenarios and the `--force` no-selection scenario fail against current code (RED provable); the PTY scenario fails against current code because the pre-marked `.opencode` is installed instead of refused. |
| E2E fidelity | One `@e2e` scenario per inventory entry; the replacement reuses the exact permanent title with `@givn.modified @e2e`; removed scenarios use the single placeholder step; lint exit 2 is permitted. |
| Visual contract | No rendered screen; `parse_visual_contract` yields `Interface::None` for the design's "Not present" section, so the gate stays inactive; the opposite classification (web) is excluded because the surface is CLI stderr text only. |
| Persona disposition | `developer--primary-user` is a confirmed persona (`givn/personas/`); Gherkin actors stay `the user`/`the consumer`; the non-visual change needs no `PERSONAS:` marker. |
| Deferrals | None remain: the PTY path is covered by a real driver, not deferred. |
| Interaction Coverage | Five inventory rows, five matrix rows, five matching `@e2e` titles; the refusal scenarios are regular error cases of the same action. |
| Risk | The dominant implementation risk (unconverted permanent scenarios) is closed by the five deltas; the residual risk is ordering the resolution before the first write, which the `--force`-keeps-persisted-selection scenario asserts. |
| Domain context | Use-case id `initialize-project`, typed `Includes: fragment: support`, `Extends: none`, capabilities `initialize-a-project` + `enable-optional-addons`; delta use-case base hash verified against `givn/specs/initialize-project/usecase.md` before the update. |
| ADR qualification | AMEND_ADR of ADR-013 is the canonical route for a refinement; amendment text is consistent across the ADR body, the chapter-09 register, and `adr/README.md`. |
| Architecture documentation | Reviewer's independent 12-row walk matches `arc42.md`; the claimed chapter edits (06, 08, 09, 10, 11) exist and match the design; all 12 chapter files exist with no ASCII-art diagrams. |
| Ubiquitous language | "Agent target" and "Harness" carry the wording; the delta introduces no term the glossary lacks, so no glossary edit is needed. |
| Fixture fallout | Every `givn init` invocation in the tests is accounted for: fixtures now pass `--agents opencode`; non-fixture call sites already pass `--agents` or a non-empty answer; the deleted step's only scenario is replaced by the delta. |

## Open questions

Questions that could not be resolved and were flagged explicitly.

| # | Question |
|---|---|
| — | none |

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `initialize-project` owns both affected capabilities; the change extends `initialize-a-project` (route reported no signal; proposal records the decision). | none needed — evidence-derived |
| Typed Includes / Extends relationships | `Includes: fragment: support`; `Extends: none` — unchanged by this change. | none needed — evidence-derived |
| Confirmed Persona coverage | `developer--primary-user` confirmed in `givn/personas/`; the CLI text surface has a named persona. | none needed — evidence-derived |
| Actor and Persona distinction | Gherkin actors are `the user` and `the consumer`; personas stay review lenses and never appear as actors. | none needed — evidence-derived |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Primary — runs `givn init` and reads the refusal | Delta refusal scenarios assert the exit code, the stable sentence, the named targets, and the unchanged project; the PTY scenario drives the interactive confirmation. |
| agent--technical-role | Indirect — the authoring agent consumes the installed skills | The change does not alter generated guidance; sync behaviour is unchanged and pinned by the existing e2e scenarios. |
| givn-maintainer--owner | Governance — an unusable initialization is rejected instead of silently producing an OpenCode-only project | The refusal-before-write rule and the ADR-013 amendment. |

## Architecture documentation (arc42) check

Independent walk of the chapter-selection table before reading `arc42.md`.

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | n/a |
| 2 | 02 architecture-constraints | No | No | Yes | n/a |
| 3 | 03 context-and-scope | No | No | Yes | n/a |
| 4 | 04 solution-strategy | No | No | Yes | n/a |
| 5 | 05 building-block-view | No | No | Yes | n/a |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — §6.1 resolves the selection before the first write and records the refusal |
| 7 | 07 deployment-view | No | No | Yes | n/a |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — §8.7 separates the init precondition from the sync fallback |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — ADR-013 amendment paragraph; register and `adr/README.md` rows |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes — Q-5 names initialization as the enforcement point |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-72 records the scripted-init compatibility consequence |
| 12 | 12 glossary | No | No | Yes | n/a — "Agent target" and "Harness" already cover the vocabulary |

- [x] All 12 rows assessed independently against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md
      saying "No" or omitting the row.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry
      in chapter 09; non-qualifying decisions name exactly one canonical target
      (none non-qualifying in this change — the single decision is an amendment).
- [x] Every MADR consequence has a counterpart entry in chapter 11 (R-72).
- [x] Every ADR candidate records the structured qualification verdict and its
      route-specific target; `Must be shared` is supporting evidence only.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

Edits applied to existing artifacts as a result of grilling.

| Artifact | Change summary |
|---|---|
| `specs/initialize-project/enable-optional-addons.feature` | Created: two `@givn.modified` scenarios now name the selection. |
| `specs/operate-product/declare-the-product-start.feature` | Created: the `@e2e` scaffolding scenario now names the selection. |
| `specs/fragments/rules-policy.feature` | Created: the fresh-project policy scenario now names the selection. |
| `specs/fragments/measure-coverage-per-project.feature` | Created: the initialized-project script scenario now names the selection. |
| `specs/keep-documentation-current/readme-enforcement.feature` | Created: the scaffolding scenario now names the selection. |
| `specs/initialize-project/initialize-a-project.feature` | Added the PTY refusal scenario; extended the non-interactive refusal scenario with the `.opencode`, `.claude`, and `--agents` assertions. |
| `design.md` | Replaced the "no PTY seam" deferral with the `script -qec` fix, recorded the new step, corrected the deleted-step rationale and merge-order constraint, and added the coverage-boundary notes. |
| `arc42.md` | Normalized the verdict to `qualification: QUALIFIED` with per-dimension evidence arrays. |
| `AGENTS.md` | Added `script` (util-linux) to the acceptance-suite prerequisites. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger (`Decides D1`, `Decides D2`).
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated with the missing scenarios and assertions.
- [x] `givn lint --change require-agent-target` exits 0 or 2 (no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
