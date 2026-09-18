# arc42 Documentation Update: retire-lint-command-adr

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal or stakeholder change. | |
| 2 | 02 architecture-constraints | No | No constraint change. | |
| 3 | 03 context-and-scope | No | No interface or external-entity change; the command name is unchanged. | |
| 4 | 04 solution-strategy | No | No strategy change. | |
| 5 | 05 building-block-view | No | No module change. | |
| 6 | 06 runtime-view | No | No runtime change. | |
| 7 | 07 deployment-view | No | No packaging change. | |
| 8 | 08 crosscutting-concepts | No | The exit-code contract is unchanged; chapter 8.8 still describes it. | |
| 9 | 09 architecture-decisions | Yes | The register loses a non-qualifying record; the naming contract moves to the specification. | ADR-027 is deleted (never archived); ADR-019 and the README reference the spec; the active count is 24. |
| 10 | 10 quality-requirements | No | No quality scenario change. | |
| 11 | 11 risks-and-technical-debt | No | No risk change. | |
| 12 | 12 glossary | No | No term change. | |

## Audit note

- Record: ADR-027 (Name the static check `givn lint`).
- Failing test: product-behavior exclusion (also single-change reversible and
  no multi-change constraint).
- Destination: `givn/specs/verify-change/usecase.md` Rules plus the lint
  capability specification (`givn/specs/verify-change/lint-command.feature`).
- Disposition: deleted; not moved to `docs/arc42/adr/archive/` because a
  record that fails the gate is not a historical ADR.

---

## Status

STATUS: DONE
