# Design Review: apply-review-fixes

## Grilling log
| # | Branch | Question | Recommended | Outcome |
|---|---|---|---|---|
| 1 | arc42 | ASCII art diagrams use box-drawing chars in ch 05, 08 | Replace with plain-text | Agreed |
| 2 | E2E fidelity | License @e2e reads source files, bypasses CLI | Reclassify as non-@e2e | Agreed |
| 3 | Testability | deduplicate @e2e Then-steps not falsifiable | Concrete assertions (placeholder hash) | Agreed |
| 4 | Missing scenarios | eject error format lacks known ids vs check | Both produce known ids list | Agreed |
| 5 | Missing scenarios | Coverage gate extraction needs build test? | Build-time sufficient | Agreed |
| 6 | Risk | Archive e2e gate absolute vs relative | Absolute check (option a), spec already reflects | Agreed |
| 7 | Scope | Lint out-of-scope vs path validation change | Clarify — uniform validation, core rules unchanged | Agreed |
| 8 | Testability | verify-e2e hook scenario vacuously passes | Add positive assertion (hash-presence check) | Agreed |

## Resolved by codebase exploration
| Branch | Finding |
|---|---|
| Testability | .fail_on_skipped() present at features_runner.rs:20 |
| Testability | run_givn() invokes real binary via CARGO_BIN_EXE_givn |
| Tech choices | has_e2e_scenarios() at parse.rs:201; dual find_change_dir confirmed |
| Tech choices | validate_id private to new.rs — extraction plan correct |
| E2E fidelity | All @e2e scenarios (except license) drive real CLI subprocess |
| Tech choices | archive.rs:176-197 confirms e2e gate gap |

## Open questions
None.

## Architecture documentation (arc42) check
| # | Chapter | Reviewer own Yes/No | arc42.md Yes/No | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Yes | Yes | Yes |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | Yes |
| 3 | 03 context-and-scope | No | No | Yes | N/A |
| 4 | 04 solution-strategy | No | No | Yes | N/A |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — box-drawing chars replaced with plain-text |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes |
| 7 | 07 deployment-view | No | No | Yes | N/A |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — box-drawing chars replaced with plain-text |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — license ADR-034 added |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes |
| 12 | 12 glossary | No | No | Yes | N/A |
- [x] All 12 rows independently assessed against proposal.md/design.md before opening arc42.md.
- [x] No row is "Match? = No" with reviewer saying "Yes" and arc42.md saying "No"/omitting.
- [x] Every non-trivial architecture/technology decision in design.md has a MADR entry in chapter 09.
- [x] Every MADR "Bad, because..." consequence has a counterpart in chapter 11.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening
| Artifact | Change summary |
|---|---|
| proposal.md | Clarified out-of-scope — path validation applies uniformly, lint rules unchanged (Q7) |
| specs/deduplicate-change-dir-detection.feature | Split @e2e scenario into two: one with valid hashes (assert success), one with placeholder hash (assert failure with specific stderr); added positive hash-presence assertion to verify-e2e hook scenario (Q3, Q8) |
| specs/validate-artifact-id-input.feature | Added "And stderr lists known artifact ids" to eject scenario matching check scenario format (Q4) |
| design.md | Updated eject code sketch with full error message including known artifact ids list (Q4) |
| docs/arc42/05-building-block-view.md | Replaced Unicode box-drawing chars with plain-text `\|` + `+--` indented tree (Q1) |
| docs/arc42/08-crosscutting-concepts.md | Replaced Unicode box-drawing chars with plain-text `\|` + `+--` indented tree (Q1) |
| specs/choose-project-license.feature | Verified already non-@e2e, no User Interaction Inventory — no edit needed (Q2) |

## Sign-off
- [x] All branches walked.
- [x] All questions resolved.
- [x] design.md reflects decisions reached.
- [x] specs/*.feature updated for any missing scenarios.
- [x] givn lint exits 0 or 2.
- [x] Architecture documentation (arc42) check completed above.

DESIGN-REVIEW: PASS
