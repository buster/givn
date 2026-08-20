# Design Review: semantic-e5-token-cap

## Findings And Resolutions

| Rank | Branch | Finding | Resolution | State |
|---:|---|---|---|---|
| 1 | Wire contract | An E5 passage cap is endpoint-level evidence, not a pair candidate. | Use additive `e5_token_cap_candidates`; keep `candidates` for exact, structural, and semantic pairs. `candidate_count` excludes cap records and `e5_token_cap_count` reports them separately. | RESOLVED |
| 2 | Index safety | One over-cap passage previously aborted the complete E5 index. | Partition fitting passages from capped passages, embed only fitting inputs, persist cap metadata, and retain deterministic findings. | RESOLVED |
| 3 | Token correctness | A post-truncation count would not prove the complete input fit. | Count with the actual untruncated/unpadded E5 tokenizer before inference; inclusive 512 boundary, 513 cap. | RESOLVED |
| 4 | Evidence provenance | A cap record without identity/hash/model provenance could not be reproduced. | Persist file, title, complete content hash, token count, limit, model, tokenizer provenance, deterministic ordering, guidance, and `refactoring_required`. | RESOLVED |
| 5 | Surface consistency | E5 cap behavior could differ between index, duplicates, Explore, advisory review, and blocking review. | Share one cap-aware passage partition and expose the additive collection across all five surfaces. | RESOLVED |
| 6 | Blocking semantics | Treating a cap as filtered would hide unresolved authoring work. | Cap findings remain visible, classification-free, blocking in semantic review, and advisory in advisory review. | RESOLVED |
| 7 | Pair integrity | Capped endpoints must not receive fabricated E5 scores or enter BGE/NLI pairs. | Exclude capped endpoints from vector-backed semantic pairs; preserve exact/structural findings independently. | RESOLVED |
| 8 | Backward compatibility | Older index documents do not contain cap metadata. | Missing `e5_token_caps` deserializes as an empty collection; fitting vector identity/hash reuse remains unchanged. | RESOLVED |
| 9 | Testability | Real E5 artifacts are too large for deterministic acceptance tests. | Use a test-only budget seam for exact/partial/over-cap counts; production uses the real local tokenizer and no test downloads. | RESOLVED |
| 10 | Scope | The complete feature matrix could over-expand the change. | Keep six focused scenarios covering index, duplicate, Explore, advisory, blocking, and exact boundary; shared production behavior covers all surfaces. | RESOLVED |

## Arc42

The Arc42 addon is enabled. The change marker assessed all twelve chapters and
updated the affected durable chapters. Chapter 07 remains `No` because there is
no deployment topology change. Chapter 09 extends ADR-060; chapter 11 records
R-72 for stale cap metadata and fabricated score risks.

## Verification

- `givn lint --change semantic-e5-token-cap` is expected to report only the
  intentional `@wip` findings before implementation.
- The runner uses strict `cucumber-rs` `.fail_on_skipped()` behavior.
- The design matrix maps all five CLI interaction entries to real compiled-CLI
  E2E scenarios.
- No production code or tasks were written during this review.

## Status

DESIGN-REVIEW: PASS
