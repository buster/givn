# Proposal: default-build-is-feature-free

## Problem / Opportunity

Retirement plan task 1.2 removes the embedding features
(`spec-retrieval`, `spec-similarity-gate`, `cross-encoder-reranker`,
`nli-evidence`) from the default build. One release-0-3-0 scenario still
asserts the opposite contract: "the default feature set contains every
declared optional Givn feature". With the default set now empty, the
scenario fails and describes a deliberately removed shipping decision.

## Proposed Solution

Modify the scenario to pin the new contract: the default feature set is
empty; the four optional model features remain declared for explicit
opt-in via `--features`; the feature-free minimal build assertion is
kept. The step definition gains the matching assertions.

## Out of Scope

Any other release-0-3-0 scenario; the feature definitions themselves
(kept as opt-in).

## Open Questions

None.
