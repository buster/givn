# Review: change-release-note

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
0 empty step bodies found across 59 step definition files.
This change touched 0 step definition files — all three scenarios reuse
existing bindings. The reused Then bindings hold real assertions:
  tests/steps/mod.rs:620       stdout_contains_quoted_then  → assert!(s.contains(..))
  tests/steps/run_execution_e2e_steps.rs:231  file_contains → assert!(content.contains(..))
  tests/steps/prose_consolidation_steps.rs:97  generated_guidance_contains → assert!(..)
  tests/steps/prose_consolidation_steps.rs:106 generated_command_contains  → assert!(..)
  tests/steps/prose_consolidation_steps.rs:128 generated_guidance_does_not_contain → assert!(..)
Pre-existing no-op steps in tests/steps/mod.rs (`placeholder`) serve
@givn.removed scenarios only and are not used by this change.
```

**@e2e tag integrity** — all three delta scenarios carry `@givn.added @e2e`;
git history shows each scenario commit removes only `@wip` and keeps `@e2e`:

```
$ git log --format='%h %s' -- givn/changes/change-release-note/specs/create-change/change-release-note.feature
60e0b73e test(e2e): Give every change one release note, shared as the subject of all its revisions
ef90fa37 test(e2e): Give every change one release note, shared as the subject of all its revisions
7b253d7d test(e2e): Give every change one release note, shared as the subject of all its revisions
a62ec8b5 docs(change-release-note): Give every change one release note, shared as the subject of all its revisions
$ git show 7b253d7d -- <feature>   # same shape in ef90fa37, 60e0b73e
-  @givn.added @e2e @wip
+  @givn.added @e2e
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| A new change's proposal carries the release-note field | `7b253d7d7977dc86f048f875652a69d5031cc8fa` | Yes — `assets/templates/proposal.md` (embedded scaffold content) |
| The proposal guidance requires a release note | `ef90fa37a32d7039b6af06ef083bcfa6fc4b4739` | Yes — `assets/instructions/proposal.md` (embedded instruction content) |
| The generated implementation guidance shares the release note | `60e0b73e117faf04993f2d24f4da13d6f1614f4a` | Yes — 13 guidance assets, 4 regenerated skill mirrors, 6 regenerated command mirrors, `AGENTS.md`, `README.md`, `docs/workflow.md` |
| Setup task (not a scenario) | `a62ec8b51a01b3bb69076461c54c9da1edf34c69` | Planning artifacts + arc42 chapter edits (process content) |

No Rust source file changed; the product content of this change is the embedded Markdown/`.tmpl` assets, which is what the specs assert.

**Promised-architecture cross-check** — every component design.md said would
be created actually exists (this change creates no component; it names edited assets):

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Proposal scaffold field | Yes | `assets/templates/proposal.md` |
| Proposal instruction rule | Yes | `assets/instructions/proposal.md` |
| Generated guidance (4 skills, 6 commands, 2 task instructions, AGENTS.md hint) | Yes | `assets/skills/givn-{propose,steps,implement,tasks}/SKILL.md.tmpl`, `assets/commands/givn-{propose,steps,implement,tasks,characterize,reverse-engineer}.md`, `assets/instructions/tasks{,-reverse}.md`, `assets/agents-project-hint.md` |
| Durable docs | Yes | `README.md`, `docs/workflow.md`, `docs/arc42/12-glossary.md`, `docs/arc42/08-crosscutting-concepts.md` |
| Rust code components | None promised | N/A (no source change) |

**Strict-mode proof** — confirmed present and passing in tasks.md setup task:

