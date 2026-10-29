# arc42 Documentation Update: default-all-addons

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal or stakeholder class changes. | |
| 2 | 02 architecture-constraints | Yes | The add-on default posture becomes an explicit project constraint. | Added TC-45. |
| 3 | 03 context-and-scope | No | No external system or user-facing surface is added. | |
| 4 | 04 solution-strategy | No | The existing manifest, registry, and config path carry the change. | |
| 5 | 05 building-block-view | Yes | `cli::init` and `cli::addons` change their default behavior. | Updated both rows. |
| 6 | 06 runtime-view | Yes | Init now writes explicit opt-outs and the add-on list reads effective defaults. | Updated the init/add-on flow. |
| 7 | 07 deployment-view | No | No deployment or packaging change. | |
| 8 | 08 crosscutting-concepts | Yes | The add-on default is a cross-cutting configuration concept. | Updated 8.11 and 8.26. |
| 9 | 09 architecture-decisions | No | The decision is project configuration policy and does not qualify for an ADR. | |
| 10 | 10 quality-requirements | Yes | A fresh project's default capabilities are a quality scenario. | Added Q-121. |
| 11 | 11 risks-and-technical-debt | Yes | An all-enabled default activates more gates and generated guidance. | Added R-126. |
| 12 | 12 glossary | Yes | The add-on default posture becomes a project term. | Added the term. |

## Notes

The change is configuration policy: every add-on is enabled unless a project
writes an explicit `false`. Existing explicit `false` values are preserved; no
project configuration is rewritten. The outline report and the
architecture-principles guidance become available in a fresh project, and both
remain individually disableable. The 12-row assessment is complete.

ADR action: NONE
ADR target: `docs/arc42/08-crosscutting-concepts.md` and the `enable-optional-addons` feature under `initialize-project`

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/initialize-project/enable-optional-addons.feature and docs/arc42/08-crosscutting-concepts.md
target_adr: null
replacement_adr: null
```

The decision is observable project-configuration behavior with an existing
component. It does not fix a new authority, deployment, or storage boundary, so
the feature and the cross-cutting concept chapter are its canonical homes.

---

## Status

STATUS: DONE
