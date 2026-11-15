# Review: add-ise-clean-room-commands

Review kind: independent
Author: add-ise-clean-room-commands implementation author (omp, deepseek-flash)
Reviewer context: independent grilling session whose decisions 0-12 the operator recorded on 2026-11-14; this authoring context applied them and owns the artifact edits

README-IMPACT: updated - Configuration
PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner
VISUAL-REVIEW: PASS
ARC42 CONFORMANCE: CLEAN

## Review context and scope

The proposal records `Review kind: independent` (`proposal.md:20-27`); the
design review records the same kind and the reviewer context above is the one
recorded context (`design-review.md:3-6`). This final review continues that
context. The `Author:` line names the change's implementation author; this
reviewer did not author the change.

The first pass recorded `REVIEW: FAIL` at `21b330e2` with F1-F12 open. This
re-audit covers the closure commits `6bfbd4d5`, `23ad9e6f`, `14973c13`,
`ab36632a`, and `ae0cbe9` against the current `ise/implementation` HEAD and
reproduces each behavioral probe below. All reviewed files are at `ae0cbe9`;
this review file is the only file this pass writes.

Read:

- `proposal.md`, `design.md`, `design-review.md`, `arc42.md`, `tasks.md`, both
  delta features, the delta use case, and the ADR fragment delta.
- `git log --name-only 21b330e2..HEAD` and the five closure commits.
- `src/ise/{mod,partition,record,verify}.rs`, `src/cli/ise.rs`, the SCM and
  boundary readers, `src/gherkin/content.rs`, and the changed fixture and step
  files.
- `docs/reference.md`, `docs/glossary.md`, `docs/domain-model.md`, the affected
  arc42 chapters, ADR-076 and the register, and `README.md`.

Ran in this pass:

- `givn status --change add-ise-clean-room-commands` — 276/277 boxes.
- `givn check tasks --change add-ise-clean-room-commands` — exit 0; 62 delta
  scenarios, 0 failed, 0 skipped; receipt at `ae0cbe9`.
- Eight targeted scenarios for F3, F4, F6, and F7 — all pass.
- `cargo test --locked --lib ise::tests::a_submodule_entry_in_the_copy_set_is_refused_as_a_repository_link`
  — passes; with the submodule branch removed it fails.
- Probes for F3, F4, F5, F6, and F7 — each fails when its production branch is
  removed; the tree was restored with `git checkout --` after every probe.
- `givn lint --change add-ise-clean-room-commands` — exit 0; 22 `SHAPE`, 118
  `SUBST`, 0 `LONG`; net delta 63 added, 1 modified, 0 removed.
- `givn check review --change add-ise-clean-room-commands` — exit 0 (output in
  Verification).

## Tasks preflight

`givn status` reports **276/277** boxes. Every Setup, scenario, review-closure,
and Closeout box is checked except `givn check review` itself:

- The Setup baseline box now records the current green permanent run
  (`980 passed, 111 skipped`, exit 0) and the stale ADR-failure history; the
  scratch feature and result file are recorded as removed
  (`tasks.md:340-355`).
- The `@e2e` REFACTOR box is checked with the fresh delta and E2E summaries
  (`tasks.md:2686-2694`).
- The Closeout boxes carry the real commands: phrase bindings
  (121/10/41/2/0), no stub bodies, no `@wip`, both transcripts, lint clean,
  the whole-change suite, and the `givn check tasks` receipt
  (`tasks.md:2940-3107`).
- The one open box is the review gate, which this PASS completes. The review
  artifact does not edit `tasks.md`; the author ticks it after the gate.

**O1 (minor, follow-up):** two present-tense sentences still say 58:
`tasks.md:143` ("one of the 58 delta scenarios") and `tasks.md:3109`
("all 58 delta scenarios pass"). The batch map is 64, the delta is 63 added +
1 modified, and the receipt is 62 regular + 2 `@e2e` = 64. The historical
setup sentence (`2 of 58 at the setup point, 2 of 64 after`) is correct. This
is a task-prose stale count, not a behavior or gate defect; it is routed as a
minor follow-up rather than a review blocker.

