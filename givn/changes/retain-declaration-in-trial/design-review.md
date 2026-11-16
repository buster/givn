# Design Review: retain-declaration-in-trial

Review kind: independent
Author: c0bf1029 (implementation author)
Reviewer context: one independent review context (omp, deepseek-flash), 2026-11-16, a fresh session with no author conversation that re-audited the design and the implementation

## Reviewed inputs

Read in full: `proposal.md`; both delta features and the delta
`specs/prepare-an-ise-test/usecase.md` with its base digest; `design.md`;
`arc42.md`; `config-lock.yaml`; the durable docs this change touches
(`assets/instructions/ise.md`, `docs/reference.md`, `docs/glossary.md`,
`docs/arc42/{02,04,05,06,08,11}.md`); the permanent `prepare-an-ise-test`
use case and its two capabilities' scenarios; and the code the design names
(`src/ise/{mod,partition,record,verify}.rs`, `src/config/declaration.rs`,
`src/e2e_boundary/mod.rs`, `src/scm.rs`, `src/cli/ise.rs`,
`tests/steps/{ise_clean_room_support,clean_room_preparation_steps,
clean_room_preparation_e2e_steps,trial_input_verification_steps,
trial_input_verification_e2e_steps}.rs`).

No implementation exists in this pass: `src/` is unchanged and no `tasks.md`
or `review.md` was written. The verdict below is the plan's.

## Decisions applied

### Q1 — the real verification rule (landed)

- **Resolution.** The trial's `givn/specification.yaml` is re-checked against
  the record like every retained entry. The checkout's declaration is
  consulted only where it derives the frozen `e2e` set and the `behavior`
  feature selection: an edit that moves `e2e:` or `behavior:` is drift, an
  edit that moves only `structure:` or `limits:` is not. A checkout without a
  declaration still refuses with `… is missing; the clean-room trial requires a
  declared Specification`.
- **Rejected.** The absolute sentence "the checkout's declaration is not
  compared" (false: `check_checkout` at `src/ise/verify.rs:186-228` loads it
  for `e2e_file_set` and `behavior_features`), and comparing the checkout's
  declaration bytes to the record's (a later editorial `limits:` or
  `structure:` edit would fail every older trial).
- **Landings.** `specs/prepare-an-ise-test/usecase.md:85` (rule);
  `design.md:279-293` (D6), `design.md:344-347` (report and refusal table),
  `design.md:423-427` (verify flow), `design.md:106` (traceability row),
  `design.md:779-783` (accepted risk); scenarios
  `specs/prepare-an-ise-test/trial-input-verification.feature:19-25` and `:27-34`.
- **Evidence.** `check_retained` iterates `record.retained_files`
  (`src/ise/verify.rs:171-182`); `check_checkout` resolves the `e2e` set and
  `behavior` features from the loaded checkout declaration
  (`src/ise/verify.rs:186-228`). The `structure:`-only scenario therefore
  proves no compared set moved, and the added-root scenario proves a moved
  `e2e:` root is named per gained path (`e2e package gained this path`).

### Q2 — a role may match the declaration, counted once (landed)

- **Resolution.** The retained set is the set union of the five role matches
  and `givn/specification.yaml`. A `contracts: ["givn/**"]` declaration plus
  one matched document prints `Retained: 2`, and
  `counts.retained == retained_paths().len() == retained_files.len()`.
- **Rejected.** Concatenating the declaration after the role matches (a role
  naming `givn/**` would count it twice and break the count invariant).
- **Landings.** `usecase.md:61` (union rule), `usecase.md:79` (record count),
  `usecase.md:116` (example); `design.md:11-13` (overview),
  `design.md:232-246` (D2), `design.md:304-308` (data model),
  `design.md:458-460` (partition rule), `design.md:96` (traceability),
  `design.md:633` (fixture); scenario
  `specs/prepare-an-ise-test/clean-room-preparation.feature:83-89`.

## Blockers closed

### B1 — impossible `Replaced: 1` (closed)

- **Finding.** `clean-room-preparation.feature:69` asserted `Replaced: 1` for
  a `givn/**` source root, but the fixture commits `givn/config.yaml`,
  `givn/commands.yaml`, and `givn/notes.md`. I reproduced the fixture tree
  with the built binary (`givn init`, `givn addons enable ise`, the source-only
  declaration, one `givn/notes.md` commit): `git ls-tree -r HEAD` lists exactly
  those three non-declaration files under `givn/`. `--full` replaces three.
- **Resolution.** Assert the fixture's real count, `Replaced: 3`, and record
  the three files. Rejected: `.git/info/exclude` entries for `config.yaml` and
  `commands.yaml` (the working tree needs both and the exclusion would add a
  second fixture mechanism for no assertion gain) and a range (forbidden).
