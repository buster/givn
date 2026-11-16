<!-- givn:base-sha256:7953c190a04eed4909012c16d586293f8872ff778c779ee0e508e1c26cad51cb -->
# Use case: prepare-an-ise-test

## Level

!

## Actors

- Developer (the consumer-project owner; the Givn maintainer when Givn rebuilds itself)
- Supporting: Independent verifier (runs the frozen End-to-end (E2E) test target and asks for the input or delivery re-check)

## Goal

Produce a clean-room trial whose frozen inputs and delivered replacement a verifier can confirm.

## Trigger

The developer starts an Ise Test.

## Preconditions

- `ise` is effective in the project, which requires `architecture_principles`.
- The declaration names the five retained roles and the replaceable source roots, and the selected revision records it.
- The checkout is clean.
- One backend is selected; no missing, mixed, or nested repository.
- A revision is available to read; the selected revision defaults to `HEAD`.
- The component's paths fall inside the declared source roots.
- A delivery identity is recorded only over a trial directory that a readable trial record names, and only after the frozen inputs re-check clean.

## Main flow

1. The developer selects the revision, or accepts `HEAD`, and names the replacement unit's paths with `--path` (a tracked file or a directory) or names the whole declared source set with `--full`; the system reads the declaration recorded at that revision and the tracked files at that revision.
2. The system partitions the tracked paths into the retained roles and the declaration itself, the kept source outside the replaced paths, the replaced paths, and the files outside the declaration, and counts the tracked files in the replaced paths' directories that the developer did not name.
3. The system refuses by name, before any write, on a missing or malformed declaration, an addon that is not effective, a dirty checkout, a missing, mixed, or nested backend, a retained path or the declaration that is missing at the revision, a `--path` value outside every source root, a `--path` value or directory expansion matched by a retained role or by the declaration, a write target inside the checkout or already existing, and a repository link or nested repository among the files written into the trial.
4. The system writes the retained roles, the declaration, and the kept source into one plain directory outside the checkout, preserving relative paths, the executable bit, and symlink targets, with no version-control metadata and no ignored or untracked files.
5. The system re-reads every written file, compares it to the file recorded at the selected revision, records a tamper-evident identity for the retained set including the declaration, the `e2e` package, and the replaced input paths, reports the counts, and prints the trial record with the revision, the replaced paths, the counts, the identities, the backend, and the running Givn version; a receipt path writes that record to the named file.
6. After the build, the developer records the delivery identity: the system re-checks the frozen inputs, computes one identity per trial path the recorded retained set does not list, and writes a delivery record outside the checkout and outside the trial, created new, which holds every field of the trial record it read plus the delivered identities; the report prints the trial path, the trial revision, the backend, the two clean input lines, the delivered path count, the abbreviated delivered digest, and the delivery record path.
7. The independent verifier confirms the frozen inputs on request; the system confirms the record belongs to this trial and this backend, then re-checks the recorded identities, the declaration included, against the written trial and the `e2e` package and reports drift per path.
8. The independent verifier re-checks the delivered replacement on request; the system runs the frozen-input re-check and then compares the delivered bytes against the recorded delivery identities, names every delivered path that is missing from the trial, whose content differs, or that the trial gained after the delivery was recorded, and prints a `Delivery:` line only in that mode.

## Extensions

