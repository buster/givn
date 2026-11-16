<!-- givn:base-sha256:7953c190a04eed4909012c16d586293f8872ff778c779ee0e508e1c26cad51cb -->
# Use case: prepare-an-ise-test

## Level

!

## Actors

- Developer (the consumer-project owner; the Givn maintainer when Givn rebuilds itself)
- Supporting: Independent verifier (runs the frozen End-to-end (E2E) test target and asks for the re-check)

## Goal

Produce a clean-room trial whose frozen inputs a verifier can confirm, and whose builder can read neither the component's Source code nor the frozen End-to-end (E2E) test package.

## Trigger

The developer starts an Ise Test.

## Preconditions

- `ise` is effective in the project, which requires `architecture_principles`.
- The declaration names the five retained roles and the replaceable source roots, and the selected revision records it.
- The checkout is clean.
- One backend is selected; no missing, mixed, or nested repository.
- A revision is available to read; the selected revision defaults to `HEAD`.
- The component's paths fall inside the declared source roots.

## Main flow

1. The developer selects the revision, or accepts `HEAD`, and names the replacement unit's paths with `--path` (a tracked file or a directory) or names the whole declared source set with `--full`; the system reads the declaration recorded at that revision and the tracked files at that revision.
2. The system partitions the tracked paths into the retained roles except the frozen End-to-end (E2E) test package, the declaration itself, the kept source outside the replaced paths, the replaced paths, the frozen package, and the files outside the declaration, and counts the tracked files in the replaced paths' directories that the developer did not name.
3. The system refuses by name, before any write, on a missing or malformed declaration, an addon that is not effective, a dirty checkout, a missing, mixed, or nested backend, a retained path or the declaration that is missing at the revision, a `--path` value outside every source root, a `--path` value or directory expansion matched by a retained role or by the declaration, a write target inside the checkout or already existing, and a repository link or nested repository among the files written into the trial.
4. The system writes the retained roles except the frozen package, the declaration, and the kept source into one plain directory outside the checkout, preserving relative paths, the executable bit, and symlink targets, with no version-control metadata and no ignored or untracked files; no path the frozen package covers enters the trial.
5. The system re-reads every written file, compares it to the file recorded at the selected revision, records a tamper-evident identity for the retained set including the declaration, the `e2e` package, and the replaced paths, reports the counts and the frozen package's count, and prints the trial record with the revision, the replaced paths, the counts, the identities, the excluded roles, the backend, and the running Givn version; a receipt path writes that record to the named file.
6. The independent verifier confirms the frozen inputs on request; the system confirms the record belongs to this trial and this backend, refuses by name when the record names a role whose files the trial does not hold and the trial holds one of them anyway, then re-checks the recorded retained identities, the declaration included, against the written trial and the `e2e` package and reports drift per path.

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
- 6: the record does not belong to this trial, names another backend, or lies inside the trial → the system refuses and names the record and the mismatch; protects the success guarantee.
- 6: the record names a role whose files the trial does not hold and the trial holds one of them → the system refuses before comparing any identity, names the offending paths, and reports no clean line; protects the success guarantee.
- 6: a recorded retained or `e2e` identity no longer matches → verification fails and names every drifting path, the declaration included; protects the success guarantee.

## Rules

