# Design Review: givn-test-harness

## Grilling log
| # | Branch | Question | Recommended | Outcome |
|---|--------|----------|-------------|---------|
| 1 | Testability | How are harness specs verified through givn lifecycle gates? | Rust step defs as subprocess | harness/ gets own givn/ with behave verify.command |
| 2 | Arc42 | ASCII art diagrams in chapters 5 and 8? | Convert to Mermaid | Agreed |
| 3 | Missing scenarios | Error-path scenarios missing? | Add sandlock unavailable, ralph missing, etc. | Agreed |
| 4 | Missing scenarios | Zero-artifact agent run? | Add scenario | Agreed |
| 5 | Testability | Harness needs givn/ directory? | Yes, scaffold own givn/ | Agreed |
| 6 | Risk | First RED phase without harness code? | Scaffold empty package, NotImplementedError | Agreed |
| 7 | Scope | Shopping list task file? | Part of this change | Agreed |
| 8 | Tech choices | Sandlock hardcoded path? | Git source with SHA | Agreed — git + subdirectory + env fallback |
| 9 | E2E fidelity | Duplicate @e2e for same action? | Merge stdout into main e2e | Agreed |

## Resolved by codebase exploration
| Branch | Finding |
|--------|---------|
| Architecture documentation | All 12 chapters exist with real content. arc42.md 12-row table matches independent assessment row-for-row. All 5 new ADRs (034-038) have corresponding entries in chapter 11 (R-9 through R-13, D-9, D-10). No contradictions between chapter content and design.md. |
| Interaction Coverage | All 5 inventory entries match matrix rows. Valid driving mechanisms throughout. |
| Testability | All three strict-mode mechanisms named and concrete (step-body discipline, mechanical audit script, single-scenario run command). All Then-steps assert concrete values. |

## Open questions
None

## Architecture documentation (arc42) check
| # | Chapter | Subagent Yes/No | arc42.md Yes/No | Match? | Content matches design.md? |
|---|---------|-----------------|-----------------|--------|---------------------------|
| 1 | 01-introduction-and-goals | No | No | Yes | N/A |
| 2 | 02-architecture-constraints | Yes | Yes | Yes | Yes |
| 3 | 03-context-and-scope | Yes | Yes | Yes | Yes |
| 4 | 04-solution-strategy | No | No | Yes | N/A |
| 5 | 05-building-block-view | Yes | Yes | Yes | Yes (ASCII fixed to Mermaid) |
| 6 | 06-runtime-view | Yes | Yes | Yes | Yes |
| 7 | 07-deployment-view | No | No | Yes | N/A |
| 8 | 08-crosscutting-concepts | Yes | Yes | Yes | Yes (ASCII fixed to Mermaid) |
| 9 | 09-architecture-decisions | Yes | Yes | Yes | Yes — 5 MADRs (034-038) |
| 10 | 10-quality-requirements | Yes | Yes | Yes | Yes |
| 11 | 11-risks-and-technical-debt | Yes | Yes | Yes | Yes — all ADR consequences reflected (R-9 through R-13, D-9, D-10) |
| 12 | 12-glossary | Yes | Yes | Yes | Yes |

- [x] All 12 rows independently assessed before opening arc42.md.
- [x] No row where subagent said "Yes" and arc42.md said "No"/omitted.
- [x] Every non-trivial tech decision in design.md has a MADR entry in chapter 09.
- [x] Every MADR "Bad, because..." consequence has a counterpart in chapter 11.
- [x] No chapter claimed as "updated" is still scaffolded placeholder.

## Changes made during hardening
| Artifact | Change summary |
|----------|---------------|
| design.md | Updated sandlock dependency to git source (`[tool.uv.sources]` with SHA `6a4ec66`, subdirectory `python/`, `SANDLOCK_PATH` fallback). Added explicit note that `harness/` gets its own `givn/` directory with behave-based `verify.command`. Added RED-phase scaffolding strategy section (empty package first, NotImplementedError bodies, non-sandlock/non-ralph scenarios first, @e2e via `verify.e2e_command`). Updated version freshness notes for sandlock. |
| specs/test-harness/test-harness.feature | Merged "User observes live progress on stdout" into "User runs the harness end-to-end" (added stdout Then-steps to main e2e, removed standalone stdout scenario). Added 5 new scenarios: zero-artifact agent run (all checks FAIL, judge defaults to 0), sandlock unavailable fallback, ralph binary not found, LLM judge unparseable JSON (score 0), SIGINT workspace preservation. All tagged `@givn.added @wip`. |
| docs/arc42/05-building-block-view.md | Converted crate structure ASCII directory tree (Unicode box-drawing characters) to Mermaid flowchart. Converted harness directory tree to Mermaid flowchart. |
| docs/arc42/08-crosscutting-concepts.md | Converted givn/ directory layout ASCII tree to Mermaid flowchart. |

## Sign-off
- [x] All branches walked.
- [x] All open questions resolved.
- [x] design.md reflects decisions reached.
- [x] specs/*.feature updated for missing scenarios.
- [x] givn lint exits 0 or 2 (exit 2 — all @wip, no parse errors).
- [x] Architecture documentation (arc42) check completed.

DESIGN-REVIEW: PASS