- 3: the declaration is malformed, not UTF-8, or unreadable at the selected revision → the system refuses, names the declaration path and the loader's or decoding reason, and writes nothing; protects the minimal guarantee.
- 3: the selected revision does not record the declaration → the system refuses, names `givn/specification.yaml` and the revision, and writes nothing; protects the minimal guarantee.
- 3: `ise` is not effective, or its `architecture_principles` prerequisite is not effective → the system refuses by name, reads no declaration, and writes nothing; protects the minimal guarantee.
- 3: neither `--full` nor `--path` is given, both are given, or `--into` is omitted → the command refuses as a usage error, names the missing or conflicting flags, exits 2, and writes nothing; protects the minimal guarantee.
- 3: the checkout is dirty → the system refuses, names every dirty path, and writes nothing; protects the minimal guarantee.
- 3: no backend owns the project, both markers sit at the root, or the project root is nested inside a worktree → the system refuses with the backend selector's message and writes nothing; protects the minimal guarantee.
- 3: a declared retained path is missing at the revision → the system refuses and names the path and the revision; protects the minimal guarantee.
- 3: a `--path` value falls outside every declared source root → the system refuses and names the path; protects the minimal guarantee.
- 3: a `--path` value, or a tracked file under a named directory, is matched by a retained role or is the declaration → the system refuses, names the path and the role or the declaration, and writes nothing; protects the minimal guarantee.
- 3: a write target is inside the checkout, already exists, is a receipt inside the trial, or is a receipt that is an ancestor of the trial → the system refuses and names the target; protects the minimal guarantee.
- 3: a copied file is a repository link or a nested repository → the system refuses and names the file; protects the minimal guarantee.
- 5: a written file differs from its content at the selected revision → the system names the drifting paths, removes the partial trial, and writes no trial record; protects the success guarantee.
- 5: the revision cannot be read → the system refuses before any write and names the path and the revision; protects the minimal guarantee.
- 5: the record cannot be written → the system names the record path, removes the trial it created, and reports no success; protects the success guarantee.
- 6: the trial directory is missing, or the trial record cannot be read → recording fails, names what is missing or unreadable, and writes no delivery record; protects the minimal guarantee.
- 6: a recorded retained or `e2e` identity already drifted → recording fails, names every drifting path, and writes no delivery record; protects the success guarantee.
- 6: the record already holds a delivery identity → recording fails, names the record, and writes no second record; protects the success guarantee.
- 6: the record does not belong to this trial or names another backend → recording fails and names the record and the mismatch; protects the success guarantee.
- 6: the receipt exists, is inside the checkout, or is inside the trial → recording fails and names the receipt; protects the minimal guarantee.
- 6: a delivered path cannot be read → recording fails, names the path, and writes no delivery record; protects the success guarantee.
- 7: the record does not belong to this trial, names another backend, or lies inside the trial → the system refuses and names the record and the mismatch; protects the success guarantee.
- 7: a recorded retained or `e2e` identity no longer matches → verification fails and names every drifting path, the declaration included; protects the success guarantee.
- 8: the record holds no delivery identity → the delivery re-check fails, names the record, and reports no clean result; protects the success guarantee.
- 8: a delivered path is missing from the trial, differs, or was gained after the record was written → verification fails and names every such path; protects the success guarantee.

## Rules

