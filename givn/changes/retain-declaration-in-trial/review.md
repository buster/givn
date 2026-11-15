# Review: retain-declaration-in-trial

Review kind: independent
Author: retain-declaration-in-trial hardening author (omp, deepseek-flash)
Reviewer context: independent final-review session (omp, deepseek-flash), 2026-11-16; this context did not author the change

## Reviewed inputs

Read in full: `proposal.md`; `design.md`; `design-review.md`; `arc42.md`;
`tasks.md`; the three delta features and the delta
`specs/prepare-an-ise-test/usecase.md` with its base digest; the permanent
`givn/specs/prepare-an-ise-test/clean-room-preparation.feature` and the
permanent use case; `config-lock.yaml`; `docs/reference.md`; `docs/glossary.md`;
`assets/instructions/ise.md`; the affected arc42 chapters 02, 04, 05, 06, 08,
11; `src/ise/{partition,mod,record,verify}.rs`;
`src/config/declaration.rs`; `src/scm.rs`; the five step files; the tasks
receipt and the committed transcript.

Probes ran at HEAD `45a228d6` with the built `givn` and the delta/permanent
feature scopes. Every production mutation was restored with `git checkout --`;
`git status` after the probes shows only the untracked `verification/`
receipt, and the committed transcript is byte-identical to its commit.

## Implementation evidence check (Step 0)

### 0. `@e2e` tag integrity

No `@e2e` tag was removed. The only tag diff in the change is
`-  @givn.modified @e2e @wip` / `+  @givn.modified @e2e` on the delta
scenario. The permanent scenario keeps `@e2e @clean-room-preparation`.

### 1. Empty step bodies

0 empty step bodies found across 5 files
(`clean_room_preparation_steps.rs`, `clean_room_preparation_e2e_steps.rs`,
`trial_input_verification_steps.rs`, `prose_consolidation_steps.rs`,
`ise_clean_room_support.rs`). The grep for `{}`, bare `pass`, bare `return`,
and whitespace-only bodies exited 1 with no match. Every new binding body calls
a fixture helper and asserts a concrete path, report line, or exit code.

### 2. Checked scenarios and their commits

| Batch | Scenarios | Commit | Evidence in the commit |
|---|---|---|---|
| `declaration-retained-partition` | 6 | `e2fb7e24` | `src/ise/partition.rs` (seed, union, declaration candidate refusal, unit test), fixtures, bindings |
| `revision-declaration-authority` | 3 | `73549893` | `src/ise/mod.rs` (revision membership and revision parse), `src/config/declaration.rs` (`from_content`), fixtures, bindings |
| `verification-qualified-drift` | 4 | `5ce19315` | verify bindings and the checkout declaration rewrite/add fixtures |
| `served-guidance-declaration-rule` | 1 | `19346249` | prose `Then` bindings over the served CLI output |
| `prepare-trial-declaration-e2e` | 1 (`@e2e`) | `8568ee27` | count binding, trial-holds-declaration binding, e2e/verify fixtures, transcript |

`givn check review` reports `✓ scenario commits passed (15 scenario(s))`.
The two protected-requirement scenarios (`A role that matches the declaration
counts it once`, `Verification reports no drift after only the checkout's
structure role changes`) pass before their batch; both commits carry their
regression test and the first carries the count/map invariant unit test. No
commit is comment-only, tag-only, task-only, or feature-only. The task-only
`45a228d6` carries no checked scenario.

### 3. Promised components

Every component `design.md` names exists: `partition()` and
`require_replaceable_paths` in `src/ise/partition.rs`; `admit` and
`revision_declaration` in `src/ise/mod.rs`;
`SpecificationDeclaration::from_content` in `src/config/declaration.rs`; the
fixtures and bindings in the five step files. `src/ise/record.rs`,
`src/ise/verify.rs`, `src/cli/ise.rs`, and `src/scm.rs` are byte-identical to
the base (`git diff 71e45e73..HEAD --` those files is empty), which matches the
design's "unchanged, with the reason recorded". No consumer imports an
implementation; `ise` consumes `scm::Backend` and the declaration loader.

