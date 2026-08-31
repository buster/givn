---
status: proposed
date: 2026-08-31
decision-makers:
  - givn maintainers
consulted:
  - unified-semantic-rulebook proposal, design, and executable specs
informed:
  - maintainers operating change review, the archive gate, and duplicate audit
---

# Run change-scoped semantic review as one unified rulebook with a pair-score cache

## Context

Three change-scoped surfaces re-derive semantic evidence with two different
rule sets. Blocking `givn check review` compares at the strict `0.9500`
boundary and judges its complete candidate list; `givn spec review` and the
archive similarity gate compare at `0.9000` but look at only a top-five
neighbour slice plus projected pool. The same pair can therefore receive
different verdicts on different surfaces, the archive validates a review with
different rules than the review it validates, and every run re-computes all
model evidence from scratch — observed runs spend minutes of saturated CPU
re-judging identical pairs across review retries and the archive re-check.

## Decision drivers

- [x] **Verdict consistency** — one change must not receive different
      semantic verdicts from its review step and its archive step.
- [x] **Runtime cost** — pair judgement must happen once per pair per change
      lifecycle, not once per invocation.
- [x] **Bounded work** — model work must scale with a fixed selection, not
      with the corpus, and must not explode at lower thresholds.
- [x] **Recall safety** — bounding work must not silently drop recommended
      pairs from the blocking set.

## Considered options

1. **Keep surface-specific rules** (status quo, ADR-061): consistent
   contract, but verdict divergence and repeated inference stay.
2. **Share evidence between the two pipelines** without unifying rules:
   fixes cost, keeps divergence.
3. **One unified rulebook with deterministic selection and a pair-score
   cache** (chosen): one comparison at one threshold, deterministic hybrid
   selection, both evidence layers on the selected pairs, judgements recorded
   per pair identity and reused.

## Decision

One change-scoped rulebook serves `givn spec review`, `givn check review`, and
the archive similarity gate:

1. embed (permanent from the hash-reused index, delta fresh);
2. compare delta↔permanent and delta↔delta, cut below the single
   `0.9000` E5 admission;
3. hybrid selection: keep at most `5` neighbours per delta scenario, cap the
   union at `20` pairs by E5 score (deterministic tie order);
4. judge the selected pairs independently with BGE and NLI (both directions,
   unchanged admission thresholds);
5. persist pair evidence in `givn/.spec-pair-cache.json` keyed by stage,
   model, and both endpoints' content/evidence hashes; hash-identical pairs
   reuse recorded judgements and every run reports reused vs. computed
   counts.

Pairs dropped by selection are **not candidates** (no report entry, no
blocking state); only emergency budget truncation still produces `*_UNSCORED`.
The `0.9500` boundary is retired; the `GIVN_SPEC_MAX_RERANK_PAIRS` budget
remains the emergency ceiling above selection (ADR-063 reconciled).

## Consequences

### Good

- Review and archive verdicts follow the same rules; the archive still
  re-evaluates the current permanent tree, so stale sign-offs cannot pass.
- Unchanged pairs are judged once per change lifecycle; reruns and the
  archive re-check reuse recorded evidence.
- Model work is bounded by the selection regardless of corpus growth.

### Bad

- Recall beyond the selection cap is consciously accepted (tracked in
  arc42 ch. 11 next to the budget-tail risk); a true duplicate ranked
  below the cap on its scenario is not judged.
- Lowering the strict boundary from `0.9500` to `0.9000` admits more
  borderline pairs into selection; the cap bounds the extra work.
- A new persistent file (`.spec-pair-cache.json`) enters project state and
  needs the same schema-version discipline as the spec index.

### Neutral

- `semantic-review.md` (ADR-046) and `review-findings.json` (ADR-054)
  contracts are unchanged renderings of the same run.
- Explore, search, and the duplicate audit keep their surface-specific
  pools; ADR-061 remains valid for those surfaces and is superseded only
  for the change-scoped review surfaces.

## Qualification

ADR-QUALIFICATION: PASS

- **alternatives** — three materially different options recorded above.
- **architectural_impact** — fixes the shared rulebook boundary, one
  persistent evidence store, and the threshold/selection contract for all
  three review surfaces.
- **durable_consequence** — reversal would re-split three surfaces, re-shape
  persisted report semantics, and re-negotiate the runtime budget contract.
- **lower_level_artifact** — executable specs pin the observable sequence;
  the cross-surface consistency and cache contract are not fully owned by
  any single feature file.
- **existing_adr_check** — supersedes ADR-061's change-scoped portion;
  revises ADR-041 (boundary), ADR-052/058 (pool assignment); ADR-063 and
  ADR-064 remain in force; ADR-046/054 untouched.
