# Review: strengthen-ise-specification

Review kind: independent
Author: strengthen-ise-specification author (omp)
Reviewer context: omp independent design-review context (deepseek-flash, tenth review, 2026-11-17)

Reviewed revision: `673da7dde218650211e0c97a73051a5d561f826e` (HEAD), with
`design.md` at blob `88aca6d1500e401c05b2372c9f534c4e269c3dec`, unchanged since
the design review at `ec4b9d71`.

## Scope and result

The change rewrites the clean-room command surface, cuts the trial record to the
`ise-manifest/v1` manifest, states the canonical checksum bytes, and adds the
`compare` report. This review re-read the FAIL findings, the repaired proposal,
design, delta specs, tasks, design review, arc42 assessment, durable chapters,
reference, served `ise` instruction, and the implementation `76159193..HEAD`.
It re-ran the task gate, the delta non-e2e scope, the delta e2e scope, the
permanent e2e tier, and three mutation probes on the repaired rules.

All seven blockers are closed. The checksum line prints 12 lowercase hex
characters with no ellipsis; an absent or unreadable copy declaration
contributes no roles and the recorded path is reported `missing` or `changed`;
the delta use case states the reference's docstring rule; the design review at
`ec4b9d71` covers the 158-scenario delta and the current `design.md` blob; a
retired `ise-trial/v1` record is refused by name; batch 12 records a commit
carrying the regression assertion; and the routing table declares all five
delta capabilities. `REVIEW: PASS` follows the evidence.

## Implementation evidence check

### `@e2e` tag integrity

- The delta has 158 scenarios: 56 `@givn.removed`, 30 `@givn.modified`, 72
  `@givn.added` (102 executable after the 56 retired placeholders are removed).
  The two repair scenarios (`Comparison names a missing declaration in the
  copy`, `Comparison refuses a retired ise-trial/v1 record by scope`) are
  regular `@givn.added` entries.
- Two delta scenarios are `@e2e`: `Developer rebuilds the whole product from its
  Specification` and `Verifier compares the spec copy with the manifest`. Both
  replace permanent `@e2e` scenarios named by `@givn.removed`; no `@e2e` tag
  was silently removed from a planned scenario. The permanent declaration
  `@e2e` scenario is unchanged.
- No delta scenario carries `@wip`.

### Step bodies

`0` empty or trivial step bodies found across the seven changed step files:
`tests/steps/clean_room_preparation_steps.rs` (261 bindings),
`tests/steps/trial_input_verification_steps.rs` (99),
`tests/steps/prose_consolidation_steps.rs` (75),
`tests/steps/instruction_integrity_steps.rs` (260),
`tests/steps/specification_declaration_steps.rs` (10),
`e2e/tests/steps/clean_room_preparation_e2e_steps.rs` (22), and
`e2e/tests/steps/trial_input_verification_e2e_steps.rs` (7). No `todo!()`,
`unimplemented!()`, or stub body remains.

### Checked scenarios and commits

All 237 tasks are checked; the delta holds 102 working scenarios (72 added,
30 modified). The recorded batch revisions are:

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
| declaration-boundary | `44168260` | exit-code-and-name assertion binding, tag removal |
| cross-capability-rebinding | `73bcd1bc0e` | two delta features, e2e fixture rebinding |
| review-blockers | `8c8ddbc7` | optional copy declaration, retired-scope pre-check, 12-hex line, docstring sentence, routing rows |

Every batch commit contains production code or a step definition/regression
test. `git show 44168260` adds the `the command exits 1 and names "<path>"`
binding in `tests/steps/specification_declaration_steps.rs` and the matching
`design.md` row; `givn check review` reports the scenario-commit check green
over 102 scenarios.

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
exports `GIVN_RESULT_SCOPE=e2e` before the e2e runner. The change receipt at
`673da7dd` records the regular scope (`100` passed, `0` failed, `0` skipped)
and the e2e scope (`2` passed, `0` failed, `0` skipped). The delta scopes pass
(`100` passed with `2` skipped under `not @e2e`; `2` passed for `@e2e`), and
the permanent e2e tier passes `119` scenarios and `781` steps.

### Design match

The runner, step-file layout, and framework match design.md. The report line
prints the stated 12 lowercase hex characters with no ellipsis, and the delta
use case states the reference's docstring rule. The e2e step reads the bare
value and asserts its length and lowercase hex, so a second implementation
that follows the stated rule passes the frozen suite.

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

