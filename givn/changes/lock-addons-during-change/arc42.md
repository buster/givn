# arc42 Documentation Update: lock-addons-during-change

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goal or stakeholder class changes. | |
| 2 | 02 architecture-constraints | No | The existing configuration authority and offline constraints are unchanged. | |
| 3 | 03 context-and-scope | Yes | The `givn addons`, `givn new`, and `givn init --force` interfaces gain lock behavior. | Updated the command/interface notes. |
| 4 | 04 solution-strategy | No | The solution strategy is unchanged; the lock enforces existing policy. | |
| 5 | 05 building-block-view | Yes | A configuration-lock component is added. | Added `config::lock` and updated CLI responsibilities. |
| 6 | 06 runtime-view | Yes | New, add-on mutation, forced init, review, and archive flows change. | Updated the relevant runtime sections. |
| 7 | 07 deployment-view | No | No packaging or deployment change. | |
| 8 | 08 crosscutting-concepts | Yes | Add-on configuration ownership and drift detection are cross-cutting. | Updated 8.11 and added the lock lifecycle. |
| 9 | 09 architecture-decisions | No | The decision is configuration policy owned by the use cases and design; no new component, authority, storage, or deployment boundary warrants an ADR. | |
| 10 | 10 quality-requirements | No | No new runtime quality scenario; the lock protects existing gates. | |
| 11 | 11 risks-and-technical-debt | Yes | Deliberate lock deletion and the TTY override are residual risks. | Added R-132/R-133. |
| 12 | 12 glossary | Yes | Add-on lock and operator override terms. | Added both. |

## Notes

The change adds a change-local YAML lock, CLI guards, a TTY-scoped operator override, and review/archive drift checks. It adds no lifecycle artifact, dependency, or JSON schema change.

ADR action: NONE
ADR target: `givn/specs/initialize-project`, `givn/specs/verify-change`, `docs/reference.md`, and `docs/arc42/08-crosscutting-concepts.md`

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/initialize-project, givn/specs/verify-change, docs/reference.md, and docs/arc42/08-crosscutting-concepts.md
target_adr: null
replacement_adr: null
```

The decision extends existing configuration policy and review gates; it does not fix a new component, authority, storage, or deployment boundary.

---

## Status

STATUS: DONE
