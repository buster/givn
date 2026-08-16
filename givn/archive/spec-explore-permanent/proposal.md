# Proposal: Explore Permanent Scenarios Before Creation

## Problem / Opportunity

An agent can currently search the permanent scenario corpus only by supplying a
free-form query. Before creating a new scenario, that makes it easy to miss an
existing scenario with the same intent, especially when both scenarios belong
to the same feature. The agent also has no reliable way to know whether a
candidate is the scenario it is currently drafting.

This causes avoidable duplicate or near-duplicate specifications and forces an
agent to perform manual, inconsistent comparisons. The existing retrieval
index already contains the permanent corpus, but there is no interface that
accepts a complete draft scenario and returns an advisory comparison.

## Proposed Solution

Provide `givn spec explore --file <draft.feature>` and
`givn spec explore --stdin`. Each invocation accepts exactly one complete
Gherkin scenario and rejects malformed input, free text, input without a
scenario, multiple scenarios, unreadable files, and conflicting input flags.

Before returning candidates, the command refreshes the permanent scenario
catalogue without recomputing unchanged scenario representations. It searches
all permanent scenarios, including other scenarios in the same feature, and
omits only the exact `(file, scenario title)` identity of a query read from an
existing permanent file.

The default response is a human-readable list of at most five candidates. Each
candidate includes its rank, title, file, and embedding score. The response
also identifies the embedding model and index version and size. With
`--format json`, the command returns a stable machine-readable response with
the query identity and content hash, index metadata, and deterministically
ordered candidates.

Exploration is advisory and read-only. It does not change feature files, read
the active change, make a review decision, or block a workflow.

## Out of Scope

- Exploring the current change or accepting a `--change` option.
- Semantic review reports, classifications, remediation verification, and
  review gating.
- Permanent duplicate-audit commands and automatic feature-file edits.
- BGE reranking, NLI classification, coverage analysis, step-definition
  resolution, and mutation testing.
- Changes to the existing `givn spec index`, `givn spec search`, or
  `givn spec review --change` contracts.

## Open Questions

None. The input forms, single-scenario boundary, top-five limit, JSON option,
and permanent-only search scope are fixed by this change.
