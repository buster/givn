# Design Review: development-timeline

Two independent grilling passes (fresh context, high effort) interrogated the
complete planning layer (`proposal.md`, `specs/**`, `design.md`,
`docs/arc42/**`, `docs/design/design-system.md`), the orchestrator merged their
findings, the operator answered the two questions that needed a decision, and
the hardening pass applied every decision and every evidence-backed finding. No
question remains open.

## Grilling log

The table is the scannable index; the full record for each question is written
in a per-question block below it.

| # | Branch | Question | Outcome |
|---|---|---|---|
| Q1 | Visual design contract | Which UI states does the report declare, and which scenario carries each state's evidence? | closed 2026-10-16 (operator) — declare only `empty`; add its `@e2e` traversal |
| Q2 | Scope | Does the plan define the report's scope window and band rule, or drop them? | closed — window and band defined; flagless command renders the project report |
| Q3 | Tech choices | How does the binary resolve a recording scope without a reporter-supplied marker? | closed — exactly-one-scope rule over existing project state |
| Q4 | Scope | Does the proposal's per-ideation promise have executable coverage? | closed — topic-scope scenario plus a topic Example in the open-report outline |
| Q5 | Risk | What holds when two processes append to the record at once? | closed — append-time ids under the store lock, parents per process, report ordered by `at` |
| Q6 | Tech choices | Which ignore destinations must the record's fenced block reach? | closed — both backends' destinations through the existing switch<br>**Superseded 2026-10-16 (re-pass): the Fossil arm was replaced — see *Re-review (2026-10-16)* below; the historical text is retained.** |
| Q7 | Missing scenarios | Is the operator-approved privacy bound observable? | closed — bounded-label scenario with the absence assertion |
| Q8 | Scope | Are the history-reading commands themselves recorded? | closed — `givn history` and `givn timeline` are excluded |
| Q9 | Interaction coverage | Is pan a separate interaction? | closed — implementation detail of the zoomable axis |
| Q10 | Testability | What is the regular verification scope, and what does it declare? | closed 2026-10-16 (operator) — `./run-tests.sh --non-e2e`; e2e scope unchanged |

### Q1 — Declared UI states and the evidence each one owes

- Status: closed 2026-10-16 (operator)
- Blocks: the Visual Design Contract's `States:` line, the per-state evidence
  files, and the set of empty-state scenarios.
- Context: the enforcing rule (`src/integrity_hook.rs:377-397`) demands a
  non-empty `<viewport>--<state>.png` for every declared state and viewport. The
  contract declared `populated, empty`, but no `@e2e` scenario traversed the
  empty state — the only empty-state scenario was regular and carried browser
  steps — so the declared `empty` state had no evidence path, and `populated`
  duplicated what every final-state `<viewport>.png` already is.
- Question: Should the report declare both `populated` and `empty` and pay for a
  state capture of each, declare only `empty` and add an `@e2e` traversal of it,
  or keep the declaration as written and leave the evidence to the change-wide
  gate?
- Recommended: declare only `empty`, add the `@e2e` empty-scope scenario, and
  keep `populated` implicit as the final-state capture.
- Options:
  1. **Declare only `empty`, with its own `@e2e` traversal** — one new inventory
     interaction and one state-evidence pair.
     In practice: the contract reads `States: empty`, and the empty-scope
     scenario commits `mobile--empty.png` and `desktop--empty.png`.
     Gain: every declared state has an enforced evidence path, and the
     declaration stays true.
     Cost: `development-report` gains one more `@e2e` interaction, and the empty
     state must be reachable through a generated report for an activity-free
     scope.
  2. **Declare both states and capture both** — a populated-state screenshot per
     scenario and viewport.
     In practice: every populated `@e2e` scenario commits `<viewport>.png` and
     `<viewport>--populated.png`.
     Gain: a symmetric declaration.
     Cost: a duplicate capture whose pixels repeat the final-state file, for no
     extra assertion.
  3. **Keep the declaration, add no traversal** — a declared state with no
     evidence.
     In practice: the archive gate fails on a missing `<viewport>--empty.png`,
     or the declaration is quietly dropped later.
     Gain: nothing.
     Cost: exactly the unenforced gap the integrity hook exists to prevent.
- Resolution: **option 1**, because a declared state without a traversal is an
  unenforced gap under the enforcing rule, while `populated` is what every
  final-state screenshot already is. Rejected: option 2, because the additional
  capture asserts nothing new; option 3, because it leaves a declared state
  unenforced. The operator accepted the consequence that the
  development-report capability gains an additional `@e2e` interaction: the
  inventory, the Interaction Coverage Matrix, and the `@e2e` titles now hold
  eight rows each.

### Q2 — The report's scope window and band rule (merged grilling Q2/Q6/F6)

- Status: closed 2026-10-16 (hardening; evidence-backed, no operator input)
- Blocks: whether the approved D5 answer is reachable at all, and the report's
  band model.
- Context: D5 records everything and falls back to the project scope, but the
  plan rendered only one change or one topic, so project-scope entries were
  recorded and never shown; the design stated no rule for a flagless
  `givn timeline`, and the design system's `Band row` "outside a scope" state
  had no owning rule.
- Question: Should the plan define the report's scope window and band rule
  (including a flagless project report), or drop the band concept as unneeded?
- Recommended: define both.
- Resolution: **defined**, because dropping the band would make D5's approved
  answer unreachable and hide recorded work. Rejected: dropping the band,
  because the record's fallback scope would then have no reader. The rule is now
  recorded in `design.md` → *Scope resolution and the report window*, the
  design system's `Band row` state stays, and the "outside a scope" band is
  named as the project report's own row.

### Q3 — How a scope is resolved without a marker file (merged Q3)

- Status: closed 2026-10-16 (hardening; evidence-backed)
- Blocks: `resolve_scope`'s definition, the intake's override precedence, and
  the scenarios that pin a change or a topic scope.
- Context: `resolve_scope` was written as "active change → active ideation topic
  → project" with no definition of *active*, and the intake payload's
  `change`/`topic` were documented as fields with no stated precedence or
  failure behaviour.
- Question: Which existing project signals decide the scope, and what do the
  payload overrides do?
- Recommended: reuse the repository's own rules rather than invent state.
- Resolution: **change scope = exactly one active change**, the rule
  `givn instructions <artifact>` already applies (`src/cli/instructions.rs:25,84-86`
  over `state::list_changes`; several active changes fall through);
  **ideation scope = exactly one unfinished topic**,
  `givn/ideation/<topic>/session.md` without its `## Completion` section (the
  repository's own session format — read before writing the rule; all three
  topics here carry that section today); **project scope = the fallback**; the
  payload's `change`/`topic` are optional overrides with the change winning, and
  a named scope that does not exist is a rejected event. Rejected: an
  `active-scope` marker file read by the reporters, because it duplicates state
  the project already derives (ADR-014), needs its own write, repair, and
  staleness path for two rules that already exist, and a reporter forwarding a
  stale marker is still wrong while the binary resolving from the project is
  not.

### Q4 — Making the per-ideation promise executable (merged Q3/Q2, S2)

- Status: closed 2026-10-16 (hardening; evidence-backed)
- Blocks: whether the proposal's per-ideation report has any executable
  coverage.
- Context: the proposal promises reports for changes *and* ideations, where the
  work happens before any change directory exists — but the delta spec had no
  topic-scope scenario, the `@e2e` set had no topic traversal, and the record
  side had no entry pinned to a topic.
- Question: Should the plan carry a topic report as a scenario (and if so, as a
  new interaction or as input variants of an existing one)?
