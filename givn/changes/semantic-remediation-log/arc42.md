# arc42 Documentation Update: semantic-remediation-log

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | Goals and stakeholders are unchanged; the change deepens existing review assurance. | |
| 2 | 02 architecture-constraints | No | No new constraint; Git history was already a runtime assumption. | |
| 3 | 03 context-and-scope | No | The user-facing surface (CLI commands) is unchanged; one existing command gains a persisted side artifact. | |
| 4 | 04 solution-strategy | No | The strategy (deterministic gates + evidence worklist) is extended, not replaced. | |
| 5 | 05 building-block-view | Yes | New module `src/spec/remediation_log.rs` inside `spec::retrieval`. | Module row updated to name the remediation-log responsibility. |
| 6 | 06 runtime-view | Yes | Blocking review gains log-writing, gate-validation, and after-state steps. | `check review` flow list and semantic stage description extended. |
| 7 | 07 deployment-view | No | No deployment surface changes; the log is a project-local file. | |
| 8 | 08 crosscutting-concepts | Yes | New persisted artifact, gate rules, Git stamp convention. | New §8.5h Remediation Log and Git Stamp. |
| 9 | 09 architecture-decisions | Yes | Qualifying new decision about where remediation evidence lives. | ADR-068 proposed; register paragraph added. |
| 10 | 10 quality-requirements | No | Auditability improves within existing quality scenarios; no new scenario recorded yet. | |
| 11 | 11 risks-and-technical-debt | Yes | The log reader joins the existing line-oriented parser debt class (D-10). | New debt row D-11 recording the reader's initial line-oriented state. |
| 12 | 12 glossary | Yes | Six new domain terms. | Remediation log, remediation log entry, review run record, Git stamp, verbatim scenario text, append-only log recorded. |

## Notes

ADR action: NEW_ADR
ADR target: docs/arc42/adr/ADR-068-remediation-log.md

The evaluation aggregation over archived logs (labelled dataset tooling) is
explicitly out of scope for this change and left to a future one; ADR-068 only
fixes the record format.

---

## Status

STATUS: DONE
