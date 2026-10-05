# arc42 Documentation Update: add-omp-agent-target

> Change-level arc42 impact assessment. Not an ADR template.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or functional-requirement change; the command set is unchanged. | |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint; the target set grows inside the existing ADR-013 boundary. | |
| 3 | 03 context-and-scope | Yes | The technical-context interface table enumerates the agent-target directories and each harness hook boundary. | The "Agent target directories" row gains `.omp` and per-target commands; a new "omp hook boundary" row records the `.omp/hooks/pre/givn-attribution.js` contract. |
| 4 | 04 solution-strategy | Yes | The strategy table's "Agent integration" and "Commit attribution" rows enumerate the targets and adapters. | "Agent integration" gains `.omp`; "Commit attribution" names the omp adapter. |
| 5 | 05 building-block-view | Yes | The production-responsibility table names what `cli::skills` synchronizes and which integration assets exist. | `cli::skills` now names commands and attribution integrations for every command-owning target; the embedded attribution-assets row names the omp integration. |
| 6 | 06 runtime-view | Yes | 6.1a documents the commit-attribution ordering per harness, and 6.12 documents what `givn skills sync` writes. | 6.1a gains the omp ordering (hook load, `tool_call` rewrite, model/thinking source, partial-context fallback); 6.12 no longer says "opencode commands" only. |
| 7 | 07 deployment-view | No | No topology or artifact change; a single CLI binary stays the deliverable. | |
| 8 | 08 crosscutting-concepts | Yes | 8.7 is the canonical description of skill and rule generation and of the rules-policy ignore patterns. | 8.7 lists the `.omp` skill and command paths and the omp attribution integration, adds `.omp` to the named target set, and adds the `.omp` ignore prefixes. |
| 9 | 09 architecture-decisions | Yes | ADR-013 owns the agent-integration boundary whose target directory list changes. | ADR-013's Consequences sentence names the `.omp` location and per-target commands, and the chapter-09 register records the third amendment (`AMEND_ADR`, no new record). |
| 10 | 10 quality-requirements | Yes | Q-5 states the target-selection contract; the attribution scenarios enumerate the harnesses. | Q-5 covers the `.omp` target and per-target commands; a new Q-96 records the omp attribution scenario. |
| 11 | 11 risks-and-technical-debt | Yes | The generated hook's revision-verb gate and the assumed omp extension contract are accepted, durable limitations. | R-77 records the missed-revision-spelling limitation; R-78 records that the automated E2E fakes the omp contract and that a live smoke run is the validation step. |
| 12 | 12 glossary | Yes | The "Agent target", "Harness", and "Slash command" terms enumerate the target locations. | "Agent target" gains `.omp`; "Harness" names omp; "Slash command" covers any command-owning target's `commands/` directory. |

## Notes

Adding a fourth agent target refines the agent-integration boundary owned by
ADR-013 rather than replacing it. The qualification gate routes a refinement to
an amendment (`AMEND_ADR`, target `ADR-013`); no new MADR file is created.

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
    - "add a fourth named target vs. reuse the existing `agents` target only; reusing `agents` would deliver skills but no commands and no attribution, so the developer would still have no honest target for omp — recorded as proposal D2 and as the design's target decision"
  architectural_impact:
    - "the set of generated destinations ADR-013 fixes for target selection changes; every consumer reads the same `skills.targets` contract and the same per-target layout rule"
  durable_consequence:
    - "reversal would remove files from `.omp/` and invalidate the generated hook path; the layout is asserted by the permanent `initialize-a-project` and `attribute-commits` specs"
  lower_level_artifact:
    - "the concrete paths and the hook behaviour are owned by `design.md`, the Gherkin delta, and `src/skills/targets.rs`; the durable one-line boundary sentence belongs to ADR-013, which already owns the selection contract and was updated in place"
  existing_adr_check:
    - "searched the active register and `docs/arc42/adr/` for agent-target and integration boundaries; ADR-013 is the owning record, and its Consequences sentence was amended rather than duplicated"
```

Version freshness: no version-bearing technology is selected. The generated
hook runs on the harness's own runtime (omp/Bun) and adds no dependency; the
test driver reuses the `node` already required by the existing OpenCode
integration E2E; the Rust change uses the pinned crates in `Cargo.lock`.

---

## Status

STATUS: DONE