- Deletion is set arithmetic: preparation removes every tracked file named with `--path`, and every tracked file beneath a directory named with `--path` at the selected revision; a glob-shaped value is never expanded; `--full` removes every tracked file under every declared `source:` root; a named path absent from the revision removes nothing and is not one of the nine refusals.
- The frozen package is never copied: the frozen set is every tracked file of the `e2e` role's resolved package at the selected revision, the resolution rule below included, except the declaration. The trial holds no path the frozen set covers, and a frozen path is neither retained, kept source, replaced, nor outside the declaration.
- The declaration is never frozen: a frozen package root that covers `givn/specification.yaml` still leaves the declaration in the trial's retained set, so every trial carries it.
- The five placement counts place every tracked file the trial copies or the replacement removes; the frozen set is counted separately by the report's frozen line, the record keeps one `e2e` identity per resolved package path and names the role in `excluded_roles`, and `Retained` equals the number of the record's `retained_files` entries.
- The declaration is an implicit retained entry, not a sixth role: every trial retains `givn/specification.yaml` under `--full` and under `--path`, no role declares it, and the copy recorded at the selected revision is the one the partition reads, so a builder reads the roles that were applied. The retained set is a union, so a retained role that also matches the declaration counts it once, and `Retained` equals the number of the record's `retained_files` entries. Naming the declaration with `--path`, or naming a directory whose tracked files include it, refuses like any retained path.
- Internal unit tests are named the same way as any other replaced path; no unit-test path is inferred from `#[cfg(test)]`, `test_*.py`, a Maven test root, or a file-name convention.
- A retained role entry that matches a directory retains every tracked file under it; an entry that matches a file retains that file, and a path matched by a role or by the declaration is never replaced.
- A tracked path that matches both a retained role and a `source:` root is retained: the retained role wins, the file stays in the trial under `--full` as well as under `--path`, and `Replaced` counts only files that are not retained. The declaration is retained on the same terms, so a `source:` root that covers it replaces nothing. A tracked path that the frozen set covers and a `source:` root also covers is frozen: it is not copied and not replaced.
- `Not replaced` counts the direct tracked files in the replaced files' parent directories that the developer did not name; it is a containment count, never a refusal, and it never changes the recorded sets. A `--full` replacement removes every tracked file under the source roots that no retained role matches and the frozen set does not cover, so the report shows `Not replaced: 0` when no retained or frozen path sits under a source root, and counts each retained file that does, the declaration included.
- `Outside the declaration` counts the tracked files that fall outside every declared role and outside every source root and are not copied; they are neither errors nor trial content. The declaration is never counted there: it is always copied.
- Only the `End-to-end (E2E) test` package is frozen; component step definitions, fakes, and harness are implementation, never frozen or diff-counted, and enter the trial only as kept source when the declaration places them inside a source root.
- The `E2E digest` covers the `@e2e` scenarios selected by tag from the `.feature` files matched by the declared `behavior` role, and every tracked file of the resolved `e2e` package; the `e2e` role's matched paths resolve to their nearest package manifest at or above them, and a matched path that resolves no manifest is its own root.
- A scenario is frozen only after it was observed failing against the pre-change implementation for the intended reason; a mid-trial test defect stops the trial as a `Spec gap: E2E test defect`, and the trial restarts after the E2E test is corrected because the `e2e` package is part of the Specification. When a behavior-corpus component scenario contradicts the frozen suite, the builder does not adjust the suite; the difference is recorded as a `Spec gap` and the trial restarts after the Specification or the suite is corrected.
- The trial is a plain directory outside the checkout, never a branch or a worktree, because a branch carries history and a Fossil checkout carries the repository database.
- Preparation reads the selected revision (`HEAD` when none is named), never the working tree, so ignored and untracked files cannot enter the trial; the declaration the partition reads and the declaration the trial holds are both the revision's copy. The checkout is never written, including the receipt, and the replaced component's Source code and its history are absent from the trial.
- Preparation and the re-check use the project's one selected backend; a missing, mixed, or nested repository is refused by name.
- Every refusal is checked before the first write, names its condition, and leaves the checkout and the target path unchanged.
- The command surface is `givn ise replace [--at <revision>] (--full | --path <path>...) --into <dir> [--receipt <path>] [--dry-run]`; `--at` defaults to `HEAD`, exactly one of `--full` or `--path` is required, and `--into` is required and must lie outside the checkout.
- `--path` is repeatable and accepts a tracked file or a directory; a directory expands to every tracked file beneath it at the selected revision, the retained-role and declaration check runs over the expansion, and a directory that contains a retained path or the declaration refuses.
- The two write targets are the trial directory and, when given, the receipt file; both lie outside the checkout, the trial directory does not exist, and the receipt does not exist, does not lie inside the trial directory, and is not an ancestor of it.
- `--dry-run` performs every read, the partition, every refusal, the counts, and the identities, writes no directory and no record, and prints the fixed report lines, the receipt path when given, the frozen line when the frozen set is non-empty, and the `Would write:` block with the sorted relative paths the trial would hold, the declaration included and the frozen set excluded.
- An omitted `--receipt` prints the trial record to stdout after the report, prints no `Receipt path` line, and writes nothing, so `givn ise verify --trial <receipt>` requires an explicit path.
- The trial record carries the revision, the named paths, the counts, the retained, `e2e`, and replaced identities, the backend, the written trial path, the excluded roles, and the running Givn version; it carries one digest per retained and `e2e` path, the per-feature scenario digest, and one rollup per identity, so a verifier names drift per path. The retained set includes `givn/specification.yaml`, so `Retained` counts the declaration and every role-retained file once, and `Retained digest` covers it.
- The record's `excluded_roles` names, sorted and in the declaration's role keys, every role whose files the trial does not hold: it holds `e2e` when the frozen set is non-empty and is empty otherwise. `Retained` never counts a frozen path, so a reader pairs the count with `excluded_roles` and `e2e_files` instead of inferring the exclusion from a smaller number.
- The record keeps the `ise-trial/v1` scope. A record written before this rule carries no `excluded_roles` and names no excluded role, which is exactly the trial it attests; a record written with it is refused by an earlier Givn through the unknown-field rule rather than read as a trial that omits nothing.
- The record's per-path identity names the entry kind (`f`, `x`, or `l`) beside the path and the executable marker, so a symlink and a regular file with the same target text are different entries.
- The report prints the fixed lines `Backend`, `Revision`, `Retained`, `Kept source`, `Replaced`, `Outside the declaration`, `Not replaced`, `Retained digest`, `E2E digest`, `Frozen package` when the frozen set is non-empty, and `Written path`, plus `Receipt path` when a receipt path was given.
- The `Frozen package` line names the excluded role and the number of frozen paths, so a developer reads that the package was deliberately not copied rather than that a role matched nothing; a resolved package that also covers the declaration counts the declaration out, because the trial holds it.
- The two printed digests are the first 12 lowercase hex characters of the rollup followed by `…`; the record carries the full digests.
- With `ise` off, preparation and the re-check refuse by name and no declaration is read; the effective project configuration decides effectiveness, and a change lock decides the change-scoped boundary gate, not these project-scoped commands. A project whose `architecture_principles` is off refuses the same way.
- The trial directory and the receipt are created with create-new semantics; an existing path is refused, never overwritten.
- The re-check refuses unless the record's written path is the given trial directory, the record's backend is the selected backend, and the record path lies outside the trial; it then refuses a trial that holds a frozen path the record excludes, asserts the recorded replaced digest, and re-checks the retained and `e2e` identities, the declaration included. The trial's declaration is re-checked against the record like every retained entry; the checkout's declaration is consulted where it derives the frozen End-to-end (E2E) test package, the `behavior` feature selection, and the frozen roots a trial-side refusal tests, so an edit that moves `e2e:` or `behavior:` is drift and an edit that moves only `structure:` or `limits:` is not.
- A frozen path is one the record's `e2e_files` lists or one under a root the current checkout's declared `e2e` role resolves; the second form catches a path the builder created under a package directory after preparation. Neither `givn/specification.yaml` nor a path the record's `retained_files` lists counts as frozen for this refusal: the declaration is the implicit retained entry, and a recorded retained path is content the record attests the trial holds. The refusal applies only to a record whose `excluded_roles` names `e2e`, because that record's trial was prepared without the package; a record that names no excluded role was prepared with the package in its retained set, so its frozen files are retained entries and the retained comparison covers them, and an open trial prepared before this rule keeps re-checking clean.
- The `e2e` identity is compared against the current checkout and never against the trial for a record that excludes the package; a package file that is missing from the checkout, differs from the recorded identity, or was gained by the checkout is named per path.
- The trial-side refusal names every offending path up to twenty and reports the count of the remainder, so a trial seeded with the whole suite produces one bounded refusal rather than one line per file.
- The trial record does not carry the verifier's runner command; the independent verifier chooses and records it outside the record.
- On unix, the executable bit and a symlink are written as recorded; on a platform without symlink support, a symlink entry refuses by name before the trial is recorded.
- The verifier names every drifting path in one run and never edits the trial; a changed component file outside the frozen inputs produces no drift.

