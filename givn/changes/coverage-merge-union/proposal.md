# Proposal: coverage-merge-union

## Problem / Opportunity

The merged coverage report misstates coverage. Merging adds the two source
reports' counters together — covered lines and measured lines alike — but both
reports measure the same source tree, so every line this repository ships is
counted twice in the denominator. The published number therefore
under-reports: the current merged report claims 37.6% line coverage while its
own per-file data shows the true union is 41.8% (6946 of 16598 lines). The
same arithmetic error exists in the built-in merge used by projects still on
the legacy two-command coverage configuration.

A consumer reading the README badge, or a maintainer judging whether
coverage moved after a change, gets a systematically wrong number.

## Proposed Solution

A merged coverage report counts every measured line once and counts a line as
covered when either source report hit it:

- Merging two reports of the same tree yields the union of measured lines as
  the denominator and the union of covered lines as the numerator.
- A line present in both reports is counted once, never twice.
- A line present in only one report still counts, once.
- The reported rate stays arithmetically consistent with the reported
  counters, so existing validation and badge rendering keep working.
- This holds for the built-in merge used by the legacy two-command coverage
  configuration, and for the repository's own project-owned merge script.

Branch counters keep their current summation: branch identity is not
retained in a parsed report, so a branch union is not computable without a
new measurement contract. This limitation is documented where the merge is
defined.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| extract-coverage-gate | EXTEND adr-qualification-gate (group support, score 3.00) | `EXTEND extract-coverage-gate` | Route matched on generic ADR/config vocabulary from this table itself — false positive at minimal score. The touched capability is the archive coverage gate's merge contract, owned by `extract-coverage-gate` in the `archival` group; the legacy merge is that gate's built-in merge path. |

## Out of Scope

- No change to instrumentation or measurement (which lines profraws record);
  only the merge arithmetic changes.
- No branch-level union (per-branch identity is not retained; documented
  limitation).
- No change to the script-path gate contract (the gate still runs the
  configured merge command and validates its output).
- No change to the distinctness check between the two source reports.

## Open Questions

- None.