### 4. Strict-mode proof

`tests/features_runner.rs:56` calls `.fail_on_skipped()`. `tasks.md` §Setup
records the pre-binding probe: the delta scenario failed with
`Step doesn't match any function` and exit 101, and
`givn check tasks` refused the change while a selected delta scenario carried
`@wip`. Both are present in the artifact.

### 5. `@e2e` Then assertions

The one delta `@e2e` scenario asserts the ten fixed report lines plus
`Receipt path`, the written trial's contents (including
`givn/specification.yaml`), and the receipt's replacement revision, backend,
paths, counts, digests, written path, and version. All assertions read the real
CLI's stdout/stderr or the files it wrote; none asserts only against a
repository or an internal call. The permanent verify `@e2e` scenario is
unchanged and still asserts the clean report and exit code.

### 6. Browser-UI check

Not applicable. The product is a CLI; the changed surfaces are subprocess
stdout/stderr, a trial directory, and a YAML record. No HTML, CSS, or
JavaScript is rendered, and no browser driver is claimed. The opposite
classification (Web UI) does not hold.

### 7. E2E command file and parallel implementations

`givn/commands.yaml` sets `verify.e2e_command: "./run-tests.sh --e2e"`, which
runs `tests/features_runner.rs` with `GIVN_FEATURES` set to the delta or
permanent corpus and the `@e2e` tag filter. Searching the tree, including
untracked files, for a second implementation of the same capability's e2e
steps finds only `tests/steps/clean_room_preparation_e2e_steps.rs`; the
`ise replace` subprocess appears in that file and in
`trial_input_verification_steps.rs`, and nowhere else. No weaker parallel
implementation exists.

### 8. E2E scope

`prepare-an-ise-test` has one normalized prepare action and one verify action;
the prepare action's delta scenario is the single modified `@e2e` scenario, and
every new prepare case (six partition/refusal cases, three revision-authority
cases) is a regular variant. No capability gains a second `@e2e` scenario for
the same action. `prose-consolidation` gains no consumer action and no `@e2e`
scenario; its existing `@e2e` scenario is unchanged.

### 9. Scope selection and receipt

The regular and E2E scopes use distinct commands (`./run-tests.sh --non-e2e`
and `./run-tests.sh --e2e`); `run-tests.sh` derives `GIVN_RESULT_SCOPE` as
`regular`/`e2e` or accepts the caller's scope, and `givn check tasks` wrote the
change receipt with `scope: "change"`, `total: 14`, `failed: 0`, `skipped: 0`,
`runner_exit: 0`, tested content
`c68f21042c3ae93302c4753bf9fbc2c338d178585416fdfe64e04fad4cf13202`, and the
change features digest. The change lock has `ise: false`, so `givn check tasks`
did not execute a change-scoped E2E scope; the recorded closeout ran
`./run-tests.sh --e2e` (114 passed, 1060 skipped) and the delta e2e scope
(1 scenario passed, 15 skipped), and this review ran the permanent prepare
`@e2e` scenario (12/12 steps passed). Observation O2 records the receipt shape.

### 10. Design match

The implementation matches what `design.md` names: `config::declaration`'s
`from_content`, `partition()`'s seeded union, `admit`'s revision read,
`revision_declaration`'s membership refusal, and
`require_replaceable_paths`'s declaration-first refusal. No dependency, config
key, report line, record field, or content scope was added. One informational
row is missing from the design's "New bindings, by file" table
(`the trial holds "docs/glossary.md"`); the binding exists in the permanent
step file and `tasks.md` §Discrepancies records it. That is an omission in an
informational table, not a different file, framework, or command.

