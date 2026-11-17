# Review: strengthen-ise-specification

Review kind: independent
Author: strengthen-ise-specification author (omp)
Reviewer context: omp independent design-review context (deepseek-flash, tenth review, 2026-11-17)

## Scope and result

The change rewrites the clean-room command surface, cuts the trial record to the
`ise-manifest/v1` manifest, states the canonical checksum bytes, and adds the
`compare` report. I re-read the proposal, the design, the delta specs, the
tasks, the design review, the arc42 assessment, the durable arc42 chapters, the
reference, the served `ise` instruction, and the implementation diff
`76159193..HEAD`. I ran the task gate, the delta non-e2e scope, the delta e2e
scope, the permanent e2e tier, and eight mutation probes.

The implementation is broadly conformant and the checksum rule reproduces the
pinned values from the stated bytes. Seven blocking findings remain: one public
report line carries an unstated ellipsis, a deleted declaration is refused
instead of reported, the delta use case states the opposite docstring rule from
the reference and the golden fixture, the design review predates the design it
was supposed to review, the retired record shape is not refused by name, and
one scenario batch records a tag-only commit. `REVIEW: FAIL` follows the
findings.

## Implementation evidence check

### `@e2e` tag integrity

- The delta has 156 scenarios: 56 `@givn.removed`, 30 `@givn.modified`, 70
  `@givn.added` (100 executable after the 56 retired placeholders are removed).
- Two delta scenarios are `@e2e`: `Developer rebuilds the whole product from its
  Specification` and `Verifier compares the spec copy with the manifest`. Both
  replace permanent `@e2e` scenarios named by `@givn.removed`; no `@e2e` tag
  was silently removed from a planned scenario. The permanent declaration
  `@e2e` scenario is unchanged.
- No delta scenario carries `@wip`.

### Step bodies

`0` empty or trivial step bodies found across the seven changed step files:
`tests/steps/clean_room_preparation_steps.rs` (217 bindings),
`tests/steps/trial_input_verification_steps.rs` (71),
`tests/steps/prose_consolidation_steps.rs` (63),
`tests/steps/instruction_integrity_steps.rs` (244),
`tests/steps/specification_declaration_steps.rs` (9),
`e2e/tests/steps/clean_room_preparation_e2e_steps.rs` (18), and
`e2e/tests/steps/trial_input_verification_e2e_steps.rs` (6). No `todo!()`,
`unimplemented!()`, or stub body remains.

### Checked scenarios and commits

All 224 tasks are checked. The recorded batch revisions are:

| Batch | Revision | Content |
|---|---|---|
| setup | `76159193af` | plan, durable docs, README command table |
| command-surface | `5bcf0c9d0c` | parser, refusal prefixes, step bindings |
| revision-declaration | `b84dbd24ce` | revision read, four declaration refusals |
| component-paths | `c5679b623f` | path expansion and role refusals |
| copy-set | `d62403e182` | symlink and repository-link refusals |
| partition-and-copy | `dceea391c1` | partition, counts, write |
| report-and-manifest | `76f5237fed` | report lines, manifest writer |
| checksums | `bc5fd98331` | path checksum, rollups, scenario digest |
| write-targets | `fce3d39f` | copy and manifest target refusals |
| compare-report | `6965c0d3` | compare report and copy-side checks |
| compare-checkout | `7c52e968` | checkout content paths and added paths |
| compare-refusals | `a9f3e8ce` | strict manifest reader |
| declaration-boundary | `f21807f8` | tag removal only (finding B6) |
| cross-capability-rebinding | `73bcd1bc0e` | two delta features, e2e fixture rebinding |

The commits for batches 1-11 and 13 contain production code or step
definitions/regression tests. Batch 12's commit changes only
`@givn.modified @wip` to `@givn.modified`; see B6.

### Promised components and contracts

`src/ise/manifest.rs` and `src/ise/compare.rs` exist; `src/ise/record.rs` and
`src/ise/verify.rs` are deleted, with no shim. `crate::cli::ise` consumes the
`crate::ise` public API and does not import `compare` or `manifest` internals.
The step files named by design.md exist. The canonical fixture helpers
(`canonical_checksum_project`, `rich_checksum_project`, `remove_declaration_blob`)
are in `tests/steps/ise_clean_room_support.rs` and import no production crate.

