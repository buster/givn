# Design Review: strengthen-ise-specification

Review kind: independent
Author: strengthen-ise-specification author (omp)
Reviewer context: omp independent design-review context (deepseek-flash, tenth review, 2026-11-17)

Reviewed revision: `ec4b9d71f89b79811cfba80a9467524a886154b4` (HEAD), with `design.md` at blob `88aca6d1500e401c05b2372c9f534c4e269c3dec`. The docs fix is `22be58d4`; `ec4b9d71` restores the six archived screenshots and does not change the change artifacts. The earlier delta review at `833b6f4d` recorded FAIL for D1. This review confirms D1 is closed and records the earlier findings.

## Earlier findings

| Finding | Disposition | Evidence |
| --- | --- | --- |
| D1, chapter 06 still refused a copy without a declaration | **CLOSED** | `docs/arc42/06-runtime-view.md:1090-1093` step 4 now says the declaration is loaded when present, that an absent or unreadable one contributes no roles, that the added-file refusal is skipped, and that the recorded declaration checksum reports the file `missing` or `changed`. `docs/reference.md:697-699` states the same rule in "Declaration in comparison". `design.md` D9 states the same rule. The old sentence "A copy without one refuses with the loader's missing-declaration message" is absent from `docs/`. |
| B4, the design review predated the design it reviewed | **CLOSED** | This review is at `ec4b9d71`; the design.md blob `88aca6d` is the reviewed text. The counts and the two cross-capability rows match the current delta, so the design review no longer predates the design. |
| O1, the Corpus Rebinding table omits the declaration title as a row | Open observation | `design.md:322` records the declaration feature, its one modified title, and six unmodified scenarios; the delta feature names the title. Not blocking. |
| O2, chapter 06 step 2 names only the manifest for a wrong scope | Open observation | The retired-record scenario also asserts the recorded `ise-trial/v1` and the expected `ise-manifest/v1` tokens; the reference says "naming the manifest and the expected value". The chapter is less specific, not contradictory. |
| O3, the unreadable copy declaration has no scenario | Open observation | The optional rule covers absent and unreadable; the absent case is pinned, and a malformed declaration is a checksum difference. The unreadable case remains untested. Not blocking. |

## Counts

| Scope | Removed | Modified | Added | Total |
| --- | --- | --- | --- | --- |
| `clean-room-preparation` | 29 | 27 | 39 | 95 |
| `trial-input-verification` | 27 | 0 | 33 | 60 |
| `specification-declaration` | 0 | 1 | 0 | 1 |
| `prose-consolidation` | 0 | 1 | 0 | 1 |
| `instruction-integrity` | 0 | 1 | 0 | 1 |
| **Delta** | **56** | **30** | **72** | **158** |

- **Tags match the claim.** `givn check review` reports `net delta: 72 added, 30 modified, 56 removed`; the delta files contain 72 `@givn.added`, 30 `@givn.modified`, and 56 `@givn.removed` tags.
- **`@e2e` count is 2.** Both are `@givn.added`: `clean-room-preparation.feature:329` and `trial-input-verification.feature:113`. The two permanent `@e2e` scenarios are replaced through the delta.
- **Working scenarios.** `givn check review` reports 102 scenario commits; the plan's 102 working scenarios (72 added + 30 modified) are implemented. `givn lint --change strengthen-ise-specification` is clean at this revision with 0 `@wip`.

## D1 closure check

| Artifact | Current rule |
| --- | --- |
| `docs/arc42/06-runtime-view.md:1090-1093` | Load the copy's declaration when present. A copy without one, or one whose declaration cannot be read, contributes no roles: the added-file refusal is skipped, and the recorded declaration checksum reports the file `missing` or `changed` like any other Specification file. |
| `docs/reference.md:697-699` | A copy whose declaration is absent or unreadable contributes no roles: the added-file refusal is skipped, and the recorded declaration path is `missing` or `changed` like any other Specification file. |
| `design.md` D9 | A copy whose declaration is absent or unreadable contributes no roles: the added-file refusal is skipped, and the recorded declaration path is `missing` or `changed` like any other Specification file. |
| `trial-input-verification.feature:142-149` | Delete `givn/specification.yaml`; assert `Specification files: 1 missing`, `- givn/specification.yaml`, `End-to-end tests: match`, and exit 1. |

The three rules agree, and the scenario asserts the outcome. The old refusal sentence is gone from the tree.

## No new contradiction

