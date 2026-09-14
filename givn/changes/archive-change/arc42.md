# arc42 Documentation Update: archive-change

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | A stakeholder expectation now covers archiving on either backend. | The archiving-developer row states the same candidate, receipt, and repair contract on Git and Fossil. |
| 2 | 02 architecture-constraints | Yes | TC-21/TC-31/TC-32/TC-33 encoded the Git-only archive and the unimplemented Fossil backend. | TC-21 scopes Git-only to the managed upgrade; TC-31 describes witness-bounded publication on the selected backend (ADR-072); TC-32 says checkout root; TC-33 covers the missing `fossil` executable. |
| 3 | 03 context-and-scope | Yes | A new external executable (Fossil CLI) and a changed archive interface contract. | Fossil CLI entity added; Fossil checkout marker row and the in-scope list updated to the implemented archive path. |
| 4 | 04 solution-strategy | Yes | The SCM backend and archive-publication strategy rows changed. | Fossil selects the Fossil backend; publication is witness-bounded and exact on both backends (ADR-072). |
| 5 | 05 building-block-view | Yes | A new module owns the Fossil implementation; the seam and archive responsibilities change. | `fossil` building block added; `scm`, `cli::archive`, and `cli::migration` rows updated. |
| 6 | 06 runtime-view | Yes | The archive flow gained the backend seam, resume republication, and backend-neutral wording; the selection flow gained the Fossil backend. | 6.7 sequence and stages rewritten; 6.13 records the Fossil upgrade refusal; 6.16 flowchart and closing paragraph updated. |
| 7 | 07 deployment-view | Yes | Operating a project now requires the selected backend executable on PATH. | 7.3 records the `git`/`fossil` prerequisite and the suite dependency. |
| 8 | 08 crosscutting-concepts | Yes | Backend selection, refusal, and publication behavior are cross-cutting concepts shared by commands. | 8.18 updated for the Fossil backend and the witness-bounded publication contract. |
| 9 | 09 architecture-decisions | Yes | The witness-bounded publication contract is architecturally significant. | ADR-072 accepted and registered; chapter 09 gains its rationale paragraph. |
| 10 | 10 quality-requirements | Yes | A new portability quality scenario for archiving on Fossil. | Q-92 added. |
| 11 | 11 risks-and-technical-debt | Yes | New ADR consequences and the changed Fossil increment. | R-65 (witness-bounded publication), R-66 (fossil test dependency), R-64 wording, D-20 updated to the remaining managed-upgrade gap. |
| 12 | 12 glossary | Yes | The publication contract and the rejected-publication state are new domain terms; the Fossil note changed. | Witness-bounded publication and unpublished candidate added; closing note updated. |

## Notes

The Fossil backend covers the archive only. The managed upgrade still refuses
a Fossil checkout by name before any write; the permanent
`scm-backend-selection` scenario "A Fossil checkout is refused by name until
the Fossil backend is available" therefore stays true and is removed or reworded
by the `maintain-project` change when its Fossil path lands.

```text
ADR action: NEW_ADR
ADR target: ADR-072
```

## Status

STATUS: DONE
