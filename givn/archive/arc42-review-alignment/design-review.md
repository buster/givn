# Design Review: arc42-review-alignment

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | Scope | Does the change include the durable Arc42 facts it introduces? | Include factual updates to affected chapters and the index; do not alter the chapter structure or scaffold. | Agreed. Proposal and design now state this boundary. |
| 2 | Tech choices | Is a prompt-layer review sufficient, or should source code be parsed? | Keep the human evidence review; architecture facts and trade-offs cannot be inferred reliably from source text. | Agreed. The limitation is explicit in ADR-035 and risk R-11. |
| 3 | Missing scenarios | Are all observable delivery surfaces covered? | Cover instructions, ejected template, and generated skill/command targets; keep disabled Arc42 as a configuration variant. | Agreed. Added the generated-guidance scenario and matrix row. |
| 4 | Testability | Can each scenario fail before the change is complete? | Use the strict Cucumber runner and capability-specific steps with concrete file/content assertions. | Agreed. `.fail_on_skipped()` is documented; the ejected path is validated from CLI output. |
| 5 | E2E fidelity | Does each e2e scenario drive the real boundary? | Use the real `givn` subprocess and assert exit status, stdout, and ejected/generated output. | Agreed. CLI is the real interface; no repository-only substitute is used. |
| 6 | Interaction coverage | Does every inventory entry map to one e2e scenario and driver? | Keep one row per CLI action and a concrete `run_givn` mechanism. | Agreed. Three inventory entries map to three matrix rows and three e2e scenarios. |
| 7 | Risk | What is the most likely failure? | Prompt drift or a superficial manual review. | Mitigated by four aligned assets, generated-target coverage, explicit clean marker, and R-11. |
| 8 | Arc42 | Does the independent 12-row impact assessment match `arc42.md`? | Yes: rows 4, 8, 9, 10, 11, and 12; all other rows No with reasons. | Confirmed. All 12 rows are present and agree. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Asset delivery | `review.md`, its instruction, skill, and command are separate cascade assets; generated skill and command files are produced by `givn skills sync`. |
| Runner | `tests/features_runner.rs` uses `.fail_on_skipped()`, and the configured e2e command uses the `@e2e` tag filter. |
| CLI boundary | `givn eject review:template` writes `givn/artifacts/review/review.md`; the capability step checks the path reported in stdout before reading it. |
| Configuration | Fresh projects write command overrides to `givn/commands.yaml`; effective configuration loads it after `givn/config.yaml`. |
| Mechanical enforcement | `givn` detects only the `REVIEW: PASS` marker. Arc42 conformance remains a human review obligation and is not represented as a new binary gate. |

## Open questions

None. The remaining human judgment is an intentional design decision recorded
in ADR-035, not an unresolved implementation question.

## Architecture documentation (arc42) check

The independent assessment was completed from proposal.md, the delta feature,
and design.md before comparing the change-level marker. The marker agrees with
every row.

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | N/A — goals and stakeholders unchanged |
| 2 | 02 architecture-constraints | No | No | Yes | N/A — no new constraint |
| 3 | 03 context-and-scope | No | No | Yes | N/A — no new system boundary |
| 4 | 04 solution-strategy | Yes | Yes | Yes | Yes — strategy and key pattern updated |
| 5 | 05 building-block-view | No | No | Yes | N/A — no production building block |
| 6 | 06 runtime-view | No | No | Yes | N/A — no product runtime flow |
| 7 | 07 deployment-view | No | No | Yes | N/A — no deployment change |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — conditional block and remediation semantics documented |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — full MADR ADR-035 added |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes — Q-26 added with human sign-off semantics |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-10 and R-11 cover drift and manual-review risk |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — conformance terms added |

- [x] All 12 rows assessed independently against proposal.md/design.md before reading arc42.md.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md saying "No" or omitting the row.
- [x] Every non-trivial architecture/technology decision in design.md has a corresponding MADR entry in chapter 09.
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in chapter 11.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.
- [x] Arc42 index updated with the new review control.
- [x] Chapter 8.14 is ordered after the existing 8.13 reverse-engineering section.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| proposal.md | Clarified that factual updates to affected durable chapters are in scope while structure/scaffold changes are not. |
| design.md | Added durable chapter files, generated target synchronization, locked runner version, and commands.yaml configuration detail. |
| specs/arc42-review-alignment/arc42-review-alignment.feature | Added generated guidance scenario and explicit clean-marker assertions. |
| docs/arc42/09-architecture-decisions.md | Expanded ADR-035 to the full MADR template. |
| docs/arc42/11-risks-and-technical-debt.md | Added the manual-review omission risk. |
| docs/arc42/08-crosscutting-concepts.md | Moved the new 8.14 section after existing section 8.13. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing generated-guidance scenario.
- [x] `givn lint --change arc42-review-alignment` parses successfully and reports only the expected `@wip` status (exit 2).
- [x] Architecture documentation check completed above.

DESIGN-REVIEW: PASS
