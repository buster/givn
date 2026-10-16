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
| Q6 | Tech choices | Which ignore destinations must the record's fenced block reach? | closed — both backends' destinations through the existing switch |
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
