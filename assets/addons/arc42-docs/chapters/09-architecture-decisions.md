# 9. Architecture Decisions

This file is the chapter-09 register/index for architecture decisions. It is
not an ADR body and not a container for multiple ADRs. One architecturally
significant decision gets exactly one ADR, stored as a separate Markdown file
under `docs/arc42/adr/ADR-NNNN-short-slug.md`.

The ADR directory has its own index at `docs/arc42/adr/README.md`. Create and
maintain the directory, files, and index entries manually; this asset does not
claim automatic scaffolding for individual ADRs. The current binary uses the
fixed `docs/arc42/` root and does not implement an `arc42.output_dir` override.

## Register

Maintain one row or link per ADR. Include the ADR ID, short title, current
status, date, and a link to the individual file. Do not paste an ADR body or a
second MADR section into this file.

| ADR | Title | Status | Date | File |
|---|---|---|---|---|

## Individual ADR contract

Before creating a file or adding a register entry, pass the Qualification gate
and evaluate all four criteria in the individual ADR's `## Qualification`
section with a concrete, decision-specific justification for each. The gate
passes only when at least one of the following is checked with a clear
justification: **Hard to reverse**, **Shapes quality attributes**,
**Structural**, or **Must be shared**. If no item can be justified, do not
create an ADR. System relevance, implementation effort, a long discussion, and
a known pattern alone do not qualify. Logging, internal naming, formatting,
local refactorings, and interchangeable HTTP clients without quality-attribute
implications are generally not ADRs. System-wide error handling qualifies only
when it is an enforced cross-cutting contract.

Each individual file uses the complete MADR structure, including
`## Qualification`, `## Context and Problem Statement`, `## Decision Drivers`,
`## Considered Options`, `## Decision Outcome`, `### Consequences`,
`### Confirmation`, `## Pros and Cons of the Options`, and
`## More Information`, with real frontmatter.
Allowed statuses are `proposed`, `rejected`, `accepted`, `deprecated`, and
`superseded by ADR-NNNN`. A superseded status must name a concrete existing
replacement ID, such as `superseded by ADR-027` or `superseded by ADR-0042`;
placeholders are invalid in completed ADRs. Keep `docs/arc42/adr/README.md` and
this register linked to the current or historical file.

## Archive

An archived historical ADR keeps its ID and filename and is moved to
`docs/arc42/adr/archive/ADR-NNNN-short-slug.md`. Update both indexes when the
path changes. Do not move ADRs into `givn/archive/<change>/`; that path is for
change artifacts, not durable architecture decisions.