### 11. Interaction coverage

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `clean-room-preparation`: prepare a trial from a selected revision | present | Developer prepares a clean-room trial from a recorded revision | Real `givn ise replace` subprocess; stdout lines, trial files, receipt YAML |
| `trial-input-verification`: re-check a prepared trial's frozen inputs | present | Verifier re-checks a prepared trial and receives the frozen-input report (permanent, unchanged) | Real `givn ise verify` subprocess; stderr drift lines and exit code |
| `create-change` (`prose-consolidation`): read the served instructions | not reproduced; the change adds a served-text assertion, not a consumer action | existing `@e2e @prose-consolidation` scenario, unchanged | Real `givn instructions ise` output |

No inventory row is missing from the two capabilities the change modifies; the
`prose-consolidation` capability has no new action, so its existing e2e
coverage is not duplicated.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| The trial and record always carry the declaration; the revision's copy is the partition authority; it is counted once; a revision without it and a replacement naming it refuse | `docs/arc42/02-architecture-constraints.md:58` (`TC-49`) | 02 affected, `TC-49` | D1-D4, Partition Rules, Refusals | `declaration-retained-partition`, `revision-declaration-authority` | `partition.rs:82-90`, `partition.rs:256-262`, `mod.rs:248-267` | Yes |
| The clean-room trial strategy names the always-retained declaration and the revision's read authority | `docs/arc42/04-solution-strategy.md:24` | 04 affected | Overview, D1, D2 | batches 1-2 | same code; scenarios assert the trial copy | Yes |
| `config::declaration` has the byte-level entry point; `ise` owns the declaration entry | `docs/arc42/05-building-block-view.md:63,90` | 05 affected | Architecture Impact, Contracts and Components | batch 2 | `declaration.rs:162`, `mod.rs:248` | Yes |
| The replace flow reads the revision declaration and refuses absence; the verify flow names the declaration among retained paths and reads the checkout's declaration only for `e2e`/`behavior` | `docs/arc42/06-runtime-view.md:1011-1025,1061-1069` | 06 affected | Admission and Replace Flow, Verify Flow, D6 | batches 1-4 | `admit`, `revision_declaration`, unchanged `check_retained`/`check_checkout` | Yes |
| `retained_files` always includes the declaration; no field or scope changes | `docs/arc42/08-crosscutting-concepts.md:1420-1424` | 08 affected | D5, Record content | batches 1-3 | unchanged `record.rs`; `identities(retained_paths())` | Yes |
| An older record lacks the declaration; an uncommitted declaration refuses | `docs/arc42/11-risks-and-technical-debt.md:137-138` (`R-151`, `R-152`) | 11 affected | Risks and Trade-offs | probe P8; scenario `Preparation refuses a revision that does not record the declaration` | old-record probe P8; `revision_declaration` | Yes |
| No ADR; no command-surface or context change; glossary is standalone | `arc42.md` rows 03, 09, 12; `docs/arc42/09-architecture-decisions.md` | 03/09/12 No | ADR qualification | — | `Cargo.toml`, CLI unchanged; glossary edits only | Yes |

ARC42 CONFORMANCE: CLEAN

The five sign-off checks: (1) every affected chapter was opened and carries
decision-specific content; (2) the chapter set was re-derived independently
and matches the twelve-row table; (3) every "Yes" chapter matches `design.md`
and the as-built code; (4) chapter 09's register has no new MADR because no
record shape, scope, or identity rule changed; (5) chapter 11 carries both
accepted risks and the old-record probe confirms `R-151`.

## Ubiquitous language conformance

`Retained path` (`docs/glossary.md:148`) now names `givn/specification.yaml`
as an implicit entry, and `Trial directory` (`docs/glossary.md:76`) names the
declaration among its members. "Checkout's declaration" and "trial's
declaration" are distinct, glossed subjects in the reference and the use case.
`docs/domain-model.md` was assessed as unchanged: the change moves no
aggregate, relationship, or boundary; the model excludes file formats and
internal components. The absence of an edit is the assessed result.

