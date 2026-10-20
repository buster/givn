# Proposal: add-readability-review-gate

## Release note

Givn reports readability findings without telling anyone what to do, and the
opt-in review gate refuses a gate finding unless the unit is under its
threshold at head (FIXED) or carries a change-scoped, owner-approved rationale
for keeping its current shape (JUSTIFIED).

## Use-Case Context

- Use-case ID: `verify-change` (existing; this change adds the
  `readability-review-gate` capability under it)
- Ideation topic: `readability-experiment`
- Seed path: none — the topic was developed directly; the evidence is
  `givn/ideation/readability-experiment/reports/readability-gate-proposal.md`,
  `givn-change-plan.md`, `gate-replay.md`, and `gate-replay-wide.md`
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner` — confirmed for the topic on 2026-10-19. Personas
  are review lenses, not Gherkin actors.

This is the second change in the confirmed two-change plan. The first change,
the readability report (`add-readability-report`), is archived. This change
supersedes the report's `action` and `recommendation` fields: the report becomes
a findings-only document at schema version 2 in the same PR. The archived
report change stays historical evidence and is not edited.

## Problem / Opportunity

The report shows structural debt but also tells the reader what to do. That
recommendation is not wanted: a finding should state the measured shape and
stop there. The owner decides what to do with it, in the review record, against
the change that touched it.

The earlier gate model was weaker than the review obligation it claimed to
enforce:

- A non-growing over-threshold unit could be `WATCH`ed forever. The project
  accumulated debt without a decision that explained why the shape was right.
- `PLAN` and `EXCEPTION` were separate markers with separate artifacts. They
  became escape hatches: a boilerplate plan or a category such as
  "legacy" satisfied the gate without describing why the current shape is
  correct or why the obvious improvement is wrong.
- Accounting was file-scoped in practice: one marker could vaguely cover a type
  and the file's module, and two types in one file could share a rationale.
- The gate analyzed the dirty working tree, so it could pass a state that no
  receipt had ever tested.

Two outcomes are enough. Either the unit is under its threshold at head, or the
owner has a substantive, evidence-based reason to keep it as it is. The gate
must name the unit, require one decision per unit, and fail closed when the
reason is ownership or scope in disguise.

## Proposed Solution

### Report v2: findings only

`givn readability report` keeps the same classifications and metrics. It no
longer emits an action, recommendation, or advice line. Human output:

```text
readability report — rust, 1 file, 1 unit, 1 finding
findings only: this report does not block review or archive

FINDING       type    src/engine.rs::Engine   god-type
              code lines 620; methods 24; public methods 9; fields 3; LCOM4 4; TCC 0.312
              type-code-lines 620 > 500
