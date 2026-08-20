# Design Review: semantic-compact-serialization

## Grilling Result

The fresh-context review identified these issues:

| Priority | Finding | Resolution |
|---|---|---|
| P0 | The planned capability step modules and registrations did not exist. | The implementation plan names `semantic_compact_serialization_steps.rs` and `semantic_compact_serialization_e2e_steps.rs`; the tasks must create and register both before RED. Existing token-cap steps will not be reused. |
| P0 | The current serializer emits `[]`, empty section labels, and empty nested fields. | The production change will replace fixed empty representations with conditional section/field emission. |
| P0 | The original feature assertions could not observe the serialized passage. | The regular scenarios now assert the generated index entry's exact expected `content_hash`; direct source tests assert the exact passage text. The E2E scenario keeps CLI stdout/exit as primary evidence and index state as secondary evidence. |
| P1 | “Complete” did not define the semantic boundary. | The design now explicitly preserves parsed non-empty fachliche content, normalizes whitespace, excludes comments/tags/paths/scores/classifications/Fingerprint metadata, and does not promise source formatting or docstring delimiter preservation. |
| P1 | Nested optional fields and whitespace-only values had inconsistent omission behavior. | The design requires one normalized `Option<String>` policy for all optional text and conditional nested field emission. |
| P1 | The feature did not enumerate non-empty optional content. | A separate regular scenario now covers descriptions, Feature/Rule Backgrounds, tables, docstrings, Scenario Outline content, and Examples through an exact expected hash. |
| P1 | The design overstated the scope of the bare `./run-tests.sh` command. | The design now distinguishes explicit `GIVN_FEATURES` targeted runs from the default permanent suite and documents hook-provided active-change scope. |
| P2 | The Arc42 marker assessed chapter 3 as unaffected while listing it as updated. | Chapter 3 is now marked `Yes`, because the existing retrieval interface's shared model-input boundary changes even though no new command is added. |

## Required Branches

### Scope

The scope matches the proposal: compact the existing complete semantic model
input, omit empty optional content, retain non-empty content, and leave model
thresholds, caps, downstream evidence, and feature files unchanged.

### Technical Choice

The existing text representation and content-hash boundary remain the smallest
correct change. Conditional ordered-line emitters avoid a new persistence schema,
public inspection command, or serializer version field. JSON remains limited to
escaping structured table/docstring values.

### Missing Scenarios

The required empty-content and non-empty-content paths are covered. Whitespace-
only optional values, nested optional fields, escaping, ordering, and hash
invalidation are covered by direct serializer tests because they are exact
representation contracts rather than new consumer interactions. Existing
token-cap and retrieval scenarios cover the unchanged boundary behavior.

### Testability

`.fail_on_skipped()` is already configured. The regular scenarios fail against
the current serializer because its hashes differ from the compact expected
hashes. Direct serializer tests fail on the current `[]` output and assert exact
lines, not only successful indexing. The E2E scenario is a CLI smoke test for
the one normalized `givn spec index` action and cannot replace the regular
representation assertions.

### E2E Fidelity and Interaction Coverage

The feature inventory contains one normalized action: rebuilding the permanent
retrieval index with `givn spec index`. It has exactly one `@e2e` scenario. The
design matrix maps it to a real compiled CLI subprocess, with exit status and
stdout as primary assertions. There are no browser, HTTP, external-service, or
digital-twin requirements.

### Risk

The main failure mode is partial omission of a nested field or accidental loss
of non-empty semantic content while removing placeholders. One normalization
helper, exact serializer tests for every promised content type, and exact hash
integration scenarios mitigate this. The content hash forces affected vectors
to refresh.

### Arc42

Arc42 is enabled. Independent assessment:

| # | Yes/No | Reason |
|---:|---|---|
| 1 | Yes | Semantic retrieval quality and complete compact input are refined. |
| 2 | Yes | A new compact serializer constraint is recorded. |
| 3 | Yes | Existing retrieval context/input contract changes without a new boundary. |
| 4 | Yes | The semantic model-input strategy changes. |
| 5 | Yes | The existing `spec::retrieval` responsibility changes. |
| 6 | Yes | Existing retrieval runtime flow consumes the compact passage. |
| 7 | No | No deployment change. |
| 8 | Yes | Shared serialization and normalization are cross-cutting. |
| 9 | Yes | ADR-062 records the omission decision. |
| 10 | Yes | A compact-input quality scenario is added. |
| 11 | Yes | Similarity inflation from empty markers is a new recorded risk. |
| 12 | Yes | Compact semantic serialization is durable glossary vocabulary. |

All twelve chapter files exist and contain substantive content. Chapter 9 has
ADR-062 in the existing MADR structure, and chapter 11 contains R-73 for its
consequences. No new deployment or external dependency is introduced.

## Hardening Completed

- Updated the regular feature scenarios to assert exact index content hashes.
- Added the rich optional-content scenario to the executable specification.
- Clarified semantic inclusion/exclusion, normalization, escaping, and outline
  boundaries in `design.md`.
- Clarified targeted versus full runner scope in `design.md`.
- Corrected chapter 3's Arc42 assessment.
- Kept the existing one-action E2E inventory and real CLI mechanism.

DESIGN-REVIEW: PASS
