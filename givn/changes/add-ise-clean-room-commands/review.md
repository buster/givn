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

The proposal records `Review kind: independent` (`proposal.md:20-27`), and the
design review records the same kind (`design-review.md:3-6`). The design review's
recorded reviewer context is the operator's 2026-11-14 grilling session whose
decisions 0-12 the design-review authoring context applied; the gate requires
the final review to record that same one context, and this review continues it
as the change's final independent review. The `Author:` line names the change's
implementation author, not this reviewer. I wrote no production code, step
definition, spec, design, task, ADR, glossary, coverage, or README file. The
review artifact is this file only.

Read:

- `proposal.md`, `design.md`, `design-review.md`, `arc42.md`, `tasks.md`, both
  delta features, the delta use case (`usecase.md`), and the ADR fragment delta
  (`specs/fragments/adr-qualification-gate.feature`).
- `git log --name-only ea543c79..HEAD`; every changed production file
  (`src/ise/{mod,partition,record,verify}.rs`, `src/cli/ise.rs`, `src/scm.rs`,
  `src/git.rs`, `src/fossil.rs`, `src/e2e_boundary/{mod,paths}.rs`,
  `src/gherkin/content.rs`, `src/config/declaration.rs`, `src/main.rs`,
  `src/cli/mod.rs`, `src/lib.rs`).
- The four delta step-definition files, `tests/steps/ise_clean_room_support.rs`,
  the permanent ADR-qualification step file, and the two committed transcripts.
- `docs/reference.md`, `docs/glossary.md`, `docs/domain-model.md`, the affected
  arc42 chapters, `docs/arc42/adr/ADR-076-clean-room-trial-record.md`,
  `docs/arc42/adr/README.md`, and `README.md`.

Ran (read-only against the repository, plus `/tmp` fixture probes):

- `givn status --change add-ise-clean-room-commands` — 239/249 boxes.
- `givn check tasks --change add-ise-clean-room-commands` — exit 0; 57 delta
  scenarios (57 passed), 0 failed, 0 skipped; receipt at `ff619517`.
- `./run-tests.sh --non-e2e` on the permanent corpus — exit 0; 980 passed,
  111 skipped.
- `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs
  ./run-tests.sh --e2e` — exit 0; 2 passed, 57 skipped.
- The delta scope in one process — 59 scenarios (59 passed), 286 steps
  (286 passed).
- `givn lint --change add-ise-clean-room-commands` — exit 0; 29 `SHAPE` and 100
  `SUBST` advisories; 0 `LONG`; net delta 58 added, 1 modified, 0 removed.
- `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings`
  through `run-tests.sh` — exit 0.
- `target/debug/givn instructions` — exit 0 (the declared run command).
- `givn outline report --json` — module metrics for the size-debt check.

The probe fixtures were written under `/tmp`; no repository file was left
modified. The one probe that changed a source file was reverted with
`git checkout -- src/ise/mod.rs` or `src/ise/verify.rs`, and `git status` was
clean apart from the untracked `verification/` receipt directory.

## Tasks preflight

`givn status --change add-ise-clean-room-commands` reports **239/249** boxes.
The 10 unchecked boxes are:

- the Setup box `Restore a passing baseline: delete the scratch feature and
  result file, run ./run-tests.sh --non-e2e, and confirm exit 0`
  (`tasks.md:332-358`); its recorded evidence is a 2026-11-15 failure at exit
  100, marked `BLOCKED` because the permanent ADR scenario still expected 0
  proposed ADRs. The cause was removed by `d5fa08ec` (ADR-076 accepted, the
  permanent scenario moved to 28 accepted); the permanent suite now exits 0
  with 980 passed and 111 skipped. The task is stale, not blocked.