### Strict mode

`.fail_on_skipped()` is present on both builders:
`tests/features_runner.rs:55,155` and `e2e/tests/e2e_runner.rs:46`. The setup
task records a non-zero unbound-step probe.

### E2E assertions and driver

Both delta `@e2e` scenarios drive the compiled binary through
`crate::harness::run_givn_with_envs` / `common::run_givn` and assert stdout and
exit code. Neither scenario asserts only a repository or database state. The
committed transcripts are
`evidence/visual/developer-rebuilds-the-whole-product-from-its-specification/transcript.txt`
and `evidence/visual/verifier-compares-the-spec-copy-with-the-manifest/transcript.txt`.

### Scope selection

`verify.e2e_command` is `./run-tests.sh --e2e --package e2e`; `run-tests.sh`
exports `GIVN_RESULT_SCOPE=e2e` before the e2e runner. The change receipt
records the regular scope (`98` passed, `0` failed, `0` skipped) and the e2e
scope (`2` passed, `0` failed, `0` skipped). The permanent e2e tier passes
`119` scenarios and `781` steps.

### Design match

The runner, step-file layout, and framework match design.md. The report line
and the docstring rule do not; see B1 and B3. The delta use case is the
specification the archive merges, so its contradiction with the reference is a
finding, not a documentation wording choice.

### Interaction coverage

The owning use case `prepare-an-ise-test` has two inventory entries; the
permanent `declare-the-specification` use case has one unchanged entry.

