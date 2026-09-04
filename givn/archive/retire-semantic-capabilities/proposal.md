# Proposal: retire-semantic-capabilities

## Problem / Opportunity

The semantic similarity gate (E5/BGE/NLI embeddings) is being removed from
givn. Fourteen capabilities in the permanent spec corpus exist only to
specify that engine, and a fifteenth (`ci-debuginfo-optimization`) is an
empty 1-line corpse. Once the embedding code is compiled out, these
capabilities describe behaviour that no longer exists, and every archive
re-executes their scenarios against step definitions that are gone.

Until now there was no way to remove a whole capability — only individual
scenarios could be removed (`@givn.removed`), never the capability
directory itself. The retirement primitive (`@givn.retired`) now exists in
the binary, so the dead corpus can finally be deleted properly, with a
tombstone recording what was retired and by which change.

## Proposed Solution

Delete these 15 capabilities from the permanent spec corpus
(`givn/specs/<cap>/`), via one `@givn.retired` delta each:

- bge-reranking-evidence
- nli-evidence
- search-evidence
- unified-semantic-rulebook
- spec-retrieval
- semantic-compact-serialization
- semantic-e5-token-cap
- semantic-model-download
- semantic-remediation-log
- semantic-reranker-budget
- semantic-review
- semantic-review-output
- semantic-scoring-throughput
- semantic-token-cap-review
- ci-debuginfo-optimization

After archive, `givn/specs/` contains none of these directories, and
`givn/specs/.retired.yaml` lists all 15 tombstone entries. The full test
suite stays green at this point (the Cargo feature flags for the embedding
code are dropped in a later, separate change).

## Out of Scope

- The other `givn spec` subcommands (`spec-duplicates`,
  `spec-explore-change`, `spec-explore-permanent`,
  `spec-progress-output`) — these describe tools that will be rebuilt
  deterministically rather than deleted; their specs are rewritten in a
  separate change.
- Dropping the `spec-retrieval` / `spec-similarity-gate` /
  `cross-encoder-reranker` / `nli-evidence` Cargo features from `default`.
- Rewriting review instructions/templates (separate change).

## Open Questions

None — the retirement list was verified against `givn/specs/` immediately
before writing this proposal.
