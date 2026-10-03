# arc42 Documentation Update: require-agent-target

> Change-level arc42 impact assessment. Not an ADR template.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or functional-requirement change; the CLI surface keeps its command set. | |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint; the existing target-selection contract is tightened, not replaced. | |
| 3 | 03 context-and-scope | No | No new external system or interface; the CLI keeps the same boundary and the context diagram's "Selected agent targets" node already describes the input. | |
| 4 | 04 solution-strategy | No | The "skills plus owned fenced blocks" strategy is unchanged; only the initialization precondition changes. | |
| 5 | 05 building-block-view | No | No module added, removed, or reassigned; `cli::init`'s responsibility row ("prompt agent targets") stays accurate. | |
| 6 | 06 runtime-view | Yes | 6.1 documents the `givn init` sequence. | The sequence now resolves the agent-target selection before the first write (step 2), refuses with a non-zero exit on an empty selection, and keeps the prompt unmarked. |
| 7 | 07 deployment-view | No | No deployment topology or artifact change; a single CLI binary stays the deliverable. | |
| 8 | 08 crosscutting-concepts | Yes | 8.7 documents skill and rule generation and the target-selection rule. | 8.7 now distinguishes the initialization precondition (explicit selection, refusal before any write) from the sync fallback (`opencode` when no `skills:` block exists). |
| 9 | 09 architecture-decisions | Yes | ADR-013 owns the agent-integration boundary whose target-selection contract is refined. | ADR-013 amended (`AMEND_ADR`): initialization requires a selected target set; the register and `adr/README.md` record the amendment. |
| 10 | 10 quality-requirements | Yes | Q-5 states the agent-ergonomics precondition. | Q-5 now names initialization as the point where "one or more agent targets" is enforced before the first write. |
| 11 | 11 risks-and-technical-debt | Yes | The ADR-013 amendment carries a compatibility consequence. | R-72 records that a scripted `givn init` relying on the implicit OpenCode default now exits non-zero until it passes `--agents`. |
| 12 | 12 glossary | No | No new domain term; "Agent target" and "Harness" already cover the vocabulary used by the refusal and the flow. | |

## Notes

The target-selection choice has real alternatives (refuse vs keep asking, and
prompt default vs unmarked), but it is a refinement of the existing
agent-integration boundary owned by ADR-013, not a new decision. The
qualification gate routes a refinement to an amendment: `AMEND_ADR` with
`target_adr: ADR-013`. No new record is created, so no new MADR file appears
in chapter 09 beyond the existing register entry.

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
    - "refuse on empty selection vs keep the prompt open (proposal D1); prompt starts unmarked vs OpenCode pre-marked (proposal D2) — both recorded in design.md ## Decisions"
  architectural_impact:
    - "the agent-integration selection contract owned by ADR-013 changes; every scripted init and every consumer must state a selection"
  durable_consequence:
    - "reversal would re-admit an implicit target set that existing scripts and generated guidance are being steered away from; the refusal is asserted in the permanent spec"
  lower_level_artifact:
    - "the rejection behaviour is specified in the initialize-project Gherkin and implemented in src/cli/init.rs; the durable boundary sentence needs the ADR because ADR-013 already owns the selection contract"
  existing_adr_check:
    - "searched the active register (docs/arc42/09-architecture-decisions.md) and docs/arc42/adr/ for target selection, agent integration, and initialization defaults; ADR-013 is the owning record, no other ADR states a conflicting default"
```

Existing-ADR check: the active register and the ADR directory were searched
for target selection, agent integration, and initialization defaults;
ADR-013 is the record that owns the selection contract, and no other active
ADR states a conflicting default.

Version freshness: the change selects no version-bearing technology — it adds
no dependency and upgrades none; the `dialoguer` version stays pinned by the
existing `Cargo.lock`.

---

## Status

STATUS: DONE
