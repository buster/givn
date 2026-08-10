# Proposal: coverage-script-workflow

## Problem / Opportunity

Projects using givn can publish coverage numbers that describe different
reports. The README badge may show only non-E2E line coverage, while the
archive gate measures separate E2E and non-E2E reports and publishes different
totals elsewhere. Long inline commands make it difficult to see which tests
were measured, how the reports were combined, or how to reproduce the result.

## Proposed Solution

The coverage result shown by givn will always be calculated from the combined
E2E and non-E2E reports. The README will show the combined line and branch
coverage at the top as badges and will show the same combined totals in its
coverage section.

Projects will document their normal test, coverage-measurement, and coverage-
merge scripts in the README. `givn/commands.yaml` will invoke those same
project-owned scripts, including an explicit merge-coverages command, instead
of hiding the workflow in large inline commands. The merged report produced by
that command will be the report linked by the badges and used for the README
summary.

## Out of Scope

The coverage gate will not introduce a percentage threshold, change the
meaning of E2E scenarios, or prescribe a particular language, test runner, or
coverage tool. Existing project test behaviour remains unchanged apart from
making its documented scripts the commands used by givn.

## Open Questions

None.
