# Proposal: semantic-e5-token-cap

## Problem / Opportunity

The complete semantic serialization of a Gherkin scenario can exceed the
512-token input limit of the E5 model tokenizer. The current indexing path
rejects the whole batch when one passage is too long. A real permanent corpus
therefore loses otherwise usable scenario vectors, and duplicate, explore, and
blocking-review commands fail instead of identifying the scenario that needs
authoring work. Any implementation that truncates the passage would also make
the semantic evidence incomplete.

## Proposed Solution

Before E5 embedding, the complete passage is measured with the actual E5
tokenizer, including its role prefix, without truncation or padding and before
model construction or inference. Passages at 512 tokens remain valid; passages
above 512 are excluded from E5 inference without aborting the index. All
fitting permanent scenarios continue to be embedded and indexed. Production
uses the real local tokenizer; acceptance tests use a deterministic test-only
budget seam.

E5, BGE, and NLI use the same complete deterministic semantic serialization.
The exact E5 serialization is the `content_hash` boundary: a hash mismatch
re-embeds the endpoint and there is no separate serializer-version field.

Every over-cap endpoint is retained in an additive
`e5_token_cap_candidates` collection, never in the normal pair `candidates`
collection. The persistent index keeps fitting vectors in its existing vector
entries and stores cap records separately; an older index whose cap collection
is absent reads it as empty. Each cap record has `status: "E5_TOKEN_CAP"`,
`file`, `title`, the complete serialization `content_hash`, `token_count`,
`limit: 512`, E5 `model`, tokenizer provenance, `refactoring_required: true`,
and deterministic shorten/split-and-rerun `guidance`. Records are ordered by
normalized project-relative file, title, and content hash.

Duplicate JSON and text output, Explore output, advisory review output, and
the blocking semantic report expose the same additive
`e5_token_cap_candidates` collection. Normal pair `candidate_count` counts
only the normal `candidates` collection; `e5_token_cap_count` is the cap
collection length and is never folded into the pair count. A cap has no vector,
rank, or E5 semantic pair score. Exact duplicate and structural-overlap pair
findings continue to be reported even when one endpoint is capped; only E5
semantic pair generation excludes the capped endpoint.

The blocking report writes cap records separately from pair candidates with
`classification: null` and `rationale: null`. Any non-empty cap collection is
unresolved and keeps blocking review non-zero. Advisory review remains
advisory and exits successfully while exposing the cap collection. Explore
keeps fitting results with their normal rank and score fields while reporting
cap records without an invented score.

## Out of Scope

- Truncating, chunking, automatically splitting, or automatically editing a
  scenario.
- Changing the E5 model, its 512-token limit, semantic thresholds, or the
  existing BGE/NLI pair token-cap contracts.
- Changing how fitting E5 scenarios are ranked, reused, or scored.
- Requiring network access or large model downloads in acceptance tests.

## Open Questions

None. The cap is inclusive at 512 tokens, the complete untruncated passage is
the measured input, and the finding is additive evidence rather than a Givn
classification.