- **Landings.** `clean-room-preparation.feature:79`; `design.md:100`
  (traceability row) and `design.md:632` (fixture row).

### B2 — false universal and the refusal table (closed)

- **Finding.** The "Before → After" row claimed the loader's
  `… is missing; the clean-room trial requires a declared Specification`
  disappears. It does not: `src/ise/verify.rs:193-200` still emits it.
- **Resolution.** Split the row by command: `ise replace` replaces the message
  with the revision membership refusal; `ise verify` keeps it because it still
  loads the checkout's declaration. Malformed declarations are recorded
  separately for both commands.
- **Landings.** `design.md:344-347`; `docs/reference.md:457-464` states the
  same rule; `docs/reference.md:384-390` aligns the refusal table.

### B3 — an absent declaration has one output (closed)

- **Finding.** `usecase.md:41`, `docs/reference.md:378` row 1, and the design's
  refusal table disagreed about which condition owns an absent declaration.
- **Resolution.** Absence at the revision is refusal 5 and names the path and
  revision; the loader reason is reserved for a declaration present in the
  revision but malformed or unreadable.
- **Landings.** `usecase.md:41` (malformed or unreadable at the selected
  revision), `usecase.md:42` (the revision does not record it);
  `design.md:477` (row 1) and `design.md:481` (row 5);
  `docs/reference.md:384` (row 1) and `docs/reference.md:388` (row 5).

### B4 — arc42 scope contradicts itself (closed)

- **Finding.** `arc42.md` marked chapter 04 affected and `STATUS: DONE`, but
  the Guidance Amendments table omitted chapter 04, chapter 04 still described
  the old partition, and `TC-49`, `R-151`, `R-152` existed nowhere.
- **Resolution.** Landed the chapter 04 edit and all three IDs before
  design-review, and kept `STATUS: DONE` honest. The remaining affected
  chapters (05, 06, 08) and the reference, glossary, and instruction assets are
  landed in the same pass.
- **Landings.** `docs/arc42/04-solution-strategy.md:24`;
  `docs/arc42/02-architecture-constraints.md:58` (`TC-49`);
  `docs/arc42/11-risks-and-technical-debt.md:137-138` (`R-151`, `R-152`);
  `design.md:520-531` (guidance table, chapter 04 row, and the landing note);
  `arc42.md:39-49` (notes) with `STATUS: DONE` at `arc42.md:58`;
  `docs/arc42/05-building-block-view.md:63,90`,
  `docs/arc42/06-runtime-view.md:1011-1025,1061-1069`,
  `docs/arc42/08-crosscutting-concepts.md:1420-1424`.

### B5 — D1 scope (closed)

- **Finding.** `design.md` said the working-tree declaration load is removed
  from both `ise replace` and `ise verify`, contradicting the Overview,
  Architecture Impact, Verify Flow, and D6, and removing `verify`'s only input
  to `check_scoring`.
- **Resolution.** Removed only from `ise replace`; `ise verify` keeps the read
  for the `e2e` set and the `behavior` selection.
- **Landings.** `design.md:218-231` (D1), consistent with
  `design.md:21-22`, `design.md:140-144`, `design.md:423-427`, and
  `design.md:279-293`.

### B6 — malformed at a non-HEAD revision is proven (closed)

- **Finding.** The External Boundary Cases claimed a malformed-at-the-revision
  case, but no delta scenario distinguished it from the permanent malformed
  fixture, which commits the malformed declaration at `HEAD`.
- **Resolution.** Added a scenario whose first revision declares the malformed
  declaration while `HEAD` declares the valid one, so a working-tree read would
  succeed and only the revision read refuses. The existing loader-reason
  binding asserts `unknown key 'component'`.
