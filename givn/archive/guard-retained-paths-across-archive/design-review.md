# Design Review: guard-retained-paths-across-archive

Review kind: independent
Author: guard-retained-paths-across-archive hardening author (omp, deepseek-flash)
Reviewer context: independent grilling session whose decisions Q1–Q5 the operator recorded on 2026-11-15; this authoring context applied them and owns the artifact edits

## Reviewed inputs

Read for this review: `proposal.md`; the permanent
`givn/specs/archive-change/usecase.md` and its base digest; the delta
`specs/archive-change/usecase.md` and both delta features; `design.md`;
`arc42.md`; `config-lock.yaml`; `docs/glossary.md`; `docs/domain-model.md`; the
durable docs this change touches (`assets/instructions/archive.md`,
`assets/instructions/ise.md`, `docs/reference.md`,
`docs/arc42/{05,06,08,11,README}.md`); and the code the design names
(`src/cli/archive.rs`, `src/config/declaration.rs`, `src/scm.rs`,
`src/git.rs`, `src/fossil.rs`, `src/e2e_boundary/{mod,paths}.rs`,
`src/ise/partition.rs`, `src/ise/verify.rs`, `src/integrity_hook.rs`,
`tests/steps/archive_proof_support.rs`).

No implementation exists in this pass: `src/` is unchanged, and no
`tasks.md` or `review.md` was written. The review verdict below is the plan's.

## Blockers closed

### B1 — Wrong base digest (closed)

- **Finding.** The delta's `givn:base-sha256` marker carried
  `9110d19960efb32af61660066b22987abc19105c975a6282f5f36fd9f0a54396`, which is
  the first permanent file's marker text, not its digest. A merge would have
  compared the permanent narrative against a non-digest.
- **Fix.** Delta line 1 now records
  `1d7d8d8bd7db3ed06a2afb4401b2c4a001bf2458fc55ece51b418424b5751e82`, the
  current `sha256sum givn/specs/archive-change/usecase.md`.
- **Landing.** `specs/archive-change/usecase.md:1`; the recompute rule is
  recorded at `design.md:71-76` and must be carried into `tasks.md`.
- **Verification.** `sha256sum` and `head -1` outputs are under
  "Verification run" below.

### B2 — Interaction Coverage Matrix had four rows for one inventory entry (closed)

- **Finding.** The matrix listed the success action plus the refusal,
  broad-root, and `ise`-off outcomes as four rows. The owning use case holds one
  inventory entry (`specs/archive-change/usecase.md:148`), and the specs
  instruction makes the other outcomes variants.
- **Fix.** One row in `design.md:444-458`, with the variants stated in prose and
  the scenario count at `design.md:460-463`.
- **Verification.** The delta use case still has exactly one
  `require-durable-retained-paths` inventory row (`usecase.md:148`).

## Decisions applied

- **Q1 — one predicate over `Backend::content_paths` (landed
  `design.md:167-173`, `design.md:191-239`, `design.md:241-251`).** An entry
  survives iff it matches a content path outside `givn/changes/<id>/` or the
  `givn/archive/<id>/` form of a content path inside it. The two earlier
  survival tests and the filesystem walk are gone. `content_paths` is the
  tracked-plus-untracked listing `givn ise verify` reads
  (`src/ise/verify.rs:199-205`); ignored files are absent, so the guard does not
  refuse for content the trial cannot read. The rule is restated in
  `assets/instructions/archive.md:52-62`, `assets/instructions/ise.md:18-23`,
  `docs/reference.md:466-474`, `docs/arc42/05-building-block-view.md:67`,
  `docs/arc42/06-runtime-view.md:416-421`,
  `docs/arc42/08-crosscutting-concepts.md:61-75`, and the use-case rules
  (`usecase.md:86-87`).