## Implementation evidence check

### `@e2e` integrity

`grep '@wip'` returns nothing; both `@e2e` tags remain on
`Developer prepares a clean-room trial from a recorded revision` and
`Verifier re-checks a prepared trial and receives the frozen-input report`.
No `@e2e` tag was removed.

### Step bodies and bindings

The four delta step files bind 174 phrases (121 + 10 + 41 + 2). No body is
empty, returns without an assertion, or contains `not implemented:`. The only
`todo!()` match in `tests/steps/` is inside an outline-report fixture string
(`outline_report_support.rs:581`), not a step body.

### Scenario commits and delta units

The batch map totals 64 units (`tasks.md:55-77`): 43
`clean-room-preparation`, 20 `trial-input-verification`, and 1
`adr-qualification-gate`. The `ADR-076` row carries
`COMMIT: d5fa08ec` (`tasks.md:2799-2824`). The `review-closure` batch carries
the five new scenarios and the F5/F6 evidence with `COMMIT: 23ad9e6f`. The
gate reports 64 scenario commits and a 62-scenario regular receipt; adding the
two `@e2e` scenarios gives 64.

### Components, contracts, and design

Every promised component exists and consumers use the declared contracts:

- `src/ise/{mod,partition,record,verify}.rs` and `src/cli/ise.rs`;
- `RevisionEntry`/`EntryKind` and the revision readers;
- `e2e_file_set` in `src/e2e_boundary` over the shared `Glob`;
- `e2e_scenario_digest` in `src/gherkin/content.rs`.

`design.md:105-113` now names `src/ise/partition.rs` for `Partition`, refusals
5-7, the counts, and `behavior_features`; `src/ise/mod.rs` keeps `replace`,
the preflight, the write/re-read, and the report. `design.md:221` points D1 at
`partition.rs`. `design-review.md:318-326` records the 2026-11-15 post-review
deviation for the size-driven split, and the component, contract, and preflight
order are unchanged. The implementation now matches the reviewed design.

`README.md:104-105` carries the two user-visible command rows; the recorded
`updated - Configuration` decision is true.

### Strict mode and E2E fidelity

`.fail_on_skipped()` is present at both registration points
(`tests/features_runner.rs:55,155`). `verify.e2e_command` is
`./run-tests.sh --e2e`; it selects `@e2e and not @givn.removed` with an explicit
scope. Both `@e2e` step files drive the compiled binary via
`common::run_givn*`/`CARGO_BIN_EXE_givn`; no HTTP or `fetch` shortcut exists.
No parallel e2e implementation was found.

### Closure probes (2026-11-15)

- **F3 — gained path and extra feature.** Both new scenarios pass. Removing the
  `CHECKOUT_GAINED_PATH` insert makes
  `Verification names an e2e package path the record does not list` fail with
  `left: Some(0), right: Some(1)`. Removing the extra-`e2e_scenarios` insert
  makes `Verification names an e2e scenario the record does not list` fail the
  same way. Both probes were reverted.
- **F4 — schema and digest.** Both new scenarios pass. Removing the
  `schema != SCHEMA` bail makes `Verification refuses a record with another
  schema` exit 0 and fail its Then; removing the `is_sha256_hex` bail does the
  same for `Verification refuses a record without a replaced digest`. Both
  probes were reverted.
- **F5 — submodule check.** The unit test
  `a_submodule_entry_in_the_copy_set_is_refused_as_a_repository_link` passes on
  the current tree and fails when the `EntryKind::Submodule` comparison is
  changed to `EntryKind::Symlink`; the companion regular-entry test passes.
  The E2E gitlink scenario remains a blob-read refusal at the interface, which
  the batch note records, and the unit test isolates the submodule branch. The
  title does not overstate the observable refusal.
