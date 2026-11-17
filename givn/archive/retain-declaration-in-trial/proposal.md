# Proposal: retain-declaration-in-trial

## Release note

`givn ise replace` copies the project's Specification declaration into every clean-room trial, counts and records it as retained, and `givn ise verify` re-checks it per path.

## Use-Case Context

- Use-case ID: `prepare-an-ise-test` (`clean-room-preparation`,
  `trial-input-verification`) and `create-change` (`prose-consolidation` for
  the served `ise` guidance sentence)
- Ideation topic: `ise-rebuildability`
- Seed path: none. No ideation change seed covers this defect; the change
  starts from a live `Ise Test`, so there is no seed status line to stamp and
  no seed question to disposition.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

The three IDs come from `prepare-an-ise-test`'s `## Personas` section; the
`create-change` narrative adds no Persona to this change. No biography is
copied here.

## Review Context

- Review kind: independent
- Reason: major change. It changes what a clean-room trial contains and what
  its record freezes: the retained set gains a mandatory member, the report's
  `Retained` count changes for every trial, and the declaration recorded at
  the selected revision becomes the only authority for the partition. Every
  later rebuild reads those trials, and reversing the change invalidates
  prepared trials and their records.

## Problem / Opportunity

A clean-room trial materializes the five declared retained roles and the kept
source. `givn/specification.yaml` is a tracked file like any other, so it
enters the trial only when the project happens to name it inside a retained
role.

A live rebuild exposed the consequence. The builder's arm re-read
`givn/specification.yaml` to learn which roles the project declares, found no
such file in the trial, and stopped at its instruction-integrity check with
`givn/specification.yaml is missing or unreadable` before writing any code.
The declaration is the index of the Specification: without it a builder
cannot tell a declared retained path from a replaceable source root, and a
verifier cannot prove which roles the trial was materialized from.

Verification cannot catch the loss today. `givn ise verify` re-checks exactly
the retained paths the record lists, and the declaration is not among them, so
a trial that lost its declaration re-checks clean. The freeze protects the
retained documents and the `e2e` package while the one document that defines
them is not frozen at all.

## Proposed Solution

Observable behaviour, from the developer's, builder's, and verifier's side:

1. **Every trial carries the declaration.** `givn ise replace` writes
   `givn/specification.yaml` into the trial under both `--full` and `--path`,
   whether or not a role names it, and reports it in `Retained`. The
   declaration is an implicit retained entry, not a sixth role: every existing
   declaration keeps working, and no project can point the entry at a file
   that is not the index it declares.
2. **The declaration is the frozen one.** The trial carries the declaration
   recorded at the selected revision, and the partition reads that same copy.
   `--at <revision>` therefore partitions by what that revision declared, and a
   builder reading the trial sees the roles that were applied.
3. **The retained set is closed over the declaration.** `Replaced` never
   counts it: a `source:` root that covers `givn/specification.yaml` does not
   remove it under `--full`, and naming the declaration, or a directory that
   contains it, with `--path` refuses by name instead of silently keeping half
   of the named directory.
4. **A revision without the declaration refuses.** A selected revision that
   does not record `givn/specification.yaml` refuses before any write, naming
   the path and the revision, because a trial without the declaration is the
   defect this change removes.
5. **The record names it.** The trial record's retained set carries
   `givn/specification.yaml` with its per-path digest, so `Retained digest`
   covers it and a verifier names it per path.
6. **Verification covers it.** `givn ise verify` re-checks the declaration
   like any other recorded retained path: a trial whose declaration is missing
   or differs is drift, printed on the retained drift line with the path.
7. **Guidance states the rule.** `assets/instructions/ise.md` says the trial
   always carries the declaration and that it is an implicit retained entry,
   not a sixth role, so a project need not list it and a builder can read the
   roles from it. A `@prose-consolidation` scenario reads the served `ise`
   instruction and asserts both sentences.

The report keeps its ten fixed lines and the record keeps its strict
`ise-trial/v1` shape: one more retained entry changes no line, no field, and no
exit code.

## Spec gap

- Outcome: `specification defect`
- Wrong artifact: `givn/specs/prepare-an-ise-test/usecase.md`. Its Main flow
  steps 2, 4, and 5 and its Rules define the trial as the retained roles plus
  the kept source and define the record's retained identity over that set, so
  the declaration is neither written nor recorded. `givn ise replace` and
  `givn ise verify` implement that contract faithfully; no implementation-only
  change can put the declaration into a trial and its record. The corpus edit
  travels with this change: the use case, the two capabilities' scenarios, and
  the served `ise` guidance.

## Capability Routing

