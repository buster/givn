# arc42 Documentation Update: strengthen-outline-guidance

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change adds no product goal or stakeholder class; it strengthens existing guidance for the same actors. | |
| 2 | 02 architecture-constraints | No | No new constraint; the existing supported/unsupported boundary and the advisory-report rule are restated. | |
| 3 | 03 context-and-scope | No | No external system, user-facing surface, or boundary is added. | |
| 4 | 04 solution-strategy | No | The change stays inside the existing asset cascade and conditional-rendering strategy. | |
| 5 | 05 building-block-view | No | No module, component, or ownership boundary is added or moved. | |
| 6 | 06 runtime-view | No | The instruction render path and the archive merge path are unchanged. | |
| 7 | 07 deployment-view | No | No packaging, deployment, or runtime dependency changes. | |
| 8 | 08 crosscutting-concepts | Yes | The outline guidance is a cross-cutting serving and readability concept; its rendering boundary, comprehension test, and advisory/finding distinction live in section 8.26a. | Updated 8.26a: both outline blocks render behind `{{#if addons.outline}}`, the review template carries no outline verdict phrase, and the comprehension and domain-signature tests are recorded. |
| 9 | 09 architecture-decisions | No | The decision is guidance policy owned by the effective-guidance and review features plus chapter 08; it fixes no new component, authority, storage, or deployment boundary. | |
| 10 | 10 quality-requirements | No | No new quality scenario is introduced; the readability contract is review guidance, not a new runtime gate. | |
| 11 | 11 risks-and-technical-debt | No | The existing advisory-outline risk stays documented; the change removes misleading phrases rather than adding debt. | |
| 12 | 12 glossary | No | Existing terms are reused (outline report, behavior batch, review finding); no new term is introduced. | |

## Notes

The change edits rendered guidance and durable documentation. It adds no source
module, no schema, and no new external interface.

ADR action: NONE
ADR target: `docs/arc42/08-crosscutting-concepts.md` and the effective-guidance, review, and synchronization features

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/obtain-effective-guidance/serve-extended-rules.feature and docs/arc42/08-crosscutting-concepts.md
target_adr: null
replacement_adr: null
```

The decision is observable guidance behavior owned by an existing use case
plus Gherkin. It fixes no component, authority, storage, or deployment
boundary, so the feature and the cross-cutting concept chapter are its
canonical homes.

---

## Status

STATUS: DONE