- **Landings.** `clean-room-preparation.feature:67-72`;
  `design.md:99` (traceability), `design.md:497` (revision boundary row), and
  `design.md:502-510` (boundary paragraph). Rejected: dropping the claim
  (the revision read is the change's point and needs an adversarial case).

## Grilling branches

- **Scope — no issue found.** The proposal's seven observable behaviours map to
  the scenarios: carries the declaration (`:20-26`, `:27-33`, `:34-41`,
  `:42-49`, `:74-82`, `:83-89`), the revision copy (`:50-59`),
  closed over the declaration (`:34-41`, `:42-49`, `:50-59`, `:74-82`),
  refusal at a revision without it (`:60-66`, `:67-73`), the record names it
  (`:27-33`), verification covers it (trial feature `:4-17`), and the served
  guidance (`create-change/prose-consolidation.feature`). Nothing extra is
  specified and "done" is the design's named lines, record keys, and exit codes.
- **Tech choices — no issue found.** No dependency is added; `from_content`
  reuses the existing strict schema; `record.rs`, `verify.rs`, and `cli/ise.rs`
  take the new member through existing sets. The simpler alternative (write the
  file without parsing it) is rejected by D1 because the partition must read the
  revision's declaration.
- **Missing scenarios — three found and added.** The Q1 pair, the Q2 union case,
  and B6. Remaining refusal paths are permanent scenarios and are unchanged.
- **Testability — no issue found.** Strict mode is `.fail_on_skipped()` in
  `tests/features_runner.rs` with undefined steps rewritten to failed `NotFound`
  (`design.md:657-670`); the not-implemented stub pattern returns
  `Err(anyhow!("not implemented: <step text>"))` per new phrase; the
  single-scenario command is recorded at `design.md:651`. Every Then asserts a
  concrete report line, path, YAML key, or exit code.
- **E2E fidelity — no issue found.** The real interface is the CLI: scenarios
  spawn `env!("CARGO_BIN_EXE_givn")`; primary assertions are stdout/stderr and
  exit codes, never an internal call. One `@e2e` scenario per inventory action;
  the new cases are variants and stay regular.
- **Interaction Coverage — no issue found.** The owner's `## Interactions` has
  two rows and the design's matrix has the same two rows with concrete CLI
  drivers (`design.md:671-687`). This change adds an assertion about served
  text, not a consumer action, so `prose-consolidation` keeps its existing
  inventory and `@e2e`.
- **Risk — the single most likely failure is a fixture count drift**, which
  already bit B1: a fixture file that matches a role or source root changes the
  asserted number. Mitigation: assert the fixture's real committed tree
  (`git ls-tree` reproduced here), record the matching files beside the
  assertion, and assert the union invariant explicitly with `Retained: 2` plus
  `retained_files.len()`.
- **Use-case and Persona context — no issue found.** `prepare-an-ise-test`
  keeps its goal, actors, trigger, preconditions, flows, guarantees, and
  capabilities; no capability is renamed, added, or retired, and no typed
  relationship exists to preserve. The three Personas are confirmed by the
  owning use case and are used as review lenses, never as Gherkin actors. The
  `create-change` narrative gains one served-text assertion only.
- **Decision ledger — no unresolved item.** Proposal D1–D6 each resolve in
  `design.md`'s `## Decisions` (D1 `:214`, D2 `:232`, D3 `:247`, D4 `:259`,
  D5 `:269`, D6 `:279`). No question originates in an ideation seed, so no seed
  reference is required. No decision remains carried: every one is foundational
  because a scenario depends on it, and each records its rejected alternatives.
- **Contracts and components — no issue found.** `design.md:181-198` names one
  implementing component per contract: `DeclarationContent` → `config::declaration`
  (two entry points over one schema, one responsibility), `CleanRoomTrial` →
  `src/ise`, `RevisionContent` → `src/scm`/`src/git`/`src/fossil`,
  `InstructionCorpus` → the served asset plus `cli::instructions`. No consumer
  imports an implementation; the composition root is the CLI. Split triggers are
  named. The change has one external boundary (the CLI) and no new external
  service, so no new port/adapter pair is required.
- **Decision timing — no issue found.** D1–D6 are answered now because scenarios
  depend on them; no carried question, no default/trigger/switch-cost/owner gap,
  and no speculative abstraction. `from_content` is needed by the current
  revision read, not by an imagined future caller.
- **Visual design contract — no issue found.** `design.md:42-82` declares
  `Interface: terminal`, viewports not applicable, the baseline text rules, the
  transcript path, and all three Personas. The transcript is produced by the
  modified `@e2e` scenario at implementation; no browser driver is claimed.
- **Deferrals — none.** No mandatory check is deferred; the implementation batch
  re-verifies the landed chapter text against the as-built code, which is not a
  deferred check.
- **Persona disposition — recorded.** `developer--primary-user`,
  `agent--technical-role`, and `givn-maintainer--owner` are confirmed and each
  has an impact row (`design.md:708-717`). No `none` disposition.
- **Interface classification — tested the opposite.** No HTML, CSS, or
  JavaScript is rendered; the surfaces are subprocess stdout/stderr, a directory
  of files, and a YAML record. The opposite classification (Web UI) does not
  hold, so `Interface: terminal` is correct.
- **ADR qualification — no ADR, and the cheaper home is named.** The candidate
  (declaration as mandatory trial member; revision as read authority) fails the
  architectural-commitment test: ADR-076 owns the record's shape and no field,
  scope, or identity rule changes; ADR-010 owns the declaration's authority and
  its writers. The cheaper home is `docs/reference.md`, the
  `prepare-an-ise-test` contract, and the named chapters, recorded once in
  `arc42.md:22-35`. Chapter 09 keeps its register and no MADR is created.