| Rule | Verdict | Check |
| --- | --- | --- |
| Optional copy declaration | PASS | The new sentence qualifies the existing "the copy's declaration derives the Specification paths the added-file refusal tests" rule. With no declaration, there are no roles, so the added-file refusal is skipped; chapter 06 step 5 remains conditional on a declared path. The reference's earlier "an edited or deleted declaration is a difference naming the path" still holds because the recorded path is compared and reported. |
| Retired `ise-trial/v1` refusal | PASS | The added scenario `Comparison refuses a retired ise-trial/v1 record by scope` asserts both scope tokens; the reference, chapter 06 step 2, and proposal D4 remain consistent. |
| Ellipsis-free checksum line | PASS | D6, the reference report example, `clean-room-preparation.feature:337`, and the pre-merge translation binding all use 12 lowercase hex characters with no ellipsis. |

The delta use case lines 105-107 keep the general "the copy's declaration derives the Specification paths the added-file refusal tests" rule. That statement remains true when the declaration is present; the reference, design, and chapter now state the absent and unreadable case. No contradiction.

## Corpus rebinding

| Check | Result |
| --- | --- |
| Permanent prepare scenarios | 56 `clean-room-preparation` (29 removed + 27 modified) and 27 `trial-input-verification` (27 removed); every title has exactly one delta op. |
| Table rows | 85 rows: 56 removed + 29 modified (27 prepare + 2 cross-capability). The two cross-capability rows are `design.md:411-412`. |
| Declaration feature | 7 scenarios including one `Scenario Outline`: one modified title `An unreadable declaration is refused by name` and six unmodified; `design.md:322` records the feature and count. |
| Collisions | No added title collides with a permanent title; no permanent prepare title is missing an op. |

## Required branch dispositions

| Branch | Disposition |
| --- | --- |
| Scope | PASS: the four added entries are within the proposal's scope; the two cross-capability entries are vocabulary-only served-text changes and the two review-blocker entries implement B2 and B5. |
| Tech choices | PASS: no new stack; the compare reader change stays in `src/ise/compare.rs` and `src/ise/manifest.rs`. |
| Missing scenarios | PASS: the four entries close B2, B5, and A17; the optional declaration is now stated in the design, reference, and chapter. |
| Testability | PASS: each added scenario asserts concrete counts, tokens, or exit codes; the missing-declaration scenario fails a refusal-first reader, and the retired-record scenario fails a serde-first reader. |
| E2E fidelity | PASS: the delta adds no new `@e2e` action; the two `@e2e` scenarios are the planned replacements, and the CLI subprocess remains the real interface. |
| Interaction Coverage | PASS: no new inventory entry; the two `@e2e` rows are already in the matrix. |
| Risk | PASS: no new risk; R-162 is unchanged. |
| Use-case and Persona context | PASS: the two cross-capability scenarios are served-text assertions under declared capabilities; personas are unchanged. |
| Decision ledger | PASS: D4, D7, D8, D9, and D13 cover the entries; D9 now states the optional copy declaration. |
| Contracts and components | PASS: no new contract or component; the `SpecCopy` contract is unchanged. |
| Decision timing | PASS: no new carried decision; D3 is unchanged. |
| change strategy | PASS: D12 remains `incremental modification`; no Ise Test runs. |
| Visual design contract | PASS: `Interface: none`; the changed surface is CLI text, and the `@e2e` evidence is a terminal transcript. |
| Deferrals | PASS: none. |
| Persona disposition | PASS: the three named personas are unchanged; no biography enters Gherkin. |
| Interface classification | PASS: the CLI is not a browser UI; the driver is a subprocess. |
| ADR qualification | PASS: no new ADR; ADR-076 is amended and the cheaper home is recorded. |
| Architecture documentation (arc42) | PASS: chapter 06 step 4 now matches D9 and the reference; the old refusal text is absent, and the other chapters and the Yes/No walk are unchanged. |
| Ubiquitous language | PASS: `spec copy`, `manifest`, `Specification files`, `Source code kept`, `Source code removed`, `End-to-end tests`, and `declaration` are used consistently in the new titles and the new sentence. |
| Structural model | PASS: `docs/domain-model.md` is unchanged; the command surface is not a domain entity. |

## Verdict

D1 is closed. The chapter 06 runtime rule, the reference's "Declaration in comparison", and D9 state one rule: an absent or unreadable copy declaration contributes no roles, the added-file refusal is skipped, and the recorded declaration path is reported `missing` or `changed`. The added scenario asserts that outcome. The counts remain 158 = 72 added + 30 modified + 56 removed, with two `@e2e` additions, and `givn lint --change` is clean.

No blocking finding remains. The task list may be signed off.

DESIGN-REVIEW: PASS
