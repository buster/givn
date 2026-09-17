# arc42 Impact Assessment: seed-artifact-lifecycle

This change adds a change-seed content schema and status convention, edits the
ideation, proposal, and archive guidance, and adds one advisory seed-status
finding to the static lint command. It does not change the artifact graph,
archive gates, or runtime services.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal changes; seed hygiene is authoring guidance plus one advisory check. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | No | No interface row changes; `givn lint` output gains an advisory line, which chapter 6.4 documents. |
| 4 | Solution Strategy | No | The existing cascade, renderer, and lint pipeline are reused. |
| 5 | Building-Block View | Yes | The `cli::lint` responsibility gains the default-scope advisory seed-status check. |
| 6 | Runtime View | Yes | Section 6.4 documents the seed scan, its labels, and its exit-code neutrality. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | New section 8.19 records the seed lifecycle convention and the advisory check. |
| 9 | Architecture Decisions | No | The advisory check and status convention are not qualified ADR candidates: no component boundary with a durable migration consequence. Canonical destination: chapters 6.4 and 8.19. |
| 10 | Quality Requirements | No | No quality attribute changes; the delta scenarios are functional checks. |
| 11 | Risks and Technical Debt | Yes | R-46's mitigation now names the advisory seed-status check. |
| 12 | Glossary | Yes | `Seed status` is a new term recorded beside the existing `Change seed` entry. |

## Status

STATUS: DONE