- **Architecture documentation — no omission.** My independent walk of the
  twelve rows before opening `arc42.md`: 01 No (no goal), 02 Yes (declaration
  retained and read authority), 03 No (same CLI surface), 04 Yes (trial
  strategy), 05 Yes (declaration entry point and `ise` responsibility), 06 Yes
  (replace/verify flows), 07 No (one binary, directory trial), 08 Yes
  (record/config concept), 09 No (no ADR), 10 No (no quality scenario), 11 Yes
  (older record and uncommitted declaration), 12 No for a new term, with the
  glossary amended for two existing entries. This matches `arc42.md`; chapter 04
  is affected there and is now edited. Chapter files:
  - all twelve exist and carry change-specific content; no scaffold or
    unfilled template.
  - no box-drawing characters and no `+---`/`|---` non-table pseudo-diagram
    blocks were found; the only diagrams are Mermaid.
  - `TC-49` is present in chapter 02; `R-151`/`R-152` in chapter 11; chapters
    04, 05, 06, and 08 describe the reviewed plan. Chapter 11 carries the
    accepted consequences of D6/D1 (older record; uncommitted declaration).
  - chapter 09 requires no new MADR, per the recorded qualification.
- **Ubiquitous language — no issue found.** `Retained path`
  (`docs/glossary.md:148`) now includes the declaration as an implicit entry,
  and `Trial directory` (`docs/glossary.md:76`) names it among the members; no
  new term is coined and no synonym is introduced. "Checkout's declaration" and
  "trial's declaration" are used as distinct, glossed subjects.
- **Structural model — assessed, no edit.** The change moves no aggregate,
  entity, relationship, or boundary; it adds a member to a trial directory and a
  record set. `docs/domain-model.md` excludes internal components and file
  formats, so the assessed result is no change, not an omitted edit.

## Files changed

| File | Lines (final) | Net change |
|---|---:|---:|
| `design.md` | 790 | +57 |
| `specs/prepare-an-ise-test/usecase.md` | 182 | +4 |
| `specs/prepare-an-ise-test/clean-room-preparation.feature` | 89 | +15 |
| `specs/prepare-an-ise-test/trial-input-verification.feature` | 34 | +17 |
| `arc42.md` | 59 | +7 |
| `docs/reference.md` | 614 | +20 |
| `docs/glossary.md` | 341 | 0 (two rows rewritten) |
| `assets/instructions/ise.md` | 60 | +4 |
| `docs/arc42/02-architecture-constraints.md` | 138 | +1 |
| `docs/arc42/04-solution-strategy.md` | 104 | 0 (one row rewritten) |
| `docs/arc42/05-building-block-view.md` | 155 | 0 (two rows rewritten) |
| `docs/arc42/06-runtime-view.md` | 1080 | +11 |
| `docs/arc42/08-crosscutting-concepts.md` | 1585 | +6 |
| `docs/arc42/11-risks-and-technical-debt.md` | 216 | +2 |

`tasks.md`, `review.md`, and `src/` were not written.

## Verification run

```text
$ sha256sum givn/specs/prepare-an-ise-test/usecase.md
7539c79c70eaa4293a57b6a743ab051eeb6cd165098733828feed442d3b39fa6  givn/specs/prepare-an-ise-test/usecase.md

$ head -1 givn/changes/retain-declaration-in-trial/specs/prepare-an-ise-test/usecase.md
<!-- givn:base-sha256:7539c79c70eaa4293a57b6a743ab051eeb6cd165098733828feed442d3b39fa6 -->

$ givn lint --change retain-declaration-in-trial
givn lint: 3 file(s) checked, 15 finding(s)
... 15 WIP findings (one per @wip scenario), advisory shape/substitution
findings; no error or refusal. Exit 2 is the expected @wip state.

$ givn status --change retain-declaration-in-trial
Change: retain-declaration-in-trial
Stage:  design-review-done
Artifacts:
  ✓ proposal
  ✓ specs
  ✓ design
  ✓ arc42-docs  (gates archive)
  ✓ design-review
  · tasks  (gates archive)
  · review  (gates archive)
Next required:  tasks
```

The delta base digest still matches the permanent `usecase.md`, so archive merges
against the right narrative. Lint reports no duplicate title, no unbound
interaction row, and no shape error; the 15 WIP findings are the unstarted
implementation batch.

## Sign-off

- [x] Every required branch walked; none records an unresolved issue.
- [x] Decisions Q1 and Q2 applied with their rejected alternatives recorded.
- [x] Blockers B1–B6 closed with landing lines.
- [x] No `@e2e` tag removed from any scenario and no mandatory check deferred.
- [x] Every proposal `D<n>` resolves in the design's Decisions ledger.
- [x] The visual design contract, Persona disposition, interface
      classification, and ADR qualification are recorded.
- [x] The arc42 chapter set was re-derived independently and matches
      `arc42.md`; all twelve chapter files are complete and diagram-clean.
- [x] `tasks.md`, `review.md`, and `src/` were not written in this pass.

DESIGN-REVIEW: PASS