- **F6 — add-on ordering.** The prepare scenario uses a malformed declaration
  and asserts `the declaration is not read`; moving `require_addons` after the
  declaration load makes it fail naming `givn/specification.yaml`. The verify
  scenario uses a corrupted record and asserts no record error; moving
  `read_record` before `require_addons` makes it fail with the record parse
  error. Both probes were reverted.
- **F7 — retained precedence.** Both the `Not replaced: 0` and
  `Not replaced: 1` scenarios pass. Removing the retained-precedence filter in
  `partition.rs` leaves the `0` scenario green (its retained role is outside the
  source root) and makes the `1` scenario fail with `Replaced: 2`,
  `Not replaced: 0`. The pair is consistent: the new scenario is the
  discriminator for retained precedence; the old scenario covers the no-retained-
  under-source case. Probe reverted.
- **R-147.** The outline module metrics remain 442 (`src/ise/mod.rs`, 513 with
  inline tests), 348 (`src/e2e_boundary/mod.rs`, 533), 485 (`src/fossil.rs`),
  508 (`src/git.rs`); only `src/git.rs` reports `module-code-lines`. The risk
  row is accurate; no correction.
- **Batch 6 RED.** The five trial-record entries retain the admitted
  `RED replay (not chronological; the implementation preceded the first run)`
  wording. It is skipped-RED disclosure under the steps instruction, not
  historical RED. **F11** is closed with no code action.
- **Batch 7 probes.** The recorded reversions are real; I reproduced the
  retained-walk removal and the first-drift `break` in the first pass. The tree
  is restored.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 introduction-and-goals | `docs/arc42/01-introduction-and-goals.md` | not affected | no goal change | no batch | command pair, same goal | yes |
| 02 TC-46, TC-47 | `docs/arc42/02-architecture-constraints.md:54-55` | Yes | D1-D10, Replace/Verify Flow | admission … verify | `src/ise`, SCM readers | yes |
| 03 CLI rows, verifier entity | `docs/arc42/03-context-and-scope.md:32,62-63` | Yes | command surface | admission, verify | `src/main.rs:203-301,632-659` | yes |
| 04 clean-room strategy | `docs/arc42/04-solution-strategy.md:24` | Yes | Overview | partition-and-write | revision read/write | yes |
| 05 `cli::ise`, `ise`, seams | `docs/arc42/05-building-block-view.md:62-63,90` | Yes | Architecture Impact | admission, trial-record | `src/ise`, `src/e2e_boundary`, `src/gherkin` | yes |
| 06 6.21 flows | `docs/arc42/06-runtime-view.md:982-1050` | Yes | Replace/Verify Flow | partition-and-write, verify | `src/ise/mod.rs:91-185`; `src/ise/verify.rs` | yes |
| 07 deployment | `docs/arc42/07-deployment-view.md` | No | one binary, directory | no batch | unchanged | yes |
| 08 8.29 record | `docs/arc42/08-crosscutting-concepts.md:1464-1520` | Yes | Data Model, Identity | trial-record, copy-kinds | `src/ise/record.rs`, `src/cli/ise.rs` | yes |
| 09 ADR-076 | `docs/arc42/adr/ADR-076-clean-room-trial-record.md`; register `:63` | Yes | D2/D4/D5 | ADR-076 | accepted; 28 accepted, 0 proposed | yes |
| 10 quality | `docs/arc42/10-quality-requirements.md` | No | functional | no batch | no metric added | yes |
| 11 R-142 … R-147 | `docs/arc42/11-risks-and-technical-debt.md:129-134` | Yes | Risks | replacement/copy/trial/verify | probes and R-147 metrics | yes |
| 12 glossary | `docs/glossary.md:73-76` | Yes | terms | trial-record | `Trial directory`, `Trial record`, `Trial E2E digest` | yes |

