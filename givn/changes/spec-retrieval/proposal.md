# Proposal: spec-retrieval

## Problem / Opportunity

The deterministic overlap gates catch identical titles and many structurally
similar scenarios, but they cannot reliably find paraphrases that describe the
same behavior with different wording. The Change 1 evidence measured missed
paraphrase pairs. An author can therefore still write a new scenario without
seeing a semantically related existing one.

## Proposed Solution

- The user can build a local index over the current permanent specifications.
- The user can search that index with a behavior description or completed
  scenario and receive the closest existing scenarios, including paraphrase
  candidates.
- Search results are advisory and never block authoring or CI; the author and
  reviewer decide whether a match is a duplicate, variant, or boundary.
- During review, the current tree is indexed before querying and undisposed
  candidates from the active delta are reported for human disposition. The
  review result remains advisory and never changes a gate result.
- A change's own delta is not matched against itself.
- The index updates incrementally when specification content changes and does
  not require a network service after the model is available locally.

## Out of Scope

- Blocking on similarity scores or introducing a score threshold gate.
- CI dependence on a downloaded production model or an external service.
- Indexing step-definition files or arbitrary test code.
- Matching a change against other open deltas by default.
- Changes to deterministic lint/review gates.

## Open Questions

- The production model and index remain local, while the test runner uses a
  deterministic local embedder so the e2e gate has no network or multi-gigabyte
  model dependency. The design records the exact model/cache behavior.
