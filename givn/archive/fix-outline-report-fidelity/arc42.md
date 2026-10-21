# Arc42 assessment: fix-outline-report-fidelity

| # | Chapter | Affected | Reason / change |
|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | FR-46 now states that the report exposes resolution labels and JSON targets; the goal itself is unchanged. |
| 2 | 02 architecture-constraints | No | No new dependency, runtime, platform, or organizational constraint; the existing offline, deterministic, production-only constraints are unchanged. |
| 3 | 03 context-and-scope | Yes | The report interface now exposes each call's resolution label and JSON target; the scope statement was updated in the external-interface table. |
| 4 | 04 solution-strategy | Yes | The shared resolution strategy is now shape-aware instead of name-based; the code-structure report row was updated. |
| 5 | 05 building-block-view | Yes | `outline::resolve` now owns call-shape matching; adapter facts, module type items, and production line accounting are described in the building-block rows. |
| 6 | 06 runtime-view | Yes | §6.19 now describes shape-aware resolution, module member types, resolution-labelled leaves, JSON `target`, and span-based production accounting. |
| 7 | 07 deployment-view | No | No deployment topology, artifact, or process changed. |
| 8 | 08 crosscutting-concepts | Yes | The outline crosscutting section now states shape-aware resolution, `name_only` non-expansion, JSON targets, module member types, and production-only line accounting. |
| 9 | 09 architecture-decisions | No | No ADR qualified: the correction refines the existing neutral analyzer/report boundary and is fully owned by this design, the Gherkin spec, and chapters 5/6/8. |
| 10 | 10 quality-requirements | Yes | Q-119 now includes the unknown-receiver `name_only` leaf and resolution labels; the existing determinism and hint-safety scenarios stay unchanged. |
| 11 | 11 risks-and-technical-debt | Yes | R-106 and R-121 now describe shape-aware resolution and `name_only`; the risk that a reader mistakes a guess for a resolved call is reduced. |
| 12 | 12 glossary | Yes | Added Call shape and Call resolution; updated Outline, Call tree, Outline report, Source unit, and Production source. |

## ADR qualification

The change was checked as one candidate: a language-neutral call-shape fact and
resolver rule. Verdict: `NOT_QUALIFIED` / `CANONICAL_ARTIFACT` at `design.md`.
The decision refines the existing neutral analyzer port and outline-report
contract; its complete rationale lives in `design.md`, the outline-report
Gherkin capability, and chapters 5, 6, 8, and 12. No existing ADR is amended or
superseded.

## Status

STATUS: DONE