## Examples

- A `passuff` project declares `source: ["crates/**", "scripts/**"]` and `e2e: ["crates/passuff-e2e"]`, and runs `givn ise replace --at 8c1f2ab --path crates/passuff-tui/src/keys/parser.rs --path crates/passuff-tui/tests/keys.rs --into /tmp/ise-passuff --receipt /tmp/ise-passuff-trial.yaml`; the trial holds 20 retained files after 12 frozen paths leave the 32 the declaration originally retained, and the report prints `Backend: git`, `Revision: 8c1f2ab`, `Retained: 20`, `Kept source: 72`, `Replaced: 2`, `Outside the declaration: 3`, `Not replaced: 2`, `Retained digest: 4f1c9a2b7d01…`, `E2E digest: 82be07c3e5a9…`, `Frozen package: e2e, not copied (12 files)`, `Written path: /tmp/ise-passuff`, and `Receipt path: /tmp/ise-passuff-trial.yaml`.
- The builder opens `/tmp/ise-passuff/givn/specification.yaml` first and reads the five declared roles and the `source:` roots that the partition applied, so it knows which paths are the Specification, which it may rewrite, and that the `e2e` role's package is recorded but absent.
- The component's directory `crates/passuff-tui/src/keys/` still holds `mod.rs` and `tests.rs`, neither named; preparation succeeds, the report says `Not replaced: 2`, and the two files remain in the trial as kept source, so the count shows what the replacement unit still contains.
- The developer runs `givn ise replace --full --into /tmp/ise-passuff`; every tracked file under `crates/**` and `scripts/**` is removed, the trial still holds `givn/specification.yaml`, the report says `Not replaced: 0`, and the trial holds only the non-frozen retained roles and the declaration.
- The developer runs `givn ise replace --path crates/passuff-tui/src/keys --into /tmp/ise-passuff`; the directory expands to every tracked file beneath it, so `parser.rs` and `mod.rs` are removed while `crates/passuff-tui/src/tui.rs` stays.
- The developer runs `givn ise replace --path crates/passuff-e2e/tests/keys.rs --into /tmp/ise-passuff` in a project whose `source:` roots cover `crates/**`; the path is a retained-role match, so the command refuses, names `crates/passuff-e2e/tests/keys.rs` and the `e2e` role, and writes nothing.
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
- A project declares `e2e: []`, so the frozen set is empty, the report prints no `Frozen package` line, the record's `excluded_roles` is empty, and the trial holds every declared retained path.
- A declaration declares `e2e: ["givn/**"]`, so the frozen package root covers the declaration itself; the trial still holds `givn/specification.yaml`, which stays a retained entry and never becomes frozen, and a re-check reports `Retained: clean` and `E2E: clean` and exits 0, because the declaration and the recorded retained entries are never treated as frozen paths.
- The developer edits `/tmp/ise-passuff/givn/specs/verify-change/usecase.md` after preparation; `givn ise verify --trial /tmp/ise-passuff-trial.yaml --at /tmp/ise-passuff` fails and names that path.
- The developer edits `/tmp/ise-passuff/givn/specification.yaml` after preparation; the re-check fails and names `givn/specification.yaml`, because the declaration is a retained entry of the record.
- The developer deletes `/tmp/ise-passuff/givn/specification.yaml`; the re-check fails and names `givn/specification.yaml`, and the builder could not have read the roles from that trial.
- The developer edits only the `structure:` role in the checkout's declaration after preparation; `givn ise verify` reports `Retained: clean` and `E2E: clean` and exits 0, because the frozen E2E package and the `behavior` selection did not move.
- The developer adds `tests/extra-e2e` to the checkout declaration's `e2e:` role; the re-check fails and names `tests/extra-e2e/Cargo.toml`, because the current E2E package gained a path the record does not list and the checkout's declaration derived it.
- A declaration lists `contracts: ["givn/**"]` and the revision holds `givn/specs/a.md`; the trial retains the declaration and `a.md`, and the report says `Retained: 2`, because the role's match on the declaration is the same entry, counted once.
- The verifier copies `crates/passuff-e2e/tests/keys.rs` into `/tmp/ise-passuff` after preparation and runs `givn ise verify`; the run refuses, names that path, and reports no clean line, because the declared `e2e` role freezes it.
- The verifier copies the whole `crates/passuff-e2e` package into the trial; the refusal names the first twenty paths in sorted order and reports the remainder as a count.
- The developer edits `/tmp/ise-passuff/crates/passuff-tui/tests/steps/tui.rs` (kept source, outside the frozen inputs); the re-check reports no drift, because component step definitions are implementation.
- The developer edits `crates/passuff-e2e/tests/keys.rs` in the current checkout after preparation; the re-check fails and names that file, because the frozen identity is compared against the checkout and never against the trial.
- The developer edits a scenario tagged `@e2e` in the current checkout after preparation; the re-check names its feature file, because the selected scenario content changed.
- The developer rewrites the record's written path to another directory; `givn ise verify` refuses and names the record and the trial, because the record does not belong to this trial.
- A new `@e2e` scenario was observed failing against the pre-change implementation for the intended reason, so it enters the frozen set; a scenario that turns out mid-trial to assert the wrong behavior stops the trial as a `Spec gap: E2E test defect`, and the trial restarts after the E2E test is corrected.
- With `ise` off, `givn ise replace` refuses by name and reads no declaration.
- A dry run prints the fixed report lines, the receipt path, the frozen line with its count, and a `Would write:` block with the sorted paths the trial would hold, `givn/specification.yaml` among them and the frozen package absent; `/tmp/ise-passuff` and the receipt remain absent.

