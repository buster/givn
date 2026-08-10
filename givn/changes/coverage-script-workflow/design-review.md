# Design Review: coverage-script-workflow

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | Scope | Does the change make the README, commands file, and archive output agree? | Use project-owned measurement and merge scripts; derive both badges and the summary only from the fresh merged report. | Resolved: design and scenarios require one merged source. |
| 2 | Tech choices | Should givn merge Cobertura counters itself? | No for the new contract. Delegate merge semantics to the project script; retain only a temporary legacy fallback for existing projects. | Resolved: `merge_command` is authoritative. |
| 3 | Missing scenarios | Can stale output or a failed merge publish a false result? | Remove old output before merge, require fresh output, and roll back README, reports, and specs on any failure. | Resolved: fresh-output guard and failure scenario added. |
| 4 | Testability | Can each scenario fail before implementation? | Use strict Cucumber execution, undefined/unimplemented steps for setup proof, and deterministic shell fixtures with exact counters. | Resolved: dedicated e2e/non-e2e modules and exact assertions. |
| 5 | E2E fidelity | Are the e2e scenarios using the real interface? | Yes. Invoke the compiled `givn` subprocess in isolated temporary projects; assert generated files and command result as the CLI outcome. | Resolved. |
| 6 | Interaction coverage | Does every inventory entry have exactly one e2e scenario and a real driver? | Use one CLI scenario for init guidance and one for archive publication. | Resolved: matrix matches both inventory entries. |
| 7 | Risk | What is the highest-risk behavioral regression? | A stale or incorrectly merged report reaches the README. | Resolved: stale output removal and fixture output that differs from old Rust arithmetic. |
| 8 | Compatibility | What happens to projects with the old two coverage commands? | Accept both legacy fields temporarily and preserve the old fallback while projects move their commands into scripts. | Resolved: schema accepts the legacy pair without changing the new path. |
| 9 | README ownership | How are top badges kept at the top without overwriting user content? | Use a title-aware `coverage-badge` managed block and a separate `coverage` summary block; remove only recognized legacy coverage badge lines. | Resolved: helper tests and exact README assertions added. |
| 10 | Zero branches | Is `0/0` branch data a 0% claim? | No. Render `n/a` while retaining raw counters in the summary. | Resolved in renderer and durable docs. |
| 11 | Architecture docs | Which durable decisions and risks change? | Update chapters 5, 6, 8, 9, 11, and 12 plus the change-level 12-row assessment. | Resolved and recorded in `arc42.md`. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Configuration | `givn/commands.yaml` is loaded after `givn/config.yaml`; the new coverage keys therefore belong in the commands file without changing the overlay graph. |
| README | `agents_block::apply` appends blocks, so a title-aware helper is required for the badge placement rule. |
| Rollback | Archive already backs up README and permanent specs; coverage now adds configured report state and README restoration uses the existing backup bytes. |
| Strict mode | `tests/features_runner.rs` already uses `.fail_on_skipped()`, so the change reuses the proven runner. |

## Open questions

None. The project-owned merge script defines the tool-specific union semantics;
givn validates the emitted report rather than guessing how overlapping source
files should be combined.

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | Yes |
| 2 | 02 architecture-constraints | No | No | Yes | Yes |
| 3 | 03 context-and-scope | No | No | Yes | Yes |
| 4 | 04 solution-strategy | Yes | Yes | Yes | Yes |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes |
| 7 | 07 deployment-view | No | No | Yes | Yes |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes |
| 10 | 10 quality-requirements | No | No | Yes | Yes |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes |
| 12 | 12 glossary | Yes | Yes | Yes | Yes |

- [x] All 12 rows assessed independently against proposal.md/design.md.
- [x] No row is an unreviewed mismatch.
- [x] Each non-trivial decision has ADR-029 or ADR-030 in chapter 09.
- [x] New failure modes have R-13 through R-16 in chapter 11.
- [x] Updated chapters contain substantive documentation, not placeholders.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `design.md` | Added fresh-output, transaction, zero-branch, legacy compatibility, internal primitive, and step-separation decisions. |
| `coverage-script-workflow.feature` | Added exact merged counters, script names, badge placement, and merge-failure outcomes. |
| `docs/arc42/` | Updated building blocks, runtime, cross-cutting coverage contract, decisions, risks, and glossary. |
| `src/` and assets | Implemented the reviewed command, report, README, and rollback boundaries. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the hardening decisions.
- [x] `specs/**/*.feature` contains the required scenarios.
- [x] `givn lint --change coverage-script-workflow` exits 2 for the intended `@wip` scenarios and has no parse errors.
- [x] Architecture documentation check completed.

DESIGN-REVIEW: PASS