- Deletion is set arithmetic: preparation removes every tracked file named with `--path`, and every tracked file beneath a directory named with `--path` at the selected revision; a glob-shaped value is never expanded; `--full` removes every tracked file under every declared `source:` root; a named path absent from the revision removes nothing and is not one of the nine refusals.
- The declaration is an implicit retained entry, not a sixth role: every trial retains `givn/specification.yaml` under `--full` and under `--path`, no role declares it, and the copy recorded at the selected revision is the one the partition reads, so a builder reads the roles that were applied. The retained set is a union, so a retained role that also matches the declaration counts it once, and `Retained` equals the number of the record's `retained_files` entries. Naming the declaration with `--path`, or naming a directory whose tracked files include it, refuses like any retained path.
- Internal unit tests are named the same way as any other replaced path; no unit-test path is inferred from `#[cfg(test)]`, `test_*.py`, a Maven test root, or a file-name convention.
- A retained role entry that matches a directory retains every tracked file under it; an entry that matches a file retains that file, and a path matched by a role or by the declaration is never replaced.
- A tracked path that matches both a retained role and a `source:` root is retained: the retained role wins, the file stays in the trial under `--full` as well as under `--path`, and `Replaced` counts only files that are not retained. The declaration is retained on the same terms, so a `source:` root that covers it replaces nothing.
- `Not replaced` counts the direct tracked files in the replaced files' parent directories that the developer did not name; it is a containment count, never a refusal, and it never changes the recorded sets. A `--full` replacement removes every tracked file under the source roots that no retained role matches, so the report shows `Not replaced: 0` when no retained path sits under a source root, and counts each retained file that does, the declaration included.
- `Outside the declaration` counts the tracked files that fall outside every declared role and outside every source root and are not copied; they are neither errors nor trial content. The declaration is never counted there: it is always copied.
- Only the `End-to-end (E2E) test` package is frozen; component step definitions, fakes, and harness are implementation, never frozen or diff-counted, and enter the trial only as kept source when the declaration places them inside a source root.
- The `E2E digest` covers the `@e2e` scenarios selected by tag from the `.feature` files matched by the declared `behavior` role, and every tracked file of the resolved `e2e` package; the `e2e` role's matched paths resolve to their nearest package manifest at or above them, and a matched path that resolves no manifest is its own root.
- A scenario is frozen only after it was observed failing against the pre-change implementation for the intended reason; a mid-trial test defect stops the trial as a `Spec gap: E2E test defect`, and the trial restarts after the E2E test is corrected because the `e2e` package is part of the Specification.
- The trial is a plain directory outside the checkout, never a branch or a worktree, because a branch carries history and a Fossil checkout carries the repository database.
- Preparation reads the selected revision (`HEAD` when none is named), never the working tree, so ignored and untracked files cannot enter the trial; the declaration the partition reads and the declaration the trial holds are both the revision's copy. The checkout is never written, including the receipt, and the replaced component's Source code and its history are absent from the trial.
- Preparation and the re-check use the project's one selected backend; a missing, mixed, or nested repository is refused by name.
- Every refusal is checked before the first write, names its condition, and leaves the checkout and the target path unchanged.
- The command surface is `givn ise replace [--at <revision>] (--full | --path <path>...) --into <dir> [--receipt <path>] [--dry-run]`; `--at` defaults to `HEAD`, exactly one of `--full` or `--path` is required, and `--into` is required and must lie outside the checkout.
- `--path` is repeatable and accepts a tracked file or a directory; a directory expands to every tracked file beneath it at the selected revision, the retained-role and declaration check runs over the expansion, and a directory that contains a retained path or the declaration refuses.
- The two write targets are the trial directory and, when given, the receipt file; both lie outside the checkout, the trial directory does not exist, and the receipt does not exist, does not lie inside the trial directory, and is not an ancestor of it.
- `--dry-run` performs every read, the partition, every refusal, the counts, and the identities, writes no directory and no record, and prints the ten fixed report lines, the receipt path when given, and the `Would write:` block with the sorted relative paths the trial would hold, the declaration included.
- An omitted `--receipt` prints the trial record to stdout after the report, prints no `Receipt path` line, and writes nothing, so `givn ise verify --trial <receipt>` requires an explicit path.
- The trial record carries the revision, the named paths, the counts, the retained, `e2e`, and replaced-input identities, the backend, the written trial path, and the running Givn version; it carries one digest per retained and `e2e` path, the per-feature scenario digest, and one rollup per identity, so a verifier names drift per path. The retained set includes `givn/specification.yaml`, so `Retained` counts the declaration and every role-retained file once, and `Retained digest` covers it.
- The trial record's `replaced_digest` is the rollup over the replaced set's identities at the record's `revision`: it describes the trial input that was removed, never the delivered bytes. The delivery identity is separate and named for it: an optional `delivery` block carries `delivered_files` (one identity per delivered path) and `delivered_digest` (their rollup), and a record without the block holds no delivery identity. The block adds no schema version: the record keeps `schema: 1` and `content_scope: ise-trial/v1`.
- The record's per-path identity names the entry kind (`f`, `x`, or `l`) beside the path and the executable marker, so a symlink and a regular file with the same target text are different entries.
- The report prints the fixed lines `Backend`, `Revision`, `Retained`, `Kept source`, `Replaced`, `Outside the declaration`, `Not replaced`, `Retained digest`, `E2E digest`, and `Written path`, plus `Receipt path` when a receipt path was given.
- The two printed digests are the first 12 lowercase hex characters of the rollup followed by `…`; the record carries the full digests.
- With `ise` off, preparation, recording a delivery identity, and the re-check refuse by name and no declaration is read; the effective project configuration decides effectiveness, and a change lock decides the change-scoped boundary gate, not these project-scoped commands. A project whose `architecture_principles` is off refuses the same way.
- The trial directory and the receipt are created with create-new semantics; an existing path is refused, never overwritten.
- The re-check refuses unless the record's written path is the given trial directory, the record's backend is the selected backend, and the record path lies outside the trial; it then asserts the recorded replaced digest and re-checks the retained and `e2e` identities, the declaration included. The trial's declaration is re-checked against the record like every retained entry; the checkout's declaration is consulted only where it derives the frozen End-to-end (E2E) test package and the `behavior` feature selection, so an edit that moves `e2e:` or `behavior:` is drift and an edit that moves only `structure:` or `limits:` is not.
- The delivery identity is recorded by `givn ise deliver --trial <record> --at <dir> --receipt <path>`; `--receipt` is required, because a delivery identity with no file cannot be re-checked later, and the delivery record is created new outside the checkout and outside the trial.
- Recording a delivery identity confirms the record belongs to the trial first, on the same terms as the re-check: the recorded written path is the given trial directory, the recorded backend is the selected backend, and the record path lies outside the trial. A mismatch refuses and names the record and the trial, the backend, or the placement.
- The delivered set is every regular file and symlink under the trial directory whose path the recorded `retained_files` does not list: the replaced paths the builder recreated, the kept source the builder may rewrite, and any path the builder created. Directories carry no identity; a delivered symlink is identified by its target text, not by the content it points at. The retained identity and the delivered identity therefore partition the trial directory.
- The delivery record holds every field of the trial record it read, unchanged, plus the `delivery` block; the input record file is never rewritten, so one trial yields one input record and one delivery record.
- Recording a delivery identity requires the frozen inputs to re-check clean: a drifting retained or `e2e` identity refuses and names every drifting path, and no delivery record is written. A replaced path that has no delivered file is allowed, and the record carries both lists so a reader compares them.
- Recording a delivery identity refuses a record that already holds one, a record it cannot read, a missing trial directory, a receipt that exists or lies inside the checkout or the trial, and a delivered path it cannot read.
- The delivery identity names no revision: the delivered bytes are the trial directory's bytes at the moment the identity is recorded, and the record's `revision` stays the trial input revision. A delivered commit in the checkout is not re-read.
- The delivery report prints the trial path, the trial revision, the backend, `Retained: clean`, `E2E: clean`, `Delivered: <n>`, the abbreviated delivered digest as `Delivered digest: <12 hex>…`, and `Receipt path: <path>`.
- Delivery mode is `givn ise verify --trial <record> --at <dir> --delivery`: it runs the frozen-input re-check and then compares the delivered bytes. It refuses a record that holds no `delivery` block, naming the record and the missing identity, and it names every delivered path that is missing from the trial, whose content differs, or that the trial gained after the record was written.
- The default re-check keeps its comparisons and its three clean lines; on a record that holds a delivery identity it adds `Delivery: not checked`. Delivery mode prints a fourth `Delivery: clean` line on a clean run and the delivery drift lines on a failing one.
- A record written before this change, and a record `givn ise replace` writes now, holds no `delivery` block and stays readable with its exact meaning. An older Givn refuses a delivery record by its unknown field instead of ignoring the block.
- The trial record does not carry the verifier's runner command; the independent verifier chooses and records it outside the record.
- On unix, the executable bit and a symlink are written as recorded; on a platform without symlink support, a symlink entry refuses by name before the trial is recorded.
- The verifier names every drifting path in one run and never edits the trial; a changed component file outside the frozen inputs produces no drift in the default re-check, and is delivery drift once a delivery identity was recorded.
- The `e2e` package, the `@e2e` scenario selection, and the retained set are the only frozen inputs; the delivered set is evidence of what the builder wrote, not a second freeze of the Specification.

