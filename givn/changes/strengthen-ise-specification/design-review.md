# Design Review: strengthen-ise-specification

Review kind: independent
Author: strengthen-ise-specification author (omp)
Reviewer context: omp independent design-review context (deepseek-flash, delta re-review, 2026-11-17)

Reviewed revision: `833b6f4d8c32fea37a59e951d9331938bebb0de3` (HEAD), with `design.md` at blob `59e6bcb161a38e0091ab3576615794d686605f70`. The earlier review at `aef08fab` recorded PASS for 154 scenarios. This delta review covers the design edits after that PASS: the two cross-capability scenarios (`73bcd1bc`), the two review-blocker scenarios and the count correction (`8c8ddbc7`, `d05eaea7`), and the batch 13/14 map in `tasks.md`.

## Earlier review

The earlier PASS covered the command surface, the manifest cut, the checksum rule, the four prepare refusals, the enumeration scenarios, and the both-side drift rule. This review does not reopen those decisions; it re-checks the counts, the four added entries, the two cross-capability rows, and the three compare rules named by the review finding B4.

## Counts

| Scope | Removed | Modified | Added | Total |
| --- | --- | --- | --- | --- |
| `clean-room-preparation` | 29 | 27 | 39 | 95 |
| `trial-input-verification` | 27 | 0 | 33 | 60 |
| `specification-declaration` | 0 | 1 | 0 | 1 |
| `prose-consolidation` | 0 | 1 | 0 | 1 |
| `instruction-integrity` | 0 | 1 | 0 | 1 |
| **Delta** | **56** | **30** | **72** | **158** |

- **Tags match the design.** `givn check review` reports `net delta: 72 added, 30 modified, 56 removed`; `design.md:322` states the same totals and the 39 + 33 added split. The corrected sentence no longer calls the two review-blocker scenarios cross-capability entries, and the 72 added count is correct because both new scenarios are `@givn.added`.
- **`@e2e` count is 2.** Both are `@givn.added`: `clean-room-preparation.feature:329` and `trial-input-verification.feature:113`. The two permanent `@e2e` scenarios are replaced through the delta; no scenario loses its tag.
- **Working scenarios.** `givn check review` reports 102 scenario commits; the plan's 102 working scenarios (72 added + 30 modified) are implemented. `givn lint --change strengthen-ise-specification` is clean at this revision with 0 `@wip`, so the earlier review's 98 `@wip` composition is historical.

## Added scenarios

| Scenario | Decision | Contract | Discriminating fixture |
| --- | --- | --- | --- |
| `A project-root End-to-end entry covers the whole tracked tree` | D8 | `docs/reference.md` Report and End-to-end enumeration | `e2e: ["."]`, `source: ["src/**"]`, `src/kept.rs`, `notes.md`; asserts 2 E2E files, 0 removed, and a copy holding only the declaration. A reader that treats `.` as a package path with no `Cargo.toml` removes `src/kept.rs` and fails. |
| `An End-to-end entry that overlaps a source root wins over the removal` | D8 | `docs/reference.md` Report and End-to-end enumeration | `e2e: ["tests/e2e"]`, `source: ["tests/**"]`, `tests/e2e/runner.rs`, `tests/unit/helper.rs`; asserts 1 E2E file and 1 removed. A reader that removes or copies the E2E match reports 2 removed or 0 E2E files and fails. |
| `Comparison names a missing declaration in the copy` | D9 for the `missing` rule | `docs/reference.md` Compare and Declaration in comparison | Prepared copy; delete `givn/specification.yaml`; asserts `Specification files: 1 missing`, `- givn/specification.yaml`, `End-to-end tests: match`, and exit 1. A reader that refuses the copy before the report fails. The durable contract is contradicted by chapter 06 (D1). |
| `Comparison refuses a retired ise-trial/v1 record by scope` | D4, D7 | proposal D4 and `docs/reference.md` Manifest | Writes the retired field set (`schema`, `content_scope`, `backend`, the three digests, the three maps); asserts naming `ise-trial/v1` and `ise-manifest/v1` and no subject line. A serde-first reader reports `unknown field 'schema'` and fails. |