Finding F3: the glossary has no term entry for "Specification declaration",
although the delta use case, the reference, the refusal text, and the served
guidance now use "the declaration" as a named subject. The `Run declaration`
entry is a different concept. This is a vocabulary gap, not a synonym drift;
add one glossary row (genus: the tracked `givn/specification.yaml` that names
the five retained roles and the source roots; contrast: `Run declaration` and
`Source root`) and record it in this change.

## Use-case and Persona conformance

The delta use case keeps the stable ID, goal, actors, trigger, main flow,
extensions, rules, guarantees, capabilities, and interactions of
`prepare-an-ise-test`; it adds no capability and no typed relationship. The
scope rejection criteria pass on the delta document:

- Goal and success guarantee each name one outcome.
- The primary actor is the Developer, named in the trigger.
- Both capabilities pass placement; neither is a component or channel.
- No mutation is asserted under an observing guarantee.
- Every extension names its main-flow step (3, 5, or 6) and adds no success outcome.
- Preconditions are established by the flow, the trigger is an act, and no guarantee is open.
- No mechanism vocabulary appears in the goal, trigger, or guarantees.
- No leaf names three actor goals; Includes/Extends remain none.

PERSONAS: developer--primary-user

The confirmed Personas are `developer--primary-user`,
`agent--technical-role`, and `givn-maintainer--owner`, all from
`prepare-an-ise-test`'s `## Personas`; the design's impact table gives each a
concrete impact (prepares the trial, reads the report/record/refusals, owns the
declaration and backend parity). No Persona is invented or silently promoted.

## Visual review

The design declares `Interface: terminal`, viewports and states not
applicable, and one committed transcript for the modified `@e2e` scenario:
`evidence/visual/developer-prepares-a-clean-room-trial-from-a-recorded-revision/transcript.txt`.

The transcript shows the real `givn ise replace` run with `Retained: 4`, the
ten fixed lines, the `Receipt path` line, and exit 0; the trial holds
`givn/specification.yaml`, and the receipt records the declaration. The
`developer--primary-user` can reach the goal (the trial it receives contains
the index); the `agent--technical-role` reads the count, the path, and the
record key; the `givn-maintainer--owner` sees the declaration committed and
frozen. No finding names a contract rule failure. The evidence set is
complete: the delta has exactly one `@e2e` scenario, and the permanent
`trial-input-verification` e2e scenario has no delta scenario and no text
change.

VISUAL-REVIEW: PASS

## Coverage

No coverage report exists for this change. The files in `coverage/` are dated
2026-11-15 22:46, before the implementation commits (2026-11-16 00:0x), and
`non-e2e-cobertura.xml`, `e2e-cobertura.xml`, and `cobertura-coverage.xml` do
not contain `revision_declaration` or `from_content`; the reports are stale
pre-change evidence. Per the review instruction, this review classifies from
inspection and the archive receipt will carry the measured result.

| Changed behavior | Classification | Evidence / gap |
|---|---|---|
| Declaration seeded into the retained union; role match counted once; `Retained == retained_files.len()` | covered | Scenarios 1, 2, 8, 9; unit test; probes P1, P7 |
| Declaration candidate refusal (exact path and directory expansion) | covered | Scenarios 3, 4; probe P2 |
| Revision membership refusal and revision parse authority | covered | Scenarios 5, 6, 7; probe P3 |
| `from_content` strict schema | covered | Loader unit tests plus scenarios 7 and the permanent malformed-declaration scenario |
| Trial copy and record entry of the declaration | covered | Scenarios 1, 2; e2e scenario; probes P1, P8 |
| Verify re-checks the recorded declaration; checkout declaration only derives `e2e`/`behavior` | covered | Scenarios 10-13; probes P4, P5 |
| Non-UTF-8 declaration bytes at the revision (`str::from_utf8` branch) | bucket 2: missing test coverage | Finding F2: add a fixture committing invalid UTF-8 bytes at the selected revision and assert the refusal names the declaration; the current fixtures only cover valid-UTF-8 malformed YAML |
| `blobs.pop()` empty-result branch in `revision_declaration` | bucket 1: dead code | Finding F1: `Backend::revision_blobs` (`src/scm.rs:209-231`) already refuses a requested path the backend cannot read, so the branch is unreachable |
| Redundant unit tests | none | The added unit test asserts the union count/map invariant; the scenarios assert the CLI count for a different fixture |