## Examples

- A `passuff` project declares `source: ["crates/**", "scripts/**"]` and runs `givn ise replace --at 8c1f2ab --path crates/passuff-tui/src/keys/parser.rs --path crates/passuff-tui/tests/keys.rs --into /tmp/ise-passuff --receipt /tmp/ise-passuff-trial.yaml`; the report prints `Backend: git`, `Revision: 8c1f2ab`, `Retained: 215`, `Kept source: 72`, `Replaced: 2`, `Outside the declaration: 3`, `Not replaced: 2`, `Retained digest: 4f1c9a2b7d01…`, `E2E digest: 82be07c3e5a9…`, `Written path: /tmp/ise-passuff`, and `Receipt path: /tmp/ise-passuff-trial.yaml`.
- The builder opens `/tmp/ise-passuff/givn/specification.yaml` first and reads the five declared roles and the `source:` roots that the partition applied, so it knows which paths are the Specification and which it may rewrite.
- The component's directory `crates/passuff-tui/src/keys/` still holds `mod.rs` and `tests.rs`, neither named; preparation succeeds, the report says `Not replaced: 2`, and the two files remain in the trial as kept source, so the count shows what the replacement unit still contains.
- The developer runs `givn ise replace --full --into /tmp/ise-passuff`; every tracked file under `crates/**` and `scripts/**` is removed, the trial still holds `givn/specification.yaml`, the report says `Not replaced: 0`, and the trial holds only the retained roles and the declaration.
- The developer runs `givn ise replace --path crates/passuff-tui/src/keys --into /tmp/ise-passuff`; the directory expands to every tracked file beneath it, so `parser.rs` and `mod.rs` are removed while `crates/passuff-tui/src/tui.rs` stays.
- The developer runs `givn ise replace --into /tmp/ise-passuff`; the command refuses as a usage error, names `--full` and `--path`, exits 2, and writes nothing.
- The developer runs `givn ise replace --path docs/reference.md --into /tmp/ise-passuff`; the refusal names `docs/reference.md` because it falls outside every declared source root, and nothing is written.
- The developer runs `givn ise replace --path givn/specification.yaml --into /tmp/ise-passuff`; the refusal names `givn/specification.yaml` as retained, and nothing is written.
- The developer runs `givn ise replace --path givn --into /tmp/ise-passuff`; the directory expands to the declaration, the refusal names `givn/specification.yaml` as retained, and nothing is written.
- The developer runs `givn ise replace --at 8c1f2ab --full --into /tmp/ise-passuff` while `HEAD` declares an extra `limits:` entry that 8c1f2ab does not; the partition follows 8c1f2ab, the trial's declaration is 8c1f2ab's copy, and the report counts the paths that revision declares.
- The developer runs `givn ise replace --at 8c1f2ab --full --into /tmp/ise-passuff` against a revision that predates the declaration; the refusal names `givn/specification.yaml` and the revision, and nothing is written.
- The developer runs `givn ise replace --at 8c1f2ab --full --into /tmp/ise-passuff` against a revision whose declaration carries an unknown key while `HEAD`'s declaration is valid; the refusal names `givn/specification.yaml` and the loader's reason, and nothing is written.
- The developer runs preparation with `--into ./trial` inside the checkout; the refusal names the target, and the checkout keeps no trial directory.
- The developer runs preparation into `/tmp/ise-passuff` a second time; the refusal names the existing directory and leaves it untouched.
- The developer names `--receipt /tmp/ise-passuff/record.yaml`; the refusal names the receipt because it lies inside the trial, and nothing is written.
- The developer names `--receipt /tmp` while the trial is `/tmp/ise-passuff`; the refusal names the receipt because it is an ancestor of the trial, and nothing is written.
- The declaration names `contracts: ["docs/cli-contract.md"]`, but no such file exists at the selected revision; the refusal names `docs/cli-contract.md` as a retained path missing at the revision.
- The checkout has one uncommitted file; the refusal names the dirty path and writes no trial.
- The developer names `--path crates/passuff-tui/src/keys/parsr.rs` (a typo); the path is absent from the revision, so nothing is removed, the report says `Replaced: 0`, and the developer sees that the component file stayed in the trial.
- The developer edits `/tmp/ise-passuff/givn/specs/verify-change/usecase.md` after preparation; `givn ise verify --trial /tmp/ise-passuff-trial.yaml --at /tmp/ise-passuff` fails and names that path.
- The developer edits `/tmp/ise-passuff/givn/specification.yaml` after preparation; the re-check fails and names `givn/specification.yaml`, because the declaration is a retained entry of the record.
- The developer deletes `/tmp/ise-passuff/givn/specification.yaml`; the re-check fails and names `givn/specification.yaml`, and the builder could not have read the roles from that trial.
- The developer edits only the `structure:` role in the checkout's declaration after preparation; `givn ise verify` reports `Retained: clean` and `E2E: clean` and exits 0, because the frozen E2E package and the `behavior` selection did not move.
- The developer adds `tests/extra-e2e` to the checkout declaration's `e2e:` role; the re-check fails and names `tests/extra-e2e/Cargo.toml`, because the current E2E package gained a path the record does not list and the checkout's declaration derived it.
- A declaration lists `contracts: ["givn/**"]` and the revision holds `givn/specs/a.md`; the trial retains the declaration and `a.md`, and the report says `Retained: 2`, because the role's match on the declaration is the same entry, counted once.
- The developer edits `/tmp/ise-passuff/tests/e2e/steps/keys.rs`; the re-check fails and names that file, because every file of the `e2e` package is frozen.
- The developer edits `/tmp/ise-passuff/crates/passuff-cli/tests/steps/tui.rs` (kept source, outside the frozen inputs); the re-check reports no drift, because component step definitions are implementation.
- The developer edits a scenario tagged `@e2e` in the current checkout after preparation; the re-check names its feature file, because the selected scenario content changed.
- The developer rewrites the record's written path to another directory; `givn ise verify` refuses and names the record and the trial, because the record does not belong to this trial.
- A new `@e2e` scenario was observed failing against the pre-change implementation for the intended reason, so it enters the frozen set; a scenario that turns out mid-trial to assert the wrong behavior stops the trial as a `Spec gap: E2E test defect`, and the trial restarts after the E2E test is corrected.
- With `ise` off, `givn ise replace` refuses by name and reads no declaration.
- A dry run prints the ten fixed report lines, the receipt path, and a `Would write:` block with the sorted paths the trial would hold, `givn/specification.yaml` among them; `/tmp/ise-passuff` and the receipt remain absent.
- The builder rewrites `crates/passuff-tui/src/keys/parser.rs` and keeps `crates/passuff-tui/src/keys/mod.rs`; the developer runs `givn ise deliver --trial /tmp/ise-passuff-trial.yaml --at /tmp/ise-passuff --receipt /tmp/ise-passuff-delivery.yaml`; the report prints `Retained: clean`, `E2E: clean`, `Delivered: 3`, `Delivered digest: 9e8fe3ae7d01…`, and `Receipt path: /tmp/ise-passuff-delivery.yaml`, and the delivery record lists the two recreated source files and the kept source file, and lists no retained path.
- The trial record written by preparation keeps `replaced_digest: d7edf8b7…` after the delivery is recorded, because that value describes the replaced input at revision `768e2c65`; the delivery record carries `delivery.delivered_digest: 9e8fe3ae…` beside it.
- The developer edits `/tmp/ise-passuff/crates/passuff-tui/src/keys/mod.rs` after the delivery was recorded; `givn ise verify --trial /tmp/ise-passuff-delivery.yaml --at /tmp/ise-passuff --delivery` fails and names that path, while the same record without `--delivery` reports `Retained: clean`, `E2E: clean`, and `Delivery: not checked`.
- The builder adds `/tmp/ise-passuff/crates/passuff-tui/src/keys/views.rs` after the delivery was recorded; delivery mode fails, names that path, and reports that the trial gained it.
- The developer runs `givn ise deliver` twice into different receipts; the second run fails and names the record, because a delivery is recorded once.
- The verifier runs `givn ise verify --trial /tmp/ise-passuff-trial.yaml --at /tmp/ise-passuff --delivery` against a record `givn ise replace` wrote; the run fails, names the record, and says it holds no delivery identity, instead of reporting clean.
- The build merges two replaced files into one; the seal lists the paths the builder wrote, the record keeps both input lists, and an input path with no delivered file is no refusal.
- The developer edits a retained file before sealing; recording fails, names the retained path, and writes no delivery record, so a delivery identity is never recorded over drifted input.

