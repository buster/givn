# Proposal: unified-semantic-rulebook

## Problem / Opportunity

When a change's scenarios are reviewed, givn compares them against the
permanent corpus with three models: the embedding comparison finds candidate
pairs, and two evidence models (a reranker and an entailment model) judge
whether the candidates are truly problematic. Today two different rule sets do
this job at different moments:

- The classification gate compares with a strict cut-off (0.95) and judges
  **every** pair above it.
- The advisory comparison and the archive re-check compare with a looser
  cut-off (0.90), but look at only a fixed handful of neighbours per scenario.

Consequences an owner can observe:

1. **Same change, different verdicts.** The advisory report and the
   classification worklist can disagree about the same pair because they
   follow different rules.
2. **The archive checks with different rules than the review it validates.**
   The archive re-check exists so a stale sign-off cannot slip through — but
   it re-judges with the advisory rule set, not the strict one.
3. **The work is repeated.** Every run redoes the whole comparison from
   scratch: the same pair is judged by the same model several times over one
   change's life (review, retries, archive). Judging long scenario texts
   costs real compute — observed runs spend most of their wall time on model
   inference with the CPU saturated for many minutes.

## Proposed Solution

One rule set for every review surface:

1. **One comparison.** All scenarios are embedded (permanent ones once and
   cached; the change's ones at review time). The change's scenarios are
   compared against the permanent corpus and against each other. Pairs below
   0.90 similarity are cut.
2. **One selection.** From the remaining pairs, up to a fixed neighbour count
   per scenario is kept, capped at one shared limit for the whole run (20
   pairs). Oversized scenarios are reported as token caps and must be
   shortened — unchanged.
3. **One judgement.** Both evidence models run on the selected pairs only;
   each recommends independently, as today.
4. **Same rules at every moment.** The change review and the archive re-check
   apply this rule set. The archive re-check still evaluates against the
   current permanent tree (so a stale sign-off cannot slip through) and still
   carries over earlier classifications for unchanged pairs.
5. **Each pair is judged once.** Pair judgements are recorded against the
   pair's content identity; a re-run reuses them and reports how many
   judgements were reused versus freshly computed.
6. **Fixed selection limits.** The neighbour count and the pair cap are
   constants of the shipped build, not environment-tunable — consistent with
   the enforcement rule of `release-override-hardening`.

The advisory report and the classification worklist stay as they are; both
are generated from the same run.

## Out of Scope

- The duplicate audit and the explore/search surfaces keep their existing
  evidence rules.
- Permanent↔permanent corpus hygiene stays with the duplicate audit.
- Batching the evidence models' inference (explicitly deferred by ADR-064).
- Model identity verification for locally provided artifacts
  (`release-override-hardening`).

## Open Questions

None. Decisions taken with the product owner:

- Comparisons include change-scenario↔change-scenario pairs (catches
  duplicates born inside one change).
- Selection is hybrid: per-scenario neighbour count, then one shared cap
  (20).
- The strict 0.95 cut-off is replaced by the single 0.90 threshold
  everywhere; the selection limits bound the workload instead of the
  threshold.
- Judgement symmetry (each pair in both directions) is retained; batching
  stays deferred.
