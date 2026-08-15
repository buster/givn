# Proposal: cross-encoder-reranker

## Problem / Opportunity

Independent scenario embeddings are fast and useful for candidate retrieval, but
they judge two scenarios without seeing them together. Repeated test structure
can therefore rank valid boundary scenarios as if they were duplicates.

The current hard embedding-score gate is not a sufficient semantic decision
function. A second model must compare both complete scenarios jointly before a
review decision is made.

## Proposed Solution

Keep the local embedding index as the fast candidate-retrieval stage. For each
candidate pair, compare the two complete scenarios jointly and use that pair
assessment to rank and gate the review candidates.

The comparison must use the complete scenario title and observable steps. It
must distinguish a true duplicate from a valid boundary such as text output
versus JSON output or two different commands with similar setup.

Review checks the permanent scenario tree and the current change against the
permanent tree. A true duplicate remains a review failure until the duplicate
is deleted. A valid but overly similar boundary remains a review failure until
the scenario is rewritten clearly enough for the pair assessment to fall below
the selected reranker boundary.

The pair model and its decision boundary are local, versioned binary behavior.
No project setting, CLI flag, environment variable, or review note may weaken
the gate. Interactive single-query search remains advisory.

## Out of Scope

This change does not call a hosted model service, train a model during review,
or use project-specific string lists to identify boilerplate.

It does not replace deterministic title, fingerprint, or structural overlap
checks. It does not make the interactive `givn spec search` command blocking.

## Open Questions

The exact local multilingual pair model and its calibrated score boundary are
resolved in the design from the existing local model support and a reproducible
corpus measurement. They are not project-configurable after the binary is
built.