## Minimal guarantee

A refusal leaves the checkout byte-for-byte unchanged and writes no trial directory, receipt, or delivery record.

## Success guarantee

A trial directory outside the checkout holds the declared Specification, its declaration included, and the kept source with the component's paths removed; when a receipt path was given, the trial record's retained and `e2e` evidence re-checks against it; and when the developer recorded a delivery identity, the delivered bytes re-check against it per path.

## Personas

- developer--primary-user
- agent--technical-role
- givn-maintainer--owner

## Capabilities

- clean-room-preparation
- trial-input-verification
- trial-delivery-identity

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| clean-room-preparation | prepare a clean-room trial from a selected revision (the `--path` and `--full` forms, the nine refusals, dry run, and both backends are variants of this action) | Developer prepares a clean-room trial from a recorded revision |
| trial-input-verification | re-check a prepared trial's frozen inputs (a clean run, retained drift, `e2e` drift, current-checkout drift, and a record that does not belong to the trial are variants of this action) | Verifier re-checks a prepared trial and receives the frozen-input report |
| trial-delivery-identity | record the delivery identity of a finished trial (the required receipt, the clean-input precondition, the single-recording rule, and the receipt refusals are variants of this action) | Developer records a trial's delivery identity and receives the delivery report |
| trial-delivery-identity | re-check a delivered trial's replacement bytes (a changed replaced path, a changed kept-source path, a gained path, a missing path, and a record without a delivery identity are variants of this action) | Verifier re-checks a delivered trial and receives the delivery report |