- **Q2 — advice matches the entry's shape (landed `design.md:174-178`,
  `design.md:211-216`, refusal example `design.md:221-231`).** A literal entry
  is answered with its archived literal path; a wildcard entry beginning with
  `givn/changes` is answered with the same pattern under `givn/archive`. A
  wildcard that reaches the tree through another prefix prints no replacement
  pattern, so the guard never proposes a narrower Specification than the entry.
  Scenario rows updated in
  `specs/archive-change/require-durable-retained-paths.feature:27-30`: the
  wildcard row now asserts `givn/archive/**`.
- **Q3 — declare `Interface: terminal` (landed `design.md:38-55`).** The
  contract parses as `VisualInterface::Terminal` under
  `parse_visual_contract` (the `- Interface:` value is exactly `terminal`). Two
  transcripts are committed: the failure reproduction at
  `evidence/visual/archive-is-blocked-when-a-declared-retained-path-moves-with-the-change/transcript.txt`
  and the success path at
  `evidence/visual/consumer-archives-a-change-whose-declared-retained-paths-survive-the-move/transcript.txt`.
  The `VISUAL-REVIEW: PASS` placeholder is recorded below.
- **Q4 — accept the symlink hole (landed `design.md:182-185`,
  `design.md:519-523`, `docs/arc42/11-risks-and-technical-debt.md:136`).** A
  declared retained path outside the change tree that is a symlink into it is
  invisible to the guard, and the trial cannot see through symlinks either. No
  canonicalization is added.
- **Q5 — pin the three unasserted branches (landed `design.md:186-189`,
  feature `:24`, `:32-41`, `:43-51`, counts `design.md:460-463`).** The refusal
  outline now asserts that no pending archive receipt exists
  (`feature:24`); the 21-entry fixture asserts the bounded list and `+1 more`
  (`feature:33-41`); the malformed declaration asserts the loader refusal
  (`feature:43-51`).
- **Grill corrections (landed).** The moved
  `glob_matches_path_or_parent` is added to the `pub(crate)` re-export list in
  `src/e2e_boundary/mod.rs` (`design.md:95-96`); the design states that
  `walk_project` cannot be reused because it drops `givn/changes` and
  `givn/archive` (`design.md:98-101`); the scenario titles in the design match
  the feature text exactly (design `:444-463`, feature `:5`, `:14`, `:33`,
  `:43`, `:54`, `:62`).

## Required branch walk

### Scope

The delta contains exactly the proposal's behaviour: a pre-mutation archive
refusal for retained role entries, the unaffected-declaration cases, the
`ise`-off read behavior, the served guidance, and the unchanged rebuild
refusal. Nothing from the out-of-scope list is specified: no declaration
rewrite, no layout or merge change, no `ise replace`/`verify` change, no
`source:` root refusal (D1), no pending-candidate re-check (D3), no
declaration-schema or project-health change. "Done" is unambiguous: exit
non-zero before the move with the named refused entry and archived advice, or
the move proceeds with the declaration unchanged.

Finding resolved during the walk: the proposal's sentence "matches no tracked
path that stays" was the pre-Q1 basis; the authoritative rule is the design's
content-path predicate. The proposal remains the recorded request; the design
records the refinement.

### Tech choices

The guard reuses the existing declaration loader, `Glob`, and the backend
content listing; it adds no crate, module, CLI flag, or storage. Alternatives
considered and rejected: a new guard module (one consumer, no contract), a
`revision_files` basis (misses untracked evidence that the rename moves), a
filesystem walk (cannot reuse `walk_project`; duplicates listing logic), and
symlink canonicalization (Q4). A single-file implementation satisfies the
architecture-principles exception: one external input (the declaration file),
simple comparison rules, one consumer.

### Missing scenarios

Extensions 1 (refusal, unreadable declaration), the main flow, the two rules
(pattern semantics, `ise` gating), and the fresh-only rule each have a
scenario. The three previously unasserted branches are pinned by Q5. The
`source:` branch is out of scope by D1 and has no scenario. The pending-resume
branch is out of scope by D3 and is covered by the existing archive resume
scenarios. The wildcard-advice fallback (an entry reaching the tree through
another prefix) affects only the advisory text; the refusal itself still names
the role, entry, and moved path, so it has no separate scenario and is stated
as a rule at `design.md:211-216`.