`Spec gap outcome: implementation defect`. Before the repair, the as-built
behavior contradicted the merged Specification at B1, B2, and B5; B3 corrected
the delta use case to the rule the reference and the golden fixture already
state. One outcome was recorded before any fix.

## Probes and outcomes

The three repair probes run at `673da7dd` mutate the repaired rules. Every
probe was reverted with `git checkout -- <path>`; the working tree was clean
after each revert.

| # | Mutation | Scenario run | Observed result |
|---|---|---|---|
| P1 | `src/ise/manifest.rs`: `abbreviate` appends `…` | `Developer rebuilds the whole product from its Specification` (`@e2e`) | FAILED: `Specification checksum: "7d7a8df82f9f…" is not 12 hex characters` (length 15, expected 12) |
| P2 | `src/ise/compare.rs`: refuse a copy whose declaration is absent instead of treating it as no roles | `Comparison names a missing declaration in the copy` | FAILED: stdout `""` does not print `Specification files: 1 missing` |
| P3 | `src/ise/manifest.rs`: drop the `scope`/`content_scope` pre-check | `Comparison refuses a retired ise-trial/v1 record by scope` | FAILED: stderr `... does not match ise-manifest/v1: unknown field 'schema', expected one of ...` does not name `ise-trial/v1` |

The eight probes from the earlier review at `6e55ba9a` are prior-revision
evidence; their targets outside the repaired lines (`partition.rs`, the
`cli::ise` prefixes, the compare difference kinds, the checkout added paths,
the checksum format guard, and the served instruction) are unchanged by the
repair.

## Finding dispositions

Every blocker from the prior review is closed. The repair batch is
`8c8ddbc7` (`review-blockers`) except B4 (`design-review.md` at `673da7dd`)
and B6 (batch 12, `44168260`).

| Finding | Disposition | Evidence |
|---|---|---|
| B1 - the checksum report line carries an unstated ellipsis | **CLOSED** | `src/ise/manifest.rs:251` `abbreviate` returns `digest.chars().take(12)` with no suffix. `e2e/tests/steps/clean_room_preparation_e2e_steps.rs:374-393` reads the bare value and asserts 12 lowercase hex characters. `docs/reference.md:525`, chapter 06, and the delta scenario state the same rule. P1. |
| B2 - a deleted copy declaration is refused, not reported missing | **CLOSED** | `src/ise/compare.rs:89` treats the declaration as `Option`; `copy_declaration` returns `None` for an absent or unreadable file, so the added-file refusal is skipped and the recorded `checksums.specification` comparison reports the path `missing`. The added scenario deletes `givn/specification.yaml` and asserts `Specification files: 1 missing`, `- givn/specification.yaml`, `End-to-end tests: match`, exit 1. P2. |
| B3 - the delta use case states the opposite docstring rule | **CLOSED** | `specs/prepare-an-ise-test/usecase.md:81` now says the docstring loses the line break after the opening delimiter and the line break before the closing delimiter, matching `docs/reference.md:549` and `src/gherkin/content.rs`. The golden scenario still pins `"line one\n  indented\nline four"`. |
| B4 - the design review predates the design it reviews | **CLOSED** | `design-review.md` records `Reviewed revision: ec4b9d71`, `158 = 72 added + 30 modified + 56 removed`, and the two cross-capability rows. `design.md` blob `88aca6d...` is the reviewed blob and `git log ec4b9d71..HEAD -- design.md` is empty. `givn check review` reports `net delta: 72 added, 30 modified, 56 removed`. |
| B5 - the retired `ise-trial/v1` record is not refused by name | **CLOSED** | `src/ise/manifest.rs:52-61` `recorded_scope` reads `scope`/`content_scope` before deserialization and refuses a mismatched scope naming the recorded and expected values. The added scenario writes the full retired field set (`schema`, `content_scope`, `backend`, the digests, the maps) and asserts both scope tokens. P3. |
| B6 - batch 12 records a tag-only commit | **CLOSED** | `tasks.md` batch 12 records `44168260`; `git show 44168260` adds the `the command exits 1 and names "<path>"` binding in `tests/steps/specification_declaration_steps.rs` and the matching design row, beside the tag removal. |
| B7 - the Capability Routing table omits two delta capabilities | **CLOSED** | `proposal.md:160-161` adds `prose-consolidation` -> `EXTEND prose-consolidation` and `instruction-integrity` -> `EXTEND instruction-integrity`. `givn check review` reports both capabilities already owned and no routing refusal. The fragments capability is owned by `givn/specs/fragments`, so the gate refused `NEW in fragments` and accepted the `EXTEND` row. |

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
- **O5 - permanent e2e binding retires with its scenario.** The
  `report_abbreviates_the_digests` binding in
  `e2e/tests/steps/clean_room_preparation_e2e_steps.rs:91-116` matches the
  permanent scenario the delta removes. It now asserts the settled 12-hex line
  instead of the retired ellipsis. It stays until the archive merge removes the
  scenario, because deleting it now would leave that permanent e2e step
  unbound.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 introduction-and-goals | `docs/arc42/01-introduction-and-goals.md` | No change | D12 | No goal task | No goal or stakeholder edit in the diff | Yes |
