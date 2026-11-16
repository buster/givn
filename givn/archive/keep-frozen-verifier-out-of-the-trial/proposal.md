# Proposal: keep-frozen-verifier-out-of-the-trial

## Release note

`givn ise replace` keeps the frozen End-to-end (E2E) test package out of the clean-room trial while still recording its identity, and `givn ise verify` refuses a trial that holds a path the frozen package covers.

## Use-Case Context

- Use-case IDs: `prepare-an-ise-test` (`clean-room-preparation` and
  `trial-input-verification`)
- Ideation topic: `ise-rebuildability`; the change implements the settled rule
  that the frozen End-to-end (E2E) test package runs outside the builder's write
  authority during an Ise Test.
- Seed path: none. The defect comes from a live `passuff` Ise Test dry run, not
  from an ideation change seed, so there is no seed status line to stamp and no
  seed question to disposition.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

The three IDs come from `prepare-an-ise-test`'s `## Personas` section. The
change writes terminal text and a record field, so it names confirmed Personas
and copies no biography here.

## Review Context

- Review kind: independent
- Reason: major change. It changes what a clean-room trial contains, so every
  record written by this binary attests a different trial content than a record
  of the same scope written by an earlier one; it adds a record field and a
  verification refusal; and it removes the builder's only local acceptance
  signal. Reversing it requires another record rule, a re-issued trial for every
  open Ise Test, and re-establishing the frozen-boundary guarantee.

## Problem / Opportunity

A `passuff` trial seeded by `givn ise replace --full` retained 32 files, and the
retained set included `crates/passuff-e2e`, the frozen acceptance suite. The
builder could read the tests, the mocks, and the exact strings they assert. It
reconstructed the product and the frozen suite passed 8 of 8 on the first
build.

That pass proves nothing about the Specification. The trial's own `Spec gap`
records say why: two records classify the tests as depending on undeclared
operation names and an undeclared notification log, and five record exact
strings the Specification never states, including a heading, two rendered
labels, two error messages, and a notification title. The builder reconstructed
the product from the tests, not from the Specification, and the frozen suite
could not tell the difference.

The cause is that preparation copies the `e2e` role into the trial like every
other retained role. The record already separates the package's identity
(`e2e_files`, `E2E digest`) from the retained set, but the copy set does not
follow that separation: `retained` is the union of all five roles plus the
declaration, and the trial receives it whole. A trial that carries the suite
gives the builder read authority over its own acceptance evidence, and the
verifier then reports `E2E: clean` for a suite the builder could read.

## Proposed Solution

Observable behavior, from the developer's and verifier's side:

1. **The trial holds no path of the frozen package.** `givn ise replace`
   writes the declaration, the four non-frozen retained roles, and the kept
   source. A path the declared `e2e` role resolves to is not written, counted
   as retained, or counted as replaced.
2. **The record still names the package.** The record keeps one identity per
   frozen path and the `E2E digest`, and a new field names the roles whose
   files the trial does not hold. A reader learns from the record that the
   exclusion is deliberate rather than a missing role.
3. **Verification still covers the package.** `givn ise verify` compares the
   recorded frozen identities against the current checkout and names every
   path that is missing, differs, or was gained, exactly as it does today.
4. **A trial that holds a frozen path refuses.** Verification refuses by name,
   listing the offending paths, before it reports any clean line.
5. **The report stays honest.** `Retained` counts only what the trial holds,
   `E2E digest` keeps its value, and one report line states that the frozen
   package was not copied and how many files it covers.
6. **The cost is stated.** A builder can no longer run the acceptance suite
   during reconstruction. The `@e2e` scenarios stay in the behavior corpus, so
   the Specification still states what is accepted, and the component's own
   scenarios stay in the trial as the builder's local feedback.

## Capability Routing

`givn spec route --change keep-frozen-verifier-out-of-the-trial` reports signal:
`EXTEND givn-workflow-enforcement` (use case `verify-change`, score 27.00). The
match comes from the scaffolded proposal's `givn check` sentence, the word
`route`, and generic goal words. That capability enforces change gates; it reads
no trial directory, no trial record, and no ADR register. All three rows
therefore deviate, and the Rationale column records why.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `clean-room-preparation` | `EXTEND givn-workflow-enforcement` (`verify-change`, score 27.00, signal; matched on `givn check`, `route`, and generic words) | `EXTEND clean-room-preparation` | What changes is what a trial contains and what the report counts. `prepare-an-ise-test` owns trial materialization; `givn-workflow-enforcement` owns change gates and reads no trial directory. |
| `trial-input-verification` | `EXTEND givn-workflow-enforcement` (same recommendation, same score) | `EXTEND trial-input-verification` | What changes is one verification refusal and the set the trial-side comparison runs over. `prepare-an-ise-test` owns the frozen-input re-check. |
| `adr-qualification-gate` | `EXTEND givn-workflow-enforcement` (same recommendation, same score) | `EXTEND adr-qualification-gate` | Recording ADR-078 moves the active register from 29 to 30 accepted decisions, and the permanent scenario with its binding asserts the new count in this change. `adr-qualification-gate` owns the register; `givn-workflow-enforcement` owns change gates and reads no ADR register. |

