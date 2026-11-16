# Proposal: add-delivery-identity-to-trial-record

## Release note

`givn ise deliver` records a per-path identity of a finished trial's delivered
replacement, and `givn ise verify --delivery` re-checks the delivered bytes and
names every path whose content changed.

## Use-Case Context

- Use-case ID: `prepare-an-ise-test` (`clean-room-preparation`,
  `trial-input-verification`, and the new `trial-delivery-identity`)
- Ideation topic: `ise-rebuildability`
- Seed path: none. No ideation change seed covers this defect; the change
  starts from a live `Ise Test` arm, so there is no seed status line to stamp
  and no seed question to disposition.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

The three IDs come from `prepare-an-ise-test`'s `## Personas` section. The
change writes terminal text and a record file, so it names confirmed Personas
and copies no biography here.

## Review Context

- Review kind: independent
- Reason: major change. It extends the durable `ise-trial/v1` record with a
  second, later-written identity, adds a third `givn ise` subcommand and a
  second mode of `givn ise verify`, and fixes a compatibility rule for records
  already on disk. Every rebuilt product hands over through that record, and
  reversing the change requires another record change plus re-sealing every
  delivered trial.

## Problem / Opportunity

The trial record's `replaced_digest` is a rollup over the replaced paths'
identities **at the record's `revision`**: the files the partition removed from
the trial input. `givn ise verify` compares the recorded retained paths and the
recorded `e2e` package and scenario digests. It never reads a replaced path,
because after preparation those paths are absent from the trial.

A live rebuild exposed what that costs. The arm recorded `replaced_digest
d7edf8b7…`, computed by `givn ise replace --at 768e2c65`, the parent of the
delivered commit. The delivered replaced files hash to `9e8fe3ae…`. Nothing in
the record or in verification mentions the second value, so:

- an edit to a replaced file after the rebuild cannot be detected, and
- the field's name reads as though it described the delivered replacement.

The second point is the defect that turns into a false assurance: the record
answers "which replacement arrived" with "which paths were removed". ADR-076
recorded the same gap as an accepted consequence: "the identity covers the
recorded set; a file the builder adds after preparation is not drift"
(`R-143`). A verifier who holds only that record cannot tell whether the trial
still holds the bytes the builder delivered.

The change adds the missing identity and makes the two names state their
objects.

## Proposed Solution

Observable behavior, from the developer's and verifier's side:

1. **A delivery identity exists.** `givn ise deliver --trial <record> --at
   <trial dir> --receipt <path>` reads a prepared trial's record, re-checks the
   frozen inputs, and writes a delivery record that holds every field of the
   trial record plus a `delivery` block with one identity per delivered path
   and a rollup. The receipt path is required: a delivery identity with no file
   cannot be re-checked later.
2. **The delivered set is the trial minus the retained set.** The identity
   covers every regular file and symlink under the trial directory that the
   recorded retained set does not list: the replaced paths the builder
   recreated, the kept source the builder may rewrite, and any path the builder
   created. The retained identity and the delivered identity then partition the
   trial directory, so no path escapes both. Directories carry no identity.
3. **The seal refuses instead of attesting.** Recording fails by name when the
   frozen inputs already drifted, when the record already holds a delivery
   identity, when the trial directory is missing, when the record cannot be
   read, when the receipt exists or lies inside the checkout or the trial, and
   when a delivered path cannot be read. A failure writes no delivery record.
4. **The seal reports what it bound.** The report prints the trial path, the
   trial revision, the backend, `Retained: clean`, `E2E: clean`, the delivered
   path count, the abbreviated delivered digest, and the delivery record path.
5. **Verification can fail.** `givn ise verify --trial <record> --at <dir>
   --delivery` runs the frozen-input re-check and then compares the delivered
   bytes. It names every path that is missing from the trial, whose content
   differs, or that the trial gained after the delivery was recorded, and it
   exits nonzero.
6. **A record without a delivery identity refuses the delivery re-check.** The
   run fails and names the record and the missing block; it does not fall back
   to the input re-check and report clean.
7. **The default re-check does not claim a delivery check.** Without
   `--delivery`, a record that holds a delivery identity prints `Delivery: not
   checked` beside the two clean lines. A record without the block prints the
   three lines it prints today.
8. **The two names state their objects.** `replaced_digest` means the replaced
   set at the record's `revision`, never the delivered bytes; the delivery
   identity is `delivery.delivered_digest` and `delivery.delivered_files`.
9. **Compatibility.** The record keeps `schema: 1` and `content_scope:
   ise-trial/v1`. A record written before this change stays readable with its
   exact meaning and holds no delivery identity. A record written by
   `givn ise deliver` is refused by an older Givn by its unknown field, which
   is the intended failure: an older reader cannot check the block and must not
   ignore it.

## Capability Routing