- the `@e2e` scenario's REFACTOR box for `Verifier re-checks a prepared trial
  and receives the frozen-input report` (`tasks.md:2651`); its evidence block
  is filled but the box is unchecked.
- the seven Closeout boxes (`tasks.md:2745-2816`): phrase binding, empty step
  bodies, `@wip`/`@e2e` integrity, transcripts, `givn lint`, whole-change
  verification, `givn check tasks`, and `givn check review`.

The review did not stop at the unchecked boxes: the permanent non-E2E suite,
the delta non-E2E receipt, the delta E2E scope, lint, and the task receipt are
all green from the current tree. The unchecked boxes remain an authoring defect
in `tasks.md`; the setup evidence contradicts the current tree, and the
closeout record is incomplete.

## Implementation evidence check

### `@e2e` tag integrity

`grep -rn '@wip' givn/changes/add-ise-clean-room-commands/specs/` returns
nothing; `grep -rn '@e2e' ...` returns exactly the two delta scenarios:

- `clean-room-preparation.feature:5` — Developer prepares a clean-room trial
  from a recorded revision.
- `trial-input-verification.feature:5` — Verifier re-checks a prepared trial
  and receives the frozen-input report.

No `@e2e` tag was removed. The two tags are on the normalized interaction
actions in `usecase.md:129-134` and design.md's Interaction Coverage Matrix
(`design.md:759-764`).

### Empty and stub step bodies

`grep -nE 'fn [a-zA-Z0-9_]+\([^)]*\) *\{ *\}'` over the four delta step files
and `ise_clean_room_support.rs` returns only fixture string literals such as
`"pub fn parse() {}\n"`, never a step body. `grep -rn 'not implemented:'
tests/steps/` returns nothing. `unimplemented!`/`todo!()` appears once in
`tests/steps/outline_report_support.rs:581`, inside a fixture source string for
the outline report, not in a step definition. The four delta files bind 160
phrases (117 + 10 + 31 + 2); the 117 + 10 prepare bindings serve 42 scenarios,
and the 31 + 2 verify bindings serve 16.

No empty, no-op, or stubbed step body exists in this change.

### Scenario commits and the 59th delta unit

The batch map (`tasks.md:55-70`) accounts for 58 scenarios: 42
`clean-room-preparation` and 16 `trial-input-verification`, and the review gate
reports 58 scenario commits. The 59th delta unit is the `@givn.modified` ADR
fragment scenario in `specs/fragments/adr-qualification-gate.feature:5-16`. It
has no row in the batch map, no task, and no `COMMIT:` line; its work is
carried by `d5fa08ec` (ADR-076 accepted, the permanent scenario moved from 27
to 28 accepted). `givn check review` counts it as a delta capability and fails
on the undeclared capability (see F1). The unit is unaccounted for in
`tasks.md` and in the proposal's routing table.

### Components, contracts, and consumers

Every component design.md promises exists:

- `src/ise/mod.rs` (replace, preflight, report), `src/ise/partition.rs`,
  `src/ise/record.rs`, `src/ise/verify.rs`, `src/cli/ise.rs`.
- `RevisionEntry`/`EntryKind` and the `revision_files`/`revision_blobs`
  contract in `src/scm.rs`, `src/git.rs`, `src/fossil.rs`.
- `e2e_file_set` in `src/e2e_boundary/mod.rs`, over the shared `Glob` in
  `src/e2e_boundary/paths.rs`.
- `e2e_scenario_digest` in `src/gherkin/content.rs`.

Consumers use the declared contracts. `cli::ise` calls the public
`ise::replace`/`ise::verify` and the public `DriftLine`/`VerifyReport`; `ise`
uses `scm::Backend`, `e2e_boundary::e2e_file_set`,
`gherkin::content::e2e_scenario_digest`, and the config declaration. No
consumer imports an implementation module to reach around a contract. The
composition root is `src/main.rs`; the implementation tests are inside
`src/ise`.

### Strict-mode proof

`.fail_on_skipped()` is present at both registration points
(`tests/features_runner.rs:55,155`). The setup task records the scratch unbound
step probe: the plain path exits 1 with "Step doesn't match any function" and
the result file records `{"failed":1,...}`; the `--exact` path exits 101
without a result file (`tasks.md:303-331`). The proof is present and passing.

### Real E2E interface and the exact command

`verify.e2e_command` is `./run-tests.sh --e2e` (`givn/commands.yaml:2-4`).
`run-tests.sh` selects `@e2e and not @givn.removed` and passes
`GIVN_FEATURES` to `tests/features_runner.rs`; the tag scope is explicit. The
`@e2e` step files are
`tests/steps/clean_room_preparation_e2e_steps.rs` and
`tests/steps/trial_input_verification_e2e_steps.rs`. Every step drives the
compiled binary via `common::run_givn*`/`CARGO_BIN_EXE_givn`; there is no
`reqwest`, `fetch`, or XMLHttpRequest in either file. The primary assertions
are stdout and exit code, with the trial and record contents as secondary
evidence.

A tree search for a parallel end-to-end implementation for the same phrases
found only these files and the shared regular step file; no second
implementation exists. The E2E scope run is recorded: 2 passed, 57 skipped, 0
failed.

### Implementation against design.md

The command surface, report lines, record fields, refusal order, and verify
order match design.md. Two deviations need the author:

1. `design.md:105-110` names `src/ise/mod.rs` as carrying the partition, and
   `design.md:219`/`:520` repeat that placement. The as-built partition lives
   in `src/ise/partition.rs` (300 physical lines; `tasks.md:1375-1378`
   records the refactor move). The component (`ise`) and its contract are
   unchanged, but design.md's named file layout is stale and the change never
   updated design.md or re-ran design-review for the split.
2. `README.md:90-104` lists `givn skills sync` and `givn commit` in its
   Configuration command table. The new user-facing commands
   `givn ise replace` and `givn ise verify` are absent; the README records no
   ISE capability. The README impact decision is therefore
   `updated - Configuration`, but the edit has not been made (F8).

The `@e2e` command scope, the driver, and the four step-file layout are exactly
what design.md named.

### Interaction coverage

`usecase.md:129-134` has two inventory entries; design.md's matrix has the same
two rows; both `@e2e` titles exist in the delta features and drive the real
binary. No inventory entry is unmapped and no extra matrix row exists.

### Fabrication audit

- **Batch 6 (trial-record) RED.** All five scenarios record
  `RED replay (not chronological; the implementation preceded the first run)`
  (`tasks.md:1936-1946`, `:1975-1985`, `:2011-2031`, `:2051-2061`,
  `:2103-2113`). That is an admitted skipped RED, permitted by the steps
  instruction `TESTS.5`; it is not a chronological RED-before-fix record. The
  record is transparent, so there is no fabricated chronology. No action is
  needed on the evidence; do not present these replays as historical RED.
- **Batch 7 (verify) probes.** The probes are reversions of live checks: the
  retained-walk skip, the `deny_unknown_fields` removal, the missing
  `[serde(default)]`, the add-on-before-record ordering, the first-drift
  `break`, the missing-path `continue`, and the inverted digest comparison
  (`tasks.md:2190-2200`, `:2320-2330`, `:2410-2420`, `:2520-2530`,
  `:2600-2610`). I reproduced two: disabling `check_retained` makes both
  "Verification names an e2e package file changed in the trial" and
  "Verification names a retained file changed in the trial" fail; inserting
  `break` after the first retained drift makes "Verification names every
  drifting path in one run" print one drift and fail. The tree was restored.
  The probes are real discrimination tests, not historical RED.
- **The gitlink scenario is not discriminating.** With
  `require_copy_set`'s `EntryKind::Submodule` branch removed, "Preparation
  refuses a repository link in the copy set" still passes: the copy-set blob
  read fails first and names `vendor/lib`
  (`tests/steps/clean_room_preparation_steps.rs:825-840` only requires the
  path). The author already records this limit (`tasks.md:1340-1344`,
  `:1632-1656`). The nested-marker sibling does fail without the check, but it
  covers the marker half, not the submodule half. This is bucket 2, not an
  acceptable limit: strengthen the Then to require "repository link" or add a
  scenario that isolates the submodule half.
- **The architecture-principles scenarios do not prove the no-read
  precedence.** `architecture_principles_off_project()` starts from
  `ise_project()`, which commits a valid declaration
  (`tests/steps/ise_clean_room_support.rs:684-695`); the verify counterpart
  prepares a valid record and only then disables the add-on
  (`trial_input_verification_steps.rs:85-90,131-134`). Only the `ise`-off
  fixtures use a malformed declaration (prepare) or a corrupted record
  (verify) to prove the check runs first. The architecture-principles half of
  design D10 and `usecase.md:75-76` is behaviorally correct (I probed it:
  architecture-principles off with a malformed declaration refuses
  `architecture_principles`, not the declaration), but the permanent scenarios
  cannot distinguish the ordering. This is bucket 2.
- **The gained e2e path and extra feature branches are untested.**
  `CHECKOUT_GAINED_PATH` (`src/ise/verify.rs:28,222-225`) and the extra
  `e2e_scenarios` path branch (`:264-270`) are in the implementation and in
  the reason table (`design.md:649-658`, "extra path"), but the 16-scenario
  verify feature has no scenario that adds an e2e package file or a new
  `@e2e` feature. I probed both: adding `tests/e2e/steps/new.rs` prints
  `E2E drift: ... (the checkout's e2e package gained this path)` and adding a
  new `@e2e` feature prints the selected-scenarios reason. Behavior is correct;
  permanent coverage is missing. Bucket 2.
- **The `replaced_digest` and schema checks are untested.** `read_record`
  rejects a schema other than 1 and a `replaced_digest` that is not 64 hex
  (`src/ise/verify.rs:91-112`). No delta scenario exercises either; the
  "record it cannot read" scenario uses a wrong `content_scope`. I probed a
  truncated `replaced_digest`: it refuses with the record and the "no 64-hex"
  reason. Bucket 2.
- **The e2e-in-trial scenario is intentionally a retained-drift variant.**
  "Verification names an e2e package file changed in the trial" is caught by
  `check_retained`, not by `check_checkout`, because the e2e package is also a
  retained set (`design.md:541-544`). Disabling `check_retained` makes it fail;
  the retained reason is the design's assigned reason. The title is accurate
  and the scenario protects a distinct frozen-file guarantee; keep it as a
  variant. No strengthening, merge, or re-title is required.
- **R-147's numbers are correct.** `givn outline report --json` reports
  `module-code-lines` 442 for `src/ise/mod.rs` (513 non-blank, non-comment
  lines including the inline test module), 348 for `src/e2e_boundary/mod.rs`
  (533), 485 for `src/fossil.rs` (709 physical), 508 for `src/git.rs`; only
  `src/git.rs` reports the 500-line module hint. The row
  (`docs/arc42/11-risks-and-technical-debt.md:134`) states the module metric,
  not the file `code_lines` metric (467/365/489/512). No correction is needed.

No step body is empty. The checked scenarios' commits contain the batch
implementation or regression tests (the batch map, 58 commits); the ADR
fragment is the exception recorded above. Every promised component exists and
every consumer uses its declared contract.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 introduction-and-goals | `docs/arc42/01-introduction-and-goals.md` | not affected | no goal change | no batch | adds a command pair, no goal | yes |
| 02 constraints TC-46, TC-47 | `docs/arc42/02-architecture-constraints.md:54-55` | Yes | D1-D10, Replace/Verify Flow | `admission-refusals` … `verify` | `src/ise`, `src/scm`, `src/git`, `src/fossil` | yes |
| 03 CLI rows, verifier entity, in-scope list | `docs/arc42/03-context-and-scope.md:32,62-63` | Yes | command surface, record | `admission-refusals`, `verify` | `src/main.rs:203-301,632-659`; `src/cli/ise.rs` | yes |
| 04 clean-room trial strategy | `docs/arc42/04-solution-strategy.md:24` | Yes | Overview | `partition-and-write` | `src/ise/mod.rs` revision read and write | yes |
| 05 `cli::ise`, `ise`, two read seams | `docs/arc42/05-building-block-view.md:62-63,90` | Yes | Architecture Impact | `admission-refusals`, `trial-record` | `src/cli/ise.rs`, `src/ise`, `src/e2e_boundary`, `src/gherkin/content.rs` | yes |
| 06 6.21 replace and verify flows | `docs/arc42/06-runtime-view.md:982-1050` | Yes | Replace Flow, Verify Flow | `partition-and-write`, `verify` | `src/ise/mod.rs:91-185`; `src/ise/verify.rs:57-82` | yes |
| 07 deployment | `docs/arc42/07-deployment-view.md` | No | one binary, a directory | no batch | unchanged | yes |
| 08 8.29 record, identity, placement | `docs/arc42/08-crosscutting-concepts.md:1464-1520` | Yes | Data Model, Identity | `trial-record`, `copy-kinds` | `src/ise/record.rs`; `src/cli/ise.rs`; both backends | yes |
| 09 ADR-076 | `docs/arc42/adr/ADR-076-clean-room-trial-record.md`; `docs/arc42/adr/README.md:63` | Yes | D2/D4/D5, record scope | `durable-docs` | ADR status accepted; register row 63; count 28 accepted, 0 proposed | yes |
| 10 quality requirements | `docs/arc42/10-quality-requirements.md` | No | functional scenarios only | no batch | no quality metric added | yes |
| 11 R-142 … R-147 | `docs/arc42/11-risks-and-technical-debt.md:129-134` | Yes | Risks and Trade-offs | `replacement-refusals`, `copy-kinds`, `trial-record`, `target-and-copy-refusals`, `verify` | R-142 absent-path probe, R-145 Fossil fixture, R-147 outline metrics | yes |
| 12 glossary | `docs/glossary.md:73-76` | Yes | Report/record terms | `trial-record` | `Trial directory`, `Trial record`, `Trial E2E digest`, scoped `E2E identity` anti-term | yes |

Independent chapter re-derivation against the arc42-docs table agrees with
`arc42.md`'s 12 rows: 2, 3, 4, 5, 6, 8, 9, 11, 12 affected; 1, 7, 10 not. All
12 chapter files exist and contain decision-specific content; no box-drawing
or `+---`/`|---` pseudo-diagram was found. Every affected chapter statement
matches the design and the implementation. `arc42.md:53` records
`STATUS: DONE`, and the assessment's placement table matches the committed
docs; `docs/domain-model.md` is unchanged because the model excludes command
parsing, file formats, and internal Rust components, so the no-op is the
assessed result.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary holds `Trial directory`, `Trial record`, and `Trial E2E digest`,
and scopes the `E2E identity` anti-term to the change verification receipt
(`docs/glossary.md:73-76`). The specs, design, code, report lines, and record
use those terms; the printed `E2E digest` is not called an `E2E identity`.
`docs/domain-model.md` is left unchanged: this change adds architecture,
command parsing, and file formats, which the model excludes, not a domain
structure. No new domain term or structural relationship was invented.

One contract wording is inconsistent and is a finding (F7): the use-case rule
at `usecase.md:62` and design partition bullet at `design.md:641-642` say a
`--full` replacement reports `Not replaced: 0`; a retained file under a
`source:` root stays in the trial and is a direct sibling of the replaced
files, so the report can be `Not replaced: 1`. The implementation and the
containment-count definition are correct; the unqualified `0` claim is not.

## Coverage

No coverage receipt exists in this change; the archive will carry the measured
result. Classification from inspection and the probes:

- **Bucket 1 — dead code:** none. Every new symbol is consumed: `partition.rs`
  by `ise`, `record.rs` by replace and verify, `verify.rs` by `cli::ise`,
  `RevisionEntry` by `ise` and the SCM factories, `e2e_file_set` by replace and
  verify, `e2e_scenario_digest` by the record and verify.
- **Bucket 2 — missing test coverage:**
  - F3: a gained e2e package path and an extra `@e2e` feature path;
  - F4: a wrong record schema and a non-64-hex `replaced_digest`;
  - F5: the submodule half of refusal 9 (the gitlink scenario is
    non-discriminating);
  - F6: the architecture-principles-off no-read precedence for prepare and
    verify.
- **Bucket 3 — legitimately hard to test:** none claimed. The re-read race is
  covered by the injectable-writer unit test (`src/ise/mod.rs:540-585`); the
  target-symlink race is a residual risk recorded as R-144, not a gap. The
  Rust module metrics and the permanent suite cover the remaining paths.

No classification outside these three buckets is used. The author's
"coverage limit" note for the gitlink scenario is not a fourth bucket; it is
reclassified here as bucket 2. No redundant unit test duplicates a scenario
behavior; the two new unit tests cover the writer comparison the CLI cannot
race. No `@wip` tag remains and no implementation-layer detail appears in the
spec.

## Use-case and Persona conformance

`prepare-an-ise-test` keeps its identifier, goal, actors, preconditions, main
flow, extensions, rules, minimal and success guarantees, and the two
capabilities `clean-room-preparation` and `trial-input-verification`. The
typed relationships are `Includes: none` and `Extends: none`. The use-case
scope rejection criteria do not hold: one goal, a primary actor named by the
trigger, capabilities placed under that goal, extensions keyed to main-flow
steps 3/5/6, concrete preconditions, domain vocabulary, no component-shaped
name, and no relationship misuse. The success guarantee's conditional second
clause is the verifier's confirmation of the same goal, not a second goal.

The three confirmed Personas are `developer--primary-user`,
`agent--technical-role`, and `givn-maintainer--owner`; every one has a
per-capability impact row (`design.md:766-778`), and the Gherkin names the
domain actors (developer, verifier), never a Persona biography. No Persona is
invented or silently promoted. The persona dispositions are below.

## Visual review

The design declares `Interface: terminal`, viewports/states not applicable,
design system none (`design.md:36-67`). The committed terminal evidence is:

- `evidence/visual/developer-prepares-a-clean-room-trial-from-a-recorded-revision/transcript.txt`
  — the real `givn ise replace` run: the ten fixed lines plus `Receipt path`,
  the trial file list, and the `ise-trial/v1` record.
- `evidence/visual/verifier-re-checks-a-prepared-trial-and-receives-the-frozen-input-report/transcript.txt`
  — the real `givn ise verify` run: `Trial:`, `Retained: clean`, `E2E: clean`,
  exit 0.

The contract's baseline holds in both records: refusals name their condition
and subject, no Rust module/struct/field name appears in the printed output,
the exit codes are the contract's, the ten lines and the conditional receipt
line are present, and the record uses the domain field names. The Git
transcript is one backend; the Fossil executable/symlink and `\s` behavior is
covered by the regular `clean-room-preparation` scenario and was probed
against a real Fossil checkout with `allow-symlinks on`.

Persona findings, each dispositioned:

- `developer--primary-user`: the prepare transcript shows the program's
  placement counts, the written trial path, and the record path, so the
  developer can see what was handed over. No finding.
- `agent--technical-role`: the prepare report and record use the fixed line
  names and stable field names; the verify transcript prints the clean
  identity lines. No finding.
- `givn-maintainer--owner`: the record carries the backend, revision, counts,
  per-path identities, and the `ise-trial/v1` scope; the Fossil parity is
  proven by the regular scenario and the probe, not by a second transcript.
  No contract rule fails.

VISUAL-REVIEW: PASS

## Overlap dispositions

The gate reports 29 `SHAPE` pairs involving this delta; every pair is recorded
below. `duplicate` is not used: no pair asserts the same behavior with the same
inputs. `variant` is a different input or refusal form of the same action;
`boundary` is a different rule, file class, or pre-launch check.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Preparation refuses when the declaration is missing | Preparation refuses a path outside every source root | variant |
| Preparation refuses when the declaration is missing | Preparation refuses a repository link in the copy set | variant |
| Preparation refuses when the declaration is missing | Preparation refuses a nested repository marker in the copy set | variant |
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

## Split-or-keep

The lint gate reports 0 `LONG` scenarios; no row is required.

| Scenario | Decision | Reason |
|---|---|---|

## Removed and added pairs

The delta has 0 removed scenarios and 58 added scenarios plus the one
`@givn.modified` ADR fragment. There is no removed+added pair to classify. The
modified ADR scenario supersedes its permanent 27-accepted predecessor; the
new text and the permanent corpus both carry 28 accepted and 0 proposed.

## README impact

Decision: `updated - Configuration`. The change adds two user-facing commands
and a report/record contract. `README.md`'s Configuration command table
(`README.md:90-104`) lists `givn skills sync` and `givn commit` but not
`givn ise replace`/`givn ise verify`; the README has no ISE capability
sentence. The section has not been edited. F8 records the missing edit; the
`updated - Configuration` decision is not evidence that the edit exists.

## Deferrals

None. No `@e2e` tag was removed, no mandatory check was deferred, and no
operator decision is required by this report.

## Findings

Severity labels: blocker = the review cannot pass until it is fixed; major =
unfinished behavior or coverage; minor = artifact accuracy or bookkeeping.

1. **F1 — blocker, capability routing.** The delta carries the
   `adr-qualification-gate` fragment scenario
   (`specs/fragments/adr-qualification-gate.feature:1-16`), but
   `proposal.md:108-111` declares only `clean-room-preparation` and
   `trial-input-verification`. `givn check review` fails: "delta capability
   'adr-qualification-gate' is not declared in the Capability Routing table."
   Fix: add the capability with an `EXTEND adr-qualification-gate` decision
   (or remove the fragment delta if it is out of scope), then re-run the gate.
   This also accounts for the 59th unit in F10.
2. **F2 — blocker, design conformance.** `design.md:105-110` names
   `src/ise/mod.rs` as owning the partition, and `design.md:219`/`:520` repeat
   the placement. The as-built module is `src/ise/partition.rs` (300 physical
   lines; moved in `tasks.md:1375-1378`). The component and contract are
   unchanged, but the design's named file layout is stale and design-review
   was not re-run for the split. Fix: update design.md's Architecture Impact
   (and any file references) and re-run design-review before sign-off.
3. **F3 — blocker, coverage.** `CHECKOUT_GAINED_PATH`
   (`src/ise/verify.rs:28,222-225`) and the extra `e2e_scenarios` feature path
   (`:264-270`) are implemented and documented as an external boundary
   (`design.md:649-658`, "extra path"), but no delta scenario adds an e2e
   package file or a new `@e2e` feature. The 16-scenario verify feature has no
   such case. I probed both and both refuse by name; permanent coverage is
   missing. Fix: add one variant scenario (new e2e file and/or new feature)
   that asserts the gained path/reason.
4. **F4 — blocker, coverage.** `read_record` rejects a schema other than 1
   and a `replaced_digest` that is not 64 hex
   (`src/ise/verify.rs:91-112`); design Verify Flow step 4 requires the digest
   assertion. No delta scenario exercises either branch; "Verification refuses
   a record it cannot read" uses a wrong `content_scope`. I probed a truncated
   `replaced_digest`: it refuses with the record and the "no 64-hex" reason.
   Fix: add a record variant (wrong schema and/or bad digest) or a unit test
   that asserts the named refusal.
5. **F5 — blocker, discrimination.** "Preparation refuses a repository link
   in the copy set" passes with `require_copy_set`'s submodule branch removed:
   the copy-set blob read fails first and names the same `vendor/lib`
   (`tests/steps/clean_room_preparation_steps.rs:825-840`; the author records
   the limit at `tasks.md:1340-1344,1632-1656`). The nested-marker sibling
   covers the marker half only. Fix: strengthen the Then to require the
   repository-link reason (or add an isolating scenario). This is bucket 2,
   not a fourth-bucket exception.
6. **F6 — blocker, precedence coverage.** The architecture-principles-off
   fixtures start from a valid declaration
   (`tests/steps/ise_clean_room_support.rs:684-695`) and a valid prepared
   record (`trial_input_verification_steps.rs:85-90,131-134`). Only the
   `ise`-off fixtures use a malformed declaration or corrupted record to prove
   the add-on check runs before the declaration/record read. The
   architecture-principles half of design D10 and `usecase.md:75-76` is
   correct (I probed architecture off + malformed declaration: it refuses
   `architecture_principles`), but the permanent scenarios cannot distinguish
   the ordering. Fix: use the malformed/corrupted fixtures in both
   architecture-principles-off scenarios.
7. **F7 — blocker, spec wording.** `usecase.md:62` and `design.md:641-642`
   say a `--full` replacement reports `Not replaced: 0`. A retained file under
   a `source:` root stays in the trial and is a direct sibling of the replaced
   files, so the report is `Not replaced: 1` (my `/tmp` probe: retained
   `src/keep.rs`, source `src/**`, `--full`; `Replaced: 2`, `Not replaced: 1`).
   The retained-precedence rule at `usecase.md:61` and design-review Blocker 1
   already state the exception; the unqualified `0` claim contradicts them.
   Fix: qualify the rule with "when no retained file sits under a source
   root" or remove the claim.
8. **F8 — blocker, README impact.** The change is user-visible (two commands,
   a report, and a record), but `README.md:90-104` was not edited and has no
   ISE row or sentence. `README-IMPACT: updated - Configuration` is the
   required decision; the edit is missing. Fix: add the two commands (or an
   ISE capability sentence) to the README and keep the decision line truthful.
9. **F9 — major, tasks bookkeeping.** `givn status` reports 239/249. The Setup
   baseline box is stale and contradicts the current 980/980 permanent run;
   the `@e2e` REFACTOR box and seven Closeout boxes are unchecked. Fix: run the
   closeout commands, update the stale baseline evidence, and check every box
   before the next review pass.
10. **F10 — major, delta accounting.** The batch map totals 58 and says all 58
    delta scenarios are `@givn.added` (`tasks.md:55-70`), but the change has a
    59th unit, the `@givn.modified` ADR fragment. It has no batch, task, or
    `COMMIT:` line; its work is in `d5fa08ec`. Fix alongside F1: declare the
    unit's batch/commit evidence and the route.
11. **F11 — minor, evidence wording.** Batch 6's five trial-record RED
    records are admitted skipped RED (`tasks.md:1936-1946` and siblings), not
    chronological RED. The disclosure is compliant with the steps instruction;
    the record must not be read as RED-before-fix. No code action.
12. **F12 — minor, stale contradiction.** `tasks.md` Contradiction 6 says
    design.md lists `cli::archive`/`cli::migration` as `RevisionContent`
    consumers, but the current design table names only `ise` and the table's
    note says there is no caller outside `src/scm.rs`. Update the contradiction
    text or drop it.

## Sign-off checklist

- Fabrication audit: no empty steps; the batch-6 skipped RED and the batch-7
  probes are recorded honestly; F5 is the one non-discriminating scenario.
- Every checked task has a verified commit or regression test, except the
  unaccounted ADR fragment (F10); every promised component exists.
- Every consumer uses only its declared contract.
- Strict-mode proof present and passing.
- Scenario execution evidence: `givn check tasks` 57/57 at `ff619517`; delta
  E2E 2/2, 57 skipped; permanent non-E2E 980/980, 111 skipped.
- Coverage classification is from inspection and probes; the archive receipt
  will carry the measured result. Bucket-2 findings F3-F6 are open.
- Dead code: none. Missing tests: F3-F6. Hard-to-test bucket 3: none.
- Redundant unit tests: none.
- No `@wip`; no implementation-layer detail in the spec.
- Canonical E2E policy: 2 normalized actions, 2 matrix rows, 2 `@e2e`
  scenarios; variants are regular; real CLI driver; no HTTP/fetch.
- The exact `verify.e2e_command` file was identified; no parallel e2e
  implementation exists; the scope is selected explicitly.
- Implementation matches design except the stale partition file layout (F2)
  and the README decision (F8).
- Interaction coverage: 2 inventory entries, 2 matrix rows, 2 `@e2e` titles
  and real step drivers.
- No finding is excused with an invented fourth bucket.

## Verification

`givn check tasks --change add-ise-clean-room-commands`:

```text
[Summary]
3 features
57 scenarios (57 passed)
271 steps (271 passed)
  ✓ change verification receipt: 57 scenario(s), 0 failed, 0 skipped at ff6195179f6be2c0c7514bb6f536d4a8392d8120
  ✓ verify passed
```

`givn check review --change add-ise-clean-room-commands` (current tree, with
this review file present) — exit 1:

```text
net delta: 58 added, 1 modified, 0 removed
  ✓ overlap dispositions passed
  ✓ scenario commits passed (58 scenario(s))
  ✓ change verification receipt passed (57 scenario(s) at ff6195179f6be2c0c7514bb6f536d4a8392d8120)
  ✓ scenario commits passed (58 scenario(s))
error: review checks failed:
  integrity checks failed:
  review.md actual verdict is FAIL
  capability routing decisions do not match the built delta:
  delta capability 'adr-qualification-gate' is not declared in the Capability Routing table — the proposal records no 'NEW in <usecase-id>' or 'EXTEND <capability>' decision for it
```

The gate's 29 `SHAPE` and 100 `SUBST` advisories are recorded or dispositioned
above; `givn lint` exits 0. The review's blockers are F1-F8; F9 and F10 are
bookkeeping findings. The code behavior is green, but the change is not
signable while the routing, design, coverage, spec-wording, and README
findings are open.


## Finding closure

Every finding below is closed by the implementation author after the review
above. This section changes no verdict: `REVIEW: FAIL` stays the reviewer's
line until the reviewer re-checks and flips it. Commits: `6bfbd4d5` (artifacts),
`23ad9e6f` (tests and fixtures), `14973c13` and `ab36632a` (tasks), plus the
pre-existing `d5fa08ec` for the ADR register.

| ID | Fix | Test | Commit |
|---|---|---|---|
| F1 | `proposal.md`'s Capability Routing table gains `adr-qualification-gate` with `EXTEND adr-qualification-gate`, and the prose names `specs/fragments/adr-qualification-gate.feature` and its `@givn.modified` scenario. | `givn check review` now prints `capability 'adr-qualification-gate' already owned by .../givn/specs/fragments` and reports no routing error; it previously failed on the undeclared capability. | `6bfbd4d5` |
| F2 | `design.md` Architecture Impact now names `src/ise/partition.rs` for `Partition`, refusals 5 to 7, the counts, and `behavior_features`; `src/ise/mod.rs` keeps `replace`, the preflight, the write and re-read, and the report. D1's realization points at `partition.rs`. `design-review.md` records the split as post-review deviation 4 (2026-11-15, `src/ise/mod.rs` exceeded 500 code lines). | Static read of `design.md:103-110,221` against the tree; `givn check review` reports no design-conformance error. | `6bfbd4d5` |
| F3 | Two delta scenarios: `Verification names an e2e package path the record does not list` (`CHECKOUT_GAINED_PATH`, untracked `tests/e2e/steps/extra.rs`) and `Verification names an e2e scenario the record does not list` (extra `e2e_scenarios` branch, untracked `givn/specs/demo/extra.feature` with one `@e2e` scenario). Each Then asserts the named path and its reason. | Both scenarios pass. Discrimination probes: disabling the gained-path insert makes the first fail (`left: Some(0)`, `right: Some(1)`); disabling the extra-feature insert makes the second fail the same way. Both probes reverted with `git checkout -- src/ise/verify.rs`. | `23ad9e6f` |
| F4 | Two delta scenarios: `Verification refuses a record with another schema` (`schema: 2`) and `Verification refuses a record without a replaced digest` (`replaced_digest: abc123`). Thens assert the record name plus `schema`/`a trial record is schema 1`, and `replaced_digest`/`the record is no trial evidence`. | Both scenarios pass. Discrimination probes: narrowing the schema bail and the digest bail each make their scenario fail with exit 0 instead of 1. Probes reverted. | `23ad9e6f` |
| F5 | The gitlink scenario is kept; its end-to-end assertion is redundant with the blob-read refusal, recorded in the batch note and the scenario entry. The submodule half is now proven directly: `a_submodule_entry_in_the_copy_set_is_refused_as_a_repository_link` calls `require_copy_set` with a `Submodule` entry; companion test `a_regular_entry_in_the_copy_set_passes_the_link_check` proves the check is not a blanket refusal. The title is not retitled: it states the observable refusal, not the reason. | `cargo test --locked --lib ise::tests` → 4 passed. Probe: replacing the `EntryKind::Submodule` comparison with `EntryKind::Symlink` fails the unit test (`a repository link is refused: ()`); reverted, test re-added verbatim. | `23ad9e6f` |
| F6 | Prepare: `architecture_principles_off_project()` now commits a malformed declaration (`component: ["x"]`), the Given says `and a malformed declaration`, and the scenario adds `the declaration is not read`. Verify: the scenario's Given is `a prepared Git trial with a corrupted record` and the Then adds `verification reports no record error`. | Both scenarios pass. Discrimination probes: moving `require_addons` after the declaration load in `admit` makes prepare fail naming `givn/specification.yaml`; moving `read_record` before `require_addons` in `verify` makes the re-check fail with the record parse error. Probes reverted. | `23ad9e6f` |
| F7 | `usecase.md:62` and `design.md`'s `Not replaced` bullet now say `--full` reports `0` when no retained path sits under a source root and counts a retained file under one. New scenario `A full replacement counts a retained file under a source root as not replaced` (`structure: ["src/keep.rs"]`, `source: ["src/**"]`) asserts `Replaced: 1`, `Not replaced: 1`, and the trial holds `src/keep.rs`. The `0` case is the existing scenario `A full replacement removes every tracked file under the source roots` (retained role `givn/specs/**` outside the source root, asserts `Not replaced: 0`); no duplicate scenario was added. | The new scenario passes; the existing `0` scenario still passes. | `6bfbd4d5` (wording), `23ad9e6f` (scenario) |
| F8 | `README.md` Configuration table gains `givn ise replace` and `givn ise verify` rows, one sentence each, naming the Ise Test and the frozen retained files and `e2e` package. `README-IMPACT: updated - Configuration` is unchanged and now true. | Static read of `README.md:104-105`. | `6bfbd4d5` |
| F9 | Setup baseline box records the current green run (`980 passed, 111 skipped`, exit 0) and the history of the stale ADR failure; the `@e2e` REFACTOR box is ticked with the fresh delta and e2e summaries; the seven Closeout boxes carry the command output: phrase bindings (121/10/41/2/0), no stub bodies, no `@wip`, both transcripts, lint clean (22 SHAPE, 118 SUBST, 0 LONG), the whole-change suite, and the receipt. `givn status` reports 276/277 tasks; the one open box is the reviewer's `givn check review`. | `givn status --change add-ise-clean-room-commands`. | `14973c13`, `ab36632a` |
| F10 | The ADR fragment is its own batch row `ADR-076` (one `@givn.modified` scenario) with `COMMIT: d5fa08ec`; the batch map totals 64 units (63 added, 1 modified): 43 `clean-room-preparation`, 20 `trial-input-verification`, 1 `adr-qualification-gate`. | `givn check tasks` reports 64 scenario commits; the ADR entry's commit message contains `ADR-076`, so the batch label resolves. | `14973c13` |
| F11 | Confirmed; no code action. The batch-6 trial-record entries keep their `RED replay (not chronological; the implementation preceded the first run)` wording, which the steps instruction permits. This closure does not present them as historical RED. | Read of `tasks.md` batch 6. | — |
| F12 | Contradiction 6 no longer claims `cli::archive`/`cli::migration` are consumers; it records that `ise` is the only `RevisionContent` consumer and that the earlier text was stale, matching design-review clarification 2. | Read of `tasks.md` Contradictions 6. | `14973c13` |

### Runner results

```text
cargo fmt --all -- --check                                        exit 0
cargo clippy --locked --workspace --all-targets -- -D warnings    exit 0
cargo nextest run          1646 passed, 0 skipped                 exit 0
./run-tests.sh --non-e2e    980 passed, 111 skipped               exit 0
./run-tests.sh --e2e        111 passed, 980 skipped               exit 0
GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e
                             62 passed, 2 skipped                 exit 0
GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --e2e
                              2 passed, 62 skipped                exit 0
givn lint --change add-ise-clean-room-commands                     exit 0 (clean)
givn check tasks --change add-ise-clean-room-commands             exit 0
  62 scenarios / 296 steps; receipt 62 scenario(s), 0 failed, 0 skipped
```

Every closure commit after the tested tree is documentation-only, so
`givn check tasks` re-records the receipt and reuses it across those revisions;
the reader takes the bound revision from
`givn/changes/add-ise-clean-room-commands/verification/tasks-verification.json`
rather than from this prose. Both `verify` runs rewrote the verify transcript
with new `/tmp` paths and the permanent e2e run rewrote three archived
screenshot pairs, so those files were restored with `git checkout --` before
the tasks commit. `givn check review` now fails only on `review.md actual
verdict is FAIL`; the routing, overlap, scenario-commit, and receipt checks
pass. The disposition table dropped nine rows whose pairs no longer
shape-match after the added scenarios; the gate reports no undispositioned
pair.

REVIEW: FAIL
