# arc42 Documentation Update: keep-agent-prompt-open

> Change-level arc42 impact assessment. Not an ADR template.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or functional-requirement change; the CLI command set is unchanged. | |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint; the target-selection contract is refined, not replaced. | |
| 3 | 03 context-and-scope | No | No new external system or interface; the context diagram's "Selected agent targets" node already describes the input. | |
| 4 | 04 solution-strategy | No | The "skills plus owned fenced blocks" strategy is unchanged. | |
| 5 | 05 building-block-view | No | No module added, removed, or reassigned; `cli::init`'s responsibility row stays accurate. | |
| 6 | 06 runtime-view | Yes | 6.1 documents the `givn init` sequence. | Step 2 now records that the interactive prompt opens with `.opencode` marked and re-asks until a target is marked, while a non-terminal empty selection refuses. |
| 7 | 07 deployment-view | No | No topology or artifact change; a single CLI binary stays the deliverable. | |
| 8 | 08 crosscutting-concepts | Yes | 8.7 documents the target-selection rule. | 8.7 separates the interactive re-ask from the non-terminal refusal. |
| 9 | 09 architecture-decisions | Yes | ADR-013 owns the agent-integration boundary whose interaction wording changes. | The existing ADR-013 amendment sentence and its register rows now record the re-ask and the pre-marked `opencode` default (`AMEND_ADR`, no new record). |
| 10 | 10 quality-requirements | Yes | Q-5 states the agent-ergonomics precondition. | Q-5 now records the re-ask for the interactive path and the refusal for the non-terminal path. |
| 11 | 11 risks-and-technical-debt | No | No new consequence: R-72 already covers the scripted refusal, which this change keeps. | |
| 12 | 12 glossary | No | No new domain term; "Agent target" and "Harness" cover the vocabulary. | |

## Notes

The choice is a refinement of the existing agent-integration boundary owned by
ADR-013: the interactive prompt re-asks instead of closing, and the scripted
refusal stays. The qualification gate routes a refinement to an amendment
(`AMEND_ADR`, target `ADR-013`); no new MADR file is created.

```text
ADR action: AMEND_ADR
ADR target: docs/arc42/adr/ADR-013-agent-integration.md

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: AMEND_ADR
canonical_artifact: docs/arc42/adr/ADR-013-agent-integration.md
target_adr: ADR-013
replacement_adr: none
evidence:
  alternatives:
    - "interactive re-ask vs closing refusal (proposal D1 covers the piped analogue); pre-marked opencode vs unmarked list — both recorded in design.md"
  architectural_impact:
    - "the agent-integration selection contract owned by ADR-013 changes for the terminal surface; every consumer must still select a target"
  durable_consequence:
    - "reversal would re-introduce either the lost dialog or the implicit fallback the previous change removed; the re-ask is asserted in the permanent spec"
  lower_level_artifact:
    - "the prompt loop is implemented in src/cli/init.rs and specified in the initialize-project Gherkin; the durable boundary sentence belongs to ADR-013, which already owns the selection contract"
  existing_adr_check:
    - "searched the active register and docs/arc42/adr/ for initialization defaults and target selection; ADR-013 is the owning record and its amendment sentence was updated in place"
```

Version freshness: no version-bearing technology is selected — no dependency
change; `dialoguer` stays pinned by the existing `Cargo.lock`.

---

## Status

STATUS: DONE
