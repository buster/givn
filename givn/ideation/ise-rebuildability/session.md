# Ideation session: ise-rebuildability

## Phase

Handoff complete 2026-11-14. Phase 5: Use Case Elaboration complete
2026-11-14. The six
Phase 5 review findings were applied 2026-11-14 (optional `--at`, the
`--dry-run` and receipt rules, the restart wording, the em-dash scope, and the
noun sweep). Phase 4: Use Case Definition reviewed and corrected 2026-11-14.
Phase 3: Event Storming closed 2026-11-14: the term set is closed
(eight confirmed, no candidates, eight rejected) and `Q16` is applied. `Q4`
closed 2026-11-14 with `givn/specification.yaml`; `Q5` closed 2026-11-14 with
the `e2e` package as the declared role and the tag as its scope; `Q3` closed
2026-11-14 with `givn ise prepare` and `givn ise verify`, both bound to
`addons.ise`. `Q6` closed 2026-11-14 with the `ise` -> `architecture_principles`
prerequisite, so the `ise` guidance uses `Component` and the other established
nouns; the invented nouns rejected during the `Q6` sweep are gone from every
living artifact, and the prepare flag is `--replace`. `Q8`
closed with `Opt-in addon`; `Q9` closed 2026-11-13 with "revisit all seven
terms one by one" oriented on The Ise Principle's wording; `Q10` closed
2026-11-13 with `Specification`, contrasted with `Source code`; `Q11` closed 2026-11-13 with
`End-to-end (E2E) test`, keeping `black-box` as the no-shared-code qualifier;
`Q12` closed 2026-11-14 with `Constant Ise` (Pattern 01's exact name); `Q13`
closed 2026-11-14 with `Ise Test` (Ise's own act noun), with `Constant Ise` as
the one-unit scope and `Dry-Run Ise` / `Full replacement` as the two execution
modes. `Q14` closed 2026-11-14 with `Spec gap`, the four-way classification
record with exactly four outcome values, and every living topic artifact
carries the rename. `Q15` closed 2026-11-14 with **no new term**: `Stack
recipe` is rejected, and Givn's existing `Instruction` plus the id convention
(`<id>-examples`, `<id>-<stack>`) carry the on-demand instruction, so canvas B
says `C: request the stack instruction` / `EB2: A stack instruction was
served`. `Q16` closed 2026-11-14 with `Change strategy` confirmed: the
recorded choice among incremental modification, `Constant Ise`, and a
whole-product rebuild, held as the design's `Decides D<n>` entry with the
`Ise Test` mode (`Dry-Run Ise` or `Full replacement`) beside it when a rebuild
runs and no new mandatory field. The term record has no candidate rows.

## Settled constraints (2026-11-14)

Six decisions the user made in discussion on 2026-11-14. They bind `Q3`,
`Q4`, `Q5`, the Event Storming rows, and every later artifact; a statement
that contradicts one is a defect, not a proposal.

1. **Deletion is set arithmetic on declared paths.** Givn removes exactly the
   paths a project declares as the replacement unit and its internal unit
   tests. It never infers what a unit test is from stack-specific markers
   (`#[cfg(test)]`, `test_*.py`, Maven test roots). No language knowledge
   enters deletion. `Q4` fixed where the paths are declared: the component's
   paths are trial input (`givn ise prepare [--at <revision>]
   [--replace <path>...]`), each path must fall inside the `source:` roots
   declared in `givn/specification.yaml`, and the paths named with `--replace`
   are recorded in the trial receipt.
2. **`Not replaced` warning by containment.** After preparation, if the
   replaced component's own directory still holds files you did not name with
   `--replace`, Givn reports the count. That is containment arithmetic, not
   classification.
3. **Component step definitions, fakes, and harness are implementation.** They
   are never frozen, and Givn performs no diff accounting of added, modified,
   or deleted step definitions; strict TDD evidence plus coverage measurement
   govern that layer. Component scenarios (`.feature` text) stay in the
   Specification set.
4. **Only the `End-to-end (E2E) test` target is frozen** as acceptance
   authority during an `Ise Test`.
5. **Changes to neighboring components are not a Givn artifact.** Changes
   outside the replaced component are legitimate; the judgment belongs in
   review guidance as one question ("name the neighboring components changed
   outside the replaced component and say why"), not in code or a receipt
   field.
6. **E2E freeze applies after qualification.** A new E2E scenario must first
   be observed failing against the pre-change implementation for the intended
   reason. Mid-trial test defects are recorded as `Spec gap: E2E test defect`,
   stop the trial, and the trial restarts after the E2E test is corrected. The
   `e2e` package is part of the Specification, so the settled constraint stays
   true.

## Documentation policy note (2026-11-14)

The em-dash ban applies to authored living documents. The three history
sources (`questions.md` closed entries, `domain-terms.md` history, and persona
append-only records) are never edited for it, and existing em-dashes are not
rewritten: 165 remain across the topic by design (`questions.md` 97,
`domain-terms.md` 17, persona records 32, `frame.md` 9, `eventstorming.md` 10).
Text authored or edited from this date carries no em-dash. The rejected nouns
follow the same rule: they survive only in history.

## Persona inventory

| Stakeholder type in Frame | Project persona (`givn/personas/`) | Topic file | Status |
|---|---|---|---|
| Consumer-project owner (developer / maintainer) | `developer--primary-user` | `personas/developer--primary-user.md` | Reuse, base `7bf38024`; confirmed 2026-11-13 (`Q7`) |
| Authoring / rebuilding agent (technical role / automation caller) | `agent--technical-role` | `personas/agent--technical-role.md` | Reuse, base `7bf38024`; confirmed 2026-11-13 (`Q7`) |
| Givn maintainer (owner) | `givn-maintainer--owner` | `personas/givn-maintainer--owner.md` | Reuse, base `7bf38024`; confirmed 2026-11-13 (`Q7`) |

Three personas. The external contributor / independent verifier is not a
separate file: `givn-maintainer--owner` carries the bare-clone and clean-room
frozen-verifier lens (user decision 2026-11-13, `Q7`).

## Progress

- 2026-11-13: Synthesized all three workspace tab analyses (`baee9a4`,
  `d436f74`, `e468528`) against `https://iseprinciple.org/` (Draft v0.5, Rules
  1–3, Core Patterns 01–04, WIP Drafts 05–07), `givn` (`src/config/manifest.rs`,
  `src/config/extensions.rs`, `assets/instructions/*.md`, `tests/steps/*`),
  `givn-harness` (`benchmarks/catopia-v2`, `benchmarks/catopia-spring-v2`,
  `docs/findings/2026-10-31-givn-0.9.12-to-0.9.16.md`), and Passuff
  (`/home/buster/projects/gh-notify`).
- 2026-11-13: User selected **Option 2** (`Q1` closed): enter `givn-ideation`
  in `givn` first (paired with `givn-harness` Catopia rebuild/evolution
  evaluation), then validate transfer on Passuff and Givn self-rebuildability.
- 2026-11-13: Frame research completed (`givn status`, `givn spec tree`,
  `README.md`, `docs/glossary.md`, `docs/domain-model.md`,
  `docs/arc42/02-architecture-constraints.md`,
  `docs/arc42/05-building-block-view.md`, `givn/personas/*.md`,
  `enable-optional-addons.feature`, `serve-extended-rules.feature`).
- 2026-11-13: Scaffolded topic `givn/ideation/ise-rebuildability/` with
  `frame.md`, `questions.md` (`Q1` closed, `Q2` active, `Q3`–`Q6` parked),
  `domain-terms.md` (5 candidate terms with collision checks), and `session.md`.
- 2026-11-13: Frame boundary review passed; mechanical corrections applied
  (see below).
- 2026-11-13: User confirmed the Frame with **Option 1** (`Q2` closed):
  goal, scope boundary, exclusions, and four stakeholder types stand.
  `frame.md` is `Status: confirmed 2026-11-13 (user)`.
- 2026-11-13: Drafted four reused topic-local persona files under `personas/`
  (record-only `Reuse:` headers, base `7bf38024`, dated 2026-11-13 entries) and
  appended `Q7` to `questions.md` with the reuse, split, and drop options.
- 2026-11-13: User selected **Option 3** (`Q7` closed): three personas. Deleted
  `personas/external-contributor--open-source-consumer.md`, removed the `Q7`
  pending note from the other three, folded the bare-clone and frozen-verifier
  stances into `personas/givn-maintainer--owner.md`, and recorded the decision
  in `frame.md` beside the stakeholder table.
- 2026-11-13: Wrote `eventstorming.md`: 20 events in four chains, five Mermaid
  canvases, reverse narrative, 11 invariants, question-dependent behavior for
  `Q3`–`Q6`, and advisory reactions from the three confirmed personas.
- 2026-11-13: Updated `domain-terms.md` to eight candidates (three from the
  event chain: `Stack recipe`, `Change strategy`, `Mismatch triage`), corrected
  the Frame first-use lines, and appended `Q8` (term check) to `questions.md`.
  `Q3`–`Q6` stay parked.
- 2026-11-13: Event Storming boundary review applied. The appended term check
  was split: `Q8` is now the immediate naming dialog for the addon posture
  (its placeholder collides with the glossary anti-term `opt-in default` and
  with "an addon switched on" in `match-guidance-to-enabled-addons.feature`),
  and `Q9` is the phase term check for the other seven terms. Canvas fixes:
  one issuing actor per command, the misplaced policy replaced by the
  configuration it read, `E` markers on events, the frozen verifier drawn as a
  record, and A4's read model corrected.
- 2026-11-13: User selected **Option 3: `Opt-in addon`** (`Q8` closed):
  `domain-terms.md` confirms that row and records `Default-off addon` and
  `Experimental addon` as rejected candidates with their reasons; the name
  carries the planned `docs/glossary.md:9` anti-term amendment and the
  `match-guidance-to-enabled-addons.feature:23,86` disambiguation when the
  change lands. `eventstorming.md` now uses the name (domain line, invariant 1,
  question-dependent table) and keeps the `ise` addon id for the instance.
  `frame.md` question status updated. `Q9` followed.
- 2026-11-13: User closed `Q9` with **Option 2: revisit all seven terms one by
  one, oriented on The Ise Principle's own wording**: `Replacement package` is
  "very generic", "The spec would be fine as overall term. There are specs and
  there is code." Read `/home/buster/projects/iseprinciples/index.html`
  (Draft v0.5) for the canonical vocabulary and marked `Replacement package`
  `rejected` in `domain-terms.md` with its reason.
- 2026-11-13: Appended `Q10` (Term 1 of 7): `Specification` (the spec) versus
  `Source code` as the recommended pair, `Product specification` as the
  additive alternative, and widening `Permanent specification corpus` as the
  no-new-noun alternative, each with its recorded collision check. `Q11`–`Q16`
  carry the other six terms one at a time.
- 2026-11-13: User closed `Q10` with **Option 1: `Specification` (the spec),
  contrasted with `Source code`**. `domain-terms.md` adds two `confirmed` rows
  (`Specification`, `Source code`) with genus + differentia, contrast pair,
  concrete sentence, and recorded collision checks, and keeps `Replacement
  package` `rejected` as history. `frame.md`, `eventstorming.md`, the open
  `Q3`/`Q4` bodies, and both term-using persona files carry the new pair; the
  closed question bodies in `questions.md` keep the old noun under a dated
  terminology note. Rename evidence (occurrences of the rejected noun): 32
  before, 19 after, all in history (`questions.md` closed bodies, `session.md`
  Progress entries, the `domain-terms.md` rejected row, and the two original
  persona stances); `frame.md` and `eventstorming.md` hold 0. Canvas D1 was
  re-rendered with `mmdc` (exit 0, 258,491 bytes, SVG carries `Record: frozen
  Specification`); all five canvases re-render with exit 0 (288,161 / 201,844 /
  326,681 / 258,491 / 208,990 bytes) and every `E`-marked event label was
  found. `domain-terms.md` first-use cells were repointed (`Stack recipe`
  `eventstorming.md:27`, `Change strategy` `:68`, `Mismatch triage` `:33`,
  `Black-box verifier` `frame.md:234`, `Component replacement` `frame.md:73`,
  `Opt-in addon` `frame.md:88`).
- 2026-11-13: User closed `Q11` with **Option 2: `End-to-end (E2E) test`**,
  keeping `black-box` as the qualifier for the no-shared-code rule.
  `domain-terms.md` confirms that row (genus + differentia, contrast with
  `Component test`, the `passuff` `80x24`/`120x40` sentence, recorded collision
  check) and adds `Black-box verifier` as `rejected`; `frame.md`,
  `eventstorming.md`, and the current-state sections of `session.md` carry the
  rename, while the two term-using persona files gained dated entries instead
  of rewrites. Rename evidence: `Q11` recorded 15 occurrences at opening; after
  the sweep every remaining hit (23 word occurrences across 16 lines) is
  history (`questions.md` closed bodies and resolutions, `session.md` dated
  records and the closed queue row, and the `domain-terms.md` rejected row plus
  the confirmed row's contrast cell); `frame.md`, `eventstorming.md`, and the
  persona running records hold 0. Canvas labels changed in C, D1, and D2; all
  five canvases were re-rendered with `mmdc` (exit 0, 288,086 / 201,842 /
  326,797 / 258,476 / 209,023 bytes) and every expected label was found
  (`EC4: E2E test identity was bound into
  the receipt`, `Record: E2E-test-bound change receipt`, `C: freeze the
  Specification and the E2E tests`, `Record: frozen E2E test digest`, `C: run
  the frozen E2E tests`, `ED3: The frozen E2E tests were executed against the
  replacement`, `C: observe the E2E test result`). `domain-terms.md` first-use
  cells were repointed (`Stack recipe` `eventstorming.md:29`, `Change
  strategy` `:70`, `Mismatch triage` `:35`, `End-to-end (E2E) test` `:30`).
  `Q12` was appended for the next dialog.
- 2026-11-14: User closed `Q12` with **Option 2: `Constant Ise`** (Ise
  Pattern 01's exact name, tagline "Delete and rebuild only the impacted unit
  within one module's boundary."). `domain-terms.md` confirms that row (genus +
  differentia, contrast with `Full rebuild` and incremental modification, the
  `passuff` key-parser sentence, recorded collision check), changes
  `Component replacement` to `rejected` with its reason, and updates the
  `Change strategy` candidate to name `Constant Ise`. `frame.md` writes
  `Constant Ise` at `:73`, removes the mixed phrase `Constant-Ise unit
  replacement` at `:111`, and closes the question block. `eventstorming.md`
  names `Constant Ise` in the intro and the B4 policy and adds the closed
  `Q12` row to the question-dependent table. Rename evidence: `Component
  replacement` had 23 word occurrences across 5 files before the sweep; after
  it every remaining hit is history (the `Q12` option and resolution, the
  `domain-terms.md` collision-check command and rejected row, the dated
  `session.md` entries, and the closed `Q9`/`Q12` bodies in `questions.md`),
  and `frame.md` and `eventstorming.md` hold 0. No Mermaid block changed: the strategy name
  appears in no canvas label, so no canvas was re-rendered. `domain-terms.md`
  first-use cells were repointed (`Stack recipe` `eventstorming.md:31`,
  `End-to-end (E2E) test` `:32`, `Mismatch triage` `:37`, `Change strategy`
  `:72`). `Q13` was appended for the next dialog.
- 2026-11-14: User closed `Q13` with **Option 1: `Ise Test`** (Ise section
  `03 The Ise Test`'s act noun, "Set the old code aside for one module or the
  whole system, and build it again from the spec alone."), with `Constant Ise`
  as the one-unit scope and `Dry-Run Ise` / `Full replacement` as the two
  execution modes. `domain-terms.md` confirms that row (genus + differentia,
  contrast with `End-to-end (E2E) test` and `Change`, the `passuff` key-parser
  sentence, recorded collision check), changes `Reconstruction trial` to
  `rejected` with its reason, and updates the counts to six confirmed, three
  candidates (`Q14`–`Q16`), and six rejected. `eventstorming.md` carries the
  rename in the domain line, the confirmed-terms intro, the state-holder table,
  the chain D heading and lead, rows D1/D3/D6, the D1 canvas event label, the
  invariants (including the builder-subject rewrite "the builder in an Ise Test
  never edits its frozen inputs"), and a closed `Q13` row in the
  question-dependent table. `frame.md` closes `Q13`, opens `Q14`, and renames
  the G5 "rebuild trial" reference. Rename evidence: `Reconstruction trial` had
  22 word occurrences before the sweep; after it 20 remain, all history (the
  terminology note, the closed `Q13` body and resolution, the `domain-terms.md`
  rejected row and its collision-check command, the dated `session.md` entries,
  and the dated `agent--technical-role` entry), and `frame.md` and
  `eventstorming.md` hold 0. The `agent--technical-role` topic file gained a
  dated 2026-11-14 entry instead of a rewrite. Canvas D1 was re-rendered with
  `mmdc` (exit 0, 258,541 bytes) and the SVG carries `ED1: The Ise Test inputs
  were frozen`; no other canvas label changed. `domain-terms.md` first-use
  cells were repointed (`Stack recipe` `eventstorming.md:33`, `End-to-end
  (E2E) test` `:34`, `Mismatch triage` `:39`, `Change strategy` `:74`).
  `Q14` was appended for the next dialog.
- 2026-11-14: User closed `Q14` with **Option 1: `Spec gap`** (Ise's phrase
  "you have found a gap in the spec" as the umbrella name for the four-way
  classification record), accepting the limitation that only the
  `specification defect` outcome is a gap in the Specification; the definition
  records the limitation in one sentence and lists exactly four outcome values
  (`E2E test defect`, `implementation defect`, `specification defect`,
  `acceptable undocumented change`). `domain-terms.md` adds the confirmed
  `Spec gap` row (genus + differentia, contrast with `Disposition` and
  `Coverage gap`, the missing-exit-code sentence, recorded collision check),
  changes `Mismatch triage` to `rejected` with its reason, and updates the
  counts to seven confirmed, two candidates (`Q15`-`Q16`), and seven rejected.
  `eventstorming.md` carries the rename in the confirmed-terms intro, the
  state-holder table, row B3, the chain D heading and lead, rows D4-D6, the
  D2 heading and canvas, the reverse narrative, the invariant, the
  question-dependent table, and the `agent--technical-role` advisory reaction.
  `frame.md` carries it in Problem 4, scope capability 3, gap G6, and the
  open-question state. Rename evidence: `Mismatch triage` had 32 word
  occurrences across 25 lines before the sweep; after it 24 word occurrences
  across 18 lines remain, all history (the terminology note, closed `Q14`
  body and resolution, the `domain-terms.md` rejected row and its
  collision-check command, the closed `Q12`/`Q13` bodies, and dated
  `session.md` Progress entries plus the closed queue row), and the living
  artifacts (`frame.md`, `eventstorming.md`, `personas/*.md`) hold 0. Canvas
  D2 was re-rendered with `mmdc` (exit 0, 209,137 bytes) and the SVG carries
  `ED4: A difference was observed`, `C: record the spec gap`, `ED5: The spec
  gap was classified`, and `Record: spec gap record`. `domain-terms.md`
  first-use cells were repointed (`Spec gap` `eventstorming.md:25`, `Stack
  recipe` `:35`, `End-to-end (E2E) test` `:19`, `Change strategy` `:76`).
  `Q15` was appended for the next dialog.
- 2026-11-14: User closed `Q15` with **Option 4: add no new term**: `Stack
  recipe` is rejected, and Givn's existing `Instruction` plus the id naming
  convention (`<id>-examples`, `<id>-<stack>`) carry the on-demand
  instruction. The accepted consequence is that no short handle exists:
  sentences say "the stack instruction", "the Rust instruction", or name the
  served id (`steps-rust`, `dialog-examples`), and the pairing rule (`steps` ->
  `steps-rust`, `dialog` -> `dialog-examples`) replaces the term.
  `domain-terms.md` changes the `Stack recipe` row to `rejected` with the
  no-term reason, contrast, and recorded collision check, and the counts become
  seven confirmed, one candidate (`Q16`), eight rejected. `eventstorming.md`
  carries the removal in the confirmed-terms intro, the state-holder table,
  the chain B heading and lead, rows B1/B2, the canvas B heading, the
  `C_STACK` command and `EB2` event labels, the chain B caption, invariant 3,
  the question-dependent table (new closed `Q15` row), and the
  `agent--technical-role` advisory reaction. `frame.md` carries it in the
  Vision, scope capability 2, gap G2, and the open-question state. Rename
  evidence: `eventstorming.md` held 12 `recipe` words across 12 lines
  (`Stack recipe` on 7 of them), `frame.md` held 4, and the remaining 2 are
  dated `agent--technical-role` record entries kept as history per the sweep
  rule; after the sweep `frame.md` and `eventstorming.md` hold 0, and the 36
  `stack recipe` occurrences left in the topic sit in `questions.md` (18),
  `session.md` (14), and the `domain-terms.md` rejected row (4). Canvas B was
  re-rendered with `mmdc` (exit 0) and the SVG carries `C: request the stack
  instruction`, `EB2: A stack instruction was served`, and
  `EB4: A change strategy was selected`. `domain-terms.md` first-use cells
  were repointed (`Change strategy` `eventstorming.md:79`; the others did
  not move). `Q16` was appended for the last dialog.
- 2026-11-14: User closed `Q16` with **Option 2: Keep `Change strategy`**,
  the recorded choice among incremental modification, `Constant Ise`, and a
  whole-product rebuild, held as the design's `Decides D<n>` entry with the
  `Ise Test` mode (`Dry-Run Ise` or `Full replacement`) beside it when a
  rebuild runs and no new mandatory field. Accepted consequences: the noun
  names a decision the existing ledger already records, `design.md` gains no
  mandatory field, every mode sentence says "Ise Test mode" because
  `Full replacement` sits one word from a whole-product rebuild, and prose
  keeps `change strategy` qualified because "strategy" is generic.
  `domain-terms.md` confirms the row (genus + differentia, contrast with
  `Change` and with the two rebuild values, the parser-swap sentence, the
  recorded collision check) and the counts become eight confirmed, no
  candidates, eight rejected: the Event Storming term set is closed.
  `eventstorming.md` carries the confirmation in the confirmed-terms intro,
  row B4's policy, and a closed `Q16` row in the question-dependent table;
  canvas B needed no label change because it already read
  `C: record the change strategy` / `EB4: A change strategy was selected`,
  and its re-render exits 0 (201,854 bytes) with both labels present. Event
  Storming exit checks: all 20 `E`-marked event labels (`EA1`–`ED6`) remain,
  the four used aggregates (`Project configuration`, `Change`,
  `Effective guidance`, `Permanent specification corpus`) are the declared
  boundaries, each command has one issuing actor, the `Q3`–`Q6` qualifiers
  remain (8 `[Qn]` markers), and `givn lint` reports 58 files clean.
  `Change strategy` is kept, so no rename sweep ran; the term now appears in
  every living artifact (`frame.md` 2, `eventstorming.md` 7,
  `domain-terms.md` 11, and `questions.md` 57 including the closed dialog),
  and the `domain-terms.md` first-use cell moved from `eventstorming.md:79`
  to `:31` because the intro now names the term. `Q3` was de-staled against
  `src/scm.rs`, `src/git.rs`, `src/fossil.rs`, `src/proof.rs`,
  `src/cli/check.rs`, and `assets/default-manifest.yaml`: revision identity,
  clean state, and revision-tree readers exist on both backends, no command
  uses the tree readers, and `givn check` has no `ise` artifact. `Q3` is the
  next dialog.
- 2026-11-14: Recorded the user's six settled constraints under "Settled
  constraints" and folded them into the still-open `Q3`, `Q4`, and `Q5`.
  `Q4` was rewritten as the `Specification`-set shape and declaration-site
  question with three options, per-option integrity-warning coverage, recorded
  collision checks, and a recommendation, after verifying the configuration
  facts: `OverlayDoc` has no `deny_unknown_fields` (`src/config/overlay.rs:535`),
  `validate_config_ownership` refuses only `verify`/`run`/`coverage` (`:120`),
  `POLICY_KEYS` is the eight-key list (`:28`), `Verify` holds `e2e_command`
  (`src/config/manifest.rs:107-117`), `Coverage.e2e_path` is a Cobertura
  report path, not a test root (`:221-243`), `assets/config.yaml.tmpl`
  scaffolds only `givn_config_version`, `givn/commands.yaml` scaffolds
  `verify`, `coverage`, and `run`, `givn lint` renders advisory findings
  without changing its exit codes (`src/cli/lint.rs:537-566`), and
  `givn check` refuses on every finding. Contradiction audit:
  `eventstorming.md` rows D1-D4, the frozen-Specification state-holder row,
  canvas D1, the reverse narrative, the read-only invariant, and the `Q4` row
  of the question table claimed a frozen Specification and unqualified
  neighbor changes; `frame.md` Problem 4, the two-layer vision bullet, the
  external-contributor stakeholder row, and G5/G6 claimed frozen
  specifications, frozen component scenarios, and "step glue"; `domain-terms.md`
  `Ise Test` and `Spec gap` carried the same frozen-Specification wording. All
  now state a read-only Specification, an E2E-only acceptance freeze, declared
  set arithmetic, a residue count, and review-gated blast radius. All five
  canvases were re-rendered with `mmdc` (exit 0, 288,199 / 201,922 / 326,750 /
  257,154 / 209,069 bytes); canvas D1 carries `C: freeze the E2E tests`,
  `R: Specification declaration`, `Record: frozen E2E test digest`, and
  `C: build from the declared Specification`; all 20 `E`-marked event labels
  remain; `givn lint` reports 58 files clean (exit 0).
- 2026-11-14: User closed `Q4` with **Option 2: a dedicated
  `givn/specification.yaml`**, corrected on the spot: the file declares exactly
  two things, the five retained roles (`behavior`, `structure`, `contracts`,
  `limits`, `e2e`) as path globs and one `source:` list of replaceable roots
  (production code plus component-test roots), with no `components:` key and no
  per-component file list because internal components and their files do not
  belong in a permanent project declaration. The replaced unit is trial input:
  `givn ise prepare --at <revision> --remove <path>` records the removal set in
  the trial receipt and refuses a path outside the declared `source` roots;
  residue is computed at trial time from the removal set's directories. The
  project-level warnings are four: declared path missing; path declared in two
  roles; `.feature` outside the `behavior` globs; E2E target missing or with a
  dependency edge into `source`. Rejected: Option 1 because the lenient
  `OverlayDoc` top level silently ignores a misspelled block name, so a
  disabled declaration produces no finding, and `givn/config.yaml` already
  carries `artifacts:` and `apply:`, so a third structural block behind a
  lenient top level makes an invisible declaration more likely; Option 3
  because convention cannot express the `source` partition, so deletion
  becomes the inference constraint 1 forbids, and three of the four warnings
  cannot exist. `questions.md` records the full option set and resolution;
  `frame.md` names the file in scope capability 4, gap G3, and the settled
  constraint, and moves the active question to `Q5`; `eventstorming.md` names
  it in the Specification state-holder row, row C1, row D1's trial removal set,
  row C2's settled dependency warning, the Specification invariant, and the
  closed `Q4` row of the question-dependent table; `domain-terms.md` first-use
  cells were re-checked against the edited files and still point at the same
  lines (`frame.md:61`, `:65`, `:74`, `:89`; `eventstorming.md:6`, `:19`,
  `:25`, `:31`). No Mermaid block changed, so no canvas was re-rendered.
  `Q5` was then appended with the verified verifier facts (`givn/commands.yaml`
  `verify` block; one shared `features_runner` binary for both scopes; 108
  `@e2e` scenarios in 51 files; 8 `use givn::` step files, 33 referencing
  `givn::`, 22 of them serving `@e2e` steps), the exact `specs.md` sentences at
  `:137-140` and `:164-169`, and the contradiction check against
  `prose-consolidation.feature:124-135` and `create-change/usecase.md:46,48`.
- 2026-11-14: User closed `Q5` with **Option 1, narrowed to an `e2e`
  package**: a separate E2E package holds the runner and every `@e2e` step
  definition, is declared as the `e2e` role in `givn/specification.yaml`, and
  links no production source; `verify.e2e_command` runs it and fails on an
  unbound or skipped step; feature files stay shared, so `@e2e` and component
  scenarios may live in one `.feature` file. The phrase-uniqueness variant is
  rejected with the user's reason recorded in `questions.md`. The user closed
  `Q3` with **Option 2**: `givn ise prepare` and `givn ise verify`, bound to
  `addons.ise`, with the full input, refusal, step, report, and verification
  contract recorded in `questions.md`. The earlier Q5 draft stays as a closed
  superseded record so the identifier resolves once. Sweep: `frame.md`
  capability 4 and gaps G4/G5/G6, `eventstorming.md` rows A1/B4/C2/C3/C4/D1/D3,
  the state-holder harness row, the invariants, the question table, and the
  maintainer persona reaction carry the two closures; canvases A and D1 were
  re-rendered with `mmdc` (exit 0, 288,125 / 257,105 bytes) and carry
  `Config: per-addon default posture` and `C: givn ise prepare` with every
  expected event label; the `/tmp` copies were removed; `domain-terms.md`
  first-use cells were re-checked and still point at `frame.md:61,65,74,89`
  and `eventstorming.md:6,19,25,31`. `Q6` was rewritten as the coupling
  question, with the verified addon, lock, template, and
  `architecture_principles` facts and a recommendation for no coupling.

- 2026-11-14: User closed `Q6` with **Option 2: `ise` refuses to enable
  without `architecture_principles`**, chosen so the `ise` guidance uses the
  established vocabulary (`Component`, contract, `Specification`, `Source
  code`) instead of invented nouns. Consequences: enabling `ise` refuses by
  name until the prerequisite is on; disabling `architecture_principles`
  refuses while `ise` is on, naming `ise` first; `config-lock.yaml` records
  `ise: false`, and a mid-change enable needs `--operator --reason` as any
  addon change; `Component` stays defined only in
  `assets/instructions/architecture-principles.md` with no glossary promotion.
  The no-coupling options (unconditional and conditional wording) and
  auto-enable are rejected in `questions.md`. Sweep: living artifacts no longer
  use `removal set`, `residue`, `unmatched`, or `blast radius`; sentences say
  "the component's paths", "the paths named with `--replace`", "files in those
  directories you did not name", "files outside the declaration", and
  "neighboring components", and the trial report lines are `Replaced`, `Not
  replaced`, and `Outside the declaration`. `frame.md`, `eventstorming.md`,
  and the live sections of `session.md` carry the wording; canvas A gained
  `EA6: An addon change was refused by its prerequisite`; `givn ise prepare
  --at <revision> --replace <path>` replaces the earlier `--remove` draft in
  living text. `use-case-definitions.md` is drafted: new leaves
  `declare-the-specification` and `prepare-an-ise-test`, modifications to
  `initialize-project`, `upgrade-a-project`, `create-change`, and
  `verify-change`, and the `support` fragment's `instruction-integrity`
  capability.
- 2026-11-14: Use Case Definition boundary review recorded. The `support` row
  now names the fragment and its `instruction-integrity` capability correctly;
  `initialize-project` records that a fresh init cannot make `ise` effective;
  `upgrade-a-project` records the actual staged set and the extension the
  declaration scaffold needs; `verify-change` moves the four declaration
  findings to `givn lint`; the new leaves gained a boundary and a written
  `declare-the-specification` flow. The eventstorming A1/B3 policies and the
  advisory terminology were corrected. One directional finding was recorded:
  the Event Storming Chain C/D actor labels named `Givn`/`Givn maintainer`
  where the contracts name the Developer; it was applied 2026-11-14 (see the
  Use Case Definition review).
- 2026-11-14: Use Case Elaboration complete. Eight documents were written by
  four independent contexts and consolidated in one review covering schema,
  identity and actors, settled-fact agreement, contract quality, banned
  vocabulary, personas, and stakeholders. Mechanical corrections: the
  `create-change` and `instruction-integrity` preambles before `## Level` were
  removed; the fragment title lost its `(fragment)` suffix; the
  `instruction-integrity` Goal was reduced to one outcome; the
  `obtain-effective-guidance` capability now references the obligations and
  content `instruction-integrity` owns and lists the maintainer persona; the
  `declare-the-specification` main flow and success guarantee no longer name
  the `source:` key; the `prepare-an-ise-test` refusal step no longer says
  `ise` off; the `verify-change` diagram names the Developer, its primary
  actor; eight em-dashes were removed from `use-cases/` and one from
  `use-case-definitions.md`; `givn ise prepare`'s optional flags are written
  with their brackets; the frozen acceptance authority is named `package`
  throughout, matching `verify-change`; the initialization success guarantee
  leaves the declaration project-owned instead of claiming it is never
  rewritten; the declaration flow reads the current declaration before the
  save. The eventstorming `Q3` command cell and the Frame's
  active-step line were aligned with the completed phase. Two directional
  findings remain: the optional `--receipt` against the always-written trial
  record, and the mid-trial `E2E test defect` restart wording in the settled
  constraint.
- 2026-11-14: Handoff applied. Published the readable topic document to
  `docs/ideation/ise-rebuildability.md` and added it to `docs/INDEX.md` under
  Historical Handoffs. Promoted the three confirmed personas,
  `developer--primary-user`, `agent--technical-role`, and
  `givn-maintainer--owner`, by appending the section 4 record entries and
  2026-11-14 closure lines for `Q3`, `Q6`, and `Q14`; no earlier entry was
  rewritten. The six change seeds stay `Status: ready` with their dependencies
  and suggested start order unchanged. The term merge is deferred: no
  `docs/glossary.md` row was merged, and the exact eight rows, six rewrites,
  five anti-term additions, two scenario titles, the arc42 sentence, and the
  two `assets/instructions/specs.md` amendments with their pinned scenarios
  are recorded in `add-ise-addon-and-specification` and
  `enforce-ise-e2e-boundary`. No change directory was created.

## Frame boundary review (2026-11-13)

Rubric from `givn instructions ideation`: research-protocol completeness,
stakeholder list with interests.

| Check | Result |
|---|---|
| Research protocol records what exists | Present: Ise primary source, addon registry, architecture/domain guidance, instruction delivery and size, verification/receipt machinery, Givn acceptance boundary, `givn-harness`, Passuff baseline. |
| Research protocol records what the permanent specs already own | Present: seven spec-owning areas named with the feature that owns each. |
| Research protocol records the gaps | Present: `G1`–`G7`. |
| Stakeholder list with interests | Present: four stakeholder types, each with a stated interest; `Q7` folded the external contributor lens into the maintainer. |
| Problem names one sentence per gap | Present: four numbered gaps. |
| Vision states what changes for the stakeholders | Present: Replacement package, two-layer test boundary, strategy selection, empirical proof before default. |
| Scope boundary separates Givn from companion repos | Present: Givn capabilities 1–4 in scope; `givn-harness` and Passuff recorded as companion validation tracks; five exclusions listed. |
| Cited repository facts reproduce | Verified by command: 9 addons with all defaults `true` (`src/config/manifest.rs:313`), 21 instruction files and 4,397 lines, 9 `tests/steps/` files with `use givn::` (14 occurrences), `TC-2`, `TC-3`, `TC-14`, `TC-31`, `TC-33` present, `src/integrity_hook.rs:442-449` checks only transcript non-emptiness, `GIVN_RESULT_FILE` used by `src/cli/check.rs` and `src/cli/archive.rs`. |

Corrections applied during the Frame review:

- Passuff evidence: `crates/passuff-cli/tests/steps/tui.rs` calls
  `passuff::tui::keys::parse_keys` (no glob import) and
  `crates/passuff-cli/tests/support/world.rs:572` builds the inbox through
  `InboxService`; `scripts/verify.sh:57` was cited as the shared runner line.
- `givn-harness` evidence: paths qualified to
  `benchmarks/catopia-v2/public/task-followup.md:28` and
  `benchmarks/catopia-v2/private/grader/run_grader.py`; the grader marks
  `P*/T*/M*/A*` ungraded through `_petorania_supported`
  (`run_grader.py:463-471`, `518-523`); legacy data is proven by importing
  `public/legacy-export.csv` into the graded workspace (`check_r12`), not by a
  Stage 1 data directory carried forward.
- `Q4` citations: `domain-modeling.md:271` (not `:237`) owns the no-DB-schemas
  and no-class-names rule; the nonexistent `TC-2.6` reference became the
  one-authoritative-definition rule at `domain-modeling.md:24` and `:282`.
- `Q3`–`Q6`: `Status` normalized to the schema value `open`; the phase that
  settles each question moved into its `Blocks` line.
- `domain-terms.md`: `First use` cells point at real occurrences; collision
  checks record reproducible `grep -ric` commands and their real results.

## Event Storming review (2026-11-13)

Rubric from `givn instructions ideation`: symmetric cause and effect, missing
event partners, a canvas showing every event, command, aggregate, and actor
with distinguishable kinds, every aggregate a declared consistency boundary,
line/picture agreement, a canvas that renders, and a recorded collision check
per `confirmed` term.

| Check | Result |
|---|---|
| Aggregates are declared consistency boundaries | Four used of the six declared in `docs/domain-model.md`: `Project configuration`, `Change`, `Effective guidance`, `Permanent specification corpus`. `Development record` and `Persona collection` are declared and untouched. No aggregate was minted. |
| Non-aggregates are drawn as their own kinds | Replacement package, Stack recipe, Black-box verifier as `Spec`; lock file, receipts, frozen package and verifier digests, triage record, and the per-addon default posture as `Record`/`Config`; reconstruction harness as `X`. Each canvas carries an aggregate caption, and the state-holder table states why each is not an aggregate. |
| Every numbered event is drawn | 20 events (`A1`–`A5`, `B1`–`B4`, `C1`–`C5`, `D1`–`D6`) across five canvases, each label carrying `E` plus its row (`EA1`–`ED6`); all 20 labels were found in the rendered SVGs. |
| Event partners exist | Every event has a triggering actor and a named command. `D4` is marked `[consequence]` and records the observation on the `Change` trial. `A5` and `C3` are refusals: `Givn [guard]` issues no command, so each guard reaches its event by a dotted edge while one actor issues the command. |
| Canvas size | 16, 12, 15, 14, 10 nodes; all within the 2–20 bound. |
| Renders | 5 of 5 `mmdc` runs exit 0 with `PUPPETEER_EXECUTABLE_PATH=/usr/bin/google-chrome`; SVG output 202–327 KB. |
| Reverse narrative | `ED5` walked back through `D4`, `D3`, `D2`, `D1`, `C4`, `C1`, `C5`, `B3`, `A4`, `A2`, `A1` to project initialization. |
| Invariants | 11, each naming its enforcement site; `Q3`–`Q6` qualify the affected ones. |
| Question-dependent behavior | Table present for `Q3`–`Q6` (open) and `Q7` (closed 2026-11-13). |
| Persona reactions | Light advisory reactions from `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`. |
| Term collision checks | Eight candidate rows, none confirmed. The addon posture's placeholder `Opt-in addon` collides with the glossary anti-term `opt-in default` and with "an addon switched on" in `match-guidance-to-enabled-addons.feature`; its naming dialog is `Q8`, and the other seven terms ride the term check `Q9`. (After this review, `Q8` closed 2026-11-13 with `Opt-in addon`; see Progress.) |

Corrections applied during the Event Storming boundary review:

- **One issuing actor per command:** `A5` and `C3` previously shared a command
  node with the Developer, giving `C: toggle an addon` two issuers. The guard
  now reaches its event by a dotted refusal edge, and each command keeps one
  issuer.
- **Policy replaced by the configuration it read:** canvas A put a `P` node
  between the command and the aggregate to express "an absent key resolves
  false". That is a lookup, not a reaction rule; the node is now
  `Config: per-addon default posture, open Q6` with a dotted read into
  `givn init`, which also draws A1's recorded read model.
- **Event markers:** event labels gained the `E` marker (`EA1` ... `ED6`) so
  the legend and the picture name the same kinds.
- **Frozen verifier kind:** the frozen verifier was drawn as `Spec` while the
  state-holder table classifies the frozen snapshot as a record; it is now
  `Record: frozen black-box verifier digest`.
- **A4 read model:** the row named the written `config-lock.yaml`; it now names
  `Effective addon set`, which `givn new` reads.
- **Chain/aggregate map:** the intro listed Chain B without `Change`, which
  row B4 uses; each chain now names both boundaries it touches.
- **Addon name:** the canvas no longer relies on the contested placeholder; the
  instance is named by its registered id (`ise` addon). `Q8` closed 2026-11-13
  with `Opt-in addon` as the posture's name.
- **First-use lines:** `domain-terms.md` was re-pointed after the edits
  (`Stack recipe` `eventstorming.md:25`, `Change strategy` `:66`, `Mismatch
  triage` `:31`).

## Use Case Definition review (2026-11-14)

Fresh-context boundary review against the `Use Case Definition` rubric;
findings advisory, mechanical corrections applied.

| Check | Result |
|---|---|
| Sizing | The two new leaves each carry one verb phrase and one primary actor. `prepare-an-ise-test` stays one goal: its success guarantee names the trial and the verifier's confirmation of that same trial, not a second actor goal, and `verify-an-ise-test` remains rejected as a separate leaf. The `+` summaries own no direct capabilities; their children carry the delta. |
| Goal shape | The summary goal deltas no longer name `givn/specification.yaml` or `givn instructions ise`; file, command, and schema names sit in the capability sections. |
| Level discipline | Each `!` owns its capabilities directly. `support` is the fragment at `givn/specs/fragments/fragment.md:1` with 21 capabilities, including `instruction-integrity`; it is not a use case. The `+` summaries own none. |
| Mutation containment | `declare-the-specification` changes the declaration; `prepare-an-ise-test` changes the trial directory and record, and its observing `trial-input-verification` capability sits under the changing goal. |
| Extension discipline | Every refusal branches from the validation or write step and protects the minimal guarantee; no extension adds a new actor goal or capability. |
| Preconditions and flow | `declare-the-specification` had a trigger (the developer enables `ise`) that contradicted its `ise`-effective precondition; the trigger is now the declaration edit, the missing write step is added, and the success guarantee no longer claims trial-time `--replace` resolution. |
| Implementation independence | Goal, trigger, flows, and guarantees use domain language; the flow now says recorded identities and repository links, while the rules keep the concrete digest and Git terms. |
| Relationship semantics | `Includes` is unconditional; the new leaves declare none, matching the four permanent use cases that declare none; no `Extends`; the graph is acyclic. |
| Boundary and stakeholders | Both new leaves gained a Boundary line; the three confirmed personas and their tags are unchanged. |

Corrections applied:

- The summary row is `support` (fragment), not a use case. The file
  `givn/specs/fragments/fragment.md:1` is `# Fragment: support`, with 21
  capabilities including `instruction-integrity`
  (`givn/specs/fragments/instruction-integrity.feature`). The capability moved
  into the delta, the false "included by every use case" claim was replaced
  (four permanent use cases declare no Includes), and the fragment consumes
  B1-B3; B4 is the design decision owned by `create-change`.
- `initialize-project`: a fresh init cannot make `ise` effective. The registry
  default is `false`, no addon-selection flag exists, and
  `src/cli/init.rs:150-155` scaffolds only addons whose declared default is
  `true`, so a fresh init writes no declaration. The scaffold arrives through
  `givn addons enable ise` (`src/cli/addons.rs:103-115,132-176`) or a forced
  re-init restoring an explicit `ise: true` choice
  (`src/cli/init.rs:59-62,138-149`). The delta now also names the `Addon` row
  at `docs/glossary.md:8` and the all-defaults rule at
  `givn/specs/initialize-project/usecase.md:57`, beside `TC-14` and the
  template lines.
- `upgrade-a-project`: the staged set is `givn/config.yaml`,
  `givn/commands.yaml`, the ignore files, and `managed_generated_paths`
  (`src/cli/migration.rs:620-645`; `src/skills/targets.rs:575-619`), and
  `apply_managed_upgrade` writes only the config and the commands backfill
  (`src/cli/migration.rs:567-608`). The declaration scaffold therefore extends
  both the write and the staged set; it is not already allowlisted. The stale
  A4 consumption was removed.
- `verify-change`: `givn check review` bails on every finding
  (`src/cli/check.rs:193`), while `givn lint` carries `Severity::Advisory` and
  advisory findings keep exit 0 (`src/cli/lint.rs:537-571,284-293`). The four
  declaration findings move to `check-specification-health` (`givn lint`); the
  blocking `@e2e` boundary stays in `givn check tasks`.
- `declare-the-specification`: the trigger is the declaration edit; the flow
  writes the declaration without exposing a partial role set; the success
  guarantee covers only what the flow establishes.
- `create-change`: `change-strategy` and `spec-gap` match the existing
  kebab-case noun capability convention (`prose-consolidation`,
  `change-release-note`, `visual-design-contract`) and each gets a same-named
  `.feature`; no change. The consumed C4 event was replaced by D4, the observed
  difference the classification consumes.
- `maintain-project` covers three children, not one; the omitted
  `inspect-development-history` and `make-usecases-canonical` join the
  unchanged list.
- `eventstorming.md`: A1 now says a fresh init omits the `ise` key; B3 adds the
  `Change strategy` obligation; the invariant and Q4 row say advisory findings.

Directional findings (not applied):

- The Event Storming actor labels differ from the use-case primary actors:
  C2/C3/C4 name `Givn` as the command issuer and D1 names `Givn maintainer`,
  while the contracts name the Developer (the Givn maintainer is the
  self-rebuild case). Correcting the closed canvases needs a separate Event
  Storming edit and re-render.
- The confirmed Frame's scope capability 4 says `givn check` validates the
  declared boundary. The corrected mapping keeps the blocking `e2e` boundary
  in `givn check tasks` and puts the four advisory declaration findings in
  `givn lint`; the Frame stays true for the blocking check and was not edited.
- Applied 2026-11-14: the Event Storming actor labels and the Frame capability-4
  sentence were corrected; canvases C and D1 re-rendered with `mmdc` (exit 0,
  expected labels present).

## Handoff corrections (2026-11-14)

- `--into` is required and lies outside the checkout (`plan.md` `S4`); the
  clean-room seed records it as an accepted decision, and
  `use-cases/prepare-an-ise-test.md` carries the required form.
- `S13` yields two seeds, `move-givn-e2e-to-black-box-package` and
  `enable-ise-on-givn-and-run-a-dry-run`; `S14` is a decision checkpoint with
  no seed. The handoff records six seeds.
- The four unsettled items live as open questions in their owning seeds:
  manifest coverage and the proof scope in `enforce-ise-e2e-boundary`, the
  stack scope in `split-stack-instructions`, and the `ise` instruction
  ownership in `add-ise-addon-and-specification`.
- Both `S13` seeds make the external evidence from `plan.md` `S7` to `S12` a
  prerequisite for starting.

## Next steps

Handoff completed 2026-11-14. The user runs `/givn-propose` per seed in start
order: `add-ise-addon-and-specification` first; then
`enforce-ise-e2e-boundary` and `split-stack-instructions` in parallel; then
`add-ise-clean-room-commands`; then `move-givn-e2e-to-black-box-package`;
then `enable-ise-on-givn-and-run-a-dry-run`. Nothing was auto-started, no
change directory was created, and the term merge waits for the seeds that
make each row true.

## One-by-one term queue

`Q9` closed with "revisit all seven". Each row is one question. The middle
column is the exact Ise wording; where Ise has no word, the row says so and the
question is whether the concept needs a name at all. Occurrence counts are
across this topic's `frame.md`, `eventstorming.md`, `questions.md`,
`session.md`, `domain-terms.md`, and `personas/`.

| Q | Rejected / current term | Ise wording or status in the source | Occurrences | Note |
|---|---|---|---|---|
| `Q10` | `Replacement package` → `Specification` | `Specification` vs. `Source code` ("the specification is the source of truth, and the source code is replaceable") | 32 before, 0 in the Frame and the canvas after | Closed 2026-11-13 (user): `Specification`, contrasted with `Source code`; `Source code` added as a confirmed term |
| `Q11` | `Black-box verifier` → `End-to-end (E2E) test` | `End-to-end (E2E) tests` (kept unchanged) and `Component tests` (scenarios kept, test setup rewritten) | 15 before, 0 in the Frame and the canvas after | Closed 2026-11-13 (user): `End-to-end (E2E) test` with `black-box` as the no-shared-code qualifier; `@e2e`, `verify.e2e_command`, and `Verification scope` keep their names; `Black-box verifier` kept as rejected |
| `Q12` | `Component replacement` → `Constant Ise` | `Constant Ise` (Pattern 01) | 14 before, 0 in the Frame and the canvas after | Closed 2026-11-14 (user): `Constant Ise`; `Component replacement` kept as rejected; "unit"/"module boundary" mapped to Givn's `Component` (`assets/instructions/architecture-principles.md:74`) |
| `Q13` | `Reconstruction trial` → `Ise Test` | `Ise Test` (the act), `Dry-Run Ise` (throwaway build), `Full replacement` (production switch) | 22 before, 0 in the Frame and the canvas after | Closed 2026-11-14 (user): `Ise Test`, with `Constant Ise` as the one-unit scope and `Dry-Run Ise` / `Full replacement` as the two execution modes; `Reconstruction trial` kept as rejected |
| `Q14` | `Mismatch triage` → `Spec gap` | No Ise noun; Ise Rule 2 ("Anything not in the spec is free to change"), the "gap in the spec" reading in `How to Rebuild`, "Fix the spec first", and Pattern 05's verb "triage behavioral diffs into spec rules vs. quirks" | 32 before, 0 in the Frame and the canvas after | Closed 2026-11-14 (user): `Spec gap`, the classification record with exactly four outcome values (`E2E test defect`, `implementation defect`, `specification defect`, `acceptable undocumented change`); its definition records that only `specification defect` is a gap in the Specification; `Mismatch triage` kept as rejected |
| `Q15` | `Stack recipe` | No Ise counterpart. Givn's own id convention (`<id>-examples`, `<id>-<stack>`) and idiom set | 28 before, 0 in the Frame and the canvas after | Closed 2026-11-14 (user): no new term; `Instruction` plus the id convention carry the concept, so sentences say "the stack instruction" or name `steps-rust` / `dialog-examples`; `Stack recipe` kept as rejected |
| `Q16` | `Change strategy` | No Ise noun. Ise Rules 1–3 plus the pattern choice (`Constant Ise`, `Dry-Run Ise`, `Strangler Ise`, full rebuild) | 36, unchanged (name kept) | Closed 2026-11-14 (user): `Change strategy`, the recorded choice among incremental modification, `Constant Ise`, and a whole-product rebuild, held as the design's `Decides D<n>` entry with the `Ise Test` mode (`Dry-Run Ise` / `Full replacement`) beside it when a rebuild runs and no new mandatory field; no term (ledger only) and `Rebuild scope` rejected; the one-hit collision is the verb phrase at `assets/instructions/start.md:137` |

Sweep rule: nothing in `eventstorming.md`, `frame.md`, or `use-case-definitions.md`
is renamed until its own question closes, so each concept is written once under
its final name. Persona files keep their dated record entries as written; a
renamed term gets a new dated entry, never a rewrite of the old one.

`use-case-definitions.md` was written after `Q6` closed. The term set
is closed: `Q10`–`Q16` settled as `Specification` (with `Source code`),
`End-to-end (E2E) test`, `Constant Ise`, `Ise Test`, `Spec gap`, no new term
for the on-demand instruction, and `Change strategy`; `domain-terms.md` has no
candidate rows.