Independent re-derivation agrees with `arc42.md`'s affected set: 2, 3, 4, 5,
6, 8, 9, 11, 12; 1, 7, 10 not. All 12 chapter files have decision-specific
content; no box-drawing or pseudo-diagram was found. `arc42.md:53` is
`STATUS: DONE`; `docs/domain-model.md` is correctly unchanged because the model
excludes command parsing, file formats, and internal Rust components.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`Trial directory`, `Trial record`, and `Trial E2E digest` remain the glossary
terms; the `E2E identity` anti-term stays scoped to the change verification
receipt (`docs/glossary.md:73-76`). The specs, design, report, and record use
them consistently. **F7 closed:** `usecase.md:62` and design.md's
`Not replaced` bullet now say `--full` reports 0 only when no retained path sits
under a source root, and counts a retained file under one; the implementation
and the two scenarios agree. No structural model change was needed.

## Coverage

- **Bucket 1 — dead code:** none. Every new symbol is consumed; the partition,
  record, verify, and CLI paths are reachable.
- **Bucket 2 — missing test coverage:** none open. F3, F4, F5, and F6 are
  closed with permanent tests and discriminating probes. The new scenarios are
  in the delta and the unit test isolates the submodule branch. No other
  external-input branch is unexercised in the design's boundary table.
- **Bucket 3 — hard to test:** none claimed. The re-read race is unit-tested
  with the injectable writer; the target-symlink race is R-144, a recorded
  residual risk.

No classification outside the three buckets is used. No redundant unit test
duplicates a scenario behavior; the new unit tests cover the submodule branch
and the pre-existing writer comparison. No `@wip` remains and no
implementation-layer detail is in the spec. The archive receipt will carry the
measured coverage; this review classifies from inspection and the probes.

## Use-case and Persona conformance

`prepare-an-ise-test` keeps its goal, actors, preconditions, main flow,
extensions, rules, guarantees, and the two capabilities. The use-case rejection
criteria do not hold: one goal, a named primary actor, capabilities placed under
that goal, step-keyed extensions, concrete preconditions, domain vocabulary, no
component-shaped name, and no relationship misuse. The three confirmed Personas
have per-capability impact rows (`design.md:766-778`); the Gherkin names the
domain actors (developer, verifier), never a Persona biography. No Persona was
invented or silently promoted.

## Visual review

The contract is `Interface: terminal`, viewports/states not applicable, design
system none (`design.md:36-67`). The committed evidence is:

- `evidence/visual/developer-prepares-a-clean-room-trial-from-a-recorded-revision/transcript.txt`
  — real `givn ise replace` report, trial list, and `ise-trial/v1` record.
- `evidence/visual/verifier-re-checks-a-prepared-trial-and-receives-the-frozen-input-report/transcript.txt`
  — real `givn ise verify` clean report and exit 0.

The baseline holds: refusals name condition and subject, no Rust module/struct/
field name appears, exit codes are the contract's, the ten fixed lines and the
conditional receipt line are present, and the record uses domain field names.
The Fossil executable/symlink/`\s` behavior is covered by the regular scenario
and the first-pass probe.

Personas: `developer--primary-user` can read the placement and write targets;
`agent--technical-role` gets stable line and field names; `givn-maintainer--owner`
gets backend/revision/counts/per-path identities and the `ise-trial/v1` scope.
No Persona finding remains.

VISUAL-REVIEW: PASS

## Overlap dispositions