| Inventory entry | Matrix row | Existing `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `clean-room-preparation`: write the spec copy for the whole product or named component paths | Yes | `Developer rebuilds the whole product from its Specification` | Compiled `givn ise rebuild-all --into <copy> --manifest <file>` subprocess |
| `trial-input-verification`: compare the spec copy and the checkout with the manifest | Yes | `Verifier compares the spec copy with the manifest` | Compiled `givn ise compare --manifest <file> --copy <dir>` subprocess |
| `specification-declaration`: declare the five roles and the replaceable source roots | Yes (unchanged) | `The declared roles are read by the project-health pass` | Compiled `givn lint` subprocess |

No inventory entry is unmapped; no delta scenario adds a second `@e2e` for the
same action.

### Corpus rebinding and retired surface

Every permanent scenario in `clean-room-preparation` (56) and
`trial-input-verification` (27) has exactly one `@givn.removed` or
`@givn.modified` operation; every modified/removed title exists in the
permanent corpus; no added title collides. The three other delta features
modify one permanent title each. A sweep of the permanent corpus for `trial`,
`receipt`, `ise replace`, `ise verify`, `Retained:`, `Replaced:`,
`Kept source:`, `Outside the declaration:`, `Not replaced:`, `--full`, and
`--receipt` finds only unrelated archive/change-receipt vocabulary and the
`outline --path` flag; the ISE scenarios that carry those tokens are all
covered by the delta.

### Spec gap outcome

`Spec gap outcome: implementation defect`. The as-built behavior contradicts
the merged Specification at B1, B2, and B5; B3 corrects the delta use case to
the rule the reference and the golden fixture already state. One outcome is
recorded before any fix.

## Probes and outcomes

| # | Batch | Mutation | Scenario run | Observed result |
|---|---|---|---|---|
| P1 | 13 | `assets/instructions/ise.md`: `always carries` -> `sometimes carries` | `Served ise guidance states that the trial always carries the declaration` | FAILED: `the served ise guidance does not state that the spec copy always carries the declaration` |
| P2 | 7 | `src/ise/manifest.rs`: drop the `kind` byte from `path_checksum` | `Preparation records the canonical checksums for a pinned fixture` | FAILED: `specification_checksum` observed `96f6b5f9...`, expected `35444598...` |
| P3 | 5 | `src/ise/partition.rs`: remove the Specification-role precedence filter | `A whole-product rebuild keeps a Specification file under a source root` | FAILED: report printed `Source code removed: 3`, expected `1` |
| P4 | 2 | `src/ise/mod.rs`: print `does not record` in the no-blob branch | `Preparation refuses a declaration the revision records but cannot read` | FAILED: stderr did not carry `returned no blob` |
| P5 | 1 | `src/cli/ise.rs`: use the `ise replace` prefix | `Preparation refuses while the ise addon is off` | FAILED: stderr did not start with `ise rebuild-all: ` |
| P6 | 9 | `src/cli/ise.rs`: print `+` for every difference kind | `Comparison names a changed Specification file in the copy` | FAILED: stdout printed `+ givn/specs/demo/spec.feature`, expected `-` |
| P7 | 10 | `src/ise/compare.rs`: return no added checkout paths | `Comparison names an added End-to-end file in the checkout` | FAILED: stdout printed `End-to-end tests: match`, expected `1 added` |
| P8 | 11 | `src/ise/manifest.rs`: accept every checksum value | `Comparison refuses a manifest with a malformed checksum` | FAILED: no `ise compare:` refusal, command did not exit 1 with the manifest named |

Every probe was reverted with `git checkout -- <path>`; the working tree is
clean after the review except for this artifact.

## Findings

### B1 (blocker) - the checksum report line carries an unstated ellipsis

- **Location.** `src/ise/manifest.rs:239` `abbreviate` returns the first 12 hex
  characters plus `…`; `src/cli/ise.rs:128-130` prints it;
  `e2e/tests/steps/clean_room_preparation_e2e_steps.rs:367-385`
  (`report_prints_the_checksum_lines`) strips `…` and fails a line without it.
- **Contradicting source.** `docs/reference.md:513`: "The report prints the
  first 12 lowercase hex characters; the manifest keeps the full value."
  `docs/reference.md:417` shows `Specification checksum: 4f1c9a2b7d01` with no
  ellipsis. `design.md` Step Definitions says "Exact 12-hex match, no
  ellipsis." The delta scenario `clean-room-preparation.feature:337` says "12
  lowercase hex characters each", with no ellipsis.
- **Probe.** A real rebuild on a temporary project printed
  `Specification checksum: 2de041d8fd87…`; the committed e2e transcript prints
  `7d7a8df82f9f…`.
- **Effect.** A second implementation that prints the stated 12 hex characters
  fails the frozen e2e suite; the suite currently certifies the unstated
  character.
- **Small fix.** Delete the ellipsis from `abbreviate` and from the e2e step;
  the old `report_abbreviates_the_digests` binding (whose permanent scenario
  the delta removes) also expects the ellipsis and must be removed with it.
- **Classification.** implementation defect; bucket 2 (the assertion must match
  the Specification).

### B2 (blocker) - a deleted copy declaration is refused, not reported missing

- **Location.** `src/ise/compare.rs:70` calls `copy_declaration(&copy)?` before
  the comparison; `copy_declaration` at `:300` reads
  `<copy>/givn/specification.yaml` with `read_to_string` and propagates the
  error.
- **Contradicting source.** `docs/reference.md:693-696`: "The copy's declaration
  is compared like every other Specification file, so an edited or deleted
  declaration is a difference naming the path." The delta use case example
  (`specs/prepare-an-ise-test/usecase.md:143`) says deleting
  `givn/specification.yaml` reports `Specification files: 1 missing`.
- **Probe.** I prepared a copy with `ise rebuild-component ... --manifest`,
  deleted `copy/givn/specification.yaml`, and ran `ise compare`. Exit `1`,
  stdout empty, stderr
  `ise compare: read the spec copy's declaration '.../copy/givn/specification.yaml': No such file or directory (os error 2)`.
- **Coverage.** `Comparison names a missing Specification file in the copy`
  deletes `givn/specs/demo/spec.feature`, not the declaration; no scenario
  covers the declaration's missing or malformed case.
- **Small fix.** Treat the copy declaration as optional for the added-file
  refusal: if it is absent or unreadable, skip that refusal and let the
  recorded `checksums.specification` comparison report `1 missing` / `1
  changed`; add a scenario deleting `givn/specification.yaml` and asserting
  `Specification files: 1 missing` with `- givn/specification.yaml`.
- **Classification.** implementation defect plus missing test; bucket 2.