`givn spec route --change retain-declaration-in-trial` reports signal:
`EXTEND givn-workflow-enforcement` (use case `verify-change`, score 17.00),
matched on the `givn check` invocation and on generic words. That capability
enforces change gates; it does not materialize or verify a trial, and it does
not serve the `ise` instruction. All three rows therefore deviate, and the
Rationale column records why.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `clean-room-preparation` | `EXTEND givn-workflow-enforcement` (`verify-change`, score 17.00, matched on `givn check` and generic words) | `EXTEND clean-room-preparation` | What changes is what a trial contains and which declaration the partition reads. `prepare-an-ise-test` owns trial materialization; `givn-workflow-enforcement` owns change gates and reads no trial. |
| `trial-input-verification` | `EXTEND givn-workflow-enforcement` (same recommendation, same score) | `EXTEND trial-input-verification` | What changes is the recorded retained set and its re-check. `prepare-an-ise-test` owns the frozen-input re-check; no change-gate capability reads a trial record. |
| `prose-consolidation` | `EXTEND givn-workflow-enforcement` (same recommendation, same score) | `EXTEND prose-consolidation` | The new sentence lives in the served `ise` instruction. `create-change`'s `prose-consolidation` already owns the scenarios that assert served guidance text, so the sentence is asserted there rather than under a change gate. |

No capability is new: all three already own scenarios in
`givn/specs/prepare-an-ise-test/` and `givn/specs/create-change/`, and no use
case's `## Interactions` inventory gains a row because no consumer action
changes.

## Out of Scope

- What the declaration may contain, and the five roles: no new key, no sixth
  role, no change to the strict loader's schema.
- `givn archive`'s retained-path guard, the archive merge, and the retained
  paths a change may declare. The change touches no archive behaviour.
- Enabling `ise` on this repository, declaring Givn's own Specification, and
  any self-rebuild. The addon stays off.
- The `e2e` frozen set, the `@e2e` scenario digest rule, the replaced digest,
  and the record's identity rule.
- Running the frozen `e2e` suite and building the replacement: the independent
  verifier and the rebuilding agent, both outside Givn.
- Re-checking the checkout's declaration during verification: the recorded
  retained set stays the trial's authority (D6).
- A record schema bump: `ise-trial/v1` keeps its shape and its reader (D5).

## Open Questions

### D1 — Which declaration is authoritative, and which copy the trial carries

- Question: `givn ise replace` reads a selected revision's tracked files but
  currently loads the declaration from the working tree. When `--at` names an
  older revision, the partition can apply the current declaration while the
  trial carries the older one, so a builder reads roles that were never
  applied. Which declaration decides the partition, and which copy does the
  trial hold?
- Options:
  1. **The declaration recorded at the selected revision** — the partition
     parses the revision's `givn/specification.yaml` and the trial copies the
     same bytes; gain: one authority, the trial is self-consistent, and a
     builder sees the applied roles; cost: the declaration must be tracked at
     the selected revision, so an older revision without it refuses; in
     practice: `givn ise replace --at 8c1f2ab --full --into /tmp/t` partitions
     by 8c1f2ab's declaration and the trial holds 8c1f2ab's file.
  2. **The working tree declaration (today)** — the partition keeps loading
     the checkout's file; gain: no admission reorder; cost: a trial can pair
     the old declaration with roles applied from the new one, which is the
     defect one step later; in practice: after the developer commits a new
     `structure:` entry, `--at <old>` writes a trial whose declaration lacks
     that entry while the partition used it.
  3. **Both must agree** — the working tree decides the partition and the run
     refuses when the revision's copy differs; gain: no reorder; cost: the
     clean-checkout precondition makes disagreement impossible for a tracked
     declaration, so the check refuses only the cases that option 1 handles
     correctly; in practice: `--at <old>` refuses instead of reproducing the
     old Specification.
- Disposition: carried — the deciding artifact is `design.md`, whose Decisions
  ledger resolves D1; the owner is the change author.

### D2 — How the declaration enters the retained set and the report

- Question: Should the declaration be a sixth declared role, an implicit
  retained entry, or a separate report line and record field?
- Options:
  1. **Implicit retained entry** — the partition always retains
     `givn/specification.yaml`; the report's `Retained` and the record's
     `retained_files` gain one entry and no line or field is added; gain: every
     existing declaration keeps working, the digest covers the declaration, and
     a verifier's per-path output already names it; cost: `Retained` grows by
     one for every project, so a recorded count moves; in practice: a project
     that declares `behavior: ["givn/specs/**"]` prints `Retained: 4` for three
     matching files plus the declaration.
  2. **A sixth role `declaration:`** — every project declares where its index
     lives; gain: explicit and configurable; cost: every existing declaration
     becomes invalid, the strict loader gains a key, and a project can point
     the entry at a file that is not the declaration; in practice: an existing
     `givn/specification.yaml` refuses with `missing role 'declaration'`
     until it is edited.
  3. **A separate line and record field** — `Declaration: givn/specification.yaml`
     beside the ten lines, and a `declaration` field in the record; gain: the
     declaration is visible without reading `Retained`; cost: one path gains a
     second evidence channel that must agree with `retained_files` and the
     digest; in practice: a verifier must check the path twice and the two can
     disagree.
