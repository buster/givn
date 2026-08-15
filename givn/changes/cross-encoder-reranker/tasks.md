# Tasks: cross-encoder-reranker

## Setup

- [x] Confirm strict Cucumber mode with `.fail_on_skipped()` and run the new
  feature path using `GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub`. The
  targeted suite exits 0 after the reranker steps are implemented.
- [x] Confirm `cargo check --no-default-features --locked` and
  `cargo clippy --workspace --all-targets --features spec-retrieval -- -D warnings`.
- [x] Check the FastEmbed-rs `TextRerank` API and multilingual
  `BGERerankerV2M3` against the locked source and Context7 on 2026-08-16.

## Scenario: Review evaluates semantic candidates through the pair reranker

- [x] RED/GREEN/REFACTOR: The real CLI now retrieves candidates, scores both
  pair directions, prints raw/sigmoid scores, and fails above the branch
  boundary. Targeted Cucumber result: 1 feature, 6 scenarios, 33 steps passed
  with the deterministic reranker stub.
- [x] COMMIT: Pending branch implementation commit.

## Scenario: Review does not block a valid boundary rejected by the pair reranker

- [x] RED/GREEN/REFACTOR: The valid stderr/JSON-style boundary fixture passes
  despite its high embedding similarity. Targeted scenario passes.
- [x] COMMIT: Pending branch implementation commit.

## Scenario: Review applies the pair reranker to current scenarios and the projected permanent tree

- [x] RED/GREEN/REFACTOR: Projected duplicate removal passes through the real
  CLI and reranker path. Targeted scenario passes.
- [x] COMMIT: Pending branch implementation commit.

## Scenario: Review reports pair-reranker setup failure

- [x] RED/GREEN/REFACTOR: `GIVN_SPEC_RERANKER=missing` produces an explicit
  non-zero setup error. Targeted scenario passes.
- [x] COMMIT: Pending branch implementation commit.

## Benchmark

- [x] Real Rust benchmark: 208 entries, top-K 5, 697 standalone candidates,
  588.07 seconds; integrated review: 839 candidates, 750.67 seconds.
- [x] Manual comparison recorded in `docs/cross-encoder-benchmark.md`.
- [x] Adoption decision recorded: negative result; do not merge as final gate
  without a duplicate-specific classifier or additional evidence.