## Out of Scope

- The `e2e` boundary check that `givn check tasks` runs: the package resolution,
  its disjointness from the `source` roots, the runner-command coverage, and the
  advisory placement findings.
- The `behavior` role: the `@e2e` scenarios stay in the trial, so the
  Specification still states which behaviours are accepted and keeps the
  scenario digest the record compares.
- The archive guard over declared retained paths, the declaration retention
  rule, and the implicit retained entry.
- The delivery identity, `givn ise deliver`, and `givn ise verify --delivery`.
  The change that adds them is in review on this branch; this change touches no
  field of its `delivery` block and no glossary row it owns.
- The nine preparation refusals, the target containment checks, the identity
  algorithm, and the rollup shape.
- Enabling `ise` on this repository, declaring Givn's own Specification, and any
  self-rebuild. The addon stays off.
- Building the replacement and running the frozen suite: the rebuilding agent
  and the independent verifier, both outside Givn.

## Open Questions

### D1: What the trial holds in place of the frozen package

- Question: The frozen package is not copied. Does the trial record the absence
  with a note file, with a listing of the paths it excludes, or with nothing at
  all? The answer fixes whether the trial still contains exactly one revision's
  files.
- Options:
  1. **Nothing at all.** Gain: the trial remains exactly the revision's files
     minus two declared sets, so the write-time comparison stays "every written
     file equals its revision content" and no synthesized file needs a rule.
     Cost: a builder reads a declaration that names `e2e:` and finds no such
     directory, and may report the absence as a defect. In practice:
     `crates/passuff-e2e` is simply absent from `/tmp/ise-passuff`.
  2. **A note file at the trial root.** Gain: the builder learns the exclusion
     without reading the Specification. Cost: the trial holds a file that is not
     at the revision, so the copy set, the write-time comparison, and the record
     each need an exception, and the builder may edit or delete the note freely.
     In practice: `/tmp/ise-passuff/ISE-TRIAL.md` states that the acceptance
     suite is frozen and absent.
  3. **A listing of the excluded paths.** Gain: the builder knows the package
     layout it cannot see. Cost: the listing exposes the test file names, which
     describe the covered areas, and it needs the same synthesized-file
     exception as the note. In practice: a file at the trial root lists
     `crates/passuff-e2e/tests/keys.rs` and its siblings with no contents.
- Disposition: carried. The deciding artifact is `design.md`, which resolves D1
  under the change author's ownership.

### D2: Where the frozen identity is verified from

- Question: The trial no longer holds the package. Does verification keep
  comparing the recorded frozen identities against the current checkout, and
  what does it refuse when the checkout's package is missing or differs?
- Options:
  1. **Keep the checkout comparison and refuse per named path.** Gain: the
     existing comparison already reads the checkout, so the identity is still
     verified and every drift names a path. Cost: a project that moves the
     package between preparation and verification reports drift rather than a
     clean re-check, which is the current behaviour. In practice:
     `E2E drift: crates/passuff-e2e/tests/keys.rs (the checkout differs from the
     record)` and exit 1.
  2. **Compare the package against the trial.** Gain: one root for every
     identity. Cost: the comparison fails on every recorded path, because the
     trial deliberately holds none, so the identity stops being usable. In
     practice: every run reports the whole package as missing from the trial.
  3. **Stop verifying the package once it is not copied.** Gain: no comparison
     to maintain. Cost: the frozen suite's identity is recorded and never
     checked, so a suite edited after preparation passes unnoticed. In practice:
     a changed assertion in `crates/passuff-e2e/tests/keys.rs` is invisible.
- Disposition: carried. The deciding artifact is `design.md` (D2).

### D3: How the report states the exclusion

- Question: The report's `Retained` count drops by the frozen files and
  `E2E digest` keeps its value. Does a reader learn from the report that the
  package was deliberately not copied, and if so on which line?
- Options:
  1. **One conditional report line naming the role and the file count.** Gain:
     the developer sees the exclusion at the moment the trial is handed over,
     and the count matches the recorded identities. Cost: the report gains a
     line, so the scenario that asserts the fixed lines changes. In practice:
     `Frozen package: e2e, not copied (2 files)` between `E2E digest` and
     `Written path`.
  2. **Rely on `Retained` alone.** Gain: no new line and no scenario change.
     Cost: a smaller `Retained` value carries no explanation, and a reader
     cannot tell a deliberate exclusion from a shrinking declaration. In
     practice: `Retained: 2` beside `E2E digest: 82be07c3e5a9…` with nothing
     saying the suite is absent.
  3. **An unconditional line for every project.** Gain: one report shape. Cost:
     it prints for projects whose declaration declares no `e2e` role, where
     nothing is frozen and the line states an empty fact. In practice: a project
     with `e2e: []` prints `Frozen package: e2e, not copied (0 files)`.
