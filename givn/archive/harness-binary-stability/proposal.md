# Proposal: harness-binary-stability

## Problem / Opportunity

The repository acceptance suites fail on a clean environment (fresh CI runner
or a cold build tree) even when every scenario would pass: the acceptance
suite and the coverage replay each build the ready-fixture process on demand
the first time a run-execution scenario needs it, and that build recompiles
the suite's givn binary with the default (feature-free) feature set. Every
scenario scheduled after that point receives the wrong binary — the semantic
scenarios observe "spec retrieval is unavailable" and the review seed observes
the retrieval gate silently absent. The failure is deterministic in a cold
tree and invisible in a warm tree, which makes it self-perpetuating: CI's
restored cache never contains the fixture binary, so the swap repeats on every
run while local runs stay green.

## Proposed Solution

The acceptance harness and the coverage replay guarantee the fixture process
exists before any scenario runs, so no scenario ever observes a binary that
was replaced underneath the running suite:

- Running the acceptance suites (`./run-tests.sh`, with or without a scope,
  with or without `--e2e`) completes on a cold tree with the same results as
  on a warm tree.
- The on-demand build remains as a fallback, but a scenario never triggers it
  in a prepared environment.
- Coverage measurement (`./measure-coverage.sh`) replays the same groups with
  the same guarantee: the fixture process is prepared before the first group
  runs, so no replay group receives a replaced binary.

The observable contracts this change must preserve are pinned by permanent
scenarios, which acted as the regression guards that exposed the defect:
the development/installed build distinction
(givn/specs/spec-discovery/release-strict-model-overrides.feature) and the
review disposition gates (givn/specs/verification/review-disposition-gates.feature).

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| (none — no capability delta) | EXTEND spec-explore-change (group support, score 8.00) | no capability change | Route matched on generic tool vocabulary — false positive. The change is repository-harness infrastructure: it edits repo-owned scripts only, produces no observable givn command behaviour, and therefore carries no delta spec and creates or extends no capability (precedent: migrate-0-4-0-to-0-5-0, group-specs-by-use-case — infrastructure changes without delta specs). |

## Out of Scope

- No change to observable givn command behaviour, gate decisions, or spec
  semantics.
- No change to the release-build fixtures' on-demand release build: it is
  guarded by a run-once latch and the first scenario blocks until it
  completes, so no cross-suite binary swap is possible there.
- No CI cache-key or cache-rotation changes: with the fixture prepared up
  front, the cold cache state is harmless.
- No change to the coverage merge arithmetic (separate change).

## Open Questions

- None.