| 02 architecture-constraints | `docs/arc42/02-architecture-constraints.md` | TC-46/47/49/51 updated | D5-D11 | Batches 1-11 | Constraints name `rebuild-*`, `compare`, and `ise-manifest/v1` | Yes |
| 03 context-and-scope | `docs/arc42/03-context-and-scope.md` | Verifier and command rows updated | D5, D9 | Batch 1, 9-11 | Rows name the three commands and one `-`/`+` line per path | Yes |
| 04 solution-strategy | `docs/arc42/04-solution-strategy.md` | Clean-room row updated | Scope, D5-D9 | Batches 1-11 | Row names the two rebuild commands, the manifest, and compare | Yes |
| 05 building-block-view | `docs/arc42/05-building-block-view.md` | `cli::ise` and `ise` rows updated | Architecture Impact | Batches 1-11 | Rows name the three commands, the partition, and the manifest | Yes |
| 06 runtime-view | `docs/arc42/06-runtime-view.md` | §6.21 rewritten | D5-D10 | Batches 1-11, 14 | §6.21 states the four declaration cases, the optional copy declaration, and the comparison steps | Yes |
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
`Specification files: match` and `End-to-end tests: match`. The rebuild
transcript prints `Specification checksum: 7d7a8df82f9f` and
`End-to-end checksum: f41d020038de`, both 12 lowercase hex characters with no
ellipsis. The evidence is complete for both delta `@e2e` scenarios.

VISUAL-REVIEW: PASS

## Coverage classification

- **Bucket 1 (dead code) - delete.** `src/ise/record.rs` and
  `src/ise/verify.rs` are deleted; `E2eBoundary::contains` and its test are
  deleted. No production dead branch was found. O1 is a naming cleanup; O5 is
  the transitional permanent binding whose scenario the delta removes.
- **Bucket 2 (missing coverage) - added.** B2's `Comparison names a missing
  declaration in the copy` and B5's `Comparison refuses a retired
  ise-trial/v1 record by scope` are added, and B1's e2e assertion reads the
  bare 12-hex value. Probes P1-P3 confirm each assertion discriminates. The
  new-shape `Comparison refuses a manifest whose scope is not ise-manifest/v1`
  stays for the `scope` mismatch; the retired-field-set scenario covers the
  old shape.
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

- [x] The task gate passes at `673da7dd` (receipt: 100 regular scenarios, 2
      e2e, 0 failed, 0 skipped).
- [x] The delta non-e2e scope passes (100 passed, 2 skipped); the delta e2e
      scope passes (2 passed, 0 failed); the permanent e2e tier passes 119
      scenarios and 781 steps.
- [x] Three mutation probes discriminate the repaired rules: the ellipsis line
      (P1), the optional copy declaration (P2), and the retired-record scope
      check (P3).
- [x] The canonical checksum values are pinned by
      `Preparation records the canonical checksums for a pinned fixture` and
      `Preparation records the canonical checksums for a richer feature`
      (`specification_checksum 35444598...`, `end_to_end_checksum 00580162...`),
      which pass in the delta scope.
- [x] The retired surface is replaced in the merged corpus; the two
      cross-capability scenarios are the only extra delta entries outside the
      two prepare features and they are `@givn.modified`.
- [x] The design review is current with `design.md` at `ec4b9d71` and covers
      the 158-scenario delta (B4).
- [x] The report checksum line is the stated 12 lowercase hex characters with
      no ellipsis (B1).
- [x] A deleted declaration is reported as `missing` (B2).
- [x] The use case and the reference state one docstring rule (B3).
- [x] A retired `ise-trial/v1` record is refused by name (B5).
- [x] Every recorded scenario commit carries its assertion (B6).
- [x] The Capability Routing table declares `prose-consolidation` and
      `instruction-integrity` (B7).

`givn check review --change strengthen-ise-specification` passes the run
declaration, lint, overlap dispositions, capability ownership, scenario
commits, and receipt checks with this verdict.

REVIEW: PASS