## Minimal guarantee

A refusal leaves the checkout byte-for-byte unchanged and writes no trial directory or receipt.

## Success guarantee

A trial directory outside the checkout holds the declared Specification, its declaration included, and the kept source with the component's paths removed and with no path the frozen End-to-end (E2E) test package covers, and, when a receipt path was given, the trial record's retained and `e2e` evidence re-checks against it.

## Personas

- developer--primary-user
- agent--technical-role
- givn-maintainer--owner

## Capabilities

- clean-room-preparation
- trial-input-verification

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| clean-room-preparation | prepare a clean-room trial from a selected revision (the `--path` and `--full` forms, the nine refusals, dry run, and both backends are variants of this action) | Developer prepares a clean-room trial from a recorded revision |
| trial-input-verification | re-check a prepared trial's frozen inputs (a clean run, retained drift, `e2e` drift, current-checkout drift, and a record that does not belong to the trial are variants of this action) | Verifier re-checks a prepared trial and receives the frozen-input report |

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
  READ --> PARTITION["Partition retained, kept source, replaced, frozen, outside"]
  PARTITION --> CHECK["Check the preparation conditions"]
  CHECK -->|refused| REFUSE["Name the condition; write nothing"]
  CHECK -->|accepted| WRITE["Write the plain trial directory, the frozen package absent"]
  WRITE --> IDENTIFY["Re-read and record identities"]
  IDENTIFY --> RECORD["Write the trial record and report counts"]
  VERIFIER([Independent verifier]) --> RECHECK["Confirm the record belongs to the trial"]
  RECHECK -->|frozen path present| REFUSE
  RECHECK --> DRIFT["Re-check the recorded identities; report drift per path"]
```