`givn spec route --change add-delivery-identity-to-trial-record` reports
signal: `EXTEND givn-workflow-enforcement` (use case `verify-change`, score
17.00). The match comes from the scaffolded proposal's `givn check review`
sentence and the word `route`, not from any trial behavior. That capability
enforces change gates; it reads no trial directory and no trial record. The
single row therefore deviates, and the Rationale column records why.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `trial-delivery-identity` | `EXTEND givn-workflow-enforcement` (`verify-change`, score 17.00, signal; matched on `givn check` and generic words) | `NEW in prepare-an-ise-test` | No permanent capability records or re-checks a trial's delivered replacement. `givn-workflow-enforcement` owns change gates and reads neither a trial directory nor a trial record. The trial record belongs to `prepare-an-ise-test`, which owns both existing trial capabilities, so the new capability is a third one under that use case. |

No other capability gains a scenario. The clarified meaning of
`replaced_digest` is a record rule, so the delta use-case document states it
under `## Rules`, and no `trial-input-verification` scenario changes: the
default re-check keeps its comparisons, its reasons, and its three clean lines.

## Out of Scope

- What `givn ise replace` copies, its partition, its nine refusals, its report,
  and its record fields other than the new optional block.
- The retained comparison, the `e2e` package and scenario rules, and the
  checkout's declaration handling in the default re-check.
- The identity algorithm (`path`, kind, executable marker, bytes or symlink
  target) and the rollup shape; the delivery identity reuses them.
- A delivered revision field, a commit binding, or a re-check of the delivered
  commit in the checkout. The delivery identity covers the trial directory.
- Archive behavior: no gate reads a delivery identity, and no archive path
  changes.
- A record version bump, a `ise-trial/v2` reader, and a migration of records
  already written.
- Enabling `ise` on this repository, declaring Givn's own `Specification`, and
  any self-rebuild. The addon stays off.
- Running the frozen `e2e` suite and building the replacement: the independent
  verifier and the rebuilding agent, both outside Givn.

## Open Questions

### D1: Which artifact carries the delivery identity, and which command writes it

- Question: After a rebuild, the replaced paths' delivered bytes need an
  identity a verifier can re-check. Which file holds it, and which command
  computes it? The answer fixes whether the identity is evidence or a
  restatement of what the checker just read.
- Options:
  1. **A third subcommand writes a new record carrying the input fields plus a
     `delivery` block** (`givn ise deliver --trial <record> --at <dir>
     --receipt <path>`). It reads the record, re-checks the frozen inputs,
     hashes the delivered set, and writes a new record with create-new
     semantics. Gain: the identity is taken at a defined moment by a step that
     is not the checker, one file pairs input and delivery evidence, and the
     input record stays untouched. Cost: one more command, and the delivery
     record repeats the input fields. In practice:
     `givn ise deliver --trial /tmp/ise-passuff-trial.yaml --at
     /tmp/ise-passuff --receipt /tmp/ise-passuff-delivery.yaml` prints
     `Delivered digest: 9e8fe3ae7d01…` and writes the second file.
  2. **`givn ise verify` writes the block when asked** (`--delivery --receipt
     <path>`). Gain: no new command. Cost: the identity is computed from the
     same bytes the same run compares, so it always matches and detects
     nothing, and the verifier becomes a writer. In practice:
     `verify --delivery --receipt /tmp/d.yaml` seals a trial the builder edited
     after the arm closed, then reports it clean.
  3. **A separate record with its own `ise-delivery/v1` scope naming the trial
     record by digest.** Gain: the trial record keeps one author, and each file
     stays small. Cost: a verifier must hold and pair two files, the reader
     accepts two scopes, and a mismatched pair is a new refusal class. In
     practice: `/tmp/delivery.yaml` carries `trial_record_digest: …` and the
     re-check needs `--trial /tmp/trial.yaml` beside it.
- Disposition: carried. The deciding artifact is `design.md`, whose Decisions
  ledger resolves D1 under the change author's ownership.

### D2: Which names the two identities carry, and the compatibility rule

- Question: `replaced_digest` names the replaced set at the trial input
  revision, and a reader took it for the delivered replacement. Does the
  existing field get renamed, does the new identity get a name that cannot be
  confused with it, and what happens to records already written?
- Options:
  1. **Keep `replaced_digest`, name the new block `delivery` with
     `delivered_digest` and `delivered_files`.** Gain: every record already
     written stays readable with its exact meaning, the scope stays
     `ise-trial/v1`, and the two names sit side by side in one file; an older
     Givn refuses a delivery record by its unknown field instead of ignoring
     the block. Cost: the old name stays, so the object is stated by the block
     name, the glossary, and the use-case rules rather than by the field alone.
     In practice: `replaced_digest: d7edf8b7…` beside
     `delivery: {delivered_digest: 9e8fe3ae…, …}` in one record.
  2. **Rename `replaced_digest` to `replaced_input_digest` and bump the scope to
     `ise-trial/v2`.** Gain: the field name states its object without a second
     lookup. Cost: a second scope, a permanent alias reader for every record
     already written, and `replaced_paths` needs the same rename to stay
     coherent. In practice: a v1 record is read through a mapping branch, and a
     v2 record is refused by every older binary.
  3. **Reuse `replaced_digest` for the delivered rollup after the rebuild.**
     Gain: no new field. Cost: one field would mean two objects at two times,
     which is the defect this change removes. In practice: the same key carries
     `d7edf8b7…` before the rebuild and `9e8fe3ae…` after it, and a reader
     cannot tell which one a given file holds.
