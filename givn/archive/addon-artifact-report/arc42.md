# arc42 Documentation Update: addon-artifact-report

## Impact assessment

This change extends the `givn addons list` report with an observed-state check
against each addon's declared materialization. It adds no new command, no new
lifecycle artifact, and no new external interface.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | Goals, stakeholders, and quality attributes are unchanged; the change refines one existing command's report. | |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint is introduced or removed. | |
| 3 | 03 context-and-scope | No | The external boundary is unchanged: `givn addons list` already reads the project directory. No new external system or user-facing surface appears. | |
| 4 | 04 solution-strategy | No | The addon register stays the single source of addon facts; only where materialization is declared moves onto the entry. No strategy-level approach changes. | |
| 5 | 05 building-block-view | Yes | `cli::addons` gains an owned declaration and a reporting responsibility. | The `cli::addons` row now names addon materialization, the shared declaration, and the report with per-difference suggestions; a new internal component `cli::addon_report` implements the comparison. |
| 6 | 06 runtime-view | Yes | The `givn addons list` flow gains filesystem probes, a managed-block reader, suggestions, and an exit-status rule. | Section 6.12 records the observed states, the four differences and their literal suggestions, the no-write rule (including the change lock), and the exit-status split. |
| 7 | 07 deployment-view | No | Packaging, the single binary, and embedding are unchanged; no runtime dependency is added. | |
| 8 | 08 crosscutting-concepts | No | No cross-cutting concept (logging, error handling policy, configuration precedence) changes. The report reuses the existing effective-addon resolution. | |
| 9 | 09 architecture-decisions | Yes | The change qualifies one new architectural decision. | ADR-076 `addon-materialization-declaration` records that an addon's filesystem effect is declared on its registry entry and read by both consumers; the chapter-09 register and `adr/README.md` link it. |
| 10 | 10 quality-requirements | No | No new quality target or measurable scenario is introduced; the report is deterministic and local. | |
| 11 | 11 risks-and-technical-debt | Yes | ADR-076 records one "Bad, because" consequence that becomes a standing risk. | Each registry entry must declare an explicit `None` and its author must confirm the addon writes nothing; the compiler cannot verify that judgement. |
| 12 | 12 glossary | Yes | Two terms are introduced to keep the stored setting and observed state distinct. | Glossary gains **Addon materialization** (anti-term: installation) and **Materialization state** (`present`/`missing`/`partial`, kept separate from `enabled`/`disabled`). |

## Notes

Qualification recorded once, per the Arc42 addon instruction:

```text
Architectural commitment: AddonSpec owns an addon's filesystem effect; the scaffold routines and the report are consumers of one declaration.
Real alternatives: registry-declared materialization; a reporting-only check table; deriving paths by re-running scaffolds into a probe directory.
Durable consequence: reverting to a side table restores two copies of every declared path and loses the compile-time exhaustiveness guarantee, requiring a re-check of all scaffold routines.
Existing-decision search: docs/arc42/09-architecture-decisions.md and docs/arc42/adr/ (including archive/) checked. ADR-023 governs managed config refresh, not addon path declaration; ADR-026 establishes arc42 as an addon but does not place its files. No record covers this boundary.
ADR action: NEW_ADR
ADR target: docs/arc42/adr/ADR-076-addon-materialization-declaration.md
```

Chapters 01, 02, 03, 04, 07, 08, and 10 are untouched.

---

## Status

STATUS: DONE