- Disposition: carried — the deciding artifact is `design.md` (D2); the owner
  is the change author.

### D3 — What a replacement naming the declaration means

- Question: `--path givn/specification.yaml`, or a `--path` directory whose
  tracked files include it, names a path the trial always retains. Does it
  refuse, get skipped silently, or remove the declaration?
- Options:
  1. **Refuse, naming the path** — the candidate set is checked for the
     declaration before any write; gain: one rule for every retained path, and
     a developer who names `givn` learns that the index stays; cost: a broad
     `--path givn` must be narrowed to the replaceable subtrees; in practice:
     `--path givn` refuses naming `givn/specification.yaml`.
  2. **Skip it silently** — the declaration stays and the rest of the named
     directory is replaced; gain: no extra refusal; cost: the developer
     believes `givn` was replaced while one of its files was not, and the
     `Replaced` count is the only signal; in practice: `--path givn` prints
     `Replaced: 3` and keeps the declaration without a word.
  3. **Remove it** — the declaration is replaceable like any source file; gain:
     `--path` means exactly what it names; cost: a trial can be prepared
     without its index, which is the defect this change removes; in practice:
     `--path givn/specification.yaml` writes a trial whose builder cannot read
     the roles.
- Disposition: carried — the deciding artifact is `design.md` (D3); the owner
  is the change author.

### D4 — Whether a revision without the declaration is a new refusal class

- Question: The use case names nine refusals and one of them is "a retained
  path is missing at the revision". A revision that predates the declaration
  now removes an implicit retained entry from the trial. Is that a tenth
  refusal with its own wording, or an instance of the existing one?
- Options:
  1. **An instance of the retained-path-missing refusal** — the declaration is
     a retained entry, so its absence refuses with the path and the revision
     under condition 5; gain: nine conditions stay nine, the exit code and the
     documented list stay stable; cost: the message must make clear that the
     missing path is the declaration, not a declared role; in practice:
     `ise replace: the declaration 'givn/specification.yaml' is absent at
     revision '8c1f2ab'; a trial always carries it`.
  2. **A tenth refusal** — its own numbered condition and wording; gain: the
     declaration's absence is easy to find in the reference table; cost: every
     count of the refusals in the corpus, the reference, and the arc42
     chapters moves, and the condition duplicates a settled rule; in practice:
     the reference table grows a row and the use case grows an extension.
- Disposition: carried — the deciding artifact is `design.md` (D4); the owner
  is the change author.

### D5 — Whether the record gains a field or changes its content scope

- Question: The record's retained set gains a member. Does `ise-trial/v1` gain
  a field for the declaration, or a new content scope?
- Options:
  1. **Neither** — the declaration is one more `retained_files` entry and
     `retained_digest` covers it; gain: an older verifier re-checks a new trial
     and a new verifier re-checks an older trial, because both compare the
     recorded set; cost: nothing in the record says "this path is special";
     in practice: `retained_files` holds a `givn/specification.yaml` key with
     its 64-hex digest and no other field changes.
  2. **A `declaration` field** — one named path plus its digest beside
     `retained_files`; gain: self-describing; cost: two places to keep in step,
     an older verifier refuses the record's unknown field under
     `deny_unknown_fields`, and the same path hashes twice; in practice: a
     preparation with a current build produces a record an older `givn ise
     verify` rejects.
  3. **A new content scope `ise-trial/v2`** — gain: an explicit compatibility
     boundary; cost: every record reader branches on the version for a shape
     that did not change, and the change is not a format change; in practice:
     a v1-only verifier refuses every new trial for no shape reason.
- Disposition: carried — the deciding artifact is `design.md` (D5); the owner
  is the change author.

### D6 — Whether verification also re-checks the checkout's declaration

- Question: `givn ise verify` re-checks the trial's recorded retained paths and
  the current checkout's `e2e` package and selected scenarios. Should it also
  compare the current checkout's `givn/specification.yaml` to the recorded
  one?
- Options:
  1. **No** — the recorded retained set stays the trial's authority, exactly as
     for every other retained path; gain: a committed declaration edit after a
     trial is not drift, so a verifier is not asked to compare two Specs;
     cost: a project can move its declaration after a trial without a drift
     report; in practice: the developer edits the declaration after
     preparation; `givn ise verify` reports the trial clean and names no path,
     and the recorded declaration remains the evidence of what was applied.
  2. **Yes** — add the checkout's declaration to the drift comparison; gain:
     the current and frozen Specs cannot silently diverge; cost: every later
     editorial edit to the declaration fails verification of every older
     trial, and the rule that a changed file outside the frozen inputs produces
     no drift no longer holds; in practice: adding a `limits:` entry to the
     declaration turns an unrelated trial's clean report into drift.
- Disposition: carried — the deciding artifact is `design.md` (D6); the owner
  is the change author.
