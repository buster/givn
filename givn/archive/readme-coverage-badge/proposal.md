# Proposal: readme-coverage-badge

## Problem / Opportunity

The current archive guidance shows the coverage badge inside a `## Coverage`
section near the end of the README. Many projects display a coverage badge
prominently at the top of the README, directly under the title, where visitors
see it first. The scaffolded README template also contains no coverage badge
slot.

## Proposed Solution

The archive guidance will place the coverage badge directly after the README
title, followed by the prose totals and the reproduction command block, so a
project's README advertises its measured coverage at the top. The scaffolded
README template will include the badge line under the title as a documented,
project-level slot that the archive workflow fills in.

## Out of Scope

Coverage calculation, badge colour, the managed `<!-- givn:begin:coverage -->`
marker block, and the archive gate are unchanged.

## Open Questions

None.