- Disposition: carried. The deciding artifact is `design.md` (D3).

### D4: The record field and the compatibility rule

- Question: The trial's content is part of what the record attests. Does the
  record gain a field naming the exclusion, and does `ise-trial/v1` stay the
  scope?
- Options:
  1. **Add `excluded_roles` and keep `ise-trial/v1`.** Gain: every record
     already written stays readable, and a record written from now on states
     which roles its trial omits, so `Retained` is unambiguous. Cost: a record
     written by this binary is refused by an earlier binary through the
     existing unknown-field rule. In practice: `excluded_roles: [e2e]` beside
     `counts: {retained: 2, …}` and `e2e_files: {…}`.
  2. **Bump the scope to `ise-trial/v2`.** Gain: the scope alone separates old
     and new records. Cost: every record on disk becomes unreadable, so every
     open Ise Test must be re-prepared, and a permanent alias reader must exist
     for the old scope. In practice: `givn ise verify` on last week's receipt
     refuses with the schema message.
  3. **Add no field.** Gain: one unchanged record shape. Cost: an old record
     (suite in the trial) and a new one (suite absent) are indistinguishable
     except through `givn_version`, which is a string no reader interprets. In
     practice: `Retained: 32` and `Retained: 30` both look like ordinary
     counts.
- Disposition: carried. The deciding artifact is `design.md` (D4). This change
  and `add-delivery-identity-to-trial-record` both alter the record: that change
  adds an optional `delivery` block, this one an optional `excluded_roles`
  field. Both edits are additive, neither renames a field, and both keep
  `ise-trial/v1`.

### D5: The refusal for a trial that holds a frozen path

- Question: What does the guarantee mean mechanically, and what happens when a
  trial already holds a path the frozen package covers?
- Options:
  1. **`givn ise replace` never writes one, and `givn ise verify` refuses a
     trial that holds one.** Gain: the guarantee is checked on both sides, so a
     trial assembled by another path (a copied directory, a retained archive, a
     hand-edited record) cannot pass a re-check. Cost: one more refusal path in
     verification, and a project that must keep a file under the package root in
     the trial cannot. In practice: a verifier copies
     `crates/passuff-e2e/tests/keys.rs` into the trial and the re-check fails
     naming that path.
  2. **Only preparation guarantees it.** Gain: no verification change. Cost: a
     trial that gains a frozen path after preparation is verified clean, so the
     guarantee is a preparation property rather than a trial property. In
     practice: the planted file is invisible to `givn ise verify`.
  3. **Refuse a replacement path that resolves under the frozen package, and
     change nothing in verification.** Gain: one refusal site. Cost: the same
     as option 2 for a trial that was not prepared by this command, and
     `--path` already refuses a role-matched path. In practice:
     `--path crates/passuff-e2e/tests/keys.rs` refuses, and a copied file after
     preparation is still invisible.
- Disposition: carried. The deciding artifact is `design.md` (D5).

### D6: The builder's lost local acceptance signal

- Question: A builder can no longer run the frozen suite during reconstruction.
  What replaces it, and what does the developer lose that no Givn artifact
  restores?
- Options:
  1. **The behavior corpus's component scenarios are the builder's local
     feedback, and the loss is recorded as a risk.** Gain: the builder still
     runs the scenarios that state the accepted behaviour, and the risk is
     visible to the reviewer and the operator. Cost: a builder meets the frozen
     suite only when the verifier runs it, so a missing mock or a structural
     mismatch surfaces after the build rather than during it. In practice: the
     builder runs `cargo test` against the component's own scenarios and never
     sees `crates/passuff-e2e`.
  2. **Ship a stub package that fails every test.** Gain: the builder sees the
     package's shape. Cost: a stub that names the tests reintroduces the leak at
     the assertion-name level, and a stub that does not name them tells the
     builder nothing. In practice: an empty `crates/passuff-e2e/tests/` reveals
     nothing and still occupies the path.
  3. **Keep the package when the declaration marks it optional.** Gain: a
     project can choose. Cost: the trap the change closes comes back behind a
     declaration toggle, and the live dry run's leak becomes a configuration
     mistake rather than a defect. In practice: `e2e_readable: true` reproduces
     the 8-of-8 pass.
- Disposition: carried. The deciding artifact is `design.md` (D6).
