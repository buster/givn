# Tasks: retire-semantic-capabilities

Pure-deletion change: each task authors and validates one capability's
retirement delta (zero-scenario `@givn.retired` feature in `specs/`).
The deletion itself is performed by `givn archive`, which also records
the tombstone; the teardown task verifies the post-archive state.
RED/GREEN/REFACTOR does not apply to a deletion — there are no scenarios
and no step definitions to write.

## Setup

- [x] Set up: confirm the 15 retirement deltas exist under `specs/<cap>/<cap>.feature` with the original Feature lines, zero scenarios, and `@<cap>` tags matching the directory names exactly. `givn lint --change retire-semantic-capabilities` clean.
  - [x] COMMIT: `03d8eab`
  - [x] COMMIT: `d870968`

## Retirement deltas

- [x] Author + validate retirement delta for bge-reranking-evidence
- [x] Author + validate retirement delta for nli-evidence
- [x] Author + validate retirement delta for search-evidence
- [x] Author + validate retirement delta for unified-semantic-rulebook
- [x] Author + validate retirement delta for spec-retrieval
- [x] Author + validate retirement delta for semantic-compact-serialization
- [x] Author + validate retirement delta for semantic-e5-token-cap
- [x] Author + validate retirement delta for semantic-model-download
- [x] Author + validate retirement delta for semantic-remediation-log
- [x] Author + validate retirement delta for semantic-reranker-budget
- [x] Author + validate retirement delta for semantic-review
- [x] Author + validate retirement delta for semantic-review-output
- [x] Author + validate retirement delta for semantic-scoring-throughput
- [x] Author + validate retirement delta for semantic-token-cap-review
- [x] Author + validate retirement delta for ci-debuginfo-optimization

## Teardown

- [x] After archive: `ls givn/specs/` shows none of the 15 directories; `givn/specs/.retired.yaml` lists all 15; `cargo test` green.
