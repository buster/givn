# Design Review: fix-remediation-delta2delta

## Scope

The proposal, feature delta, and design describe one slice: make the semantic
remediation verifier able to verify prior classifications whose right endpoint
lives in the change's own delta (a fresh-repository, first-change scenario with
no permanent specifications). Automatic classification, automatic spec edits,
BGE/NLI second-stage evidence, and the advisory review path remain out of scope.

## Grilling Results

- **Why is the delta-to-delta path blocked today?** The old guard required
  `right_scope == "permanent"` and `right` resolved from the permanent index;
  any `current-change` right endpoint hit the stale-classification bail.
- **What proves remediation actually happened for a delta-to-delta pair?**
  Same observable contracts as permanent pairs, but recomputed against two
  active-delta endpoints: removal (DUPLICATE), consolidation into an outline
  (VARIANT), or rewording below `0.9500` with an explicit distinction
  (VALID-BOUNDARY).
- **Why recompute instead of trusting the report score?** The report score can
  be stale; the endpoint evidence is re-embedded with the configured embedder
  at verify time, matching the permanent path's independence from current
  admission.
- **Does the fix regress permanent-pair verification?** No; the permanent arm
  is byte-identical except the guard now branches on `right_scope` before
  resolving the right endpoint.

## Sign-off

Design approved. The self-review semantic gate for this change is green: the
regression scenarios stage each fixture under a distinct change id with
distinct long narrative markers, so their surfaces are E5-separable and the
real-embedder run produces zero semantic candidates. Full regression coverage
(10 scenarios) is retained.
