# arc42-docs — givn-test-harness

## 12-row assessment

| # | Chapter | Affected? | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | The harness is a testing tool, not a change to givn's goals or stakeholders. |
| 2 | Architecture Constraints | **Yes** | New constraints added (HC-1 through HC-5): Python 3.12+/uv runtime, sandlock for isolation, ralph for agent orchestration, opencode LLM configuration, isolated workspace requirement. Added §2.6 and CV-9. |
| 3 | Context and Scope | **Yes** | New external entity `givn-harness` added to context diagram, external entities table, technical interfaces table, in-scope list, and out-of-scope list. |
| 4 | Solution Strategy | No | The harness does not change givn's core strategy. It is an adjunct testing tool. |
| 5 | Building Block View | **Yes** | New §5.5 documenting `harness/` package structure: modules (cli, workspace, sandbox, runner, evaluator, judge, report, index, types) and their responsibilities, plus test layout (behave features/steps). |
| 6 | Runtime View | **Yes** | New §6.12 documenting the autoresearch loop: workspace creation → givn init → sandlock sandbox construction → ralph agent execution → evaluator checks → LLM judge scoring → reporting. Learn-then-confine detail included. |
| 7 | Deployment View | No | The harness runs locally as a development tool; no deployment changes. |
| 8 | Cross-cutting Concepts | **Yes** | New §8.14 documenting autoresearch loops, workspace isolation, agent execution, sandbox confinement (learn-then-confine + default-allow network), evaluation (automated checks + LLM-as-judge), reporting (per-run markdown + cross-run index), and the "no digital twin for LLM" principle. |
| 9 | Architecture Decisions | **Yes** | Five new ADRs: ADR-034 (separate Python package), ADR-035 (behave + step-body discipline), ADR-036 (sandlock learn-then-confine), ADR-037 (LLM-as-judge scoring), ADR-038 (no digital twin for LLM). |
| 10 | Quality Requirements | **Yes** | Six new quality scenarios (Q-24 through Q-29) covering: end-to-end harness execution, complete-successful outcome, incomplete-failed outcome, cross-run comparability, learn mode, and confine mode. |
| 11 | Risks and Technical Debt | **Yes** | New §11.4 with five harness-specific risks (R-9 through R-13): LLM nondeterminism flakiness, agent timeout/incomplete artifacts, overly restrictive sandlock profiles, un-interceptable kernel paths, biased LLM judge. Two new deliberate debts (D-9, D-10): stale learn-then-confine profiles, circular dependency for harness's own testing. |
| 12 | Glossary | **Yes** | Nine new terms: autoresearch loop, givn-harness, ralph, sandlock, learn-then-confine, LLM-as-judge, variant, cross-run index, workspace. |

## Status

STATUS: DONE
