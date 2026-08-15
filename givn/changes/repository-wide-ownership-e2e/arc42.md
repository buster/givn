# arc42 Impact Assessment: repository-wide-ownership-e2e

| # | Chapter | Affected | Reason | Summary |
|---|---|---|---|---|
| 1 | Introduction and Goals | Yes | Repository-owned acceptance is a maintainability and correctness goal. | Givn's own gates prove the review/archive workflow without a consumer checkout. |
| 2 | Architecture Constraints | Yes | The change fixes the verification boundary and requires isolated, deterministic CLI fixtures. | The fixture uses the compiled Givn binary, TempDir, local commands, and no network. |
| 3 | Context and Scope | No | No new external product interface is introduced. | Existing Givn CLI commands are exercised from their own repository. |
| 4 | Solution Strategy | Yes | Black-box CLI acceptance is the chosen verification strategy. | Review and archive are tested through subprocess output and exit status. |
| 5 | Building Block View | Yes | New fixture step modules and shared support are added to the test system. | CLI E2E bindings and shared fixture helpers are explicit components. |
| 6 | Runtime View | Yes | Review, archive, and pre-merge failure flows become executable. | The fixture lifecycle and CLI sequence are documented in design.md. |
| 7 | Deployment View | No | No release artifact or deployment topology changes. | The change is test support only. |
| 8 | Crosscutting Concepts | Yes | Isolation, strictness, hook configuration, and CLI evidence are cross-cutting test rules. | TempDir, GIVN_FEATURES, fail-on-skipped, and local hook commands are documented. |
| 9 | Architecture Decisions | Yes | The repository boundary and real-subprocess test boundary are deliberate decisions. | The change records why Givn owns these fixtures and why no consumer dependency is used. |
| 10 | Quality Requirements | Yes | Review receipt, archive publication, duplicate-title absence, and failure preservation are new quality scenarios. | The three Gherkin scenarios provide the acceptance signals. |
| 11 | Risks and Technical Debt | Yes | Fixture drift, pre-merge failure coverage, and child-process coverage require mitigation. | Risks and mitigations are recorded in design.md and the durable chapter. |
| 12 | Glossary | Yes | Ownership fixture, overlap disposition, retained scenario, and net-change receipt are used as domain terms. | Terms are added to the durable glossary. |

## Status

STATUS: DONE
