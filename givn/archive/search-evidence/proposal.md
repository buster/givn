# Proposal: search-evidence

## Problem / Opportunity

`givn spec search` ranked results with a single E5 embedding score. That score is
symmetric: it cannot distinguish contradictory scenarios, e.g. "the green border
follows API key focus" against "the red border does not follow API key focus".
Users get a top-ranked candidate whose semantics are the opposite of their query,
with no signal in the output that this happened. BGE and NLI cross-encoders are
already run for this exact reason on the explore and review flows, but search
never applies them.

## Proposed Solution

`givn spec search` gains an optional evidence mode. Users opt in with a
`--evidence` flag; by default the command behaves exactly as today.

In evidence mode, each listed candidate also carries cross-check results from
the two cross-encoders, when the local installation supports them:

- BGE verdict (recommended vs. filtered) with its raw score.
- NLI verdict (recommended vs. filtered) with entailment / neutral /
  contradiction probabilities.

Evidence is shown for the presented candidates; candidates without evidence
(because a cross-encoder is unavailable in the build or cannot process the
pair) are explicitly marked as unscored, never silently presented as filtered.
When neither cross-encoder is available, the command still answers with the
plain ranking and states that no evidence was produced.

## Out of Scope

- The E5 ranking, its score, and its top-k selection stay unchanged.
- Explore and review output formats stay unchanged.
- No automatic re-ranking of results based on evidence: evidence is reported,
  not applied.
- No changes to BGE/NLI models, thresholds, or directional scoring logic.
- No cache or budget changes.

## Open Questions

None.