- Resolution: **input variants of the existing action.** A regular scenario
  reports a topic scope ("The report of an ideation topic names the work done
  before the change exists"), the record side gains "Activity during an ideation
  belongs to its topic scope", and the existing open-report `@e2e` scenario
  becomes a Scenario Outline with a `change` and a `topic` Example — one
  normalized action with input variants, so no second inventory row and no
  duplicate coverage of one action.

### Q5 — Two writers on one append-only store (merged Q5/F5)

- Status: closed 2026-10-16 (hardening; evidence-backed)
- Blocks: the record's `id`/`parent` contract, the report's ordering rule, and
  the append-only scenario's claim.
- Context: the contract minted `id` "from the count of existing lines at first
  append in this process" and the append-only scenario claimed the new entry
  "follows it in time order". A command and a harness intake flush can append
  concurrently, which mints duplicate ids; and because an intake flush can land
  after a later-timestamped command entry, file position and time order
  genuinely differ.
- Question: What holds when two processes append at once?
- Resolution: `id` is minted **at append time under the store's exclusive
  append lock** from the store's current last line; `parent` links are scoped to
  ids minted by the same process, so every parent names an entry that exists;
  the **report orders entries by `at`**, never by file position; the append-only
  scenario now claims only that the earlier entry stays unchanged and the new
  one is appended without a rewrite, and two new scenarios pin the contract
  (distinct ids, parent links resolve). Rejected: leaving the contract as
  written, because it would fail under the concurrency the intake itself
  introduces.

### Q6 — The record's ignore destinations (merged F7 and a v4-pro finding)

- Status: closed 2026-10-16 (hardening; evidence-backed)
- Blocks: `ensure_ignored`, TC-34's constraint text, and the Fossil arm of the
  record's repository hygiene.
- Context: the design said the record stays out of repository history "with a
  managed ignore block" but never named a destination; a Git-only `.gitignore`
  would leave a Fossil project's record trackable, and the repository already
  owns a backend switch for exactly this.
- Question: Which destinations does the record's fenced block reach?
- Resolution: **follow the existing switch** (`src/skills/rules.rs::write_ignores`)
  with the record's own `# givn:begin:history` block — `.gitignore` plus the
  worktree-local exclude file (resolved through
  `git rev-parse --git-path info/exclude`) for Git, and
  `.fossil-settings/ignore-glob` plus the repository-local `ignore-glob` setting
  for Fossil. Named in `design.md` (D1) and TC-34, and covered by two scenarios
  ("The record stays out of the repository's own history" for Git, "A Fossil
  project ignores the record through its own ignore file" for Fossil).
- Superseded 2026-10-16 (design re-pass): the Fossil arm of this resolution was
  replaced — see *Re-review (2026-10-16, after the implementation-phase design
  correction)* below. The historical text above is retained as written.

### Q7 — Making the privacy bound observable (merged Q7)

- Status: closed 2026-10-16 (hardening; evidence-backed)
- Blocks: D2's observable behaviour.
- Context: D2 was approved as "metadata plus a bounded 120-character label", and
  the design bounded it in `bound_label`, but no scenario asserted the bound or
  the absence of the complete payload — the operator-approved privacy boundary
  had no test that could fail.
- Question: Should the spec pin the label bound and the absence of complete
  payloads?
- Resolution: **yes** — "A recorded label is bounded to 120 characters" asserts
  both the ≤ 120-character label with its truncation mark and that no complete
  command text is stored.

### Q8 — Recording the commands that read the record (merged Q8, S5)

- Status: closed 2026-10-16 (hardening; evidence-backed)
- Blocks: the capture-point table.
- Context: the capture points record every command, so `givn history` and
  `givn timeline` would append entries about themselves: every report's own read
  would become the newest entry in the next report, and the store's size would
  depend on how often it is read.
- Question: Are the history-reading commands themselves recorded?
- Resolution: **no** — both commands run without a recorder, recorded in the
  capture architecture as an explicit exclusion, with the scenario "Reading the
  development record does not extend it" (asserting after both a `givn history`
  read and a `givn timeline` render).

### Q9 — Pan as an interaction (merged Q9, S6)

- Status: closed 2026-10-16 (hardening; evidence-backed)
- Blocks: the interaction inventory and the `@e2e` set.
- Context: the report renders a zoomable axis and the design's rendering
  paragraph wired both zoom and pan, which raised the question whether panning
  is its own consumer interaction with its own row.
- Question: Does pan need an inventory row and an `@e2e` scenario?
- Resolution: **no** — pan is recorded as an implementation detail of the
  zoomable axis (it changes which slice of the already-rendered range is in
  view, not what the report contains), so it adds no inventory row, no matrix
  row, and no `@e2e` scenario; the zoom scenario remains the axis-state
  evidence.

### Q10 — The regular verification scope and the two receipts

- Status: closed 2026-10-16 (operator)
- Blocks: `verify.command`, the Local Runnability and Coverage Process
  Boundaries text, and the CI/release prerequisites.
- Context: `verify.command` is `./run-tests.sh`, and the bare form runs the
  whole suite — `@e2e` included — so the regular archive receipt would run the
  browser tier and depend on Node and Chromium, while the design described the
  bare command as the "unit/integration scope", which the script contradicts.
  The change introduces the browser tier, so this is the first time the
  difference costs a receipt.
- Question: Should the regular scope narrow to the non-e2e tag set (leaving the
  browser tier to the e2e scope), or should both scopes keep running everything?
- Recommended: narrow the regular scope to `./run-tests.sh --non-e2e` and keep
  `verify.e2e_command` as `./run-tests.sh --e2e`.
- Options:
  1. **Narrow the regular scope to `--non-e2e`** — the regular receipt runs
     exactly the `not @e2e` tag set; the e2e receipt runs the browser tier.
     In practice: `givn/commands.yaml` declares
     `command: "./run-tests.sh --non-e2e"`, and an archive still produces a
     regular and an e2e receipt.
     Gain: a regular verification run needs no browser, and the receipt scopes
     match the tags they claim.
     Cost: the CI and release workflows must install Node/Chromium before their
     e2e scope, and the README's command table follows the declaration.
  2. **Keep the bare command and install the browser for the regular scope** —
     one command, no declaration change.
     In practice: the regular receipt silently includes the browser tier.
     Gain: no workflow or README edit.
     Cost: the regular scope needs a browser and a Node project, and the two
     receipts overlap.
- Resolution: **option 1**, because the receipt scopes must match the tag set
  each one claims, and the coverage measurement already splits the same way.
  Rejected: option 2, because it makes the regular scope depend on a browser it
  does not assert anything with. The operator's decision is recorded as the
  design's `verify.command`; `givn/commands.yaml` + README and the
  Node/Chromium install step before the first suite invocation in
  `.github/workflows/ci.yml` and `release.yml` (which joins the design's
  Architecture Impact table) are **implementation requirements named in the
  design, not edits made here**.

## Resolved by codebase exploration

Items answered without user input by exploring the existing codebase (and the
reviewers' evidence-backed findings merged into them).

| Branch | Finding |
|---|---|
| Missing scenarios (F11) | The record never reaching repository history, the intake's exit codes (`2` for unparsable stdin, `0` with the rejected event named on stderr), and the printed report path were implied but unasserted; the per-model token total was cheap. All four are now scenarios or extra step assertions. |
| Testability (F-attestation) | The single-scenario run command used `--name`, which is not this repository's canonical form; it now uses `--exact "<relative feature path>::<scenario title>"`. The report's phase derivation was an unnamed rule; it is now stated and the CLI `@e2e` asserts the concrete `implement` stage. |
| CLI surface (F10) | `givn timeline --open` was unproposed, and `--json` had no scenario. Both are dropped: opening stays the developer's step (as the proposal states), and the machine-readable view is `givn history --json`, which already exists. The default output path (`givn/history/reports/<scope>.html`, chapter 07's footprint) is now named. |
| Architecture documentation (F12) | `arc42.md` row 1 claimed "No product goal … changes", which the change's own FR-level behaviour contradicts; it is now Yes with FR-45 added to chapter 01. |
| Ubiquitous language (F13) | `band` was used by the design, the design system, and the glossary's own entries but had no glossary entry; **Band** is now recorded with its anti-term (`lane` stays reserved for the rejected reference view). |
| ADR qualification (F15) | ADR-073's existing-ADR check and More Information did not name the receipt boundary it consumes; ADR-070/ADR-072 are now named, with the record's non-ownership of the proof boundary stated. |
| Use-case scope (grilling Q4) | The use case needed the explicit rule that recording is the observation substrate of the observing goal; the rule line now rejects the mutation-under-an-observing-goal pattern and keeps the single use case. |
| Advisories (v4-pro) | The design's opencode note still named the replaced `@opencode-ai/plugin` 1.18.14 dev pin (the working copy now pins `@opencode/plugin` 2.0.26 — the same claim in `arc42.md` was corrected too); chapter 11's R-87 gap is kept with a one-line note (nothing references the retired id, so renumbering would only add ambiguity); the feature files' comment blocks duplicated the inventory and were removed. |
| Interface classification | Tested in the opposite direction: the report renders HTML/CSS/JS in a browser, so it is a Web UI with a browser driver, and an HTTP client is not a valid driver. The CLI capabilities stay CLI interactions. The classification is unchanged and correct. |
| Decision ledger | D1, D2, D6 carry the operator's 2026-10-16 answers; D3, D4, D5 are decided in `design.md` with their rejected alternatives; D4's surface and D5's window were corrected during hardening. No proposal question is unresolved or unowned. |
| Deferrals | None requested and none recorded: no mandatory check is deferred in this change. |
| Risk | The most likely implementation failure remains the external harness contracts (R-84/R-88) and the best-effort store losing entries (R-85, D-22); both keep their named mitigations (defensive reporters, per-harness fakes in the e2e tier, dropped-event counters, a gap notice in the report). |

## Open questions

None. Every question above is closed, and no finding was left unapplied.

| # | Question |
|---|---|
| — | None. |

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | One new use case `inspect-development-history` owns all three capabilities; each capability has one goal, and the proposal's routing decision records the route's no-signal verdict with its rationale. The goal ("see how a change or an ideation was developed") is observed, and recording is now explicitly its substrate. | Operator approved the proposal at propose; the observation-substrate rule line was added during hardening. |
| Typed Includes / Extends relationships | Both are `none`, and no capability is silently promoted into a reusable fragment: the record has exactly one consumer-facing story. | Recorded as written. |
| Confirmed Persona coverage | Two personas confirmed: `developer--primary-user` (reads the report) and `agent--technical-role` (drives the intake as an automation caller). | Operator confirmed at propose. |
| Actor and Persona distinction | Gherkin actors stay the roles the use case declares (Developer, Authoring agent); Persona IDs are review lenses only and appear in no scenario step. | Recorded. |
| Scope rejection criteria (mutation under an observing goal) | The use case observes; nothing it does changes the work it observes — no gate, no proof, no exit code, no artifact lifecycle. | Rejection criterion satisfied by the new rule line. |

### Persona disposition

One row per confirmed persona, recording the relevance and the evidence.

| Persona | Relevance | Evidence |
|---|---|---|
| `developer--primary-user` | The report is read by the developer and answers the requester's three questions: slowest step, agent fan-out, waiting. | `development-report` scenarios (ranked longest step, question-wait totals, subagent fan-out) and the `@e2e` browser scenarios asserting on the rendered report; the developer is the consumer of the generated file's printed path. |
| `agent--technical-role` | The coding-agent harness whose activity is reported and that drives the intake as an automation caller. | `agent-activity-reporting` scenarios: per-harness reporter installation, the intake's per-event rejection and exit codes, and the `@e2e` outline driving all three generated reporters against the real intake. |

## Architecture documentation (arc42) check

The addon is enabled, so all 12 rows were assessed against `proposal.md`,
`specs/**`, and `design.md` before `arc42.md`'s own claims were read.

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes (the change adds a product requirement) | Yes | Yes | Yes — FR-45 added during hardening |
| 2 | 02 architecture-constraints | Yes (record and report constraints) | Yes | Yes | Yes — TC-34 now names both ignore destinations |
| 3 | 03 context-and-scope | Yes (new interfaces, entity, record file) | Yes | Yes | Yes — the `givn timeline` row was corrected (no `--open`, no JSON model) |
| 4 | 04 solution-strategy | Yes (record/report concern spans capture, storage, intake, rendering) | Yes | Yes | Yes |
| 5 | 05 building-block-view | Yes (history, cli::history, cli::timeline, embedded assets) | Yes | Yes | Yes |
| 6 | 06 runtime-view | Yes (capture flow and intake/report flow) | Yes | Yes | Yes — §6.18's timeline paragraph was corrected during hardening |
| 7 | 07 deployment-view | Yes (`givn/history/` footprint, reporters, Node project) | Yes | Yes | Yes — the report default path matches `givn/history/reports/` |
| 8 | 08 crosscutting-concepts | Yes (ownership, reporters, best-effort, scope, privacy, ignore, scale, packaging, browser tier) | Yes | Yes | Yes — scope, ordering, and ignore paragraphs follow the hardened record contract |
| 9 | 09 architecture-decisions | Yes (ADR-073 plus two non-qualifying candidates) | Yes | Yes | Yes |
| 10 | 10-quality-requirements | Yes (intake, reproducibility, privacy, scale, parity scenarios) | Yes | Yes | Yes — Q-99's rendering-in-time-order claim matches the ordering rule |
| 11 | 11 risks-and-technical-debt | Yes (store growth, contract drift, best-effort loss, browser tier, harness gaps) | Yes | Yes | Yes — the R-87 gap is now explained rather than silent |
| 12 | 12-glossary | Yes (the change's terms, including `band`) | Yes | Yes | Yes — **Band** added with its anti-term |

- [x] All 12 rows assessed independently against proposal.md/design.md before
      reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md
      saying "No" or omitting the row. Row 1 was exactly that finding (the
      change adds a requirement, `arc42.md` claimed it did not) and was
      corrected during hardening.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry
      in chapter 09 (ADR-073), and the two non-qualifying candidates name
      exactly one canonical target each (`design.md` for report packaging,
      `design.md` for the browser-driver application of an existing process
      policy).
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in
      chapter 11 (the process-per-flush and stale-reporter costs → R-84/R-88;
      the unrepaired record → D-22).
- [x] Every ADR candidate records the structured qualification verdict and its
      route-specific target; `Must be shared` is supporting evidence only.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.
      Chapter-file integrity: all 12 files exist with decision-specific
      content; no box-drawing characters and no ASCII pseudo-diagrams were
      found in any of them; the only `placeholder` hits describe the README
      placeholder feature, not scaffolding.

## Changes made during hardening

Edits applied to existing artifacts as a result of grilling.

| Artifact | Change summary |
|---|---|
| `design.md` | New section *Scope resolution and the report window* (exactly-one-scope rules, payload overrides, project report window, band rule, ordering by `at`, the rejected marker file); D1 names both ignore destinations; D4 drops `--open`/`--json` and names the default path; D5 points at the new section; the record table's `id`/`parent` rows and the `HistoryRecorder`/`HistoryStore`/`resolve_scope` primitives follow the new contract; the capture architecture excludes `givn history`/`givn timeline`; Report Generation gains the phase-derivation rule, the ordering rule, the pan note, and the default path; the runner and single-scenario commands were corrected (`--non-e2e`, `--exact`); Local Runnability, the E2E section, Coverage Process Boundaries, and the Architecture Impact table record `verify.command: ./run-tests.sh --non-e2e` and the CI/release prerequisites as implementation requirements; the Interaction Coverage Matrix gained the empty-scope row and the topic outline note (8 rows); the Visual Design Contract declares `States: empty` and points the state evidence at the empty-scope scenario; the opencode note records the current `@opencode/plugin` 2.0.26 pin; the Use-Case Traceability table gained rows for the privacy bound, the read-only recording rule, the ideation scope, and the empty-state e2e. |
| `specs/inspect-development-history/usecase.md` | The interaction inventory gained the empty-scope row (8 rows, 1:1 with the matrix and the `@e2e` titles), and the Rules section gained the observation-substrate rule. |
| `specs/inspect-development-history/development-record.feature` | Inventory comment block removed; append-only claim corrected; six scenarios added (`@givn.added @wip`): concurrent writers, ideation topic scope, bounded label, read-only recording, Git repository hygiene, Fossil ignore. |
| `specs/inspect-development-history/development-report.feature` | Inventory comment block removed; empty-state scenario rewritten to report-level assertions; per-model token total folded into the token scenario; three scenarios added (topic report, ordering by recorded time) plus the `@e2e` empty-scope scenario; the CLI `@e2e` gained the printed path and the concrete `implement` stage; the open-report `@e2e` became a Scenario Outline over `change`/`topic`. |
| `specs/inspect-development-history/agent-activity-reporting.feature` | Inventory comment block removed; two intake exit-code scenarios added. |
| `docs/arc42/01-introduction-and-goals.md` | FR-45 added (record + report). |
| `docs/arc42/02-architecture-constraints.md` | TC-34 names both backends' ignore destinations. |
| `docs/arc42/03-context-and-scope.md` | The `givn timeline` interface row corrected (scope forms, default path, no `--open`, no JSON model). |
| `docs/arc42/06-runtime-view.md` | §6.18's timeline paragraph corrected (project window, default path, no JSON model, no opener). |
| `docs/arc42/08-crosscutting-concepts.md` | Record section: append-time ids, ordering by `at`, parent scope; the scope rule states the exactly-one rule, the overrides, and the project window; the ignore block names both destinations; the stale `--json` claim removed. |
| `docs/arc42/11-risks-and-technical-debt.md` | One-line note explaining the retired R-87 id. |
| `docs/arc42/12-glossary.md` | **Band** added with its anti-term. |
| `docs/arc42/adr/ADR-073-development-record-ownership.md` | Existing-ADR check and More Information now name ADR-070/ADR-072 and the record's non-ownership of the proof boundary. |
| `docs/design/design-system.md` | The screens table declares the report's `empty` state and its evidence, with a populated report as the implicit final state. |
| `givn/changes/development-timeline/arc42.md` | Row 1 flipped to Yes with FR-45; row 12 counts fourteen entries and names `band`; the opencode pin note corrected; a hardening-pass note records the cross-chapter updates. |

Not edited (implementation requirements named in `design.md`): `givn/commands.yaml`
(`verify.command` → `./run-tests.sh --non-e2e`), the README's command table, and
the Node/Chromium install steps in `.github/workflows/ci.yml` and
`release.yml` before each workflow's first suite invocation.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in `design.md`'s
      `## Decisions` ledger or is carried with a named owner (D1/D2/D6 answered
      by the operator on 2026-10-16; D3/D4/D5 decided in `design.md`).
- [x] All questions are resolved (see *Open questions*).
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for every missing scenario that was
      identified; every new scenario is `@givn.added @wip` and no `@e2e` tag was
      removed anywhere.
- [x] `givn lint --change development-timeline` exits 2 — 3 files checked, 41
      findings, **all of kind `[SUBST] advisory`** (the permanent-corpus-versus-
      delta scenario-subset advisories); no parse error, no error-kind finding.
- [x] Architecture documentation (arc42) check completed above.
- [x] Consistency invariants verified: the use-case inventory, the design's
      Interaction Coverage Matrix, and the `@e2e` titles hold the same eight
      rows in the same order (checked mechanically; the count grows from seven
      by the operator-approved empty-scope interaction); the Visual Design
      Contract's `States:` line, its capture paragraph, the design system's
      screen table, and the integrity hook's state-evidence expectation agree;
      `givn status --change development-timeline --json` reports stage
      `arc42-docs-done` with `design-review` as the next required artifact.

**DESIGN-REVIEW: PASS**

## Re-review (2026-10-16, after the implementation-phase design correction)

**Why it ran.** Implementation of Tasks 10–11 (the record store's ignore
mechanism) STOPPED under the never-deviate rule with the failure mode
reproduced on 2026-10-16: the managed `# givn:begin:history` block written into
`.gitignore` dirtied the working tree before the command acted, so
`givn upgrade`'s clean-checkout precondition refused
(`src/cli/migration.rs:356-357` — "automatic managed upgrade requires a clean
checkout; no files were changed", exit 1); and the permanent e2e fixture
`tests/steps/run_execution_e2e_steps.rs::git_managed_project` committed the
store, so every later append showed ` M givn/history/events.jsonl` and
`operate-product/declare-the-product-start.feature::The upgrade commit includes
the backfilled run declaration` failed at "the working tree is clean". A first
correction (Git arm → `.git/info/exclude`; Fossil arm → the repository setting;
`.gitignore` only while no repository exists; permanent-fixture adaptation) was
applied, then interrogated by one fresh-context grilling pass. The grill
returned one operator-level question (Q-A) and findings F1–F11, all inside the
same mechanism; the operator settled Q-A; the design authority applied the
second correction round; this hardening pass verified the applied text against
every finding and wrote this record.

**Q-A — the Fossil arm: decided 2026-10-16, option 2.** The rule is the
ownership split: the recorder never writes a versioned file, and the record path
reaches the versioned destinations only through the commands that own them
(`givn init`, `givn skills sync`, `givn upgrade` — the existing
`write_ignores` path). Four cases: a Git worktree re-asserts the
`.git/info/exclude` block on every write; no repository yet writes the
`.gitignore` scaffold fallback; a Fossil checkout without
`.fossil-settings/ignore-glob` gets the repository-level `ignore-glob` setting;
a Fossil checkout with that file gets the record path (`givn/history/*`) in the
file's managed entry list through the owning commands. Rejected: **option 1,
setting-only** — Fossil's `ignore-glob` is a *versionable* setting, so a
committed `.fossil-settings/ignore-glob` overrides it, and Givn's own rules
writer already writes that file in every managed Fossil checkout; setting-only
would therefore leave the store an `EXTRA` in exactly the projects Givn
manages, and `givn upgrade` / `givn archive` would refuse a project that did
nothing wrong — the failure the never-dirty rule exists to eliminate. Recorded
in `design.md` D1 (`:119-158`) and the record model's *Ignoring the store
without dirtying the worktree* (`:286-314`), in TC-34
(`docs/arc42/02-architecture-constraints.md:34`), §6.18
(`docs/arc42/06-runtime-view.md:521-528`), §7.4
(`docs/arc42/07-deployment-view.md:62-68`), §8.25
(`docs/arc42/08-crosscutting-concepts.md:932-954`), and the change-level
`arc42.md` note (*Ignore-mechanism correction*).

**F1–F11 outcomes** — each verified against the current artifact text of the
working tree; locations are cited per item.

| # | Outcome | Artifact location |
|---|---|---|
| F1 | Verified — the Fossil setting-override residual is named: "a committed file overrides the repository setting, so writing the setting alone would leave the store an `EXTRA`". | `design.md:135`, `tasks.md:163`, TC-34 (`docs/arc42/02-architecture-constraints.md:34`), §8.25 (`docs/arc42/08-crosscutting-concepts.md:936-941`) |
| F2 | Verified — the entry form is **`givn/history/*`**, "not `givn/history`", with the whole-path glob reason. | `design.md:137-139` and `:299-301`, `tasks.md:165-168` and `:2384-2386`, glossary (`docs/arc42/12-glossary.md:55`) |
| F3 | Verified — Task 11 requires `givn init` inside the open Fossil checkout so the managed entry list lands **before** the fixture commit, names `make_fossil_root`'s `addremove`, and extends the same adaptation to `fossil_project_with_version`. | `tasks.md:2392-2398` |
| F4 | Verified — `ensure_ignored` runs on **every** record write, "not only when the store is created, so a store that predates the repository still becomes ignored". | `design.md:288-289` and `:294`, `tasks.md:160` and `:2308-2310`, §8.25 (`:933-936`) |
| F5 | Verified — repository existence is "decided by the backend markers at the project root … **never** by `scm::select` succeeding", with the nested-root / both-markers / missing-executable reason. | `design.md:139-144` and `:306-308`, `tasks.md:169-171` and `:2314-2317`, TC-34 |
| F6 | Verified — the fixture class is declared in full — `git_managed_project`, `git_project_with_version`, `fossil_project_with_version` — with the minimal remedy per fixture (the record path joins the fixture's hand-written ignore content, or the store is removed before the fixture's commit). | `design.md:1009`, `tasks.md:2326-2335` and `:2397-2398` |
| F7 | Verified — `Ignore block` covers both forms: the fenced `# givn:begin:rules` region inside the tracked ignore file, and the record's block in a backend-local, never-tracked destination or as the Fossil managed entry. | `docs/arc42/12-glossary.md:55` |
| F8 | Applied by this record — Q6 is marked superseded in the index row and in its own block, both dated and pointing here; the historical text is retained. | `design-review.md` (Q6 index row and Q6 block) |
| F9 | Verified — R-89 records the reproduction by this change's own implementation on 2026-10-16 (Tasks 10–11 stopped on exactly this), the Fossil residual, and the guards as a fixture class. | `docs/arc42/11-risks-and-technical-debt.md:75` |
| F10 | Verified — Task 11's title stays and matches the mechanism: the fixture runs `givn init` inside the open checkout, so the record path lives in the project's own `.fossil-settings/ignore-glob` managed entry list, and the setting remains the fallback for a checkout without the file. | `specs/inspect-development-history/development-record.feature:82-87`, `tasks.md:2368` |
| F11 | Verified — only the two Git destinations are fenced `# givn:begin:history` blocks; the Fossil destinations are a "deduplicated entry list without fence markers". | `design.md:301-304`, `tasks.md:167-168` and `:2386-2387`, §8.25 (`:947-950`) |

**Branch coverage (the grilling pass), all seven branches.**

- **Sufficiency** (Task 10's Then, Task 11's Then, the permanent clean-checkout
  scenario, D1's privacy intent) — sufficient under the corrected mechanism:
  `.git/info/exclude` lives inside the git dir, so `git status --porcelain`
  emits no line for the store, and the permanent scenario's clean-tree
  assertion holds once the fixture's ignore content carries the record path;
  the Fossil Then holds in a checkout whose managed entry list carries
  `givn/history/*` (now the declared fixture requirement); `git add -A` skips
  ignored paths, so no faithful project's store can enter repository history.
- **Never-dirty residuals** — no repo-present path in the corrected write can
  dirty a tracked file; the grill's four residuals (a store that is already
  tracked; a `scm::select`-keyed fallback; a first-write-only call site; the
  Fossil versionable-setting override) became the F1/F4/F5 corrections, and the
  tracked-store boundary stays named in D1 and R-89.
- **Fixture adaptation** — sufficient and faithful for `git_managed_project`;
  the grill then found the same shape in `git_project_with_version` and its
  Fossil twin, and the F6 correction brings all three into the declared class.
- **Consistency sweep** — the corrected rule is applied consistently across
  `design.md`, `tasks.md`, TC-34, §6.18, §7.4, §8.25, R-89 (with the R-87
  retirement note), the chapter-11 README index, and the change-level
  `arc42.md`; the grill's remaining stale wording (the glossary entry, the Q6
  supersession, the loose "both real destinations are managed blocks" phrasing)
  became F7, F8, and F11.
- **Scenario title wording** — Task 11's title resolves under option 2: the
  project's own managed ignore file stays in charge, so no rename and no
  five-place rename sweep were needed.
- **Risk/arc42** — R-89 needed the reproduction fact, the Fossil residual, and
  class-phrased guards (F9); chapter 12 needed the glossary widening (F7);
  chapter 10 needed nothing.
- **No re-opening** — the corrections touch only ignore-destination text; the
  interaction inventory, all `@e2e` titles, personas, routing, the D2–D6
  decisions, and Tasks 1–9's evidence stay as approved, and both corrected
  scenarios remain `@wip` with placeholder evidence.

**Nothing else re-opened.** The uncommitted correction is exactly ten files —
`design.md`, `tasks.md`, `arc42.md`, and the `docs/arc42/` chapters 02, 06, 07,
08, 11, 12 plus the chapter README — and every diff hunk sits in
ignore-destination text (the D1 ledger and record-model subsection, two
Architecture Impact rows, the tasks' copied contract paragraph and the Task
10/11 constraint blocks, and the corresponding chapter paragraphs).
`specs/**` including `usecase.md`, `proposal.md`, and the rest of the plan are
byte-identical to the approved revision; no Task 1–9 line appears in the diff;
both corrected scenarios are still `@wip` (`development-record.feature:75`,
`:82`) and Tasks 10–11 are still unchecked with only their evidence
placeholders.

**Checks run by this pass.**

- `givn lint --change development-timeline` → exit 2: 4 files checked, 32
  findings — the 32 `@wip` scenarios awaiting their tasks (including both
  corrected ones) — plus `[SUBST] advisory` permanent-corpus subset matches; no
  parse error and no error-kind finding.
- Consistency invariants re-checked: the use-case `§Interactions` inventory, the
  design's Interaction Coverage Matrix, and the `@e2e` titles still hold the
  same eight rows in the same order; the change's specs offer eight `@e2e`
  scenarios.

**Residuals noted by this pass.** None re-opens a question; no mechanism,
decision, or scenario changes.

- R-89 says "the refusal names the offending path". That holds for the archive
  refusal (`src/cli/archive.rs:423-440` — "archive blocked: the project has
  uncommitted work: <paths>") but not for the upgrade guard, whose message
  (`src/cli/migration.rs:357`) names no path. Reported for the design
  authority; wording only.
- The Fossil residual R-89 names is the deleted-managed-file case. The variant
  the grill recorded as option 2's residual — a managed
  `.fossil-settings/ignore-glob` that predates the record path, where a plain
  command therefore writes nothing and the store stays an `EXTRA` until an
  owning command (`givn skills sync` / `givn init`) runs — is not named; it is
  repairable, and no scenario depends on it.
- Task 10's pasted acceptance evidence names the two
  `declare-the-product-start` clean-checkout scenarios, both on
  `git_managed_project`; the F6-identified consumer of
  `git_project_with_version`
  (`upgrade-a-project/apply-a-managed-upgrade.feature::Repeating the upgrade
  does not duplicate the migration`) is not among the pasted commands. The
  fixture adaptation that fixes it is declared; a third pasted run would
  complete the class evidence.

**DESIGN-REVIEW: PASS**

## Re-review round 2 (2026-10-16, second correction: recorder engagement + test-side store exclusion)

**Why it ran.** With Tasks 10–11 landed, the implementation phase ran the
permanent non-`@e2e` scope and hit a second failure class, reproduced on
2026-10-16: **(1) 4 × `the directory "givn" does not exist`** — a refused
`givn init` in a bare root still wrote the store (`HistoryRecorder::finish`
ran unconditionally: `ensure_ignored`, then the append whose `create_dir_all`
created `givn/history/`), so the four refusal scenarios at
`givn/specs/initialize-project/initialize-a-project.feature:196,206,213,221`
(assertions at `:204,211,219,226`) failed; and **(2) 10 × project-state
comparisons** read the store as a project change — `modified:
givn/history/events.jsonl` — nine through `no file in the project changed`
(`fragments/detect-the-project-repository.feature:16,23,36,43`,
`archive-change/archive-proof.feature:202,211,221,230,240`) and one through
`no project file changed`
(`obtain-effective-guidance/serve-extended-rules.feature:47`). The design
authority applied the correction — the engagement rule and the test-side
exclusion — and a fresh-context grilling pass (high effort) interrogated it;
the grill returned two blockers and six correctness findings, all answerable
from the codebase, so no operator question arose. The round-4 fixes applied
every one of them; this hardening pass verified the applied text against all
fifteen grill branches and wrote this record (it wrote nothing else).

**The two circle-breaking fixes.**

1. **Recorder engagement (product).** The recorder engages **only when the
   project root carries the Givn scaffold (`givn/`)**; before the scaffold
   exists it writes nothing and creates no directory; a successful `givn init`
   creates the scaffold and then records; once the scaffold exists every
   command records, including refusals and failures; the rule keys on the
   scaffold, never on the VCS markers. Rejected alternatives: **narrowing the
   four permanent assertions** — they pin the init contract's own promise (a
   refused init leaves no Givn project behind), so the failure was the
   product's, not the tests'; **keying on the VCS markers or on a usable
   backend** — a scaffolded project without a usable backend still records
   (Task 11a's own constraint), and `givn init` runs in a bare directory by
   design, so repository presence cannot be the trigger; **gating at dispatch**
   (`records()` before `dispatch` in `src/main.rs`) instead of at write time —
   the successful `givn init` must record its own command, which only exists
   once the process has created the scaffold, and that is exactly what the new
   scenario's second half discriminates (grill A4).
2. **Path-based helper exclusion (test-side).** The project-state comparison
   helpers exclude the record directory **by path** —
   `path == root.join("givn/history")` — scoped to the record directory only.
   Rejected alternatives: **a bare-name skip** (`Some("history")`; the
   `collect_tree` skip list matches `entry.file_name()`, so it would also drop
   any unrelated `history` directory anywhere in a fixture tree, contradicting
   "no other path's behavior changes" — grill B3); **changing the scenarios,
   their fixtures, or their assertions** — the helpers' contract is "a command
   changed nothing", and the store is legitimate runtime state of every
   scaffolded command, so deleting it in fixtures or tolerating it in the
   assertions would hide the store instead of classifying it.

**Grill branch outcomes (fresh context, high effort).** All fifteen branches
walked; the grill's citations were re-verified against the working tree by this
pass.

| # | Branch | Outcome | Disposition |
|---|---|---|---|
| A1 | The four permanent `the directory "givn" does not exist` assertions | FIXED by the engagement rule (pre-rule they fail; under the rule the refused init leaves no scaffold and no store) | rule carried in design.md/TC-34/§8.25/arc42.md; Task 11a implements it |
| A2 | Other permanent scenarios that run commands without a `givn/` scaffold | INTACT — every scaffold-less refusal asserts exit code + stderr only; successful inits still scaffold and record; nothing depends on the removed bare-root writes | no change needed |
| A3 | The new delta scenario's executability with existing step vocabulary | FAIL (blocker 1) — the second Given (`a project root without a Git worktree or a Fossil checkout`) bound to the permanent `make_init_root_without_git` fixture, which scaffolds the root, so `the directory "givn" does not exist` could never hold; a second binding for the same phrase would be an AmbiguousMatch | **fix 1**: second Given removed |
| A4 | The new scenario's discrimination | PARTIAL DEVIATION — the first half discriminates (pre-fix the refusal leaves `givn/`); the second half exists to catch a dispatch-time predicate; Task 11a's stated RED failure named the wrong step | **fix 6**: RED expectation names the `the directory "givn" does not exist` assertion |
| A5 | Interaction with Task 10/11's ignore mechanism | GAP (blocker 2) — `finish` calls `ensure_ignored` first, and a backendless root's `.gitignore` write (`upsert_block` creates the file) would violate "writes nothing"; only the store path was gated | **fix 2**: predicate pinned at the top of `finish` |
| A6 | Count updates vs actual file contents | MIXED — five current statements read 34, the header needed the section-count qualification, and "Every `@givn.added` entry carries `@wip`" was false | **fix 3** |
| A7 | arc42 / TC-34 / §8.25 statements | CONSISTENT — both halves present; no contradictory wording in 05/06/11, ADR-073, proposal, or usecase | re-verified, no change needed |
| A8 | Superseded wording in design.md's Capture Architecture | ONE SENTENCE LEFT — the capture table row and the two-commands paragraph implied every non-read command writes | **fix 4** |
| B1 | Do the named helpers cover all 10 comparison failures? | YES, exactly — 9 through `scm_backend_selection_steps.rs::snapshot_tree`/`collect_tree` (`detect-the-project-repository:16,23,36,43`; `archive-proof:202,211,221,230,240`) + 1 through `serve_extended_rules_steps.rs::project_files`/`collect_project_files` (`serve-extended-rules:47`) | helpers named in design.md + Task 11a |
| B2 | Any other project-root walker that includes the store | NONE FOUND — all tree walks audited with their callers' roots; the out-of-scope ones stay untouched | out-of-scope walkers named in Task 11a |
| B3 | Exclusion scoping | INTENT YES, IMPLEMENTATION SHAPE RISKY — a name-based skip (`Some("history")`) invites collateral exclusion; the record-exact form is path-based | **fix 5** |
| B4 | Declared in the design | YES — Architecture Impact `tests/` row + the record-model subsection, echoed in Task 11a and arc42.md | re-verified |
| C1 | No re-opening: inventory, `@e2e` titles, personas, routing | INTACT — 8 inventory rows = 8 matrix rows = 8 `@e2e` titles; personas unchanged; proposal/usecase untouched | re-verified |
| C2 | Tasks 1–11 evidence/hashes, Task 11a shape | AS DECLARED — each of Tasks 1–11 is 5/5 checked and all eleven revision identifiers resolve to commits; Task 11a is 5/5 unchecked with placeholder evidence; `@wip` = 31 pending sections; minor: the header's "Every `@givn.added` entry carries `@wip`" wording | **fix 3** ("still-pending entry" wording) |
| C3 | Other permanent-scope failure class | NONE FOUND beyond the 4 + 10 split (by helper/caller inventory; no suite was re-run) | no change needed |

**Round-4 fixes verified against the working tree.**

| # | Fix | Verified at |
|---|---|---|
| 1 | The scenario's second Given removed — the bare root already carries no VCS markers (binding `tests/steps/mod.rs:288` → `tests/common/mod.rs::make_bare_root`, "no `givn/` subtree", no `.git` created) | `development-record.feature:89-99` |
| 2 | The engagement predicate is pinned **at the top of `HistoryRecorder::finish`, before `ensure_ignored` and before the store path is created or opened** — both the ignore re-assert and the append are gated, including the `.gitignore` fallback a backendless root would otherwise receive | `tasks.md:2849-2856` |
| 3 | Delta run counts read 35 everywhere a count is stated — `:24`, `:36`, `:224`, `:3972`, `:3976`, `:4062`, `:4477`, and `arc42.md:98-99` ("Delta inventory is now 43 entries (35 non-`@e2e`, 8 `@e2e`)"); the header reads "the 34 non-`@e2e` scenario sections" with the "(the one `@givn.modified` entry is reconciled in S2b, not as a scenario task)" clause (`:3-5`), and the work order reconciles 34 sections / 35 scenarios (`:40-43`); "Every still-pending entry carries `@wip`" replaces the false `@givn.added` wording (`:26`) | as listed |
| 4 | The Capture Architecture table row and the two-commands paragraph both carry the scaffold qualification/pointer ("subject to the engagement rule …: nothing is emitted or created before the project carries the Givn scaffold"; "within the engagement rule of the record model section") | `design.md:404`, `:412-420` |
| 5 | The helper exclusion is path-based in the task text — `path == root.join("givn/history")`, explicitly "not by bare entry name", with the `entry.file_name()` collateral called out for `collect_tree` and the existing path form cited for `collect_project_files`; the out-of-scope walkers are named | `tasks.md:2862-2886` |
| 6 | Task 11a's RED expectation names the `the directory "givn" does not exist` assertion as the failure point and leaves undefined-step reporting to the pasted output ("paste whichever output the run actually produces; the run must exit non-zero either way") | `tasks.md:2887-2893` |

**Round-3 rules verified as written (this pass).**

- **Engagement rule** — design.md §Record Model → *When the recorder engages*
  (`:179-190`): scaffold-keyed, writes nothing / creates no directory before
  the scaffold, `givn init` scaffolds then records, scaffolded projects record
  refusals and failures; TC-34 (`docs/arc42/02-architecture-constraints.md:34`)
  carries the same rule ("engages only when … writes nothing and creates no
  directory … the rule keys on the scaffold, not on the VCS markers"); §8.25
  *Engagement* (`docs/arc42/08-crosscutting-concepts.md:927-934`); the
  change-level `arc42.md` rows 2 and 8 (`:15`, `:21`) and the pass note
  (`:87-100`). The `givn init` clause lives in design.md, §8.25, and Task 11a
  (`:2840-2842`); TC-34 and the arc42 rows carry the engagement and
  scaffold-not-VCS clauses.
- **The new scenario** — `development-record.feature:89-99` carries exactly
  the round-4 text (single Given `a project root without a givn/ directory`;
  refused `givn init`; `the directory "givn" does not exist`; `holds no
  development record`; successful `givn init --no-addons --agents opencode`;
  `the directory "givn" exists`; `holds a development record`), tagged
  `@givn.added @wip`. No binding hazard: literal step attributes are anchored
  (`^…$`, `cucumber-codegen-0.23.0/src/attribute.rs:418`), so the bare-root
  Given cannot collide with the longer Git-repository Given;
  both When steps match the one generic binding at
  `tests/steps/init_agent_target_selection_steps.rs:193` (the literal phrase
  `the consumer runs` plus one backquoted command captured as a whole);
  the exit steps match `tests/steps/mod.rs:503/:542`; `the directory "givn"
  does not exist` reuses the existing generic binding (`tests/steps/mod.rs:928`)
  — a second binding for that phrase would be an AmbiguousMatch — while `the
  directory "givn" exists` and the two `holds … record` steps have no binding
  today and are Task 11a's new steps.
- **Test-side exclusion declared** — design.md → *Test-side exclusion of the
  store* (`:192-204`) and the Architecture Impact `tests/` row (`:1039`);
  §8.25 *Runtime state, not project content* (`:935-940`); Task 11a
  (`:2856-2886`); `arc42.md` note (`:94-98`).

**Nothing else re-opened.** The uncommitted correction spans exactly six files
(`design.md`, `tasks.md`, `arc42.md`,
`docs/arc42/02-architecture-constraints.md`,
`docs/arc42/08-crosscutting-concepts.md`, the delta `development-record.feature`)
— +182/−23 lines, every hunk in the two corrected rules' text. No tasks.md hunk
touches Tasks 1–11 (the hunks are the header/inventory/work order, the scope
line, the Task 11a insertion, E1's count proof, sub-task 5's paste instruction,
and Done-when). `proposal.md`, `usecase.md`, and the other two delta features
are byte-identical to the reviewed revision; the interaction inventory, the
Interaction Coverage Matrix, and the `@e2e` titles still hold the same eight
rows in the same order; personas and routing are unchanged.

**Checks run by this pass.**

- `givn lint --change development-timeline` → exit 2: 4 files checked, 31
  findings — the 31 `@wip` scenarios awaiting their tasks (23 non-`@e2e`
  incl. Task 11a + the 8 `@e2e`) — plus `[SUBST] advisory` permanent-corpus
  subset matches; no parse error and no error-kind finding.
- Mechanical recount from the four delta features: 43 entries = 42
  `@givn.added` + 1 `@givn.modified`, 0 `@givn.removed`; 35 non-`@e2e` + 8
  `@e2e` (14/12/2, 14/13/1, 14/9/5, 1/1/0); `@wip` = 31 = the 31 pending
  scenario sections; Tasks 1–11 are 5/5 checked and Task 11a 0/5; every one of
  the eleven recorded revision identifiers resolves to a commit
  (`git cat-file -t`).
- Consistency invariants re-checked: 8 §Interactions rows = 8 matrix rows = 8
  `@e2e` titles; every current run-count statement reads 35 (S1's and S2b's
  pasted listings — 41/33/8 and 42/34/8 — are dated evidence from before Task
  11a's scenario existed, and E1's pending proof will paste 35 vs 8).
- No suite was run (read-only scope): the 4 + 10 failure accounting is the
  implementation phase's own observation, and the helper coverage is the
  grill's caller/root inventory re-checked by this pass.

**Residuals noted by this pass.** None re-opens a question; no mechanism,
decision, scenario, or count changes.

- `design.md:404` reads "subject to the engagement rule below", while the
  named subsection (`### When the recorder engages`, `:179-190`) sits above the
  Capture Architecture section; the row's colon carries the rule content and
  the paragraph below points correctly at "the engagement rule of the record
  model section", so nothing depends on the pointer — wording only, for the
  design authority.
- Task 11a's RED note names the `the project root holds no development record`
  / `holds a development record` pair as the undefined phrasing; `the
  directory "givn" exists` (`development-record.feature:98`) is unbound too and
  will appear in the same undefined-step report — the instruction's "paste
  whichever output the run actually produces" already covers it; wording only.
- S1's and S2b's pasted listings predate Task 11a's scenario, so they read
  41/33/8 and 42/34/8 against the current 43/35/8 inventory; they are dated
  evidence and no current statement is stale — a dated one-line note in S1/S2b
  would remove the reader's double-take, if the design authority wants one.

**DESIGN-REVIEW: PASS**

## Re-review round 3 (2026-10-16, Maki extension: fourth harness reporter)

**Why it ran.** With Tasks 1–24, Task 11a, and E1–E9 committed and the two prior
design-review sign-offs recorded, the operator extended the change to a fourth
supported harness: **Maki 0.6.3**, the one target whose reporter is a Lua plugin
written in the harness's own language, with no attribution asset to sit beside.
The design authority updated 18 artifacts (this change's `proposal.md`,
`design.md`, `arc42.md`, delta feature and `tasks.md`, plus `docs/arc42/02`,
`03`, `05`, `06`, `07`, `08`, `09`, `10`, `11`, `12` and `README.md`, ADR-013 and
the ADR register — +529/−96 lines; no code, test, asset, config, or
`design-review.md` edit). A fresh-context grill (high effort; no suite run; every
probe in `/tmp/maki-load` and `/tmp/maki-drive`) interrogated the extension
across seven verification items and eleven branches and returned **nine blockers
(B1–B9) and thirteen non-blocking findings (M1–M13)**. Every one was answerable
from the installed binary, the Lua API reference, or the codebase, so the four
decisions the grill framed (D1–D4) were settled on reproduced evidence and no
operator question arose. The correction round applied the nine blocker fixes and
the four decisions; this hardening pass re-read each applied correction against
the working tree, walked the grill's branches again, and wrote this record (it
wrote nothing else).

**The grill's reproduced findings, re-read by this pass (the probes were not
re-run — read-only scope).**

- **The exit-code clarification.** `maki --trust prompt` exits 0 and loads a
  healthy project plugin in every stdio shape the grill tested (stdout and
  stderr to files, `stdin </dev/null`, stdin closed, stdout discarded, inherited
  pipes); exit 1 is the **init.lua-raise symptom** (`✖ load init.lua files` plus
  the Lua traceback), not a redirection effect. That is what ties the load
  proof's exit-0 assertion to the fence correction: the assertion can only hold
  once the block stops raising. The same probe found no model or network call in
  the `prompt` path (`strace` shows no `connect`/`socket`; the Maki log grew 0
  bytes; ≈0.6 s wall time), and `maki --trust prompt -p …` is rejected by the
  CLI (exit 2, `unexpected argument '-p'`) exactly as recorded; a real turn
  (`maki --trust -p …`) stays out of the suite.
- **The crash shapes, and the working driver.** Firing
  `maki.api.exec_autocmds` from a *required module's* body aborts Maki (exit 134,
  `Luau VM is suspended while GC is running`), and any `maki.*` asynchronous
  call at module top level breaks `pcall(require)`
  (`attempt to yield across metamethod/C-call boundary`); a job left unawaited
  in a task scope is lost, while one awaited with `jobwait` inside the callback
  that owns it always lands. The working shape was reproduced end to end: a
  synthetic `ToolDone` dispatched the reporter's real callback, whose flush ran
  `cat <payload> | <givn> history ingest` and appeared in
  `givn history --change demo --json`; the same held for a real `-p` turn's
  `SessionEnd`.
- **The mapping-table verification — no issue found.** All 18 host events are
  accounted for (12 recorded rows plus the 6 excluded bookkeeping events), every
  field list matches the Lua API reference, the privacy rows hold
  (`data.text`/`data.input`/`data.summary` are never stored), no event carries a
  timestamp while only `ToolDone` supplies a host-measured duration, and the
  reporter's own stamps are named (`os.clock()`, design.md:741).
- **The install-path/gates findings (B1–B4).** The hyphenated module name is
  fine (`require("givn-timeline")` resolves to `.maki/lua/givn-timeline.lua`);
  absent files are created; a malformed `init.lua` logs its error and the block
  never runs. The four blockers were exactly: the raising block (B1), the false
  grants bound (B2), the clobbering install (B3), and the unstated
  ownership/local-only membership (B4). One branch finding remains unrecorded:
  a malformed `.maki/plugin.toml` denies everything with a warning, so the
  reporter is silently inert — Maki's own semantics (see residual 6).
- **The four settled decisions** (no operator question arose; each was settled
  from the reproduced evidence, with its rejected alternatives):
  - **D1 — `maki` is a full target**: skills to `.maki/skills/<name>/SKILL.md`
    and commands to `.maki/commands/<name>.md` (Markdown with optional `---`
    frontmatter and `$ARGUMENTS`), so a `maki`-only selection is equipped.
    Rejected: keeping the "no skills write" choice and documenting the gap; and
    a reporter-only target "unusable without another guidance-owning target".
  - **D2 — `.maki/plugin.toml` is merged, never clobbered** (the
    `merge_claude_settings` precedent): the three grants are added idempotently,
    a user key is never removed or flipped, surrounding content is untouched,
    and the Givn-created file names all six keys explicitly. Rejected: writing
    the asset over the user's file with a warning; and refusing and telling the
    user which grants to add.
  - **D3 — a subagent entry whose harness cannot report its tokens carries
    `data.usage = "unavailable"`** in the intake contract, so the record itself
    carries the asymmetry the new scenario names. Rejected: restating the step
    against the report's "unavailable" row or the absence of a usage entry.
  - **D4 — the pinned `maki` v0.6.3 binary joins the setup-script family and
    the CI/release prerequisites**, and the `@e2e` row stays strictly required.
    Rejected: keeping the row required only where the binary is guaranteed (a
    skip would violate `fail_on_skipped` and hide the very drift R-90 records);
    and moving the row to a separately scoped, operator-run tier.

**The applied corrections, verified against the working tree.**

| # | Correction | Verified at |
|---|---|---|
| D1 | `maki` is a full target — skills to `.maki/skills/<name>/SKILL.md`, commands to `.maki/commands/<name>.md` ("the same shape Givn already renders…, so `.maki/commands/<name>.md` needs no new template"), and its guidance "is not a substitute for writing it"; every required enumeration names Maki | design.md:437-457, :464, :627-631; `proposal.md:64-76`; TC-36 (`docs/arc42/02:36`); `docs/arc42/03:33,:64`; `05:84`; `07:75-78`; `08:892-912`; `09:64-74`; `10:103`; `12:180-181`; `ADR-013:69-90`; `adr/README:70`; change `arc42.md` rows 2/3/5/7/8/9/11/12 and the Maki note; the code-side enumerations are named as implementation work in Task 24a (`:5718-5724`; GREEN `:5798-5806` names `README.md`, `src/cli/init.rs`, `src/config/overlay.rs::KNOWN_AGENT_TARGETS`, `src/main.rs` (the `--agents` help)) |
| D2 | `.maki/plugin.toml` is merged, never clobbered; the created file names all six keys (`fs_read`/`fs_write`/`run` true, `net`/`env`/`agent` false); the "no net/env/agent bound" claim is gone from every artifact it lived in, replaced by the code's own call surface | design.md:650-659; Task 24a:5734-5737; `docs/arc42/02:36`; `ADR-013:75-76,:86-89` — no `net`/`env`/`agent` bound statement remains, and design.md:657-659 states the design does not claim a bound it cannot enforce |
| B1 | the fence exists and its body is exactly the safe form, with the rejection rationale stated | design.md:633-647 ("`pcall(require, "givn-timeline").setup()` is **wrong** and was rejected after reproduction"); Task 24a:5740-5749; `arc42.md:131-132`; the only surviving occurrences of the wrong form are its rejection statements |
| B4 | managed-generated vs merged user-owned membership and its consequences (allowlist staging, reconcile removal, `local-only` ignore entries and `givn commit`'s stage-all exclusion, no silent strict rule; merged files never force-staged, removed, or rewritten) | design.md:669-680, :1264-1265; Task 24a:5728-5733 — with residual 1 (`is_rule_path` not named) |
| D3 | the intake rule carries `data.usage = "unavailable"` for a subagent entry the harness cannot measure, mirrored in the gap-honesty row, the autocmd mapping row, chapter 06's capture row, Task 24a and the scenario | design.md:397-402, :53, :705; `docs/arc42/06:547`; Task 24a:5748-5761; `agent-activity-reporting.feature:107-115` |
| D4 | the pinned `maki-v0.6.3-x86_64-unknown-linux-musl.tar.gz` (release URL, checksum-verified, local bin dir) joins the setup-script family, the test-infrastructure paragraph, Task 24a/E1 and the CI/release requirements; the row is strictly required and never skips | design.md:750-757, :973, :986-990, :1046-1053, :1270; Task 24a:5781-5787, `:5798-5806`; `tasks.md:7529-7535`; Done-when `:9852-9858` |
| B6/B8 | the driver shape is exactly the reproduced one and the two files are named: the **test-written** `.maki/init.lua` calls `setup()`/`run()` at top level; the module body stays free of `maki.*` async calls; starts are `os.clock()`; the flush is the payload file plus `maki.fn.jobstart({ "sh", "-c", "cat <payload-file> | <givn> history ingest" })` awaited by `maki.fn.jobwait(id, <timeout>)` inside the owning callback; no `maki.fn.system`/stdin claim survives | design.md:731-748, :770-790, :1057-1060; Task 24a:5762-5779; `tests/steps/agent_activity_reporting_e2e_steps.rs:137-142` panics on `maki` today, which is why its table is named to gain the row |

**Minor findings (M1–M13), re-checked.** Applied: M1 ("All four deliver",
design.md:471), M3 (the overview's Maki exception with "no attribution asset",
design.md:16-20), M10 (Task 24a's RED note names
all **eight** phrases — "one Given, three `When` … and four `Then`/`And`",
`tasks.md:5788-5790`), M11 (the file list above), M12 (Maki as a full target, so
the existing fan-out writes its skills and commands), M13 (`os.clock()` named).
M4 is applied only in part: the proposal's bullet drops the
"commit-attribution" qualifier (`proposal.md:69` now reads "beside the
integration it already gets"), which stays loose for Maki — it receives no
pre-existing integration — and rests on the following sentence's guidance
statement; M5's phrase ("installed beside that harness's other integration")
now sits beside ch.08's Maki paragraph, which states the exception.
Counts verified mechanically: `tasks.md:4` ("the 35 non-`@e2e` scenario
sections"), `:22-26` (44 = 43 `@givn.added` + 1 `@givn.modified`; 36 + 8),
`:40-43` (Tasks 1–35 incl. Task 24a), `:224` (36 / 35), `:7509-7514` (36 vs 8),
`:9855` (Done-when 36 vs 8); E1's historical 35-vs-8 evidence is intact
(`tasks.md:7861-7872`, `43 in total`) and its dated pasted CI diffs
(`:7900-7939`) name no Maki, as they predate the extension. Not applied — M2,
M6, M7, M8, and M4 only in part — see residuals 2–5.

**Residuals noted by this pass.** None is a question; none changes a mechanism,
decision, scenario, count, or gate. The first two name work the Task 24a
implementation must also touch; the rest are stale sentences or an omitted
diagram node, recorded for the design authority's sweep.

1. **Unnamed rule membership and addon removal.** `src/skills/rules.rs::is_rule_path`
   (with the `tracked_rule_paths` discovery it feeds) is not named in `design.md`
   or Task 24a for the new paths, although its sibling `ignore_entries()` is
   (design.md:1265; Task 24a:5721-5723) and the existing three-reporter pattern
   names both (`tasks.md:5106`). Likewise `src/cli/addons.rs:407` (skill addons)
   and `:421` (command addons) keep hard-coded target lists, so `givn addons
   disable <addon>` on a `maki` project would leave `.maki/skills/<addon>` and
   `.maki/commands/<addon>.md` behind (and print a three-directory message); the
   design's reconcile sentence (design.md:672-673) covers the sync/upgrade path
   (`reconcile_addon_artifacts` fans out over the configured targets), not this
   command's path.
2. **The driver-count sentence was not updated (M2).** `design.md:903-909`
   (§Step Definitions) and its copy in `tasks.md:233-238` (Global constraints)
   still read "the three harness drivers — a fake `pi` … for omp, … for
   opencode, and … the generated Claude shim", while the same design places the
   Maki driver in that file (design.md:1057; Task 24a:5773-5778). Task S3's own
   body (`tasks.md:690`, `:716`) is dated text for the skeleton it created and is
   internally consistent.
3. **Chapter 06's lead-in contradicts its own table.** `docs/arc42/06:538-539`
   reads "Three reporters feed that intake, one per supported harness, and each
   is installed beside the commit-attribution integration that harness already
   gets" directly above the four-row table it introduces (the Maki row is
   `:547`), and Maki has no attribution asset (design.md:456). One sentence
   fixes it: "four", plus Maki's exception.
4. **M6/M7/M8 unapplied.** `docs/arc42/11:71` (R-84) still reads "The three
   generated reporters … the automated E2E fakes each harness context, so drift
   is not detected by the suite" — Maki is the fourth, its own e2e runs the real
   binary, and R-90 (`:76`) carries that drift; `docs/arc42/adr/ADR-073:111-115`
   still reads "the same boundary now carries three reporters — omp, opencode,
   and Claude Code" (the extension amended only ADR-013); and `arc42.md:98-99`'s
   dated tally "Delta inventory is now 43 entries (35 non-`@e2e`, 8 `@e2e`)"
   predates Task 24a's scenario, which the appended Maki note does not recount.
5. **Smaller stale text from the same sweep.** The Architecture Impact mermaid
   (`design.md:1237-1242`) still draws three harness reporters (no Maki node);
   the "Shared wiring" paragraph (`design.md:466-470`) claims "every asset is
   copied verbatim … added to `managed_generated_paths` … listed in
   `ignore_entries()`", which does not hold for the Maki grants asset the same
   table marks "(merged)" (`:464`, `:650-659`); the traceability clause at
   `design.md:51` ("Maki needs no new guidance location because it reads the
   existing skill and command paths") can be read as the refuted "no guidance
   write" rationale, worth one clarifying word now that §Maki (:446-456) states
   the opposite construction; and the proposal's generic "beside the integration
   it already gets" (`proposal.md:69`) is loose for Maki, which receives no
   pre-existing integration.
6. **A malformed `.maki/plugin.toml` is not recorded.** The grill's branch-3
   finding stands: Maki denies every permission with a warning for a file it
   cannot parse, so the reporter is silently inert — the same class as the
   folder-trust gate (design.md:682-690, R-90) and worth a one-line treatment.
7. **Commit-slot wording.** `tasks.md:7534-7535` ("Commit those workflow edits
   with the e2e setup, not with a scenario") now covers the Maki provisioning and
   workflow edits too, while Task 24a lists the setup script and the CI/release
   prerequisites among its own GREEN files (`:5798-5806`) and its COMMIT is the
   scenario commit (`:5818-5821`); E1's setup commit has landed
   (`1f03e66ed7db83f8082a0427e2b147129bf61e77`), so the sentence and Task 24a give
   the same edits two homes.

**Checks run by this pass.**

- `givn lint --change development-timeline` → exit 2: 4 files checked, **1
  finding** — the new scenario's `@wip` (the only `@wip` left; every other delta
  scenario's tag has been removed by its committed task) — plus the pre-existing
  `[SUBST] advisory` permanent-corpus subset matches; no parse error and no
  error-kind finding.
- Mechanical recount from the four delta features: **44** scenario entries
  (43 `@givn.added` + 1 `@givn.modified`, 0 `@givn.removed`) = 14 + 15 + 14 + 1;
  **36** non-`@e2e` (12/14/9/1) + **8** `@e2e` (2/1/5/0); `@wip` = 1.
- Invariants re-checked: the use case's §Interactions table (8 rows) = the
  design's Interaction Coverage Matrix (8 rows) = the 8 `@e2e` titles; Task 24a
  carries the canonical `--exact` command (`:5792`) and is 0/5 unchecked
  while Tasks 1–24, 11a and E1–E9 are checked with their evidence and revision
  identifiers intact.
- Diff scope: 18 modified files (+529/−96), 13 under `docs/arc42/**` and 5 in
  the change directory; 0 untracked; no code, test, asset, or config change;
  `evidence/visual/` untouched; `usecase.md` and the other three delta features
  byte-identical to the reviewed revision; `proposal.md` touched only in its
  harness bullet and its out-of-scope bullet; `tasks.md` hunks only in the
  header inventory/work order, the scope-count line, Task 24a, E1's count and CI
  lines, and the Done-when line; the two prior records in this file are
  untouched.
- No suite was run (read-only scope): the reproduced probe results above are the
  grill's own evidence, re-read against the artifacts; the mapping table was not
  re-fetched, and no Maki command was re-executed.

**DESIGN-REVIEW: PASS**

## Re-review round 4 (2026-10-17, ranking widening + Maki headless gaps)

**Why it ran.** The operator extended the change after the implementation
landed: the report's ranking must cover **every measured entry of any kind**
(the operator asked to see "single calls to subagents, what took the time" after
this repository's own report rendered an empty "Longest steps" list while its
model held seventy-five measured entries), and the Maki reporter's headless gaps
must be recorded at the strength of the evidence. The design authority updated
the planning layer in place — `design.md`, `arc42.md`, `tasks.md`, the delta
feature `development-report.feature`, and `docs/arc42/06`, `08`, `10`, `11` —
including the new delta scenario `Measured agent activity is ranked
alongside Givn's own steps` (Task 25a). A fresh-context grill (high effort,
read-only; no suite run) interrogated the extension across the seven branches
below and returned **no blocker and no operator question** plus seven
non-blocking findings (F1–F7). This hardening pass applied every finding to the
working tree and wrote this record; it re-probed nothing — the reproduced facts
below are the grill's own evidence, re-read read-only against the live log, the
store, and the report.

**The grill's branch outcomes.**

| Branch | Outcome |
|---|---|
| 1a Widened rule as recorded | SOUND — the rule states every measured entry of any kind, longest first, with the existing tie-break (earlier `at`, then lower `id`), over the whole record, kind per row, unmeasured absent rather than zero, heading kept; it matches the code's tie-break and the preserved aggregation rule (`design.md:898-914`, `:1005-1006`, `:1319-1320`). |
| 1b Model/asset change named concretely | YES, with F4 — the `kind` field and the row element are named in the design and in Task 25a, but the task located the row markup in the asset; `render_summary` emits it and the asset holds only the CSS. |
| 1c Motivation verified against the live artifacts | VERIFIED — the report written 2026-10-17 03:02 has `summary.longest_steps` = 0 rows while its model window holds 75 measured entries (id 70 `command` `skills sync` 3604 ms, id 76 tool `task` 799 ms); no value is invented. |
| 2a Bindable with the existing vocabulary | YES — the new Given is built with the shared seeded-history builder, the When already exists, and only the two Thens are new step phrases. |
| 2b The new scenario discriminates | YES — with today's `kind == "step"` filter a `tool` entry can never rank and neither the model nor the row markup carries a kind, so both Thens fail with no production change; Task 25a's undefined-step prediction was right in kind but undercounted the phrases (F3). |
| 2c The pre-existing ranking scenario under the widening | NO BLOCKER — its fixture holds only steps (3 measured plus 1 untimed tool), so the widened ranking equals the Then's expected ids and the "not measured" text stays true. |
| 3a Other consumers of the ranking's membership | NO ISSUE — only `render_summary` reads `summary.longest_steps`; no test asserts a ranking count or absence beyond the two ranking scenarios, the `@e2e` longest-measured reader is duration-based, and the permanent corpus has zero references. |
| 3b Asset/JS dependency | NO ISSUE — the asset's scripts never read the ranking (only its CSS is ranking-related), so the model change alters no script behaviour. |
| 3c Evidence rules vs. the changed asset | NO CONTENT DEPENDENCY, with F7(b) — the visual gate checks presence, not content, and the archive gate re-captures and finalizes evidence-only dirt; but every committed `@e2e` capture that renders the report changes visibly, and Task 25a's `--exact` runs would not refresh them. |
| 4 Counts and per-feature rows | MATCH — the 47-entry inventory (44 `@givn.added` + 3 `@givn.modified`; 39 non-`@e2e` + 8 `@e2e`; per feature 14/15/15/1/2) reconciles with the header, the table, the work order, the scope line, E1's proof instruction, and the Done-when. |
| 4b E1's historical evidence and prior counts | UNTOUCHED — E1's pasted "43 in total" proof and Task 24a's dated tally remain exactly as committed; they are dated evidence. |
| 4c Delta shape/lint | OK — lint exits 2 with exactly the one pending `@wip` (Task 25a's scenario) and the pre-existing `[SUBST] advisory` permanent-corpus subset matches. |
| 5a Maki's reproduced values | EXACT, NO INVENTION — the harness's own log and the live record carry the values below, and `nil, "no interactive UI attached"` is the documented `maki.task.*` return without a UI. |
| 5b `TurnStart`/`TurnEnd` headless facts | BACKED — the bundled 0.6.3 reference states both clauses, and the record's turn entries carry no duration while their usage entries exist. |
| 5c The "no `TaskStatusChanged` outside the TUI" claim | TENSION → F1 — the effect is reproduced, but the causal/TUI-only wording is stronger than the evidence and contradicts the harness's own reference ("Everything else fires under `maki -p` and sdk mode as well"). |
| 5d Reproduction date | WRONG BY ONE DAY → F2 — the evidence is 2026-10-17, not 2026-10-16. |
| 6a Chapters 06/08/10 wording matches the rule | YES — the durable docs carry the widened rule; no stale steps-only wording survives there. |
| 6b Heading stays / design system unchanged | ACCURATE — the design system has no contract line for the ranking list, and the visible heading stays "Longest steps". |
| 6c Stale-wording greps | CLEAN — `kind == "step"` survives only in the rejection statements, the code and tests to be widened, and Task 25a. |
| 7 Nothing reviewed is re-opened | CONFIRMED — the extension diff touches only the change artifacts and the four chapters; no code, test, asset, or config edit; every revision identifier, checkbox, and pasted evidence of Tasks 1–24a, 11a and E1–E9 is intact. |

**The reproduced facts this round turned on (read-only re-read).**

- **The empty ranking, on this repository's own data.** `ranked_steps` filtered
  `entry.kind == STEP`, so
  `givn/history/reports/development-timeline.html` (written 2026-10-17 03:02)
  carried `summary.longest_steps` = 0 rows while its model window held **75
  measured entries**, including record id 70 `command` `skills sync` 3604 ms and
  id 76 tool `task` 799 ms; the window holds no `subagent` entry.
- **Maki's headless subagent run (2026-10-17T01:01Z).** `maki.log`
  `01:01:40.533Z` `subagent session opened` `name "State the number 42"`, and
  `01:01:41.328Z` `subagent session closed` `duration_ms 795`,
  `input_tokens 8512`; the live record's ids 73–79 (2026-10-17T01:01Z) carry the
  `task` tool entry (id 76, `duration_ms 799`) and **no subagent span**;
  `maki.task.list()` outside the TUI answers `nil, "no interactive UI
  attached"`.
- **The harness's own driver statement** (the pinned 0.6.3 binary's bundled
  reference): "`TurnStart` and `PlanReady` come from maki-ui only. … Everything
  else fires under `maki -p` and sdk mode as well." and "`TurnEnd` fires once
  per turn and only for the main session." — so a TUI-only rule for
  `TaskStatusChanged` cannot be stated, and the turn-with-usage/no-duration
  observation is the reference's own consequence.

**Findings F1–F7 and their applied resolutions.**

| # | Finding | Applied resolution |
|---|---|---|
| F1 (medium) | The `TaskStatusChanged` wording overclaimed an event as TUI-only / absent headlessly in four places, against the harness's own reference. | All occurrences now state only the reproduced fact — in the headless run the record carried the `task` tool entry (799 ms) and no subagent span, although Maki's own log shows the subagent session opening and closing (`duration_ms 795`, `input_tokens 8512`), and `maki.task.list()` returns `nil, "no interactive UI attached"` — and say the event's coverage outside the TUI is **not established**, naming the reference's actual clause. The mapping row's label became **Headless coverage not established**; the gap bullet became "Headless Maki's subagent span is not reproduced, and the event's coverage outside the TUI is not established"; the parity sentence now says the *reproduced headless run* contributed measured tool calls and turn usage only; and the `TurnEnd` bullet's clause reads "the subagent span, when the harness reports it, is still recorded". Applied at `design.md:716` and `:732-750`, `arc42.md:147-158`, `docs/arc42/08:907-915`, and R-90 (`docs/arc42/11:76`). |
| F2 (low) | The reproduction was dated 2026-10-16 where the evidence is 2026-10-17. | Re-dated to 2026-10-17 at `design.md:733`, `arc42.md:136`, and R-90. The other 2026-10-16 dates (version verifications, earlier probe projects) are correct and untouched. |
| F3 (low) | Task 25a's RED prediction said "the two new Then steps are undefined". | Now "the three new step phrases (one Given, two Then/And) are undefined" (`tasks.md:6306-6307`). |
| F4 (low) | Task 25a located the ranking row in the asset, and its touch list omitted the stale doc comments. | The Files bullet now reads: `src/cli/timeline.rs` — the widened filter, the `kind` field, the row markup in `render_summary` (`~:1178-1186`), and the doc comments that still describe a steps-only ranking (`RankedStep` `~:362-364`, `ranked_steps` `~:711-715`, `Summary.longest_steps` `~:158-159`) — and `assets/timeline/report.html`, which gains only the kind element's **style rule** (the asset carries the CSS; the element is rendered by `render_summary`). The JSON field name stays `longest_steps`, stated in the Assertions bullet (`tasks.md:6298-6304`, `:6283-6289`). |
| F5 (low) | Task 25a stated no fixture shape, and its second Then could have stopped at the model. | The constraints now state the fixture shape (a measured agent **tool** call longer than the longest measured `step`, built with the shared seeded-history builder) and require the second Then to read the **rendered** ranking through the existing `list_entry_ids(&document, "longest-steps")` parser, so the visible kind element is exercised (`tasks.md:6269-6275`, `:6283-6289`). |
| F6 (low) | The copied contract in `tasks.md` still said "ranked longest steps". | Now "the ranked longest entries", matching `design.md:895`; the visible heading keeps "Longest steps" (`tasks.md:207`). |
| F7 (awareness) | (a) The ranking is unbounded by construction and no assertion covers or caps it; (b) Task 25a would leave the committed `@e2e` captures stale by content. | (a) One sentence added to Task 25a's constraints: the written list is uncapped **by design** — bounded only by the record's own measured entries, because the design bounds the drawn shapes, never the summary rows (`tasks.md:6294-6297`). (b) Task 25a's GREEN step now also re-runs the `@e2e` scope and commits the refreshed `evidence/visual/**` pairs, with Task 24a's precedent named, so no committed pair stays stale by content into the archive finalization (`tasks.md:6314-6323`). |

**One same-class correction beyond the seven findings.** The Turn-events bullet's
ACP clause also claimed more than its source: it read "a headless or ACP session
delivers `TurnEnd` without `TurnStart`", while the same reference paragraph F1
cites says `maki-acp` "runs the agent on its own loop and does not call the
dispatcher yet, so plugins loaded under ACP receive no turn events". The bullet
now reads "`maki-acp` runs the agent on its own loop and does not call the
dispatcher yet (so plugins loaded under ACP receive no turn events at all),
while a headless session delivers `TurnEnd` without `TurnStart`"
(`design.md:751-759`). Nothing else in the extension was re-worded beyond
F1–F7.

**Checks run by this pass.**

- `givn lint --change development-timeline` → exit 2: 5 files checked, **1
  finding** — the pending Task 25a scenario's `@wip` (the state the header
  predicts) — plus the pre-existing `[SUBST] advisory` permanent-corpus subset
  matches; no parse error and no error-kind finding.
- Diff scope: the extension plus this record is nine files — `design.md`,
  `tasks.md`, `arc42.md`, the delta feature, `docs/arc42/06`, `08`, `10`, `11`,
  and `design-review.md` — with no code, test, asset, config, or
  `evidence/visual/` edit; the five delta features' tags, the use case's eight
  §Interactions rows, the design's eight matrix rows, and the eight `@e2e`
  titles are unchanged; Tasks 1–24a, 11a and E1–E9 keep every checkbox, pasted
  evidence, and revision identifier, and the three prior records in this file
  are untouched.
- No suite was run (read-only scope): the reproduced values above were re-read
  from `maki.log`, the live store, and the written report; no Maki command was
  re-executed.

**Residuals noted by this pass.** One, and it is dated evidence rather than a
question; it changes no mechanism, decision, scenario, count, or gate.

- Task 24a's COMMIT note reads "12 screenshot pairs" where its own commit
  refreshed twelve `evidence/visual/**` files (six desktop/mobile pairs). No
  current statement depends on the count, and Task 25a's amended GREEN text
  states the evidence set without restating a number.

**DESIGN-REVIEW: PASS**

## Re-review round 5 (2026-10-17, capture completeness: record mode, metadata, full content)

**Why it ran.** After the implementation landed, the operator extended the
change with four record-side requirements: a durable three-state capture mode
(`record.mode: on | off | full`), default metadata rich enough for a review,
always-captured full question text, and complete LLM-facing content in `full`.
A fresh-context grill (high effort, read-only; no suite run) interrogated the
planning layer across the nine branches below and returned four operator-level
questions (Q1–Q4) and twelve findings (F1–F12), with no blocker. The operator
settled the four questions; this hardening pass applied every decision and
finding to `design.md`, the delta features, `tasks.md`, the change-level
`arc42.md`, the affected `docs/arc42/` chapters, and the config scaffold
(`assets/config.yaml.tmpl`, the scaffold documentation). No code, test, or
asset changed, and no suite was run. The record below states what changed and
where; the implementation evidence remains the pending tasks' job.

**The grill's branch outcomes.**

| Branch | Outcome |
|---|---|
| 1. Mode semantics — off/on/full, absent/invalid fallback, `--with-full-content`, engagement, existing store, mid-project switching | PARTIAL → Q1/Q2 + F2. `on`/`full` and the absent/invalid fallback were coherent; `off` was not enforced at the intake or Claude's in-process hook, the flag-versus-`off` question was open, and lowering the mode had no retention statement. |
| 2. Config key — parser acceptance, scaffold block, upgrade preservation | FINDING → F3. `record:` is parsed by `src/config/overlay.rs::OverlayDoc`, not `src/config/manifest.rs`; `OverlayDoc` has no `record` field today, and a typed field would fail the whole parse on a non-string value. |
| 3. `givn history settings --json` — payload, exit behavior, discovery | FINDING → F4/F5. The payload named an unreachable `source: flag`; the command would be recorded by `records()`; human output, exits, and errors were unspecified; no scenario drove it; Claude can resolve in-process, so the pending-file mode cache was both unnecessary and contrary to `off`'s no-write rule. |
| 4. Bounds — 120/240/1000/4000/64 KiB/256 KiB, single site, assertions | FINDING → F6. Values were documented, but the intake was not named as the single enforcement site, the `content`/`question` wire schema was missing, and the caps were largely unasserted. |
| 5. Scenarios — bindability, discrimination, counts, no `@e2e` change | PARTIAL → F1/F8/F10/F11/F12. The new phrases are bindable and would fail RED; the default-metadata scenario was seeded, no scenario asserted the report facts/content, and the counts/task arithmetic were stale. |
| 6. SCM ignore — placement, Git/Fossil destinations, extended hygiene | PARTIAL/POSITIVE → F10. Destinations and the report assertion were correct, but the hygiene `Given` created no full-content entry, so the assertion was vacuous; the bare-root `.gitignore` fallback was not covered. |
| 7. Report surfacing — details facts, bounded full content | FINDING (high) → F1. The design paragraph existed but no task added the panel facts, no scenario asserted the report, and the file had no content budget. |
| 8. Consistency — TC-37, Q-103, R-91/R-92, glossary, chapters, counts | FINDING → F9/F11. Stale "never stored"/"credentials never captured" claims contradicted `full` and R-92; chapter 08 kept a single-bound claim; Q-103 overpromised per-harness fields; the scaffold cited ch.8.20 instead of 8.25. |
| 9. Defaults — `on` everywhere, fallback, no scenario assumes `full` | NO ISSUE. The default and fallback are consistent in the design, scaffold, chapters, and scenarios; only R-92's "off by default" wording needed correction (F9). |

**The four decisions, with rejected alternatives.**

- **Q1 — `--with-full-content` versus `off`: `off` is absolute.** The flag is
  an explicit per-payload override of `on`/`full` only. When the project mode is
  `off`, the flag is ignored with a one-line warning and exit 0. Documented in
  D2 (`design.md:163-199`), the mode table (`:481-489`), the intake rules
  (`:662-668`), `givn history --help`-facing wording in the design, the privacy
  chapter (`docs/arc42/08-crosscutting-concepts.md:957-980`), TC-37
  (`docs/arc42/02-architecture-constraints.md:37`), and the scaffold
  (`assets/config.yaml.tmpl:42-48`). Rejected: **letting the flag override
  `off`** — a durable privacy decision must not be silently bypassed by a
  one-shot flag; rejecting the flag with an error was also rejected because the
  manual/test path must stay non-failing and the warning already names the
  reason.
- **Q2 — lowering the mode retains captured content.** The switch is
  forward-looking; `full` → `on`/`off` never deletes, rewrites, or hides
  existing entries. Recorded in D2, the mode table, §Privacy bound, TC-37, the
  glossary's **Record mode**, and the scaffold. Rejected: **an implicit purge**
  — it destroys operator data as a side effect of a config edit and needs its
  own command and review.
- **Q3 — the report content budget is 8 MiB.** The self-contained report embeds
  full content in record order up to 8 MiB (`8_388_608` bytes) across the whole
  document, then omits the rest with the exact note
  `content omitted; see the record` on the affected entry; the store keeps every
  byte. Recorded in D2, the report-rendering rule (`design.md:592-603`), the
  report-generation bullet (`:1220-1227`), `ReportModel`/`render_report`
  (`:1311`, `:1315`), the Architecture Impact rows (`:1632-1633`), the asset
  contract (`tasks.md:233-246`), and chapters 05/06/08/12. Rejected:
  **unlimited embedding** — a long full-mode record would produce a report of
  the same order while only the viewport is visually bounded; **visual-only
  bounding** — it leaves the file itself unbounded.
- **Q4 — Maki question text is in scope via `maki.session.messages`.** At the
  wait's close the reporter reads the live transcript (works in TUI, `-p`, SDK,
  and ACP) and stores the blocking `question` tool call's
  `input.questions[]` as `data.question`, with no new grants. When no matching
  question is present (a permission prompt, plan form, or Lua float also
  produces `needs_input`, or the session has ended), the entry keeps the bounded
  title label and no full text; the wait duration is still recorded. Recorded in
  the Maki mapping and trade-off paragraph (`design.md:989`, `:1000-1006`), the
  question scenario (`development-record.feature:123-130`), Task 11d
  (`tasks.md:3309-3345`), and R-90
  (`docs/arc42/11-risks-and-technical-debt.md:76`). Rejected: **the
  `tool.question.input` slot** — an undeclared-tool slot requires **every**
  permission (`fs_read`, `fs_write`, `net`, `env`, `run`, `agent`), including
  network and agent access the reporter does not need; **deferring Maki** — the
  operator requires the question text and the transcript surface can deliver it
  without a permission widening.

**F1–F12 resolutions and anchors.**

| # | Resolution | Anchor |
|---|---|---|
| F1 (high) | The report contract now names the exact panel facts (session/title, status, stop reason, model, cost, context size/window, tool call id, output bytes, bounded error, parent/agent, retry, permission mode) and the bounded content region; a new regular scenario asserts the model/report facts, the 8 MiB embedding budget, the omission note, and a complete store; Task 25b carries the scenario and extends the E7 `@e2e` assertions to one new fact plus the content region (same action, evidence refreshed in that task); the model/asset rows and the copied contract carry the budget/omission fields. | `design.md:592-603`, `:1220-1227`, `:1311-1315`, `:1632-1633`; `development-report.feature:72-79` and `:105-111`; `tasks.md:6907-6963`, `:3462-3513`, `:233-246` |
| F2 | `givn history ingest` and the Claude `harness_hook` are named as the two `off`-enforcement sites: no append, no pending write, no ignore write, one warning and exit 0; the off scenario asserts no pending state, no ignore write, and the intake's warning in addition to no new entry. | `design.md:488`, `:664-670`, `:542-551`; `development-record.feature:133-141`; `tasks.md:3358-3407` |
| F3 | `src/config/overlay.rs::OverlayDoc` is the parse site (not `manifest.rs`); it gains a lenient raw `record` value so an invalid value is a one-warning fallback to `on`, never a parse failure; `givn upgrade` preserves the key; the scaffold cites ch.8.25 and the numeric bounds (120/240/1 000/4 000); the task file lists name `overlay.rs`. | `design.md:183-187`, `:493-499`; `docs/arc42/02-architecture-constraints.md:37`; `assets/config.yaml.tmpl:42-54`; `tasks.md:3362-3372`, `:3519-3531` |
| F4 | `givn history settings` joins the non-recorded read views (`records()` exemption alongside `history`/`timeline`); `source: flag` is dropped (`config`/`default` only); human output, `--json` payload, exit codes, and the outside-a-project error are specified; the invalid-mode scenario asserts the JSON payload and source; Task 11f/11h name the files. | `design.md:518-529`, `:700-706`; `development-record.feature:162-167`; `tasks.md:3411-3470`, `:3515-3550` |
| F5 | Claude resolves the mode in-process inside `harness_hook` (no query, no pending-file mode cache); omp/opencode query by subprocess at setup and Maki at its first event callback (the flush's jobstart/jobwait mechanism; the setup-time blocking job is unverified and unused); the pending schema stays starts/kind/label and is never written under `off`. | `design.md:536-551`, `:609-621`; `docs/arc42/06-runtime-view.md:575-581`; `tasks.md:3362-3389`, `:3411-3440` |
| F6 | The bounds table names the intake as the single enforcement site; `data.question` and `data.content` gain a wire schema with named fields, 64 KiB/256 KiB caps, `truncated`, and original `bytes`; scenarios assert the question cap/truncation, a metadata bound, a content cap/`truncated`, and the report omission note. | `design.md:466-477`; `development-record.feature:114-130`, `:146-159`; `development-report.feature:72-79`; `tasks.md:3411-3513` |
| F7 | The default-metadata table is explicitly "where the harness exposes it", with a per-harness availability table (Claude no stop reason/turn index; opencode no turn count/context; Maki model/title/transcript gaps); Q-103 and Task 11c carry the same qualification. | `design.md:390-424`; `docs/arc42/10-quality-requirements.md:104`; `tasks.md:3255-3307` |
| F8 | The default-metadata scenario now drives the generated omp reporter through the real intake and asserts the new fields; Task 11c names the driver and states that seeded records are only for fields a harness cannot expose. | `development-record.feature:114-120`; `tasks.md:3257-3289` |
| F9 | Stale privacy claims corrected: the traceability row, "never captured as structured values" plus the credential/R-92 caveat, Q-100, chapter 08's per-tier bound and single-site wording, the glossary's bounded-label/record-mode entries, R-92's default wording, the pending-state description, and `bound_label`'s primitive row; Maki's "never stored" tool/turn/compaction sentences became "stored only in `full`". | `design.md:43`, `:580-586`, `:1304`, `:991-996`; `docs/arc42/08-crosscutting-concepts.md:957-980`; `docs/arc42/10-quality-requirements.md:101`; `docs/arc42/11-risks-and-technical-debt.md:78`; `docs/arc42/12-glossary.md:182-188` |
| F10 | The Git/Fossil hygiene `Given` now writes a real full-mode `data.content` entry before asserting porcelain/extras; the Git scenario also covers the bare-root `.gitignore` fallback before `git init`; the report-generation assertion is kept. | `development-record.feature:76-98`; `tasks.md:3204-3253` |
| F11 | Counts and references recounted: the delta inventory is 54 entries = 51 added + 3 modified, 46 non-`@e2e` + 8 `@e2e`, 7 pending `@wip`; the task file schedules Tasks 1–44 (43 scenario sections + the extended-evidence Task 11b); the e2e count stays 8; chapter 02 now says four durable constraints (TC-34…TC-37); chapter 03 says eight interfaces including `givn history settings`; the scaffold cites ch.8.25; the glossary count is seventeen; the Done-when proof is 46 regular vs 8 e2e. | `tasks.md:3-7`, `:22-40`, `:42-55`, `:271`, `:10820-10821`; `arc42.md:15-25`; `assets/config.yaml.tmpl:42`; `docs/arc42/README.md:80-81` |
| F12 | Task 11b gains its own RED step (the new full-content/bare-root/pending/extra-file phrases are undefined today) and a commit instruction that names only the extended assertions and points at the Tasks 10/11 commits, never restating their titles, so the per-scenario gate sees no duplicate scenario commit. | `tasks.md:3204-3253` |

**Checks run by this pass.**

- `givn lint --change development-timeline` → exit 2: **5 files checked, 7
  findings**, all seven the pending scenarios' `[WIP]` tags (`development-record`
  six, `development-report` one) plus the pre-existing `[SUBST] advisory`
  permanent-corpus subset matches; no parse error and no error-kind finding.
- Mechanical recount: **54** scenario entries = 51 `@givn.added` + 3
  `@givn.modified`, 0 removed; **46** non-`@e2e` + **8** `@e2e`; **7** `@wip`;
  per feature 20/15/16/1/2. Task sections: **53** total = 44 non-`@e2e`
  sections (43 added-scenario sections + the extended-evidence Task 11b) + the
  E1 setup task + 8 `@e2e` tasks. The use case's eight §Interactions rows, the
  design's eight matrix rows, and the eight `@e2e` titles are unchanged.
- No suite was run (read-only hardening scope): the grill's evidence was
  re-read against the live store/log/report, and this pass changed only planning
  prose and the config scaffold. The pending tasks carry every RED/GREEN/REFACTOR
  evidence field.

**Residuals noted by this pass.** None is an open question; each is work the
pending tasks already own.

1. The F1 implementation evidence is pending: Task 25b must generate the
   over-budget fixture, prove the omission note and the store's completeness,
   extend E7's step assertions, and refresh E7's two captures. No new `@e2e`
   scenario is added, and E7's existing commit/hash/evidence stay untouched.
2. The 8 MiB budget fixture is deliberately large; Task 25b must keep it
   deterministic and assert both the embedded and the omitted sides.
3. The Maki transcript read is exercised through the real binary's Lua driver
   with a test-written `maki.session.messages` fixture (the harness has no
   offline question turn); the reporter code is the real generated asset, and
   R-90 records the residual.
4. The grill's earlier 45/43 counts are superseded by the final 46 regular
   scenarios after F1's new report scenario; every current count statement now
   uses the final numbers.
5. E1's pasted 35/8 scope proof and the 2026-10-16 final-scope runs are dated
   evidence from before the capture/report scenarios; the current E1 instruction
   and the Done-when use 46/8, and no pasted evidence was rewritten.

**DESIGN-REVIEW: PASS**

## Re-review round 6 (2026-10-17, hybrid workflow report delta)

**Why it ran.** The operator approved the hybrid workflow report on 2026-10-17
after five rendered mockups, superseding the dense horizontal timeline (D6).
The delta at `307c0755` changes `design.md` (report generation, the hybrid
section, the Visual Design Contract, the interaction matrix, D6), the usecase
interaction inventory, `development-report.feature`, the design system, and
arc42 chapters 05/06/08/11/12. A fresh-context grill (read-only planning; no
suite run) read the served design-review and specs rules, the delta, the
mockups (`/tmp/timeline-mockups/v5/index.html` and `data.js`), the existing
implementation (`src/cli/timeline.rs`, `assets/timeline/report.html`,
`src/state.rs`), and the four reporters. It re-derived the arc42 chapter set
and the ADR qualification independently. The evidence that decided the largest
finding was produced, not assumed: `givn timeline --change development-timeline`
on the live record wrote a model with **472 entries, zero `phase` transitions,
and every entry assigned to no stage** (`stages` all `None`), so the hybrid's
phase stack would have had no recorded work on the operator's own report.

**Branch outcomes.**

| Branch | Outcome |
|---|---|
| Requirement fidelity — statistics first, workflow top to bottom, phase zoom, parallel lanes, lane log, current state, mobile, hybrid V1+V2+V4 | FINDING → F1/F2/F4/F5/F13. The top-to-bottom order and the statistics were specified, but the phase backbone had no data on the operator's record, the subagent-lane rule contradicted the record, and the V4 header count was dropped. |
| Derivation precision — current phase, statuses, times, lanes, repeats, aggregates, ideation/archived, band rows | FINDING → F1/F2/F4. The artifact-id mapping, partial `tasks`, skipped precedence, archived-vs-missing, and the no-stage group were underspecified. |
| Model/scale — `workflow`/`current_state`, asset-derived lanes, 8 MiB, DOM bound, text view, empty state | FINDING → F3/F5/F7. Lanes/groups/aggregates were declared asset-derived while regular scenarios asserted them; the DOM bound held only at Fit; the KPI totals were absent from the model. |
| Specs/e2e/evidence — bindability, inventory, matrix, evidence paths, mobile assertion | FINDING → F8/F9/F10/F18. The phase expander was an un-inventoried click; two renamed e2e titles invalidated their evidence slugs; the mobile contract had no assertion; 14 delta scenarios carried no `@wip`. |
| Visual contract/design system — primitives, breakpoints, accessibility, motion, approved reference, superseded D6 | FINDING → F6/F14/F15/F16/F20. The design's breakpoints contradicted the design system; the lane-button wording nested controls; the zoom mechanism named a control the hybrid does not have; view 4 is illustrative. |
| Architecture documentation — 12 rows, chapter content, ADR qualification, glossary | FINDING → F11/F12/F17. Chapter 10 still claimed timestamp-only rendering and Fit-only bounding; the change-level assessment and glossary count were stale; no new ADR qualified. |
| Decision ledger, use-case/persona, interface classification, deferrals | NO ISSUE. D1–D6 resolve; the two Personas are unchanged and never Gherkin actors; the surface is a Web UI with the real browser driver; no deferral. The proposal's D6 is the historical decision, superseded in the design ledger (accepted residual 1). |

**Findings and resolutions.**

| # | Severity | Area | Finding | Outcome |
|---|---|---|---|---|
| F1 | high | Lanes/attribution | The rule "a lane that holds a `subagent` entry is named by that entry's `data.subagent`" renames the spawning session's lane after a subagent. The live record proves the failure: one session holds 137 entries and one subagent span; renaming it would attribute all 137 to the subagent. The positive examples were also false: Claude's transcript is read for usage only, and opencode's flush carries one session id for its batch. | fixed — a lane is subagent-named only when the span's `data.subagent` equals the lane's session id; a lane that merely contains a span keeps its name; the session title is the next name source; the report states the attribution limit per harness; R-93, the glossary, the design-system lane primitive, and the lane scenario were rewritten. |
| F2 | high | Phase coverage | The workflow stack's per-phase entries, times, lanes, and sparklines depend on artifact-appearance `phase` transitions. Production records none: `givn check --artifact` writes gate phases labelled with artifact ids, and the live store has zero transitions. A later-only first transition would also have labelled all earlier work `propose`. | fixed — the command dispatcher now emits one transition labelled with the `STAGE_ARTIFACTS` filename; gate phases are explicitly never transitions; entries before a first transition that is not `proposal.md` (or with no transition) render in one "no recorded stage transition" group; a new regular scenario asserts the gap; R-95 records the residual. |
| F3 | high | Model/testability | The delta said lanes, logs, and repeated groups are derived in the asset, yet the new regular scenarios assert lanes, repeated groups, and aggregate ticks. Without a browser they could not see those structures; the scenarios were unprovable or would have to invent a rendering target. | fixed — `WorkflowPhase` carries `lanes`, `repeated`, and Fit `aggregates`; `ReportModel` carries `unstaged`; the scaffold holds ids/names/totals only and the raw entries appear once; the text view prints the phase lines, lane names, group counts, and Fit aggregate counts; the asset still renders ticks and logs. |
| F4 | medium | Current phase | "The stage that owns the next required artifact (`STAGE_ARTIFACTS`' own index)" is not implementable against the state's ids: `arc42-docs` is not `arc42.md`, `tasks` partial would mark `tasks` active while the change is implementing, skipped precedence was unstated, and an archived change could not be told from a deleted one. | fixed — an explicit id→phase table, the `partial` `tasks` → `implement` rule, skipped precedence, `givn/archive/<id>` resolution, an outside-table fallback, and the ideation no-recorded-stage rule were added; the scenario fixture now produces a real derived state. |
| F5 | medium | KPI model | The KPI strip promised elapsed, active, waiting, tool calls, subagent spans, and cost, but `Summary`/`WorkflowPhase` carried no active, tool-call, or cost total, and the text view could not print facts the model lacked. | fixed — `Summary` gains `active_ms`, `tool_calls`, `cost_total`; active is elapsed minus waiting (measured or "not measured"); cost sums the recorded real cost in both recorded shapes and reads "unavailable" when none exists. |
| F6 | medium | Breakpoints | The Visual Design Contract said "two columns from 720 px" while the design system says one column at 720–1199 and 1200+ with statistics grids. Two implementable layouts cannot both be true. | fixed — the contract now matches the design system: one column at every breakpoint, two-column statistics from 720 px, three-column from 1200 px, full-width phase cards. |
| F7 | medium | DOM scale | The aggregation threshold applied only at Fit; at 15 m the design promised individual ticks, so a dense 15-minute window could still flood the DOM, contradicting R-94's "bounded" claim. | fixed — the sub-pixel threshold applies at every zoom; the lane scroll height is capped; R-94, chapter 08's scale paragraph, and Q-101 were aligned; the model carries Fit aggregates for the regular proof. |
| F8 | medium | Mobile | The contract declared the mobile layout but no scenario asserted it; a screenshot pair is evidence, not an assertion. | fixed — the open e2e outline now sets the mobile viewport and asserts the one-column layout and no page-level horizontal overflow; the contract and design-system verification line carry the same assertion. |
| F9 | medium | Interaction coverage | The phase-expander click changes visible content but had no inventory row, and the select scenario bundled expand + lane selection into one observable behaviour. | fixed — a "expand a workflow phase" inventory row and `@e2e` scenario were added; the select scenario now only selects a lane and reads its log; the matrix and usecase carry nine rows and nine matching titles. |
| F10 | medium | Evidence paths | The delta renamed two e2e titles, which changes their scenario slugs; the existing evidence directories would become stale while the design said the paths were merely refreshed. | fixed — the two titles keep their existing slugs for the same actions; only the new expand scenario introduces a new directory; the design's evidence bullet states this rule. |
| F11 | medium | Determinism | The report now reads the change state, but chapters 06/08, Q-99, and the design's traceability row still claimed it renders from "embedded timestamps alone"; two renders could then differ without the docs saying why. | fixed — determinism is defined over the record plus the embedded state snapshot in 06/08/Q-99 and the traceability row; R-96 records the snapshot staleness. |
| F12 | medium | arc42 assessment | The change-level `arc42.md` still described the panel/timeline asset, omitted R-93–R-96 from row 11, and claimed seventeen glossary entries after five new terms. | fixed — rows 05/06/08/10/11/12 and a dated delta note were updated; the glossary count is twenty-two; the chapter set was re-derived as 05/06/08/10/11/12. |
| F13 | low | Header fidelity | The approved V4 header's done-phase count was dropped, and the mockup's "7 findings open / running 38 m" banner was not derivable from the change state. | fixed (count) / accepted (banner) — the header states done phases of nine; it explicitly does not count findings or claim running time, because the state does not expose them (residual 3). |
| F14 | low | Zoom mechanism | The matrix said "click the zoom-in control" while the hybrid's per-phase control is Fit / 1 h / 15 m. | fixed — the matrix names the `15 m` control and the "narrower level" rule. |
| F15 | low | Accessibility | "Each lane is a focusable button" would nest the lane's focusable ticks inside a button, which is invalid semantics. | fixed — the lane header is the button and the ticks are separate focusable elements, in the design and the design system. |
| F16 | low | Lane colors | "One hue per lane" is unbounded while the system defines four lane hues. | fixed — hues cycle beyond four and the lane name, never the hue, carries identity. |
| F17 | low | Ubiquitous language | The glossary's **Lane** repeated the false naming rule and lacked **Repeated-step group** and **Current state**. | fixed — Lane was corrected and the two terms were added; **Aggregate marker** now states the every-zoom threshold. |
| F18 | low | Pending scenarios | The delta's new/changed scenarios had no `@wip` even though their steps are undefined, so lint reported a clean file while the suite would fail on them. | fixed — the 14 pending scenarios now carry `@wip`; lint exits 2 with exactly those 14 findings. |
| F19 | low | Decision ledger | The proposal's D6 disposition still says the dense timeline was approved. | accepted — the proposal preserves the 2026-10-16 decision as history; `design.md`'s D6 and its *Open Technical Questions* carry the operator's 2026-10-17 supersession, and this step's rules forbid rewriting the proposal for a resolved decision. |
| F20 | low | Reference view | The approved view-4 drawing shows eight phase rows and no header count, while the workflow has nine phases (including arc42). | accepted — reference views are illustrative; the design system now says the design's nine phases are authoritative and the drawing shows a subset. |

**The decisions behind the fixes, with rejected alternatives.**

- **Session lanes are the only lanes.** Rejected: creating a "subagent lane" for
  a span the record places in the spawning session — it invents a lane and
  attributes the spawning session's calls to the subagent (F1).
- **The phase backbone is captured, then honestly degraded.** Rejected:
  relabelling the first transition's era as the opening stage (it would have
  called all pre-`review` work `propose`), and rejecting the report when a
  record has no transition (the record is still the best available evidence).
  The dispatcher writer plus the explicit no-stage group was chosen (F2).
- **The regular scenarios assert model facts.** Rejected: making the lane,
  repeat, and aggregate scenarios browser scenarios (it would add three
  interactions and evidence sets for rendering rules), and keeping the
  derivation asset-only (then the regular scenarios assert nothing). The model
  scaffold carries ids and totals, not a second copy of the entries (F3).
- **Fit/1 h/15 m all obey one threshold.** Rejected: a fixed per-entry cap at
  15 m (it hides entries without saying how many were grouped) and a
  virtualized list (a new rendering architecture for a static file) (F7).
- **The phase expander is its own interaction.** Rejected: treating the expand
  as part of "select a timeline element" (two clicks, two observable changes,
  one row) and treating the expander as navigation (it changes the visible
  timeline, not the scope) (F9).
- **Existing e2e titles keep their slugs.** Rejected: renaming the two
  scenarios and moving their evidence directories (needless evidence churn for
  no assertion gain); the new expand scenario still gets its own pair (F10).

**Checks run by this pass.**

- `givn timeline --change development-timeline` → the live model carries 472
  entries, `stages` all `None` (no `phase` transition), and one recorded
  `subagent` entry in a payload session that also holds 137 other entries — the
  evidence for F1 and F2.
- Read the reporters: omp and Maki use one payload session; Claude reads the
  subagent transcript only for usage; opencode flushes at
  `session.execution.*` with one payload session id.
- `givn lint --change development-timeline` → exit 2: **5 files checked, 14
  findings**, all 14 the pending hybrid scenarios' `[WIP]` tags; the only other
  output is the pre-existing `[SUBST] advisory` subset matches. No parse error
  and no error-kind finding.
- Mechanical counts: **58** scenario entries = **49 non-`@e2e` + 9 `@e2e`**;
  **14 `@wip`**; per feature 1/2/13/20/22. The usecase's nine §Interactions
  rows, the design's nine matrix rows, and the nine `@e2e` titles are the same
  title set; the matrix and the features match one-to-one.
- arc42: re-derived the affected chapter set as 05/06/08/10/11/12, verified the
  chapter files' content against the amended design, and re-checked the ADR
  qualification: no new candidate (D6 is a design decision owned by
  `design.md` and the design system; ADR-073 is unchanged).
- No suite was run: this pass changed planning prose, scenarios, the design
  system, and durable documentation only. The hybrid implementation, the 14
  pending step bindings, and the evidence refresh remain the follow-up tasks'
  job.

**Residuals noted by this pass.** None is an open question; each is named work
or an accepted boundary.

1. The proposal's D6 is the historical 2026-10-16 decision; the design's D6
   supersedes it per the operator's 2026-10-17 approval. The proposal was not
   edited, per this step's rule.
2. The live record remains unstaged until the dispatcher writer exists; the
   report shows the no-stage group and the state-derived phase statuses, and
   R-95 records the gap. Existing records are not backfilled.
3. The header does not count review findings or claim a running time because
   the change state does not expose them; the approved mockup's banner was
   illustrative.
4. The reference views are illustrative; the nine-phase workflow is
   authoritative.
5. The hybrid implementation, the 14 pending scenarios, the new expand
   evidence pair, and the `Summary`/`WorkflowPhase` model work are the
   follow-up tasks; `tasks.md` was not edited here.
6. Per-event subagent identity remains R-93: no reporter today tags a tool
   entry with the subagent that made it; opencode's child-session flush is the
   only attributable session, and the report states the limit.
7. The `cost_total` normalization (cost object versus number) is specified
   but must be exercised by the follow-up implementation test.
8. The report-level "active" includes gaps between recorded entries by
   definition (elapsed minus waiting); it is labelled with that derivation and
   is not a claim of continuous work.
9. The change-level `arc42.md`'s earlier 54/46/8 statements remain as dated
   history; the new delta note carries the current 58/49/9 scope.

**DESIGN-REVIEW: PASS**