The current gate reports 22 `SHAPE` pairs involving this delta; the prior 29-row
table lost nine pairs whose scenarios changed shape after the closure scenarios
were added. The 22 current pairs:

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Preparation refuses when the declaration is missing | Preparation refuses a path outside every source root | variant |
| Preparation refuses when the declaration is missing | Preparation refuses a repository link in the copy set | variant |
| Preparation refuses when the declaration is missing | Preparation refuses a nested repository marker in the copy set | variant |
| Preparation refuses while the ise addon is off | Preparation refuses while architecture principles are off | variant |
| Preparation refuses a path outside every source root | Preparation refuses a repository link in the copy set | variant |
| Preparation refuses a path outside every source root | Preparation refuses a nested repository marker in the copy set | variant |
| Preparation refuses a path matched by a retained role | Preparation refuses a directory path that contains a retained path | variant |
| Preparation refuses a receipt inside the trial directory | Preparation refuses a receipt that is an ancestor of the trial directory | variant |
| Preparation refuses a repository link in the copy set | Preparation refuses a nested repository marker in the copy set | variant |
| Preparation counts the files left in a replaced path's directory | Preparation does not expand a glob-shaped path | variant |
| Verifier re-checks a prepared trial and receives the frozen-input report | Verification reports no drift after a kept source file changes | variant |
| Verification names a retained file changed in the trial | Verification names an e2e package file changed in the trial | variant |
| Verification names a retained file changed in the trial | Verification names an e2e package file changed in the checkout | boundary |
| Verification names a retained file changed in the trial | Verification names an e2e scenario changed in the checkout | boundary |
| Verification names a retained file changed in the trial | Verification names a recorded path missing from the trial | variant |
| Verification names an e2e package file changed in the trial | Verification names an e2e package file changed in the checkout | boundary |
| Verification names an e2e package file changed in the trial | Verification names an e2e scenario changed in the checkout | boundary |
| Verification names an e2e package file changed in the trial | Verification names a recorded path missing from the trial | variant |
| Verification names an e2e package file changed in the checkout | Verification names an e2e scenario changed in the checkout | boundary |
| Verification names an e2e package file changed in the checkout | Verification names a recorded path missing from the trial | variant |
| Verification names an e2e scenario changed in the checkout | Verification names a recorded path missing from the trial | variant |
| Verification refuses while the ise addon is off | Verification refuses while architecture principles are off | variant |

## Split-or-keep

No scenario exceeds the long-scenario threshold; 0 `LONG`. No row is required.

| Scenario | Decision | Reason |
|---|---|---|

## Removed and added pairs

The delta has 0 removed scenarios, 63 added scenarios, and 1 `@givn.modified`
ADR fragment. There is no removed+added pair. The modified ADR scenario
supersedes its permanent 27-accepted predecessor (`d5fa08ec`); the permanent
corpus now asserts 28 accepted and 0 proposed.

## README impact

`README.md:104-105` now carries `givn ise replace` and `givn ise verify` rows in
the Configuration command table, naming the Ise Test, the trial directory, and
the frozen retained/`e2e` re-check. `README-IMPACT: updated - Configuration` is
recorded and now true. F8 is closed.

## Deferrals

None. No `@e2e` tag was removed, no mandatory check is deferred, and no operator
decision is required.

## Findings

All first-pass findings are closed in this re-audit. Commits are the closure
commits; date 2026-11-15. O1 is a new minor task-prose observation routed to a
follow-up; it is not a behavior, coverage, or gate blocker.