The two cross-capability entries are `@givn.modified`, not added:

| Permanent title | Location | Delta assertion |
| --- | --- | --- |
| `Served ise guidance states that the trial always carries the declaration` | `givn/specs/create-change/prose-consolidation.feature:173` | `specs/create-change/prose-consolidation.feature` asserts `The spec copy always carries the declaration itself` and `implicit Specification entry, not a sixth role`. |
| `The ise instruction says declared retained paths survive archiving` | `givn/specs/fragments/instruction-integrity.feature:414` | `specs/fragments/instruction-integrity.feature` asserts `a declared Specification path` survives `givn archive` and names the archive location. |

Both titles match their permanent scenario exactly. The proposal's Capability Routing table declares all five capabilities, including `prose-consolidation` and `instruction-integrity`; `givn check review` reports each capability owned and the overlap dispositions passed.

## Compare rule consistency

| Rule | Verdict | Evidence |
| --- | --- | --- |
| Optional copy declaration | **FAIL** | The added scenario and `docs/reference.md:693-695` report a deleted declaration as `missing`. `docs/arc42/06-runtime-view.md:1090` still refuses a copy without a declaration. See D1. |
| Retired `ise-trial/v1` refusal | PASS | The scenario asserts both scope tokens. `docs/reference.md:483-489` refuses a wrong scope naming the expected value; chapter 06 step 2 names `ise-trial/v1`; proposal D4 names the same output. |
| Ellipsis-free checksum line | PASS | D6 states 12 lowercase hex with no ellipsis. `docs/reference.md` Report example is `4f1c9a2b7d01`; `clean-room-preparation.feature:337` asserts 12 hex; the pre-merge translation binding asserts the same value. |

## Corpus rebinding

| Check | Result |
| --- | --- |
| Permanent prepare scenarios | 56 `clean-room-preparation` (29 removed + 27 modified) and 27 `trial-input-verification` (27 removed); every title has exactly one delta op. |
| Table rows | 85 rows: 56 removed + 29 modified (27 prepare + 2 cross-capability). The two cross-capability rows are `design.md:411-412`. |
| Declaration feature | 7 scenarios including one `Scenario Outline`: one modified title, `An unreadable declaration is refused by name` (`givn/specs/declare-the-specification/specification-declaration.feature:45`), and six unmodified. `design.md:322` records the feature, the one modified title, and the six unmodified scenarios. |
| Collisions | No added title collides with a permanent title, and no permanent prepare title is missing an op. |

The table does not carry a row for the declaration title; see O1. The table and the sentence together cover all 30 modified entries.

## Findings

### D1 (blocker) - chapter 06 still refuses a copy without a declaration

- **Location.** `docs/arc42/06-runtime-view.md:1090`, Compare step 4: "Load the copy's declaration. A copy without one refuses with the loader's missing-declaration message."
- **What is wrong.** This is the pre-B2 behaviour. The added scenario `Comparison names a missing declaration in the copy` (`specs/prepare-an-ise-test/trial-input-verification.feature:142-149`) deletes `givn/specification.yaml` and asserts `Specification files: 1 missing`, `- givn/specification.yaml`, `End-to-end tests: match`, and exit 1. `docs/reference.md:693-695` says an edited or deleted declaration is a difference naming the path. `tasks.md:1798-1801` records the intended rule: the copy declaration is optional for the added-file refusal, and an absent or unreadable declaration contributes no roles. D9 covers the `missing` report but not the optionality, and the reference's "Declaration in comparison" paragraph states the deleted-declaration difference without the no-roles rule.
- **Why it matters.** A second implementation that follows chapter 06 refuses the copy, fails the delta scenario, and leaves the merged architecture document contradicting the Specification and the reference. The B2 fix is currently represented only in the scenario, `tasks.md`, and the implementation.
- **Smallest fix.** Rewrite chapter 06 step 4 to "Load the copy's declaration when it parses; an absent or unreadable declaration contributes no roles to the added-file refusal, and the recorded path is reported `missing` or `changed`." Add the same sentence to D9 and to `docs/reference.md` "Declaration in comparison" (and the delta usecase lines 105-107 if the narrative is kept), so the design, reference, and chapter agree. No scenario change is needed; the scenario already pins the behaviour.

