# arc42 Documentation Update: init-agent-target-selection

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No new goals or stakeholders; the change refines an existing interaction. | |
| 2 | 02 architecture-constraints | No | No new legal/tech/org constraints introduced. | |
| 3 | 03 context-and-scope | Yes | `givn init` gains an interactive agent-target prompt; the skill-generation surface becomes configurable. | Developer interaction row updated; skill-generation scope bullet updated; deferred-targets row clarified. |
| 4 | 04 solution-strategy | Yes | The agent-integration strategy row describes emits now conditional on target selection. | Updated the agent-integration row: configured agent targets instead of unconditional mirrors. |
| 5 | 05 building-block-view | Yes | `cli::init`, `config::overlay`, and `skills::targets` responsibilities change. | Updated module responsibilities for `cli::init`, `config::overlay` (new `load_skills_targets()`), `skills::targets` (target-driven emit). |
| 6 | 06 runtime-view | Yes | `givn init` and `givn skills sync` flows change. | Reworked flows 6.1 (prompt + persist + target-selected emit) and 6.8 (load configured targets). |
| 7 | 07 deployment-view | No | No distribution/deployment change. | |
| 8 | 08 crosscutting-concepts | Yes | Skill-generation section documents the configurable target set and the `skills:` config block. | Rewrote section 8.7 to describe target selection, per-target emit, and command emit gating. |
| 9 | 09 architecture-decisions | Yes | New ADR: configurable agent target selection with opencode default. | Added ADR-036 (context, options, outcome, consequences, confirmation). |
| 10 | 10 quality-requirements | No | No new quality scenarios; behaviour is covered by the executable spec. | |
| 11 | 11 risks-and-technical-debt | Yes | ADR-036 introduces a migration consequence for existing all-target projects. | Added R-12 (silent narrowing to opencode after upgrade). |
| 12 | 12 glossary | Yes | New domain term introduced. | Added "Agent target". |

## Notes

The `docs/arc42/README.md` index needs no structural change (no new chapters,
no renumbering). No diagrams were added or modified; the existing Mermaid
context diagram in chapter 3 remains accurate.

---

## Status

STATUS: DONE