### Testability

- **Strict mode:** `.fail_on_skipped()` on the cucumber builder is named
  (`design.md:365-369`), with a setup proof that an unbound step exits
  non-zero.
- **Stub pattern:** `todo!("TODO: implement the retained-path archive guard
  step")` is named for every unasserted step (`design.md:371-375`).
- **Single-scenario run commands:** given for the refusal, bounded report,
  malformed declaration, `@e2e`, and guidance scenarios
  (`design.md:314-359`).
- **Concrete assertions:** every Then names an observable value (exit status,
  exact path text, `+1 more`, the archived file, the absence of a pending
  receipt and of a candidate). The RED direction is real: with no guard, the
  current binary archives (verified in the failure transcript), so the
  non-zero-exit and refusal-text assertions fail before the production change.

### E2E fidelity

The capability is a CLI, and the scenario spawns `CARGO_BIN_EXE_givn` as a
subprocess (`design.md:414-441`). Its primary assertion is on the CLI result:
exit 0, the moved change, and `design.md` in the archive directory. There is one
`@e2e` scenario for the one inventory entry; refusal and flag variants are
non-`@e2e`. No `@e2e` tag was removed (checked all delta features), no HTTP or
fetch substitute is planned, and the terminal transcript rule is applied
instead of screenshots.

### Interaction Coverage

The inventory lives in the owning use case
(`specs/archive-change/usecase.md:148`) and has one entry; the design matrix has
one matching row (`design.md:444-458`). Every variant named in the use-case
document maps to a scenario in the capability feature; no extra inventory entry
or extra `@e2e` scenario is claimed.

### Risk

Most likely failure: the guard refuses in an ordinary project because the
survival predicate is too strict. Mitigation: an entry that also matches a
surviving content path or an archived counterpart passes, which keeps
`givn/**`, a directory that still holds another active change, and the
recommended archived path accepted; broad-root and ise-off scenarios pin the
two loudest cases. Second risk: the fixture enables `ise` after `ready_change`;
the fixture has no `config-lock.yaml`, so no addon drift, and the generic
archive step commits the declaration and refreshes the receipt. Third risk: the
guard's fixture reaches an earlier gate; the design names the insertion point
after the receipt check and before the artifact gates, and the refusal scenario
asserts that no pending receipt was written.

### Use-case and Persona context

Use-case identity: `archive-change`, capability
`require-durable-retained-paths`, fragment `support` capability
`instruction-integrity`. Contract, capability ownership, and typed relationships
are preserved: the capability list gains the new leaf and the Interactions
table gains its row; no capability is removed or renamed. Actors remain
interaction roles.

Scope rejection criteria, checked against
`specs/archive-change/usecase.md`:

| Criterion | Result |
|---|---|
| Goal conjunction | Pass — one verb phrase: archive with proven publication. |
| Missing primary actor | Pass — Developer is the actor and the trigger names the developer. |
| Placement failure | Pass — protecting the archive's success guarantee advances the archive goal; the name is goal-shaped, not component- or channel-shaped. |
| Mutation under an observing goal | Pass — the use case mutates (moves, merges, publishes); it is not an observing goal. |
| Extension without a step reference | Pass — every extension names its main-flow step, and the new step-1 extensions add no success outcome. |
| Inconsistent preconditions | Pass — the declaration readability precondition is established before the flow, the success guarantee is not assumed, and the trigger is an action. |
| Mechanism vocabulary | Pass — Goal, Trigger, and the guarantees name the archive and the Specification, not a matcher, loader, or file reader. |
| Component- or channel-shaped name | Pass — `require-durable-retained-paths` names the retained-path survival goal. |
| Oversized leaf | Pass — `archive-change` is an end-user goal with one actor goal in its main flow; the new capability is a leaf under it. |
| Relationship misuse | Pass — the use case `Includes` the `support` fragment; `Extends` is `none`. |