### B3 (blocker) - the delta use case states the opposite docstring rule

- **Location.** `givn/changes/strengthen-ise-specification/specs/prepare-an-ise-test/usecase.md:81`:
  "the docstring is the delimiter-stripped value with the common indentation
  removed and the delimiter-adjacent line breaks kept".
- **Contradicting source.** `docs/reference.md:549`: "the docstring has the
  common indentation removed and loses the line break that follows the opening
  delimiter and the line break before the closing delimiter." `src/gherkin/content.rs:32`
  `docstring_content` strips both line breaks. The richer golden fixture
  (`design.md`, Independent Second Implementation) expects
  `"line one\n  indented\nline four"`, with neither line break; the golden
  scenario pins that value.
- **Effect.** A second implementation that follows the delta use case (which
  archive merges into the permanent Specification) keeps the two line breaks,
  produces a different scenario checksum, and fails the golden scenario. The
  reference and the fixture disagree with the merged narrative.
- **Small fix.** Correct the use case sentence to say the line break after the
  opening delimiter and the line break before the closing delimiter are
  dropped, matching the reference and the golden bytes. If the intended rule is
  to keep them, change `docstring_content`, the reference, and every pinned
  value together and re-run design-review; the former is the smaller change.
- **Classification.** specification defect.

### B4 (blocker) - the design review predates the design it reviews

