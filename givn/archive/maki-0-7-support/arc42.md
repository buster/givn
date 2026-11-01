# arc42 Documentation Update: maki-0-7-support

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goal, stakeholder class, or success guarantee changes; the record gains fidelity only. | |
| 2 | 02 architecture-constraints | Yes | The Maki 0.7.0 host contract is now the verified constraint: `session_read`, `TaskStopped`, `SessionTitleChanged`, and the `tool_result`/`receipts` transcript shape. | Added the 0.7.0 constraint notes to the affected chapters; no new constraint class. |
| 3 | 03 context-and-scope | No | No new external system or actor; Maki remains one supported harness reporter. | |
| 4 | 04 solution-strategy | No | The solution strategy (one reporter per harness, one intake, best-effort recording) is unchanged. | |
| 5 | 05 building-block-view | Yes | The Maki reporter's module responsibilities grow (transcript, task list, span correlation, lane synthesis) inside the existing asset. | Updated the reporter asset row. |
| 6 | 06 runtime-view | Yes | The Maki capture order now includes the transcript/task-list reads, the deferred backgrounded-task close, and the session-end sweep. | Updated the Maki runtime row and follow-up paragraph. |
| 7 | 07 deployment-view | Yes | The pinned acceptance binary moves from Maki 0.6.3 to 0.7.0. | Updated the deployed test-tool note. |
| 8 | 08 crosscutting-concepts | Yes | Recorder content now covers answers, moved calls, session titles, subagent lanes, and reported cost. | Updated the reporters section. |
| 9 | 09 architecture-decisions | No | The schema extension stays inside the existing development-record contract; no new component, authority, storage boundary, or provider boundary warrants an ADR. | |
| 10 | 10 quality-requirements | No | No new runtime quality scenario; the change keeps the existing best-effort and bounded-record guarantees. | |
| 11 | 11 risks-and-technical-debt | Yes | R-90 is re-verified against 0.7.0 and gains the backgrounded-call and older-host residuals. | Updated R-90. |
| 12 | 12 glossary | Yes | Backgrounded call, session point entry, subagent lane, reported cost, and session read grant. | Added all five terms. |

## Notes

The change keeps one reporter per harness and one intake. It adds optional
payload fields (`session`, `backgrounded`, `usage.cost`), a `session` point
entry kind, and a per-event session override; all are backward compatible and
validated per event. The reporter remains a single Lua file because Maki loads
exactly one project plugin root. The pinned Maki binary is test infrastructure,
not a product dependency.

ADR action: NONE
ADR target: `givn/specs/inspect-development-history/usecase.md`, `docs/arc42/06-runtime-view.md`, `docs/arc42/08-crosscutting-concepts.md`, and `docs/arc42/11-risks-and-technical-debt.md`

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/inspect-development-history/usecase.md, docs/arc42/06-runtime-view.md, docs/arc42/08-crosscutting-concepts.md, and docs/arc42/11-risks-and-technical-debt.md
target_adr: null
replacement_adr: null
```

STATUS: DONE