No branch instrumentation was removed.

## Independent probe log

Every probe ran against the built binary and the delta/permanent feature
scopes; every mutation was restored, and the post-probe `git status` shows only
the untracked receipt.

| Probe | Mutation | Observed result |
|---|---|---|
| P1 | Removed `retained_paths.insert(DECLARATION_PATH...)` | `The trial carries the declaration...` failed: `the trial at /tmp/.../trial does not hold givn/specification.yaml`; `The trial record lists...` failed: `the record does not list givn/specification.yaml in retained_files`; `A full replacement keeps...` printed `Retained: 0`, `Replaced: 4` and failed the `Replaced: 3` Then |
| P2 | Removed the declaration candidate loop from `require_replaceable_paths` | Exact path: stderr became `'givn/specification.yaml' is outside every declared source root at revision '<r>'`, failing the retained-message Then; directory `givn`: exit 0 instead of the expected refusal |
| P3 | Replaced `revision_declaration(...)` with `SpecificationDeclaration::load(project_root)` | Scenario 5 failed (working-tree declaration lacks the `structure` entry); scenario 6's stderr became `... cannot be read from revision '<r>'` and lacked `does not record`; scenario 7 failed (working tree is valid, so the run exited 0) |
| P4 | Replaced the checkout `e2e` derivation with the record's `e2e_files` keys | Structure-only scenario stayed green; added-root scenario failed with `left: Some(0)` / `right: Some(1)` because the gained `tests/extra-e2e/Cargo.toml` was never resolved |
| P5 | Skipped `DECLARATION_PATH` in `check_retained` | Changed-declaration and missing-declaration scenarios both failed with `left: Some(0)` / `right: Some(1)` |
| P6 | Deleted "itself" from the served asset sentence | The prose scenario failed, and the captured served output showed the altered sentence |
| P7 | Replaced the retained `BTreeSet` union with a concatenated `Vec` | `A role that matches...` printed `Retained: 3` and failed the `Retained: 2` Then; the unit test failed `left: 3 / right: 2` |
| P8 | Prepared a real trial, removed the declaration from `retained_files`, removed the trial's declaration, re-ran `givn ise verify` | `Retained: clean`, `E2E: clean`, exit 0: the older `ise-trial/v1` record verifies against the paths it lists, and no entry is synthesized |
| P9 | Ran the permanent prepare `@e2e` scenario after the permanent spec update | 12/12 steps passed |
| P10 | Ran the delta non-e2e scope | 15 scenarios / 79 steps passed |

## Fabrication audit

| Scenario | Check broken and observed failing? |
|---|---|
| The trial carries the declaration when no role names it | Yes, P1 |
| The trial record lists the declaration in its retained set | Yes, P1 |
| Preparation refuses a replacement path that names the declaration | Yes, P2 |
| Preparation refuses a directory that contains the declaration | Yes, P2 |
| Preparation partitions with the declaration recorded at the selected revision | Yes, P3 |
| Preparation refuses a revision that does not record the declaration | Yes, P3 |
| Preparation refuses a malformed declaration at the selected revision | Yes, P3 (exit 0 instead of refusal) |
| A full replacement keeps the declaration under a source root | Yes, P1 (`Replaced: 4`) |
| A role that matches the declaration counts it once | Yes, P7 (`Retained: 3`) |
| Verification names a declaration changed in the trial | Yes, P5 |
| Verification names a declaration missing from the trial | Yes, P5 |
| Verification reports no drift after only the checkout's structure role changes | Protected requirement, passes before the change; it stayed green under P4 because the mutation only removed the `e2e` derivation. Its purpose is the no-drift half of D6 and it asserts a concrete clean report and exit 0; no mutation of the current code makes it fail without also changing the intended rule |
| Verification names the e2e package a checkout declaration's added root brings | Yes, P4 |
| Served ise guidance states that the trial always carries the declaration | Yes, P6 |
| Developer prepares a clean-room trial from a recorded revision (`@e2e`) | Its two changed assertions are the count and the trial's declaration, both broken in P1; the permanent copy passed 12/12 steps in P9 and the committed transcript records `Retained: 4` |

