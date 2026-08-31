# Review: unified-semantic-rulebook

## Coverage gaps

| Gap | Classification | Reason |
|---|---|---|
| real-model E5/BGE/NLI inference inside scenarios | legitimately hard to test | scenarios run with deterministic stub backends; the real-model evidence was exercised once in the review run below (20 pairs judged, exit 0) |
| pair-score cache corruption handling | legitimately hard to test | unreadable cache rebuilds silently; verdicts never depend on cache contents |

## Standing-suite reconciliation evidence

- `./run-tests.sh` (permanent scope): exit 101 with exactly the four expected
  until-merge failures — the ADR register count, the two 0.9500 boundary
  assertions, and the retired advisory-gate echo. Each is replaced by this
  change's delta and green there.
- `./run-tests.sh --e2e`: exit 0, 94 scenarios.
- Delta suite (`GIVN_FEATURES=givn/changes/unified-semantic-rulebook/specs`):
  17/17 scenarios green across the e5/gate-only, bge, nli, review-output and
  rulebook groups.

## Real-model review evidence

`givn check review --change unified-semantic-rulebook` (release build, real
E5/BGE/NLI artifacts): pair cache: 0 reused, 20 computed — the hybrid pair
cap bounded second-stage judgement to 20 pairs; report Status PASS with zero
unresolved candidates. The rulebook gate stays green under the same rules the
archive gate applies.

## Ubiquitous language

Unified rulebook, Pair cap, Neighbour count, Pair-score cache, Unselected
pair, E5 candidate — recorded in `docs/arc42/12-glossary.md` and used
consistently across specs, design, ADR-066, and the implementation.

REVIEW: PASS