```
tasks.md, "Setup — runner and strict-mode proof":
  GIVN_TEST_RUNNER=cargo GIVN_FEATURES=/tmp/opencode/strict-proof ./run-tests.sh --non-e2e
  Step failed: Step doesn't match any function
  [Summary] 1 feature / 1 scenario (1 failed) / 1 step (1 failed)
  exit=1
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps
checked for a primary assertion on the observable output, return value,
acknowledgement, or visible state of the interface named in design.md, not
repository-only; for a browser-UI capability, the step definitions checked
for actually driving the browser (click/type/navigate), not an HTTP request or
in-page `fetch()`:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| A new change's proposal carries the release-note field | Yes — the file written by the real `givn new` subprocess is the scaffold command's primary output | N/A — CLI | No |
| The proposal guidance requires a release note | Yes — stdout of the real `givn instructions proposal` subprocess | N/A — CLI | No |
| The generated implementation guidance shares the release note | Yes — the generated skill, command, and `AGENTS.md` files written by the real `givn skills sync` subprocess are the command's primary output | N/A — CLI | No |

**E2e scope check** — each capability has one `@e2e` scenario per distinct
action, not one per variant/error case:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| change-release-note | 3 | Yes — create a change; read the proposal guidance; read the generated implementation guidance (one each) | No |

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes, and confirmation the tree (including untracked files, via `git status`)
was searched for any other/parallel e2e implementation of the same
capability that might be stronger than the one being reviewed:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| change-release-note | `./run-tests.sh --e2e` → `tests/features_runner.rs` (`--tags @e2e`) → shared bindings in `tests/steps/mod.rs`, `tests/steps/prose_consolidation_steps.rs`, `tests/steps/run_execution_e2e_steps.rs` | No — `git status` shows only `tasks.md`; no parallel step implementation exists | N/A |

**Local environment check** — the local run command (design.md's Local
Runnability section) starts the full stack, including digital twins,
cleanly. This capability has no stack: the real interface is the `givn` CLI in
temporary fixture directories, and there is no browser, server, database,
queue, or digital twin:

```
$ ./run-tests.sh
Summary [37.643s] 563 tests run: 563 passed, 0 skipped   exit=0
$ ./run-tests.sh --e2e
Summary [6.987s] 88 tests run: 88 passed, 475 skipped      exit=0
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/config.yaml`; if identical, this is always a finding
(the "e2e run" is not isolating `@e2e` scenarios at all). No archive receipt
exists yet (unproven candidate), so the pre-archive counts are recorded here;
the archive receipt will carry the per-scope counts against the candidate:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 563 (permanent scope) | 88 (permanent scope, `@e2e`) | Yes — 88 < 563. In the delta scope all 3 scenarios are `@e2e`, so delta counts are equal and stated explicitly. |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `create-change` goal and guarantees unchanged; the delta adds the one release-note rule to `usecase.md` | Yes |
| Capability ownership and typed relationships preserved | `change-release-note` is `NEW in create-change`; includes the `support` fragment; extends none — unchanged | Yes |
| Confirmed Persona-relevant outcomes preserved | All three Personas find their outcome in the guidance (see disposition below) | Yes |
| Actors remain distinct from Persona biographies | Gherkin actors are Developer and the supporting Authoring agent; Personas are review lenses only | Yes |
| Migration ledger and coverage evidence complete, if applicable | N/A — no migration | N/A |

Use-case scope rejection criteria walked against `create-change`: single goal
and single success guarantee; primary actor named in Actors and Trigger;
`change-release-note` advances the create-change goal (placement passes); no
mutation under an observing goal; extension references main-flow step 1;
preconditions consistent; no mechanism vocabulary in goal/trigger/guarantees;
capability name is goal-shaped; leaf main flow has two steps; relationships
valid (fragment include, no extends). No rejection criterion holds.

```
PERSONAS: developer--primary-user, agent--technical-role, givn-maintainer--owner
```

Persona dispositions:

| Persona | Outcome preserved | Evidence |
|---|---|---|
| developer--primary-user | Authored release note in the scaffold; readable instruction rule | `assets/templates/proposal.md`, `assets/instructions/proposal.md` |
| agent--technical-role | Generated skills and commands state the shared-subject rule | 4 skill templates, 6 command templates, `AGENTS.md` hint |
| givn-maintainer--owner | One canonical sentence reused everywhere; glossary term with anti-terms | `docs/arc42/12-glossary.md:37`, zero occurrences of the superseded phrase |

## Deferred mandatory checks

None. No mandatory check was deferred, so no `DEFERRAL:`/`OPERATOR-DECISION:`
pair is recorded.

**Design.md conformance check** — implementation's actual commands, file
layout, and framework/driver choices diffed against what design.md
explicitly named. Any deviation implemented without first updating
design.md and re-running design-review is a finding, not an acceptable
shortcut:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| E2E command | `./run-tests.sh --e2e`, tag-filtered to `@e2e` | `verify.e2e_command: ./run-tests.sh --e2e` | Yes | N/A |
| Step definitions | All three scenarios reuse existing bindings; no new capability step file | No step file changed (`git log --name-only` shows no `tests/` path) | Yes | N/A |
| Release-note shape | Heading only, no second label | `## Release note` + guidance quote + sentence | Yes | N/A |
| Canonical wording | The three phrases unbroken on one line; command surfaces carry the rule | Verified on all 13 assets and all generated mirrors | Yes | N/A |
| Runner / driver | cucumber-rs via `/run-tests.sh`, real `givn` subprocess | Unchanged | Yes | N/A |
| Rust source | No Rust source change | `git diff --stat` over all commits shows no `src/` change | Yes | N/A |
| arc42 | 8.7 sentence + Release note glossary term | Both present | Yes | N/A |