No scenario stayed green while its asserted behavior was broken.

## Findings

### B1 (blocker) — the `## Step Definitions` section fails the promised-file integrity check

`givn check review` reports 42
`[integrity] Step Definitions: file ... does not exist` errors. The cause is
mechanical: `check_promised_files_exist` (`src/integrity_hook.rs:559-695`)
collects every markdown table row in the `## Step Definitions` section
(573-656), skips only the first two rows, and treats column 2 of every remaining
row as a step-definition file path. The section contains two additional tables:
"New bindings, by file" (starting `design.md:587`, columns `File | Phrase`) and
"Fixture changes in ..." (starting `design.md:625`, columns `Fixture | Change`).
Their column-2 values are phrases and prose, so the check reports
`file 'Phrase' does not exist`, `file '---' does not exist`, and every phrase
and fixture-change row as a missing file. `givn archive` runs the same check
and will refuse.

Fix: move both informational tables out of `## Step Definitions` into a new
`##` section (for example `## Bindings and Fixtures`) between
`## Step Definitions` and `## Strict Mode`, leaving the canonical
`| Capability | Step definition file | Permanent merge target |` table as the
only table in the section. Alternatively, restructure each informational row so
column 2 is an existing step-definition file path. Re-run `givn check review`
after the move.

### B2 (blocker) — the independent review context is not established

`proposal.md` records `Review kind: independent`. `design-review.md` records
`Review kind: independent` and a `Reviewer context:` value that says
"this authoring context applied them and owns the artifact edits", so the
recorded reviewer is the authoring context, not a reviewer. The review gate
requires `review.md`'s `Reviewer context:` to equal `design-review.md`'s exactly
(`src/review_context.rs:185-198`); a second independent context also violates
the one-context rule in the review instruction. This review records its own
context above; copying the design-review's string would falsely claim that this
session is the authoring context, which it is not.

