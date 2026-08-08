# Proposal: add-coverage-readme-example

## Problem / Opportunity

When the coverage addon is enabled, the archive workflow requires coverage to
appear in the project README, but the README guidance does not show what a
useful coverage section looks like. Contributors can therefore publish only a
bare percentage and omit the report link, the distinction between non-E2E and
E2E results, or the command needed to reproduce the measurement.

## Proposed Solution

The archive guidance will include an illustrative README coverage section. The
example will show a coverage badge linked to the non-E2E report, prose naming
the measured non-E2E and E2E line totals and percentages, and a shell block
showing how the non-E2E coverage is reproduced. It will make clear that the
values and commands are project-specific and that the generated coverage block
remains managed by givn.

## Out of Scope

Coverage calculation, Cobertura parsing, archive gating, badge colours, and
the project's current measured coverage are unchanged. The example will not
hard-code another project's executable name or coverage values into generated
README output.

## Open Questions

None.