## Includes

- none

## Extends

- none

## Out of scope

Boundary: the clean-room trial directory and its record. The checkout, the declaration, and the corrective change stay outside it.

- Declaring the roles and source roots: `declare-the-specification`.
- Building the replacement: the rebuilding agent, outside Givn.
- Running the frozen End-to-end (E2E) tests: the independent verifier, outside Givn.
- Classifying a difference and opening the corrective change: `create-change` (`spec-gap`).

## Diagram

```mermaid
flowchart TB
  DEV([Developer]) --> SELECT["Select the revision and the replacement paths"]
  SELECT --> READ["Read the declaration and the tracked files"]
  READ --> PARTITION["Partition retained, kept source, replaced, outside"]
  PARTITION --> CHECK["Check the preparation conditions"]
  CHECK -->|refused| REFUSE["Name the condition; write nothing"]
  CHECK -->|accepted| WRITE["Write the plain trial directory"]
  WRITE --> IDENTIFY["Re-read and record identities"]
  IDENTIFY --> RECORD["Write the trial record and report counts"]
  RECORD --> BUILD["Builder writes the replacement"]
  BUILD --> SEAL["Re-check the frozen inputs; record the delivered identities"]
  SEAL -->|refused| REFUSE
  SEAL --> DELIVERY["Write the delivery record and report it"]
  VERIFIER([Independent verifier]) --> RECHECK["Confirm the record belongs to the trial"]
  RECHECK --> DRIFT["Re-check the retained and e2e identities; report drift per path"]
  RECHECK --> DELIVERED["Re-check the delivered identities; report drift per path"]
```