```

A unit under every threshold prints `WITHIN-LIMITS` instead of `FINDING` and
carries no recommendation line. Units sort by classification, then path, then
unit id.

JSON output keeps `classification`, `metrics`, `evidence`, `methods`,
`outline`, `capabilities`, `schema_version`, and `rules_version`. `action` and
`recommendation` are removed and `schema_version` becomes `2`. The `findings`
summary counts every classification other than `within-limits`.

### The gate: two outcomes per gate finding

When the readability addon is enabled, `givn check review --change <change>`:

1. checks out the committed `HEAD`, not the dirty working tree, and fails
   closed when a production path is dirty (`commit or discard before review`);
2. derives the change base as the parent of the first reachable commit that
   touched the change directory, using Git first-parent history or Fossil
   `ancestors current`, and fails closed for a root commit's empty base, a
   missing change-directory commit, or a shallow clone;
3. analyzes every production unit the change touched at the base and at `HEAD`,
   evaluating a type across all its files and every implementor of a touched
   trait;
4. emits a finding per unit, never per file, with its classification, base and
   head size, evidence, and exact unit id;
5. requires exactly one decision per finding:
   - `READABILITY: FIXED <unit>` — the unit is under its threshold at head, or
     the finding no longer fires (the unit was removed or split);
   - `READABILITY: JUSTIFIED <unit>` — the unit is still over its threshold,
     and `givn/changes/<id>/readability/rationales/<unit-slug>.md` explains why
     the current shape is right and why the obvious improvement is wrong.
6. fails while any finding has neither decision.

A `FIXED` marker is rejected when the unit is still a finding at head. A
`JUSTIFIED` marker without a valid rationale is rejected. A finding with no
decision is `MISSING` and fails the review. `READABILITY: PASS` is required once
when any finding exists; `FAIL <reason>` always fails.

### Unit-level accounting

Every finding names its unit by the report's stable unit id:

- `type:src/engine.rs::Engine`
- `module:src/history.rs`

A rationale carries `Unit:` and covers exactly that unit. A type rationale does
not cover the file's module finding. Two types in one file need two rationales.
A file move that keeps the type name and module path keeps the unit id and
therefore does not reset the comparison.

### Marker vocabulary

The complete vocabulary is:

```text
READABILITY: FIXED <unit>
READABILITY: JUSTIFIED <unit>
READABILITY: PASS
FAIL <reason>
```

`WATCH`, `PLAN`, and `EXCEPTION` are removed. A migration plan may appear
inside a rationale; it is not a separate escape marker.

### Rationale artifact

Path:
`givn/changes/<change-id>/readability/rationales/<unit-slug>.md`.

```text
Unit: type:src/engine.rs::Engine
Classification: god-type
Finding: new-type
Evidence: 620 code lines, 9 public methods, LCOM4 4, TCC 0.312; every call site is in src/cli/check.rs.
Why this shape is right: The type owns the review state machine and its phase order; splitting it would move that order into two modules and duplicate the transition rules.
Alternatives considered: Extract the renderer and the SCM adapter. Rejected: rendering already delegates to readability::report, and SCM access is one hop through scm::Backend; both extractions add indirection without removing a responsibility.
Owner: givn-maintainer--owner
```

The gate validates the fields mechanically and deterministically: the unit and
classification match; `Finding:` names the reason; `Alternatives considered:`
rejects empty, `none`, and `n/a` and must name at least one alternative plus why
it was rejected; `Evidence:` must cite a concrete metric or a path from the
finding; `Why this shape is right:` must name the unit's responsibility; and
`Owner:` must be the confirmed owner (`givn-maintainer--owner` in this project).
It does not grade truth or plausibility; the owner judges the content.

Banned reasons, matched case-insensitively on whole-word boundaries: "not my
code", "not this change", "predates my work", "no capacity", "out of scope",
"legacy", "too risky", "no time", "will do later", "later". A temporary shape
must also carry `Follow-up:` and `Expiry:`; the gate checks the pair, not the
calendar.

### Ownership principle

Code is owned by everyone working on the project. Seeing technical debt and
leaving it because "it was not me" is not acceptable. Leave existing code
better than you found it whenever you touch it. A readability justification
cannot use ownership or scope as a reason; the gate and the review guidance say
so in the same words.

### Concrete decisions the built check makes

- A unit at exactly 500 code lines is within limits; at 501 it is a finding.
- A module at exactly 1,000 code lines is a finding; the report's `module`
  classification starts at 500.
- A data-only type is exempt from the size rule, but a data-only type with a
  method over 100 code lines or complexity 50 is a method-length finding.
- A new interface under 500 code lines with more than 50 public methods is a
  finding.
- Conditional code is counted as written. The printed size is the measured
  value, never a claim of exactness; a false-large finding from the accepted
  cfg limitation still needs FIXED or JUSTIFIED, and an existing non-growing
  one needs a rationale too (arc42 R-108).
- No findings: the gate prints `readability review: PASS` and needs no marker.

### Worked examples

**FIXED.** `Engine` is a god type at 620 code lines at the base and 480 at
head. The review record says:

```text
READABILITY: FIXED type:src/engine.rs::Engine
READABILITY: PASS
```

The gate prints `FIXED type:src/engine.rs::Engine` and
`readability review: PASS`. A `JUSTIFIED` marker here would fail: the unit is no
longer a finding at head.

**JUSTIFIED.** `Engine` is 620 at the base and 700 at head. The review record
says:

```text
READABILITY: JUSTIFIED type:src/engine.rs::Engine
READABILITY: PASS
```

and the rationale at
`givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md`
explains that the type owns the review state machine, that every call site is
inside it, and why the obvious extraction of the SCM adapter and the renderer
would add indirection without removing a responsibility. The gate validates the
`Unit:`, `Classification:`, `Finding:`, `Evidence:`, `Why this shape is right:`,
`Alternatives considered:`, and `Owner:` fields, then prints
`JUSTIFIED type:src/engine.rs::Engine` and `readability review: PASS`.

**Rejected justification.** The same 620-to-700 unit carries:

```text
Why this shape is right: not my code
```

The gate fails with
`rationale givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md: banned reason 'not my code'`.
The same failure applies to "not this change", "predates my work", "no
capacity", "legacy", "too risky", "no time", "will do later", "later", and "out
of scope". A migration plan may appear inside a rationale, but it is not a
blanket escape from the explanation.

## Capability Routing

Route reported signal: `EXTEND givn-workflow-enforcement` (use case
`verify-change`, score 17.00); `visual-review-gate` and
`review-disposition-gates` tied at 16.00.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `readability-review-gate` | `EXTEND givn-workflow-enforcement` | `NEW in verify-change` | A distinct opt-in review gate with its own marker vocabulary, unit-level rationale artifacts, and committed-head analysis. `givn-workflow-enforcement` owns always-on workflow integrity (hooks, receipts, task commits, migration coverage); folding readability into it would mix an experimental opt-in gate with those guarantees. The existing `visual-review-gate` is the precedent: a specialised review gate inside `givn check review` with its own conditional instruction. |
| `readability-report` | none — existing capability | `MODIFIED in assess-code-structure` | The change removes `action` and `recommendation` from the report's public contract and bumps the JSON schema to 2. The report capability owns that output, so its permanent spec and use case are modified rather than duplicated under the gate. |

The new capability is declared in `verify-change`'s capability list and its
interaction inventory in this change; the design records the exact row and the
complete interaction matrix.

## Out of Scope

- Grading the quality or plausibility of a rationale beyond the mechanical
  field, owner, and banned-phrase checks.
- A project-wide report run inside review; the gate analyzes only touched units
  at base and head.
- A stored baseline, watchlist, historical classification, or feedback ledger.
- Keeping `WATCH`, `PLAN`, or `EXCEPTION` as compatibility markers.
- Automatic refactoring, extraction, or code movement. The gate measures and
  requires a decision; it does not edit source.
- Languages other than Rust, or a tree-sitter adapter. The gate calls the same
  neutral analysis contract as the report.
- Enforcing expressive names or single responsibility mechanically. Those
  remain review-instruction rules; the gate enforces size, structure, the
  recorded decision, and the rationale fields.
- An outline artifact or hash requirement in the review record; the semantic
  check remains a procedural attestation.
- Automatic workspace resolution. The gate uses the report's default production
  scope (`src/`) and does not parse `Cargo.toml`.
- Making the experiment default-on or permanent, or blocking projects that have
  not opted in.
- A new CLI command or flag. The gate is part of `givn check review`. The
  report's schema bump is a revision of the existing command, not a new one.
- Test and tooling sources; they are outside the production scope.

## Open Questions

### D1 — What is the change base?

- Status: answered 2026-10-20 (user)
- Question: against which revision should the gate compare the units a change
  touched?
- Options:
  1. **Merge-base with the main branch** — the base is where the branch
     diverged. Gain: stable while the branch lives. Cost: commits merged from
     main after divergence become part of the change, so an unrelated growth on
     main is blamed on this change.
  2. **Parent of the first reachable commit that touched the change
     directory** — chosen. Gain: exactly the project state before this change
     existed. Cost: the change directory must be committed, and the gate needs
     version-control history. In practice: the first change commit creates
     `givn/changes/<id>/`; its parent is the base.
  3. **The last commit before review (HEAD)** — gain: trivial. Cost: every
     committed change looks like existing debt, so the gate cannot see growth.
- Disposition: answered — option 2, user decision 2026-10-20. The design records
  Git first-parent, Fossil `ancestors current`, root commit, shallow clone,
  rebase, merge, source-before-change-directory ordering, and the no-commit
  failure.

### D2 — Does the gate add a feedback ledger?

- Status: answered here, consistent with the report's deferred decision
- Question: should the gate add an opt-in feedback ledger now, or keep the
  manual experiment record?
- Options:
  1. **Opt-in ledger** — an append-only record of change, unit,
     classification, decision, owner judgment, and false positive/negative.
     Gain: structured evidence for the experiment exit. Cost: new product state
     that the owner must curate, with a standing risk of becoming a stored
     classification baseline.
  2. **Derive the record from review markers and rationales** — the per-change
     markers and rationale artifacts are the record; a script aggregates them
     later. Gain: no new state. Cost: the artifacts are not an experiment
     ledger, and the exit decision remains manual.
  3. **Keep the manual record** — chosen. Feedback stays in
     `givn/ideation/readability-experiment/feedback.md`; the markers and
     rationales are the per-change evidence. Gain: no product state; matches the
     report's D4. Cost: manual discipline when the owner reviews the experiment.
- Disposition: answered — option 3. No ledger is added. The gate writes no
  feedback file, telemetry, baseline, or classification cache.

### D3 — What does a marker name, and where does the rationale live?

- Status: carried
- Owner: design author (agent), decided in `design.md`
- Question: does a decision marker name the unit or the rationale artifact, and
  how is a type kept separate from its file's module?
- Options:
  1. **Unit id plus a change-scoped rationale** — the marker names the report's
     stable unit id (`type:<path>::<Name>` or `module:<path>`); the rationale
     lives under `givn/changes/<id>/readability/rationales/<unit-slug>.md` and
     carries `Unit:`. Gain: unambiguous unit-level accounting; a type and its
     module are separate findings. Cost: the marker is longer than a path.
  2. **Source unit path** — the marker names a file. Gain: short. Cost: a file
     can hold a module and several types; one path cannot say which unit the
     owner decided about.
  3. **Inline rationale in `review.md`** — no separate file. Gain: one file.
     Cost: long rationales make the review record unwieldy and mechanical
     field validation is harder.
- Disposition: carried — the design chooses option 1 and fixes the artifact
  schema, slug rule, and validation errors.

### D4 — Which findings require which decision?

- Status: carried
- Owner: design author (agent), decided in `design.md`
- Question: when may a touched over-threshold unit keep its shape, and what
  counts as an acceptable reason?
- Options:
  1. **Strict all-debt** — every touched over-threshold unit needs a rationale,
     growing or not. Gain: forces an owner decision on all debt. Cost: a
     one-line fix in a 700-line module requires a decomposition rationale even
     when nothing about the shape changed. This is close to the chosen model;
     the difference is that a FIXED unit needs no rationale.
  2. **Non-growth accepted silently** — a touched unit that does not grow
     passes without a decision. Gain: small fixes stay easy. Cost: the project
     accumulates debt with no recorded reason; this is the old `WATCH` model and
     is rejected.
  3. **Fix or justify** — chosen. A unit under its threshold at head is FIXED
     with a marker; a unit still over its threshold is JUSTIFIED with a
     substantive, evidence-based, owner-approved rationale; anything else
     fails. Gain: every persistent finding has a reason the next maintainer can
     read. Cost: every touched existing finding needs a rationale.
  4. **Advisory only** — report findings and never fail. Gain: no friction.
     Cost: no enforcement, contradicting the change goal.
- Disposition: carried — the design chooses option 3, defines the accepted
  rationale fields, and bans ownership and scope reasons.

### D5 — How is the semantic outline review enforced?

- Status: carried
- Owner: design author (agent), decided in `design.md`
- Question: does the gate require an outline artifact and hash, or only the
  recorded verdict?
- Options:
  1. **Verdict marker only** — `READABILITY: PASS` when a finding exists;
     `FAIL <reason>` fails. Gain: records the semantic judgment without new
     state. Cost: the fresh-context isolation remains procedural; the gate
     cannot prove the reader did not open source.
  2. **Outline artifact and hash** — the review record names the generated
     outline and its hash. Gain: auditable. Cost: adds a marker outside the
     fixed set and stores an artifact.
  3. **Mechanical only** — no semantic verdict. Gain: simplest. Cost: loses the
     experiment's core comprehension check.
- Disposition: carried — the design chooses option 1 and records the
  procedural limitation.

### D6 — What happens when no reachable commit touched the change directory?

- Status: carried
- Owner: design author (agent), decided in `design.md`
- Question: the base rule needs a first commit; what does the gate do when the
  change directory is not committed yet?
- Options:
  1. **Fail closed with a named error** — chosen. Gain: no silent wrong base.
     Cost: review cannot run before the change directory is committed.
  2. **Fall back to HEAD and the working tree** — gain: works before commits;
     cost: the base is the last commit, so committed growth is invisible.
  3. **Fall back to the merge-base with the main branch** — gain: works
     without a change-directory commit; cost: reintroduces D1's rejected base.
- Disposition: carried — the design chooses option 1 with the exact message
  `change directory 'givn/changes/<id>/' has no reachable commit; commit the
  change directory before review`. Review already requires a verification
  receipt bound to a committed revision.

### D7 — Which head does the gate analyze?

- Status: answered 2026-10-20 (user)
- Question: does the gate analyze the dirty working tree or the committed
  `HEAD`?
- Options:
  1. **Working tree** — gain: sees the code the developer is about to commit.
     Cost: the gate can pass a state no receipt tested, and the receipt is bound
     to a committed revision.
  2. **Committed `HEAD` plus a clean-production guard** — chosen. Gain: the
     gate analyzes exactly the revision the receipt tested. Cost: a developer
     must commit or discard production changes before review. Non-production
     paths such as `review.md` and the rationale files may stay uncommitted.
  3. **Committed `HEAD`, ignore dirty production paths** — gain: no friction.
     Cost: a review can pass while the working tree already contains different
     code.
- Disposition: answered — option 2. A dirty production path fails with
  `production paths are dirty; commit or discard before review: <paths>`.

### D8 — Does this change also revise the report?

- Status: answered 2026-10-20 (user)
- Question: remove the report's action and recommendation now, or leave the
  report at schema 1 and address it in a later change?
- Options:
  1. **Leave the report unchanged** — gain: smaller diff. Cost: the review gate
     would consume a report model that still tells the reader what to do, and
     the permanent report spec would contradict the owner direction.
  2. **Revise the report in this change (Option A)** — chosen. Gain: one public
     contract, one spec delta, one schema version. Cost: this PR touches the
     report's classifier, renderer, permanent spec, use case, tests, and E2E
     scenario. The archived report change stays historical.
  3. **New report change** — gain: clean change boundary. Cost: a second
     planning cycle for a field removal, with the gate blocked on it.
- Disposition: answered — option 2. The report becomes findings-only at schema
  version 2 in this change; its archived change is not edited.