**Interaction coverage verification** — spec's User Interaction Inventory
cross-referenced against design's Interaction Coverage Matrix, `.feature`
file, and step definitions. Every inventory entry must have a matrix row,
a matching `@e2e` scenario, and step definitions that use the promised
driving mechanism. The inventory lives in the change's `usecase.md`
(`## Interactions`, `change-release-note` rows), the repo's durable
inventory home:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| create a change | Yes | A new change's proposal carries the release-note field | Real `givn new shared-note` subprocess, scaffold file read | `user_runs` → `common::run_givn(env, ["new","shared-note"])`; `file_contains` reads the written file | Yes |
| read the proposal guidance | Yes | The proposal guidance requires a release note | Real `givn instructions proposal --change shared-note` subprocess, stdout | `user_runs` → real subprocess; `stdout_contains_quoted_then` asserts stdout | Yes |
| read the generated implementation guidance | Yes | The generated implementation guidance shares the release note | Real `givn skills sync` subprocess, generated files read | `user_runs` → real subprocess; `generated_guidance_contains`, `generated_command_contains`, `generated_guidance_does_not_contain`, `file_contains` read the written files | Yes |

## Visual Review

N/A — the change's design declares no visual interface (no
`## Visual Design Contract` section; `Interface: none` was not needed because
no screen exists). Opposite-classification test: the outputs are CLI text,
written scaffold files, and generated Markdown; nothing renders HTML, CSS, or
JavaScript in a browser, so no browser driver applies and no screenshot or
transcript is required. This matches the previous CLI-only guidance changes
(for example the term-introduction-discipline review's `Visual block N/A`).
No `VISUAL-REVIEW:` marker is recorded because no contract exists to verify.

## Retirements

N/A — this change retires nothing.

## Arc42 implementation conformance

Affected chapters were derived independently from the completed change before
reading `arc42.md`: only chapter 8 (guidance content) and chapter 12 (new
term) are affected; the other ten rows of the change-level table match.

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| 8.7 Skill and Rule Generation — generated implementation guidance states the release-note rule; `givn commit` neither composes nor validates | `docs/arc42/08-crosscutting-concepts.md:206-210` | Affected (row 8), summary matches | Arc42 impact section names 8.7; D3 decides guidance only | Scenario 3 GREEN edits the guidance assets and self-dogfood sync | 13 assets + generated mirrors + `AGENTS.md` all carry the phrase; `grep` finds the canonical sentence in every generated skill/command target | Match |
| 12 Glossary — **Release note** term with scenario-title contrast and anti-terms | `docs/arc42/12-glossary.md:37` | Affected (row 12) | Arc42 impact section names chapter 12 | Scenario 1 and 2 GREEN embed the term in the scaffold and instruction | Term used consistently in proposal, spec, design, template, and instruction; no competing synonym in changed assets | Match |
| 9 architecture decisions — no qualified ADR; D3 routed to `design.md` | `arc42.md` Notes; no MADR file created | No (row 9) | Decisions ledger records the D3 route | N/A | No `docs/arc42/adr/` change in any commit | Match |
| All other chapters (1-7, 10, 11) — no impact | unchanged | No (rows 1-7, 10, 11) | No impact claimed | No task maps to them | No edits to those chapters | Match |

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

**Result:** CLEAN.

## Ubiquitous language conformance

The durable glossary (`docs/arc42/12-glossary.md`) holds the **Release note**
term with its definition, the scenario-title contrast, and the anti-terms
(`change summary`, `changelog entry`). The proposal, the delta
`usecase.md`, the spec, the design, the scaffold, and every generated guidance
surface reuse exactly that term; the superseded phrase
`message references the scenario title` occurs zero times in the changed
assets.

**UBIQUITOUS-LANGUAGE CONFORMANCE: CLEAN**

---

## Coverage

Coverage is measured once by the archive against the committed candidate and
recorded in the verification receipt. Review does not run an instrumented
pass; classify gaps from the most recent measured evidence plus static
inspection, and say explicitly when no measured evidence exists yet.

**Coverage result:** no receipt exists yet for this change (unproven
candidate). The last published merged report (`coverage/cobertura-coverage.xml`,
2026-09-23) predates this change. It is not needed for classification here:
this change modifies no Rust production source — `git diff --stat` across all
four commits shows only Markdown/`.tmpl` assets, generated rule mirrors, and
change artifacts. No production line or branch is added, changed, or removed,
so no new uncovered region exists; the archive receipt will carry the measured
result for the candidate.

## Coverage gap classification

Every uncovered region must be classified as exactly one of the three buckets.
There are no uncovered regions introduced by this change; no fourth-bucket
language is used anywhere in this report.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| — (no production source changed) | 0 | N/A — no gap | None needed; archive receipt carries the measured result |

---

## Scenario execution evidence

Review does not re-run the suite. No archive receipt exists yet:

```
unproven candidate — no receipt yet. The archive runs both scopes once
against the published candidate revision and records the result in
givn/archive/change-release-note/verification.json.
```

**Receipt status:** not archived yet

## Overlap dispositions

For every shape-match finding involving this change's delta, one row.
Decision must be one of: `duplicate`, `variant`, `boundary`.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A goal-aligned capability ranks above a lexically similar candidate | The proposal guidance requires a release note | boundary |
| The absolute-mode policy block carries the dialog context rule | A new change's proposal carries the release-note field | boundary |
| Review passes when every shape match is disposed and reports the net-delta receipt | The proposal guidance requires a release note | boundary |

Boundary rationale: each pair shares a masked assertion shape (a CLI subprocess
whose stdout or written file is checked for a sentence) but asserts a different
invariant on a different command — route ranking, the `AGENTS.md`
absolute-mode block, the review-disposition receipt, and the proposal
instruction respectively. None is a duplicate or a variant of another.

## Split-or-keep

No delta scenario exceeds the lint threshold; no rows.

| Scenario | Decision |
|---|---|

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope in the archive
      verification receipt (review does not run the suite) — not archived yet;
      receipt status recorded above.
- [x] Every coverage gap is classified and resolved (or justified) from the
      most recent measured evidence plus static inspection — no production
      source changed, no gap.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly — no stack exists; the CLI suite starts cleanly (563/88 pass).
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (88 < 563 in the permanent scope, or
      "all scenarios are @e2e" stated for the delta scope).
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; any deviation went through a design.md update
      + design-review re-run, not a silent shortcut.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces (the
      section states "None"; no production-code primitive was added).
- [x] arc42 slug-mapping verified: every arc42 chapter slug in arc42.md
      (`08-crosscutting-concepts`, `12-glossary`) maps to a real
      `docs/arc42/<slug>.md` file.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

README-IMPACT: updated - Quickstart

**REVIEW: PASS**