- **Location.** `design-review.md` records `154` scenarios: 56 removed, 28
  modified, 70 added, and does not list the two cross-capability scenarios.
  `design.md` was changed after the `DESIGN-REVIEW: PASS` in
  `aef08fab` (Step Definitions rows for `prose-consolidation` and
  `instruction-integrity`; two Use-Case Traceability rows; the Corpus
  Rebinding sentence now says "the delta adds 72 scenarios: 39 ... 31 ... and
  two cross-capability `@givn.modified` entries"). The two delta features were
  added in `73bcd1bc`, after the design review.
- **Current state.** The delta holds `156` scenarios: 56 removed, 30 modified,
  70 added; `givn check review` reports `net delta: 70 added, 30 modified, 56
  removed`. The `design.md` sentence is numerically wrong: the two
  cross-capability entries are `@givn.modified`, not added, so the added count
  remains 70. The review skill and the served review instruction require a new
  design review when `design.md` changes: "If `design.md` changes, run
  design-review again before continuing."
- **Small fix.** Re-run the independent design review for the added rows and
  the changed corpus counts, update `design-review.md` with the current counts
  and verdict, and then return to this review.
- **Classification.** process/design-review defect; blocking.

### B5 (blocker) - the retired `ise-trial/v1` record is not refused by name

- **Location.** `src/ise/manifest.rs:52-80` deserializes `Manifest` before it
  checks `scope`. A real `ise-trial/v1` record carries `schema` and
  `content_scope`, not `scope`; `serde` fails first.
- **Contradicting source.** Proposal D4 gives the output
  `ise compare: the manifest scope is 'ise-trial/v1', expected 'ise-manifest/v1'`;
  `docs/reference.md` says the earlier record shape is refused by name;
  `usecase.md` says the same.
- **Probe.** A real old-shape record produced
  `ise compare: the manifest '.../old.yaml' does not match ise-manifest/v1: unknown field 'schema', expected one of ...`.
  The recorded `ise-trial/v1` scope is never printed.
- **Coverage.** `Comparison refuses a manifest whose scope is not
  ise-manifest/v1` only changes the `scope` field of a new-shape manifest; it
  does not exercise the retired field set.
- **Small fix.** Parse the YAML value, read `scope`/`content_scope` before
  deserialization, and refuse `ise-trial/v1` with the recorded and expected
  scope. Add a scenario that writes the old record shape and asserts the named
  refusal; keep the changed-scope scenario.
- **Classification.** implementation defect plus non-discriminating scenario;
  bucket 2.

### B6 (blocker) - batch 12 records a tag-only commit

- **Location.** `tasks.md:1630` records
  `COMMIT: f21807f8b3497173c9ec2984461dfea8d1b14c55 -- Batch: declaration-boundary`.
  `git show f21807f8` changes exactly one line:
  `@givn.modified @wip` -> `@givn.modified`. The scenario's `the command exits
  1` assertion is bound by the pre-existing `tests/steps/mod.rs:626` step.
- **Rule.** A comment-only, tag-only, task-only, or feature-only change is not
  evidence; the commit must contain the scenario's implementation or its
  regression test. The scenario protects existing behaviour, but the recorded
  commit contains neither the new assertion nor a test change.
- **Small fix.** Record the commit that introduced the delta scenario and its
  exit-code assertion (the plan commit `76159193`), or land a commit that
  contains the scenario's regression assertion; keep the existing-behaviour
  note. No production code is required.
- **Classification.** implementation-evidence defect; blocking.

### B7 (blocker) - the Capability Routing table omits two delta capabilities

- **Location.** `proposal.md` `## Capability Routing` has rows for
  `clean-room-preparation`, `trial-input-verification`, and
  `specification-declaration` only. The delta also carries `@givn.modified`
  scenarios under `specs/create-change/prose-consolidation.feature` and
  `specs/fragments/instruction-integrity.feature`, so the capabilities
  `prose-consolidation` and `instruction-integrity` are undeclared.
- **Evidence.** `givn check review --change strengthen-ise-specification`
  reports:
  `delta capability 'instruction-integrity' is not declared in the Capability
  Routing table` and `delta capability 'prose-consolidation' is not declared
  in the Capability Routing table`.
- **Small fix.** Add two rows:
  `prose-consolidation` -> `EXTEND prose-consolidation` (the permanent
  `givn/specs/create-change/prose-consolidation.feature` exists and the delta
  sits in `specs/create-change/`), and `instruction-integrity` ->
  `NEW in fragments` (the permanent feature is under `givn/specs/fragments/`,
  which `resolve_permanent_usecase` skips). Re-run design-review with B4.
- **Classification.** plan-artifact defect; blocking.

## Observations (not blockers)

- **O1 - internal retired vocabulary.** `src/ise/mod.rs` still calls itself
  "Clean-room trial preparation and verification"; the report `Counts` fields
  are `retained`, `kept_source`, `replaced`, `outside_declaration`, and
  `not_replaced`; `Partition` and the test helpers keep the same nouns. The
  user-facing report and the delta prose use the settled vocabulary, so this is
  a naming-consistency item for the next change, not a behavior defect.
- **O2 - rollup strictness is format-only.** The `src/ise/compare.rs` module
  doc says "every checksum and rollup must match", but `Manifest::read` checks
  only 64-hex format. The reference's strictness list also names format, and
  `compare` uses the maps rather than the rollups, so the doc overstates the
  check. Correct the sentence or recompute the rollups; not blocking.
- **O3 - singular report noun.** A rebuild with one End-to-end file prints
  `End-to-end tests: not copied (1 file)`; the stated line is
  `(<n> files)` and no scenario pins the singular. Either always print `files`
  or state the singular rule in the reference and add a scenario.
- **O4 - no change-scoped coverage receipt.** The closeout does not record a
  `./measure-coverage.sh` run; the archive receipt carries the merged
  measurement. Static inspection finds the new branches exercised by the delta
  and e2e scenarios, the write-mismatch branch by its unit test, and the
  remaining I/O error paths legitimately hard to inject. Not a defect at review
  time.
- **O5 - old e2e binding becomes dead.** `report_abbreviates_the_digests` in
  `e2e/tests/steps/clean_room_preparation_e2e_steps.rs:92-110` matches the
  permanent scenario the delta removes; after the merge it is unused. Remove it
  with B1's fix.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 introduction-and-goals | `docs/arc42/01-introduction-and-goals.md` | No change | D12 | No goal task | No goal or stakeholder edit in the diff | Yes |
| 02 architecture-constraints | `docs/arc42/02-architecture-constraints.md` | TC-46/47/49/51 updated | D5-D11 | Batches 1-11 | Constraints name `rebuild-*`, `compare`, and `ise-manifest/v1` | Yes |
| 03 context-and-scope | `docs/arc42/03-context-and-scope.md` | Verifier and command rows updated | D5, D9 | Batch 1, 9-11 | Rows name the three commands and one `-`/`+` line per path | Yes |
| 04 solution-strategy | `docs/arc42/04-solution-strategy.md` | Clean-room row updated | Scope, D5-D9 | Batches 1-11 | Row names the two rebuild commands, the manifest, and compare | Yes |
| 05 building-block-view | `docs/arc42/05-building-block-view.md` | `cli::ise` and `ise` rows updated | Architecture Impact | Batches 1-11 | Rows name the three commands, the partition, and the manifest | Yes |
| 06 runtime-view | `docs/arc42/06-runtime-view.md` | §6.21 rewritten | D5-D10 | Batches 1-11 | §6.21 names the four declaration cases and the comparison steps | Yes |
| 07 deployment-view | `docs/arc42/07-deployment-view.md` | No change | No deployment change | None | No packaging or runtime change | Yes |
| 08 crosscutting-concepts | `docs/arc42/08-crosscutting-concepts.md` | §8.29 states the checksum and enumeration rules | D6-D8 | Batches 5-7 | §8.29 carries the manifest scope, byte form, and enumeration | Yes |
| 09 architecture-decisions | `docs/arc42/09-architecture-decisions.md` | ADR-076 amended | D5-D7 | Setup, batches 6-7 | ADR-076 and the chapter-09 register name the amendment | Yes |
| 10 quality-requirements | `docs/arc42/10-quality-requirements.md` | Q-127 added | D6 | Batch 7 | Q-127 names the second-implementation byte proof | Yes |
| 11 risks-and-technical-debt | `docs/arc42/11-risks-and-technical-debt.md` | D-9 extended; R-160/161/162 added | Risks | Setup, batches 7-13 | R-160/161/162 and D-9 are current | Yes |
| 12 glossary | `docs/arc42/12-glossary.md` | Links the standalone glossary | Scope | Setup | `docs/glossary.md` adds `Spec copy` and `Checksum`; the retired trial terms are gone | Yes |

All twelve chapter files exist with current content. The change-level
`arc42.md` assessment matches the durable chapters and the implementation. The
`docs/domain-model.md` no-op is correct: the command surface is not a structural
model node.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

- **`Spec copy`, `Manifest`, `Checksum`, `Specification file`, `Source code
  kept`, `Source code removed`, `End-to-end (E2E) tests`, `compare`** are the
  settled vocabulary; they appear in the reference, the served `ise`
  instruction, the delta use case, and the user-facing report.
- **Retired terms** (`Trial record`, `Trial directory`, `Trial E2E digest`,
  `Frozen package`, `receipt`, `--full`, `--path`) are removed from the
  command surface and the prose this change touches; the ADR historical notes
  keep them as history.
- **O1** records the remaining internal code names; no durable document uses
  them for the current surface.
- The structural model is unchanged, as the design states; the rendered
  diagram and the boundaries/relationships tables still describe the same
  consumers.

## Use-case and Persona conformance

The delta `prepare-an-ise-test` use case keeps one goal, one trigger, the
Developer primary actor, the Independent verifier supporting actor, the five
preconditions, the five-step main flow, the refusal extensions, the rules, and
the two capabilities. The `declare-the-specification` use case keeps its one
unchanged inventory entry. No new capability or use case is introduced;
`clean-room-preparation` and `trial-input-verification` keep their identifiers
as D2 decides.

PERSONAS: developer--primary-user

The confirmed personas are `developer--primary-user` (reads the reproducible
manifest and the named actions), `agent--technical-role` (reads the stated
bytes and the four rule sentences), and `givn-maintainer--owner` (gains the
scope cut and the compatibility proof). No persona biography enters Gherkin.

## Visual review

The design declares `Interface: none`; the changed surface is the CLI report
and the comparison output. The evidence is the two committed terminal
transcripts named above. The rebuild transcript shows the eleven fixed lines
and the manifest path; the comparison transcript shows
`Specification files: match` and `End-to-end tests: match`. The evidence is
complete for both delta `@e2e` scenarios. B1 is a contract finding about the
checksum line, not a missing visual artifact.

VISUAL-REVIEW: PASS

## Coverage classification

- **Bucket 1 (dead code) - delete.** `src/ise/record.rs` and
  `src/ise/verify.rs` are deleted; `E2eBoundary::contains` and its test are
  deleted. No production dead branch was found. O5 and O1 are test/name
  cleanups, not production behavior.
- **Bucket 2 (missing coverage) - add the test.** B2 (deleted declaration),
  B5 (retired record shape), and B1 (no-ellipsis line) are the missing or
  non-discriminating scenarios; the fixes above add or correct them. The
  `Comparison refuses a manifest whose scope is not ise-manifest/v1` scenario
  is not discriminating the retired shape.
- **Bucket 3 (legitimately hard to test).** `resolved_target`'s
  no-existing-ancestor error path cannot occur on a mounted unix filesystem
  because `/` exists; the `copy_paths` and `local_checksum` read-error paths
  are OS I/O failures. Both are defensive error paths, not external-input
  validation.
- **Redundant unit tests.** The `partition.rs` set-arithmetic unit tests and
  the `write_trial` mismatch test cover branches whole fixture projects cannot
  distinguish; keep them.
- **Measured evidence.** No change-scoped coverage receipt exists; the archive
  receipt will carry the merged measurement. Review classifies from
  inspection and the executed scopes above.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Preparation refuses when the declaration is missing` | `Preparation refuses a repository link in the copy set` | boundary |
| `Preparation refuses when the declaration is missing` | `Preparation refuses a nested repository marker in the copy set` | boundary |
| `Preparation refuses a malformed declaration` | `Preparation refuses a malformed declaration at the selected revision` | boundary |
| `Preparation refuses while the ise addon is off` | `Preparation refuses while architecture principles are off` | boundary |
| `Preparation refuses a path outside every source root` | `Preparation refuses a symlink whose target resolves outside the spec copy` | boundary |
| `Preparation refuses a repository link in the copy set` | `Preparation refuses a nested repository marker in the copy set` | boundary |
| `Preparation does not expand a glob-shaped path` | `Preparation counts the files left in a removed path's directory` | boundary |
| `A named path absent from the revision removes nothing` | `A role that matches the declaration counts it once` | boundary |
| `Preparation refuses a directory that contains the declaration` | `Preparation refuses a component path that names the declaration` | boundary |
| `Preparation refuses a revision that does not record the declaration` | `Preparation refuses a declaration the revision records but cannot read` | boundary |
| `Preparation refuses a path matched by a Specification role` | `Preparation refuses a directory path that contains a Specification path` | boundary |
| `An End-to-end role that covers the declaration keeps it among the Specification files` | `A whole-product rebuild keeps the declaration under a source root` | boundary |
| `A project-root End-to-end entry covers the whole tracked tree` | `An End-to-end entry that overlaps a source root wins over the removal` | boundary |
| `Preparation refuses a manifest inside the spec copy` | `Preparation refuses a manifest that is an ancestor of the spec copy` | boundary |
| `Comparison names a changed Specification file in the copy` | `Comparison names a missing Specification file in the copy` | boundary |
| `Comparison names a changed Specification file in the copy` | `Comparison names an added Specification file in the checkout` | boundary |
| `Comparison names a missing Specification file in the copy` | `Comparison names an added Specification file in the checkout` | boundary |
| `Comparison reports no difference after the agent adds Source code` | `Comparison reports no difference after a kept Source code file is edited` | boundary |
| `Comparison refuses a file added under a declared Specification path` | `Comparison refuses a file added under an End-to-end role` | boundary |
| `Comparison refuses a deleted kept Source code file` | `Comparison refuses while the ise addon is off` | boundary |
| `Comparison refuses a deleted kept Source code file` | `Comparison refuses while architecture principles are off` | boundary |
| `Comparison names an @e2e scenario changed in the checkout` | `Comparison names a content-only change to an @e2e feature in the checkout` | boundary |
| `Comparison names an @e2e scenario changed in the checkout` | `Comparison names an added End-to-end file in the checkout` | boundary |
| `Comparison names an @e2e scenario changed in the checkout` | `Comparison names an added @e2e scenario file in the checkout` | boundary |
| `Comparison names an @e2e scenario changed in the checkout` | `Comparison names a missing Specification file in the checkout` | boundary |
| `Comparison names an @e2e scenario changed in the checkout` | `Comparison names a missing End-to-end test in the checkout` | boundary |
| `Comparison names an @e2e scenario changed in the checkout` | `Comparison names a missing @e2e feature in the checkout` | boundary |
| `Comparison names a content-only change to an @e2e feature in the checkout` | `Comparison names an added End-to-end file in the checkout` | boundary |
| `Comparison names a content-only change to an @e2e feature in the checkout` | `Comparison names an added @e2e scenario file in the checkout` | boundary |
| `Comparison names a content-only change to an @e2e feature in the checkout` | `Comparison names a missing Specification file in the checkout` | boundary |
| `Comparison names a content-only change to an @e2e feature in the checkout` | `Comparison names a missing End-to-end test in the checkout` | boundary |
| `Comparison names a content-only change to an @e2e feature in the checkout` | `Comparison names a missing @e2e feature in the checkout` | boundary |
| `Comparison names an added End-to-end file in the checkout` | `Comparison names an added @e2e scenario file in the checkout` | boundary |
| `Comparison names an added End-to-end file in the checkout` | `Comparison names a missing Specification file in the checkout` | boundary |
| `Comparison names an added End-to-end file in the checkout` | `Comparison names a missing End-to-end test in the checkout` | boundary |
| `Comparison names an added End-to-end file in the checkout` | `Comparison names a missing @e2e feature in the checkout` | boundary |
| `Comparison names an added @e2e scenario file in the checkout` | `Comparison names a missing Specification file in the checkout` | boundary |
| `Comparison names an added @e2e scenario file in the checkout` | `Comparison names a missing End-to-end test in the checkout` | boundary |
| `Comparison names an added @e2e scenario file in the checkout` | `Comparison names a missing @e2e feature in the checkout` | boundary |
| `Comparison names a missing Specification file in the checkout` | `Comparison names a missing End-to-end test in the checkout` | boundary |
| `Comparison names a missing Specification file in the checkout` | `Comparison names a missing @e2e feature in the checkout` | boundary |
| `Comparison names a missing End-to-end test in the checkout` | `Comparison names a missing @e2e feature in the checkout` | boundary |
| `Comparison refuses while the ise addon is off` | `Comparison refuses while architecture principles are off` | boundary |

The 43 `[SHAPE]` findings are distinct fixtures and observable outcomes; each
pair is retained. No `[SUBST]` finding is a merge candidate.

## Split-or-keep

| Scenario | Decision |
|---|---|

No delta scenario exceeds the 14-step threshold; no split is required.

## README impact

The README command table under `## Configuration` was edited in the plan
commit to name `rebuild-all`, `rebuild-component`, and `compare`.

README-IMPACT: updated - Configuration

## Sign-off

- [x] The task gate passes at `6e55ba9a` (98 regular scenarios, 2 e2e, 0
      failed, 0 skipped).
- [x] The delta non-e2e and e2e scopes pass; the permanent e2e tier passes
      119 scenarios and 781 steps.
- [x] Eight mutation probes discriminate the checksum, partition, declaration,
      prefix, compare, checkout-added, manifest, and served-text rules.
- [x] The canonical checksum values were independently reproduced from the
      stated bytes and rule (`specification_checksum 35444598...`,
      `end_to_end_checksum 00580162...`, per-path values match).
- [x] The retired surface is replaced in the merged corpus; the two
      cross-capability scenarios are the only extra delta entries outside the
      two prepare features and they are `@givn.modified`.
- [ ] The design review is current with `design.md` (B4).
- [ ] The report checksum line matches the stated 12 hex characters (B1).
- [ ] A deleted declaration is reported as `missing` (B2).
- [ ] The use case and the reference state one docstring rule (B3).
- [ ] A retired `ise-trial/v1` record is refused by name (B5).
- [ ] Every recorded scenario commit carries its assertion (B6).
- [ ] The Capability Routing table declares `prose-consolidation` and
      `instruction-integrity` (B7).

The blockers above are repairable without redesign. After the fixes, re-run
the affected scenarios, `givn check tasks`, and this review; the verdict can
then flip.

REVIEW: FAIL