Persona conformance: `developer--primary-user`, `agent--technical-role`, and
`givn-maintainer--owner` are confirmed on both owning documents and in the
proposal; the design records each one's relevance and evidence
(`design.md:57-61`). No Persona is invented or silently promoted, and no
biography appears as a Gherkin actor.

### Decision ledger

- D1 (retained roles only) resolves at `design.md:153-157`; `source:` roots are
  recorded as future scope and no scenario depends on them.
- D2 (unreadable declaration refuses) resolves at `design.md:158-162` and now
  has a dedicated scenario.
- D3 (fresh move only) resolves at `design.md:163-166`; no resume scenario
  depends on it.
- Q1–Q5 resolve at `design.md:167-189`.
- Carried decisions: none. No scenario depends on an open answer.

### Contracts and components

No new contract is declared. The change adds one internal helper with one
consumer (the archive command), reuses the `config::declaration` contract
unchanged, and moves one internal matcher to the module already owning the
shared path semantics. No consumer imports an implementation; `ise::partition`
imports the crate-internal `e2e_boundary` matcher, the existing module
convention. There is no external service and no second external boundary, so the
single-file exception applies.

### Decision timing

All decisions are foundational or explicitly out of scope. The refusal rule,
its message, the read gating, and the placement are required now because
scenarios depend on them. No carried question remains, so no default, trigger,
switch cost, or owner is required. No speculative abstraction exists for a
future question.

### Visual design contract

`design.md:37-44` contains the `## Visual Design Contract` section with
`- Interface: terminal` on its own line, viewports not applicable, states
refusal and success, and the existing stderr convention. The surface is
terminal text and exit codes, so no rendered options are required and none are
claimed. The failure and success transcripts are committed at the paths named
in `design.md:46-51`; the `VISUAL-REVIEW: PASS` placeholder is recorded below.

### Deferrals

None. No `@e2e` tag was removed, the e2e infrastructure is buildable, and no
mandatory check is deferred. `DEFERRAL:` and `OPERATOR-DECISION:` counts are both
zero.

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| `developer--primary-user` | Reads the refusal and the archived path to declare. | The refusal scenarios assert the named role, entry, moved path, and archived path; the success scenario asserts the archived file. |
| `agent--technical-role` | Runs archive in automated loops; must see the pre-mutation refusal instead of an unseedable archive. | Exit status plus the unchanged change tree and absent candidate are asserted. |
| `givn-maintainer--owner` | Owns the archive success guarantee and the guard's effectiveness rule. | The use-case rule, the archive gate list, and the delta feature record the same rule. |

### Interface classification

The opposite classification (Web UI) does not hold: the change emits terminal
text and an exit code, renders no HTML, CSS, or JavaScript, and exposes no
browser control. `Interface: terminal` is the fail-closed classification.

### ADR qualification

The assessment records `ADR action: NONE` (`arc42.md:20-37`). The architectural
commitment is a refusal inside the existing `cli::archive` component; there is
no component, authority, deployment, storage, or compatibility boundary move.
Real alternatives (refuse before the move, rewrite the declaration, discover
after the move) are the proposal's options. Reversal deletes one pre-mutation
check. The existing-decision search names ADR-010 (declaration schema and
read-only writers) and ADR-070 (archive proof and receipt); neither records
archive preconditions. The cheaper home is named: the `archive-change` use case
plus its Gherkin and the served `ise` obligation. No `NEW_ADR` is proposed, and
the two new risks are chapter-11 facts, not decisions.

### Architecture documentation (arc42)

Independently derived chapter set: I walked the 12-row selection table from the
proposal, specs, and design before opening `arc42.md`. My Yes/No matches every
row: 1 No, 2 No, 3 No, 4 No, 5 Yes, 6 Yes, 7 No, 8 Yes, 9 No, 10 No, 11 Yes,
12 Yes. `arc42.md` records the same set, including the glossary row (12), which
the assessment now marks Yes for the two added terms. No row is omitted.

Chapter file integrity, all 12 files under `docs/arc42/`:

| Chapter | Exists | Non-placeholder content | ASCII-art scan | Content matches design |
|---|---|---|---|---|
| 01 | yes | yes | none | No change claimed; the guard protects the existing archive success guarantee. |
| 02 | yes | yes | none | No change claimed; no constraint gains one. |
| 03 | yes | yes | none | No change claimed; the same CLI command gains a refusal. |
| 04 | yes | yes | none | No change claimed; the solution reuses the existing readers. |
| 05 | yes | yes | none | Yes — the `cli::archive` and `e2e_boundary` rows describe the guard and the shared matcher. |
| 06 | yes | yes | none | Yes — stage 3 and the fresh-archive sequence line carry the guard between the receipt check and the artifact gates. |
| 07 | yes | yes | none | No change claimed; no deployment surface moves. |
| 08 | yes | yes | none | Yes — the declaration paragraph records the archive reader, the content-path predicate, and the shape-preserving advice. |
| 09 | yes | yes | none | No new ADR; the register is unchanged. |
| 10 | yes | yes | none | No change claimed; the existing guarantee is protected. |
| 11 | yes | yes | none | Yes — R-149 states the content-path basis and R-150 the accepted symlink hole. |
| 12 | yes | yes (pointer) | none | Yes — the standalone glossary gains the two terms; chapter 12 stays the pointer. |

No box-drawing characters or `+---`/`|---` pseudo-diagram blocks were found in
any chapter. The only diagrams are Mermaid in the design and chapter files.

### Ubiquitous language

The design's two terms are now in the standalone glossary:
"Retained path" and "Content path" (`docs/glossary.md:148-149`). "Specification",
"retained role", "archive candidate", and "proven archive" keep their recorded
meanings. The rule, the design, and the served guidance use the same predicate
wording; the refusal uses "declared retained path", not a new synonym.

### Structural model

`docs/domain-model.md` was assessed. The change adds a refusal inside the
existing `archive-change` flow; it creates no aggregate, relationship, or
replaceable unit, and it moves no boundary. The model's current
Change → permanent specification corpus and Change → version-control
relationships still describe the flow, so no edit is claimed. A no-op edit was
not made.

## Verification run

```text
$ sha256sum givn/specs/archive-change/usecase.md
1d7d8d8bd7db3ed06a2afb4401b2c4a001bf2458fc55ece51b418424b5751e82  givn/specs/archive-change/usecase.md

$ head -1 givn/changes/guard-retained-paths-across-archive/specs/archive-change/usecase.md
<!-- givn:base-sha256:1d7d8d8bd7db3ed06a2afb4401b2c4a001bf2458fc55ece51b418424b5751e82 -->

$ givn lint --change guard-retained-paths-across-archive
... 7 WIP findings, 2 files checked; advisory shape/substitution findings; exit 2
```

Lint exit 2 is the expected state while seven delta scenarios carry `@wip`; no
error or refusal was reported. The failure reproduction transcript records the
current binary's defect (`givn archive` exits 0 and moves the change, then
`givn ise replace` exits 1 with "the declared limits entry
'givn/changes/repro/design.md' matches nothing"). The success transcript
records the current binary's success path (exit 0, `design.md` archived).

## Visual evidence placeholder

`design.md` declares a terminal interface. The evidence set is the two
transcripts named at `design.md:46-51`; the failure transcript is the pre-fix
reproduction and the success transcript is the current success path. At
implementation the `@e2e` step rewrites the success transcript from the executed
scenario, and the review gate re-checks the delta `@e2e` evidence.

VISUAL-REVIEW: PASS

## Sign-off

- [x] Every required branch walked; no branch records an unresolved issue.
- [x] Blockers B1 and B2 closed with recorded landings.
- [x] Decisions Q1–Q5 applied; the proposal's D1–D3 remain resolved.
- [x] No `@e2e` tag removed and no mandatory check deferred.
- [x] `design.md` records the base digest recompute rule for the tasks author.
- [x] `tasks.md`, `review.md`, and `src/` were not written in this pass.
- [x] `givn lint` parses the delta and exits 2 only for the expected `@wip`
      findings.

DESIGN-REVIEW: PASS