Fix: re-perform the design review in one genuine independent context and record
that context in both `design-review.md` and `review.md` (or record a
self-review consistently, which changes the proposal's requested kind and
needs the operator's decision). The final review then runs in that same context.

### F1 (blocker, bucket 1) — dead branch in `revision_declaration`

`src/ise/mod.rs:261-263`:

```rust
let Some((_, bytes)) = blobs.pop() else {
    bail!("the revision '{revision}' returned no content for '{DECLARATION_PATH}'");
};
```

`Backend::revision_blobs` (`src/scm.rs:209-231`) already refuses when any
requested path has no blob, so for the one requested path it either returns one
element or an error. The `else` arm is unreachable, and its message is weaker
than the seam's `'givn/specification.yaml' cannot be read from revision ...`.
Delete the dead arm (for example `let (_, bytes) = blobs.pop().expect("the scm
seam returns every requested path")` or a single-path seam method). The
`git diff` shows this branch is introduced by commit `73549893`.

### F2 (bucket 2) — non-UTF-8 declaration bytes at the revision are untested

`src/ise/mod.rs:264-265` converts the revision's bytes with
`std::str::from_utf8` and reports `{DECLARATION_PATH}: {error}`. No fixture
commits non-UTF-8 bytes, so the new error path is unexercised. Add a fixture
that commits invalid UTF-8 bytes as `givn/specification.yaml` at the selected
revision (while `HEAD` is valid) and a scenario that asserts the refusal names
the declaration and exits 1. The working-tree loader's equivalent read error is
also uncovered, but that path predates this change; the new revision conversion
is the gap this change introduces.

### F3 (ubiquitous language) — the glossary lacks "Specification declaration"

The change's use case, reference, refusal text, and served guidance name "the
declaration" as a subject; `docs/glossary.md` holds `Run declaration` and
`Specification` but no `Specification declaration` term. Add one row with the
genus (the tracked `givn/specification.yaml` that names the five retained roles
and the source roots), the contrast (`Run declaration`, `Source root`), and the
concrete sentence the docs already use. This is the change's vocabulary edit,
not a separate backlog item.

### O1 (observation) — coverage evidence is absent for this change

The `coverage/` reports predate the implementation and do not contain the new
functions. The review classifies from inspection; the archive receipt must
carry the measured branch-coverage result.

### O2 (observation) — the tasks receipt has no change-scoped E2E entry

The change lock sets `ise: false`, so `givn check tasks` recorded one
`change-proof/v1` scope (`./run-tests.sh --non-e2e`, 14 scenarios, zero failed
or skipped) and did not execute the change-scoped E2E scope. The closeout
records the full `./run-tests.sh --e2e` run (114 passed) and the delta E2E run
(1 passed, 15 skipped); this review re-ran the permanent prepare `@e2e`
scenario (12/12 steps). No behavior is unexecuted, but the receipt itself does
not carry the per-scope E2E entry the archive checklist names. The archive's
own scopes will provide it.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Preparation refuses a replacement path that names the declaration | Preparation refuses a directory that contains the declaration | variant |
| Verification names a declaration changed in the trial | Verification names a declaration missing from the trial | variant |
| A named path absent from the revision removes nothing | A role that matches the declaration counts it once | boundary |
| Preparation refuses a malformed declaration | Preparation refuses a malformed declaration at the selected revision | variant |
| Preparation refuses a retained path missing at the revision | Preparation refuses a revision that does not record the declaration | variant |
| Verification names an e2e package file changed in the checkout | Verification names a declaration changed in the trial | boundary |
| Verification names an e2e package file changed in the checkout | Verification names a declaration missing from the trial | boundary |
| Verification names an e2e package file changed in the trial | Verification names a declaration changed in the trial | boundary |
| Verification names an e2e package file changed in the trial | Verification names a declaration missing from the trial | boundary |
| Verification names an e2e package path the record does not list | Verification names the e2e package a checkout declaration's added root brings | variant |
| Verification names an e2e scenario changed in the checkout | Verification names a declaration changed in the trial | boundary |
| Verification names an e2e scenario changed in the checkout | Verification names a declaration missing from the trial | boundary |
| Verification names a recorded path missing from the trial | Verification names a declaration changed in the trial | variant |
| Verification names a recorded path missing from the trial | Verification names a declaration missing from the trial | variant |
| Verification names a retained file changed in the trial | Verification names a declaration changed in the trial | variant |
| Verification names a retained file changed in the trial | Verification names a declaration missing from the trial | variant |
| Verification reports no drift after a kept source file changes | Verification reports no drift after only the checkout's structure role changes | variant |
| Verifier re-checks a prepared trial and receives the frozen-input report | Verification reports no drift after only the checkout's structure role changes | variant |

The `[SUBST]` advisories in `givn lint` are subset observations, not shape
matches; none names a removed scenario, and no pair is a duplicate that should
be merged. The `variant` pairs isolate a distinct path, fixture, or authority;
the `boundary` pairs assert different drift sets or different rules whose
fingerprints coincide.

## Split-or-keep

No delta scenario exceeds the 14-step long-scenario threshold (the largest,
the modified `@e2e` prepare scenario, has 12 steps), so no split-or-keep
decision is required.

## README impact

README-IMPACT: none

The README's `givn ise` rows (`README.md:104-105`) document the invocation, not
the retained contents or the report count; no documented claim becomes false.
The durable detail lives in `docs/reference.md`, which this change updates.

## Deferrals

None. No mandatory check was deferred.

## Sign-off

- [x] Every checked task has a commit containing its implementation or regression test.
- [x] Every promised component exists; no consumer imports an implementation.
- [x] Strict-mode proof is present and was reproduced.
- [ ] Dead code deleted — F1 remains.
- [ ] Missing tests added — F2 remains.
- [ ] Every term recorded in the glossary — F3 remains.
- [x] Redundant unit tests removed or justified; none found.
- [x] No `@wip` tags remain; no implementation-layer detail in the delta spec.
- [x] The canonical E2E policy was applied: one `@e2e` scenario per inventory action; variants stay regular.
- [x] Every E2E scenario uses the real CLI; no HTTP/fetch substitute.
- [x] The exact `verify.e2e_command` file was identified and the tree searched for a parallel implementation.
- [x] The scope selection is recorded and the change receipt reports zero failed or skipped (O2).
- [x] Design match: no command, file-layout, framework, or driver deviation.
- [x] Interaction coverage maps every inventory entry touched by the change.
- [ ] The independent review context is recorded in both artifacts — B2 remains.
- [x] The `## Step Definitions` integrity check passes — B1 closed (see Finding closure).

REVIEW: FAIL

## Finding closure

Author-side closure for B1, F1, F2, and F3. B2 remains with the reviewer.

### B1: `## Step Definitions` integrity check

- Fix: moved the "New bindings, by file" and "Fixture changes" tables into a
  new `## Bindings and Fixtures` section immediately after `## Step
  Definitions`; the canonical file table is the only table in the checked
  section.
- Check: `givn check review --change retain-declaration-in-trial` reported 42
  `[integrity] Step Definitions: file ... does not exist` errors before the
  move and 0 after.
- Commit: `5a32d2ba`.

### F1: dead `blobs.pop()` arm

- Fix: deleted the unreachable `else { bail!("... returned no content ...") }`
  arm; the single requested blob is read with an invariant `expect`.
- Evidence: with the declaration's loose blob removed from a fixture
  repository, `givn ise replace` refuses with `read
  'givn/specification.yaml' at revision '<r>': 'givn/specification.yaml'
  cannot be read from revision '<r>'; the backend returned no blob for it`
  and writes no trial. The seam message is the one a missing blob produces;
  the deleted message never appeared. The permanent `Preparation refuses a
  retained path missing at the revision` scenario keeps the same seam refusal
  under test.
- Commit: `0707f549`.

### F2: non-UTF-8 declaration bytes

- Fix: fixture `non_utf8_first_revision_project` commits
  `NON_UTF8_DECLARATION_BYTES` in the first revision and the valid
  declaration at `HEAD`; scenario `Preparation refuses a declaration that is
  not UTF-8` asserts exit 1 and stderr carrying `givn/specification.yaml`
  and `invalid utf-8`. The delta use case, design, and reference name the
  decoding refusal.
- Test: `GIVN_FEATURES=givn/changes/retain-declaration-in-trial/specs cargo
  test --locked --test features_runner -- --exact
  "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a
  declaration that is not UTF-8"` passed 4/4 steps. Discrimination: replacing
  `std::str::from_utf8` with `String::from_utf8_lossy` made the Then fail
  with stderr `ise replace: givn/specification.yaml: missing role
  'structure'`; the mutation was restored.
- Commit: `4df99423`.

### F3: glossary row

- Fix: added `Specification declaration` to `docs/glossary.md` after
  `Specification`, with the genus, the differentia, the `Run declaration` and
  `Source root` contrasts, and anti-terms.
- Test: documentation only.
- Commit: `1858b335`.

### Verification at closure

- `cargo fmt --all -- --check`: exit 0.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: exit 0.
- `./run-tests.sh --non-e2e`: 1060 tests run, 1060 passed, 114 skipped.
- `./run-tests.sh --e2e`: 114 tests run, 114 passed, 1060 skipped.
- Delta scope: 16 scenarios, 83 steps, all passed.
- `givn check review`: 0 integrity errors; the remaining failures are B2's
  reviewer context and the reviewer-owned verdict.