| ID | First-pass finding | Closure | Evidence |
|---|---|---|---|
| F1 | `adr-qualification-gate` not routed | `6bfbd4d5`: `proposal.md:114` adds `EXTEND adr-qualification-gate` and names the fragment | `givn check review` prints `capability 'adr-qualification-gate' already owned by .../givn/specs/fragments` and reports no routing error |
| F2 | design named the partition in `src/ise/mod.rs` | `6bfbd4d5`: `design.md:105-113,221` names `src/ise/partition.rs`; `design-review.md:318-326` records the dated post-review split | static read; component/contract unchanged |
| F3 | gained e2e path and extra `e2e_scenarios` untested | `23ad9e6f`: two delta scenarios | both pass; removing each branch makes its scenario fail with exit 1 vs 0 |
| F4 | schema and `replaced_digest` checks untested | `23ad9e6f`: two delta scenarios | both pass; removing each bail makes its scenario exit 0 and fail |
| F5 | submodule half non-discriminating | `23ad9e6f`: `require_copy_set` unit test and regular-entry companion; E2E redundancy recorded | unit test passes; changing `Submodule` to `Symlink` makes it fail |
| F6 | architecture-principles no-read unproven | `23ad9e6f`: malformed declaration and corrupted record fixtures | both scenarios pass; moving each check after the read makes it fail |
| F7 | unqualified `Not replaced: 0` | `6bfbd4d5` wording, `23ad9e6f` scenario | `0` and `1` scenarios pass; removing retained precedence leaves `0` green and makes `1` fail with `Replaced: 2`, `Not replaced: 0` |
| F8 | README rows missing | `6bfbd4d5`: `README.md:104-105` | static read; `updated - Configuration` true |
| F9 | task boxes incomplete | `14973c13`, `ab36632a`: setup, `@e2e` REFACTOR, Closeout | `givn status` 276/277; only the review box remains |
| F10 | 59th unit unaccounted | `14973c13`: ADR-076 row and 64-unit batch map | `tasks.md:55-77`, `COMMIT: d5fa08ec`; gate 64 scenario commits |
| F11 | batch-6 RED admitted, no action | no commit; wording retained | read of batch 6; skipped-RED disclosure is compliant |
| F12 | stale Contradiction 6 | `14973c13`: contradiction 6 corrected | `tasks.md:191-197` names `ise` as the only consumer |
| O1 | two task sentences still say 58 | follow-up; no commit | `tasks.md:143`, `tasks.md:3109`; batch map/receipt are 64 |

## Sign-off checklist

- Fabrication audit: clean. No empty step body; batch-6 skipped RED is admitted;
  batch-7 probes are real and reverted; F5 is isolated by a unit test.
- Every checked task has a verified commit or regression test; the ADR fragment
  is the `ADR-076` row at `d5fa08ec`.
- Every promised component exists; every consumer uses its declared contract.
- Strict-mode proof present and passing.
- Scenario execution evidence: `givn check tasks` 62 regular scenarios at
  `ae0cbe9` (0 failed, 0 skipped); delta E2E 2 passed; permanent non-E2E 980
  passed, 111 skipped.
- Coverage classification: bucket 1 empty, bucket 2 closed by F3-F6, bucket 3
  none. O1 is a task-prose count, not a coverage gap.
- No dead code; no redundant unit test; no `@wip`; no implementation-layer
  detail in the spec.
- Canonical E2E policy: 2 normalized actions, 2 matrix rows, 2 `@e2e`
  scenarios; real CLI driver; no HTTP/fetch.
- `verify.e2e_command` file identified; no parallel e2e implementation; scope
  explicit.
- Implementation matches design (F2 closed) and README impact is edited (F8).
- Interaction coverage: 2 inventory entries, 2 matrix rows, 2 `@e2e` titles.
- O1 is the only residual observation and is routed as a minor follow-up; no
  finding is excused with an invented fourth bucket.

## Verification

`givn check review --change add-ise-clean-room-commands` — exit 0:

```text
Mandatory checks for 'review':
  ✓ integrity checks passed
  ✓ run declaration passed
givn lint: 3 file(s) checked — clean
net delta: 63 added, 1 modified, 0 removed
net delta json: {"added":63,"modified":1,"removed":0}
  ✓ overlap dispositions passed
capability 'adr-qualification-gate' already owned by /home/buster/.paseo/worktrees/008cxm4u/modern-mule/givn/specs/fragments
  ✓ scenario commits passed (64 scenario(s))
  ✓ change verification receipt passed (62 scenario(s) at ae0cbe917f7c5fe8620c7269641b4813c9dadd50)
  ✓ scenario commits passed (64 scenario(s))
```

The task receipt is at `ae0cbe9`; the closure commits `6bfbd4d5`, `23ad9e6f`,
`14973c13`, and `ab36632a` are all ancestors of this review. The review does
not re-run the suite: the receipt and the archived scope runs are the execution
evidence; the eight targeted scenarios and the five probes above are the
re-audit evidence.

REVIEW: PASS