## Observations (not blockers)

- **O1 - the Corpus Rebinding table omits the declaration title as a row.** `design.md:322` records the feature, its one modified title, and six unmodified scenarios; the delta feature names the title. Add the row if the table is meant to be the complete list.
- **O2 - chapter 06 step 2 names only the manifest for a wrong scope.** The retired-record scenario also asserts the recorded `ise-trial/v1` and the expected `ise-manifest/v1` tokens, and the reference says "naming the manifest and the expected value". Align step 2's wording.
- **O3 - the unreadable copy declaration has no scenario.** The optional rule covers absent and unreadable; the absent case is pinned, and a malformed declaration is a checksum difference. The unreadable case remains untested. Not blocking for this delta.

## Required branch dispositions

| Branch | Disposition |
| --- | --- |
| Scope | PASS: the four entries are within the proposal's scope; the two cross-capability entries are vocabulary-only served-text changes and the two review-blocker entries implement B2 and B5. |
| Tech choices | PASS: no new stack; the compare reader change stays in `src/ise/compare.rs` and `src/ise/manifest.rs`. |
| Missing scenarios | PASS: the four entries close B2, B5, and A17. D1 is a documentation contradiction, not a missing scenario. |
| Testability | PASS: each added scenario asserts concrete counts, tokens, or exit codes; the missing-declaration scenario fails a refusal-first reader, and the retired-record scenario fails a serde-first reader. RED and GREEN evidence is recorded in tasks batch 14. |
| E2E fidelity | PASS: the delta adds no new `@e2e` action; the two `@e2e` scenarios are the planned replacements, and the CLI subprocess remains the real interface. |
| Interaction Coverage | PASS: no new inventory entry; the two `@e2e` rows are already in the matrix. |
| Risk | PASS: no new risk; R-162 is unchanged. |
| Use-case and Persona context | PASS: the two cross-capability scenarios are served-text assertions under declared capabilities; personas are unchanged. |
| Decision ledger | PASS: D4, D7, D8, D9, and D13 cover the new entries; D1 records the missing optionality sentence in D9. |
| Contracts and components | PASS: no new contract or component; the `SpecCopy` contract is unchanged. |
| Decision timing | PASS: no new carried decision; D3 is unchanged. |
| change strategy | PASS: D12 remains `incremental modification`; no Ise Test runs. |
| Visual design contract | PASS: `Interface: none`; the changed surface is CLI text, and the `@e2e` evidence is a terminal transcript. |
| Deferrals | PASS: none. |
| Persona disposition | PASS: the three named personas are unchanged; no biography enters Gherkin. |
| Interface classification | PASS: the CLI is not a browser UI; the driver is a subprocess. |
| ADR qualification | PASS: no new ADR; ADR-076 is amended and the cheaper home is recorded. |
| Architecture documentation (arc42) | **FAIL**: chapter 06 step 4 contradicts the added missing-declaration scenario and the reference. The other chapters and the recorded Yes/No walk are unchanged. |
| Ubiquitous language | PASS: `spec copy`, `manifest`, `Specification files`, `Source code kept`, `Source code removed`, `End-to-end tests`, and `declaration` are used consistently in the new titles. |
| Structural model | PASS: `docs/domain-model.md` is unchanged; the command surface is not a domain entity. |

## Verdict

The counts match the delta tags at `833b6f4d`: 158 scenarios, 72 added, 30 modified, 56 removed, with two `@e2e` additions. The four added scenarios have decisions, contracts, and discriminating fixtures. The two cross-capability rows are present in the Corpus Rebinding table, and the retired-record and ellipsis-free compare rules are consistent with the existing ones.

One blocking defect remains: `docs/arc42/06-runtime-view.md:1090` step 4 states the pre-B2 refusal, which contradicts the added `Comparison names a missing declaration in the copy` scenario and `docs/reference.md`. The design and reference also omit the explicit optional-copy-declaration sentence. The task list may not be signed off until chapter 06, D9, and the reference state the optional rule and this delta review is re-run at the then-current revision.

DESIGN-REVIEW: FAIL