- Disposition: carried. The deciding artifact is `design.md` (D2).

### D3: Which paths the delivery identity covers

- Question: The builder may write the replaced paths, rewrite the kept source,
  and create new paths. Which of those sets does the delivery identity bind?
- Options:
  1. **Every trial path outside the recorded retained set.** Gain: the retained
     identity and the delivered identity partition the trial directory, so no
     path is outside both, and a verifier can name an edited kept-source file
     and an added file. Cost: the seal fixes implementation bytes that the
     input re-check deliberately leaves open, so a later edit to a kept-source
     file is delivery drift. In practice: an edit to `src/keys/mod.rs` after
     the seal is named in delivery mode and is no drift in input mode.
  2. **Only the recorded replaced paths.** Gain: the narrow question "do the
     delivered replacement paths still hold their bytes". Cost: a kept-source
     edit and a new file after the seal stay unbound, so the record again reads
     as complete while part of the trial is unbound. In practice: the builder
     adds `src/keys/views.rs` after the arm closes, and both modes report
     clean.
  3. **Every trial path, the retained set included.** Gain: one whole-tree
     identity covers the directory without a set difference. Cost: a retained
     path gains a second identity channel that must agree with
     `retained_digest`, and one retained edit is reported twice. In practice:
     an edit to `givn/specs/demo/spec.feature` prints a retained drift line and
     a delivery drift line for the same path.
- Disposition: carried. The deciding artifact is `design.md` (D3).

### D4: How a verifier asks for the delivery re-check, and what it refuses

- Question: The verifier already runs `givn ise verify --trial <record> --at
  <dir>`. How is the delivered check selected, and what happens when the record
  holds no delivery identity?
- Options:
  1. **A `--delivery` flag on `givn ise verify`.** The flag selects delivery
     mode: the run refuses a record with no `delivery` block and otherwise
     runs the frozen-input checks plus the delivered check. Gain: one command
     and one record answer both questions, and the trust-anchor checks are
     shared. Cost: the command gains a second mode beside the one it is named
     for. In practice: `verify --trial r --at d --delivery` on an input-only
     record exits 1 and names the record and the missing block.
  2. **A `givn ise verify-delivery` subcommand.** Gain: one action per command
     name. Cost: it repeats the record load, the belonging checks, and the
     input re-check, or shares them as private helpers; two commands then
     differ by one flag. In practice: a typo in the subcommand name verifies
     the wrong question, because `verify` still exists.
  3. **`verify` always checks delivery when the record holds a block.** Gain:
     no flag to remember. Cost: a verifier cannot ask the input question alone,
     and the meaning of the report changes with the record's content. In
     practice: the same command reports three clean lines on an older record
     and four on a delivered one, and the exit code no longer says which
     question was asked.
- Disposition: carried. The deciding artifact is `design.md` (D4).

### D5: What a delivery must satisfy before its identity is recorded

- Question: A seal can be taken over a trial whose frozen inputs already
  drifted, and a rebuild may legitimately drop a replaced path. Does recording
  require a clean input, and must every replaced path have a delivered file?
- Options:
  1. **The frozen inputs must re-check clean, and absent replaced paths are
     allowed.** Gain: a seal is never taken over a trial whose retained or
     `e2e` evidence already drifted, and a rebuild that merges or drops a file
     still seals. Cost: the record shows which input paths have no delivered
     file without a refusal. In practice: `--full` removes 12 paths and the
     builder writes 7; the delivery lists 7, and the record carries both lists
     for a reader to compare.
  2. **The inputs must be clean, and every replaced path must exist.** Gain:
     every replaced path carries a delivered identity. Cost: a rebuild that
     merges or drops a replaced file cannot be sealed, which under `--full` is
     most real rebuilds. In practice: a smaller rebuild refuses with 5 absent
     paths, and the developer must keep placeholder files to satisfy it.
  3. **No precondition.** Gain: recording never refuses on the input side.
     Cost: a seal over an edited trial records the edited bytes as the
     delivery, and a later verifier cannot tell the input had already drifted.
     In practice: an edited `givn/specification.yaml` is sealed, and delivery
     mode reports clean because the identity describes what it found.
- Disposition: carried. The deciding artifact is `design.md` (D5).
