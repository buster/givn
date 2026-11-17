# Design Review: strengthen-ise-specification

Review kind: independent
Author: strengthen-ise-specification author (omp)
Reviewer context: omp independent design-review context (deepseek-flash, tenth review, 2026-11-17)

This is the tenth review. The ninth review blocked on B4 (the
revision-recorded declaration whose blob the backend cannot return had no
distinct observable output) and B5 (chapter 06 collapsed the four declaration
refusals), and carried A17 (the project-root and E2E/source-root enumeration
scenarios), A18 (both-side drift), and A19 (chapter 03's compare row). The
revision gives the blob-absent refusal its own token, adds the two enumeration
scenarios and the both-side scenario, and fixes the two chapters. B4, B5, and
A17-A19 are closed. No blocking finding remains.

## Verification performed

- **Corpus.** The delta holds 154 scenarios: 56 `@givn.removed`, 28
  `@givn.modified`, 70 `@givn.added`. Clean-room: 95 scenarios = 29 removed,
  27 modified, 39 added. Trial-input: 58 scenarios = 27 removed, 0 modified,
  31 added. Declaration: 1 modified. The `## Corpus Rebinding` table has 83
  rows, 56 removed and 27 modified; its sentence adds the declaration feature's
  one modified title and six unmodified scenarios (the feature holds seven
  scenarios). Every permanent clean-room (56) and trial (27) scenario has
  exactly one delta op; no added title collides with a permanent title. The
  `@wip` total is 98 (27 modified clean-room, 39 added clean-room, 31 added
  trial, 1 modified declaration). The delta carries one `@e2e` in clean-room and
  one in trial; the permanent declaration `@e2e` is unchanged. D13 and the
  `## Corpus Rebinding` counts match the tags.
- **Lint.** `givn lint --change strengthen-ise-specification` exits 2 with 98
  `[WIP]`, 43 `[SHAPE]`, and 116 `[SUBST]` findings. The `@wip` exit is the
  pre-implementation stage check. The declaration-refusal `[SHAPE]` pair
  (`Preparation refuses a revision that does not record the declaration` and
  `Preparation refuses a declaration the revision records but cannot read`) is
  a masked-literal false positive: the Then tokens are `does not record` and
  `returned no blob`, and merging them would delete the discrimination B4
  required. The enumeration pair (`A project-root End-to-end entry covers the
  whole tracked tree` and `An End-to-end entry that overlaps a source root wins
  over the removal`) is also a false positive: different `e2e` entries, source
  roots, committed files, and expected counts. No real merge candidate remains
  among the 43.
- **Refusals.** `docs/reference.md:459-462`, TC-49
  (`docs/arc42/02-architecture-constraints.md:58`), chapter 06 step 5
  (`docs/arc42/06-runtime-view.md:1025-1031`), D10 (`design.md:82`), the delta
  declaration rule (`specs/declare-the-specification/usecase.md:54`), and
  proposal statement 4 now give four distinct outputs: path + revision +
  `does not record`; path + revision + `returned no blob`; path + decoding
  error; path + loader's reason. The four scenarios assert those distinct
  tokens (`clean-room-preparation.feature:292-297`, `:299-304`, `:306-311`,
  `:313-319`), and the Step Definitions table binds each phrase with the note
  that the other phrase must not satisfy it.
- **Checksums.** The revision adds scenarios only; it does not change the
  fixture bytes, the checksum rule, or the `## Independent Second
  Implementation` values. The golden scenarios' pinned 64-hex values still
  match that section.
- **Arc42.** The independent 12-row walk is 01 No, 02 Yes, 03 Yes, 04 Yes, 05
  Yes, 06 Yes, 07 No, 08 Yes, 09 Yes, 10 Yes, 11 Yes, 12 Yes; it matches
  `arc42.md`. All 12 chapter files exist with real content and no ASCII art.
  Chapter 06's declaration step names all four cases; chapter 03's compare row
  names the `-` line per changed or missing path and the `+` line per added
  path.
- **Testability.** The new blob-absent Then phrase is new, so it is unbound
  (strict runner fails) until its step is implemented; the design names the
  binding and the cross-assertion. The existing
  `preparation fails and names "givn/specification.yaml" and the revision`
  binding asserts `does not record` and cannot satisfy the new phrase.

## Ninth-review findings

| Finding | Disposition | Evidence |
|---|---|---|
| B4, the revision-recorded declaration whose blob is absent has no distinct output | **CLOSED** | `clean-room-preparation.feature:313-319` asserts `returned no blob`; the not-recorded scenario at `:292-297` asserts `does not record`; `docs/reference.md:459-462` rows 1 and 4 are distinct; TC-49, D10, the delta rule, and proposal statement 4 name `returned no blob`; the Step Definitions table binds both phrases. A reader that prints either sentence fails the other scenario. |
| B5, chapter 06 collapses the four declaration refusals | **CLOSED** | `docs/arc42/06-runtime-view.md:1025-1031` step 5 names the not-recorded, no-blob, non-UTF-8, and malformed cases with their distinct outputs. |
| A17, the project-root `e2e` entry and the E2E/source-root overlap have no scenario | **CLOSED** | `clean-room-preparation.feature:541-548` asserts `e2e: ["."]`, `source: ["src/**"]`, 2 E2E files, 0 removed, and the copy holding only the declaration; `:550-557` asserts `e2e: ["tests/e2e"]`, `source: ["tests/**"]`, 1 E2E file, 1 removed, and the copy omitting the E2E match. |
| A18, the both-side drift rule has no scenario | **CLOSED** | `trial-input-verification.feature:289-298` edits `docs/reference.md` in the copy and the checkout and asserts `Specification files: 1 changed`, exactly one `- docs/reference.md` line under `Specification files`, `Specification changed after the copy was written`, `End-to-end tests: match`, and exit 1. |
| A19, chapter 03 says compare prints one `-` line per difference | **CLOSED** | `docs/arc42/03-context-and-scope.md:64` names one `- <path>` line per changed or missing path and one `+ <path>` line per added path. |

## Fresh-design answers

| Question | Answer |
|---|---|
| Four declaration refusals: is each printed sentence distinct, and does each scenario fail for a reader that prints another case's sentence? | **YES.** The four named outputs differ; the no-blob scenario fails a reader that prints `does not record`, the not-recorded scenario fails a reader that prints `returned no blob`, the malformed scenario asserts the loader's reason, and the non-UTF-8 scenario asserts the decoding error. |
| Delta tags and counts against D13 and `## Corpus Rebinding` | **MATCH.** 56 removed + 28 modified + 70 added = 154; clean-room 29/27/39, trial 27/0/31, declaration 0/1/0; 83 rebinding rows; 98 `@wip`. |
| Do the new enumeration scenarios discriminate? | **YES.** The project-root scenario fails a reader that treats `.` as a package path with no `Cargo.toml` (0 E2E files, `src/kept.rs` removed, `notes.md` outside) and a reader that removes an E2E match under `src/**`; the overlap scenario fails a reader that removes or copies the E2E match under `tests/**` (removed 2 or E2E 0). Both pin the counts and the copy content, so the E2E set's precedence over a source root is observable. |
| Does the same-path-both-sides scenario assert one line and the stated owner? | **YES.** It asserts `Specification files: 1 changed`, exactly one `- docs/reference.md` line under the `Specification files` subject, `End-to-end tests: match`, and exit 1. A reader that prints two differences, or puts the line under `End-to-end tests`, fails. |
| Any new blocking defect the revision introduced? | **NO.** The new scenarios do not contradict D8, D9, or D10; the counts and the reference rows stay consistent. The new lint findings are advisory. |
| Remaining advisories | **None design-level.** The 43 `[SHAPE]` and 116 `[SUBST]` findings are lint advisories; the two pairs the author names are masked-literal false positives, not merge candidates. |

## Required branch dispositions

| Branch | Disposition |
|---|---|
| Scope | **PASS** |
| Tech choices | **PASS** |
| Missing scenarios | **PASS** (B4, A17, and A18 closed) |
| Testability | **PASS** (B4 closed; the new phrase is unbound before implementation) |
| E2E fidelity | **PASS** |
| Interaction Coverage | **PASS** |
| Risk | **PASS** |
| Use-case and Persona context | **PASS** |
| Decision ledger | **PASS** |
| Contracts and components | **PASS** |
| Decision timing | **PASS** |
| change strategy | **PASS** |
| Visual design contract | **PASS** |
| Deferrals | **PASS** |
| Persona disposition | **PASS** |
| Interface classification | **PASS** |
| ADR qualification | **PASS** |
| Architecture documentation (arc42) | **PASS** (B5 and A19 closed) |
| Ubiquitous language | **PASS** |
| Structural model | **PASS** |

## Verdict

B4 and B5 are closed. The four declaration refusals now have distinct output in
the scenarios and in every durable artifact. A17, A18, and A19 are closed by
the two enumeration scenarios, the both-side scenario, and the corrected
chapter 03 row. The delta tags, the Corpus Rebinding table, D13, and the lint
composition agree; the 43 `[SHAPE]` findings contain no real merge candidate.
The task list may be written.

DESIGN-REVIEW: PASS
