# arc42 Documentation Update: start-and-proportionate-work

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goal or stakeholder class is added; the entry path gains explicit proportionate outcomes. | |
| 2 | 02 architecture-constraints | No | No platform, dependency, or compatibility constraint. | |
| 3 | 03 context-and-scope | Yes | The `givn instructions` interface changes its no-active guidance shape. | Updated 3.2: no-id guidance carries `next_command`, `next_skill`, and `overview`; `start` is a served policy id. |
| 4 | 04 solution-strategy | Yes | The lifecycle entry strategy now separates state discovery from request routing. | Updated 4.1 Lifecycle and Guidance rows. |
| 5 | 05 building-block-view | Yes | `cli::init`, `cli::instructions`, and `skills::targets` change responsibilities. | Updated 5.3. |
| 6 | 06 runtime-view | Yes | Init onboarding, startup guidance, and skill synchronization change. | Updated 6.1, 6.3, and 6.12. |
| 7 | 07 deployment-view | No | No packaging or deployment change. | |
| 8 | 08 crosscutting-concepts | Yes | Entry routing, generated skill retirement, and upgrade staging are cross-cutting. | Added 8.28 and updated 8.7. |
| 9 | 09 architecture-decisions | No | The decision is guidance and entry policy owned by the use cases, design, and chapter 08. It adds no component, authority, storage, or deployment boundary requiring an ADR. | |
| 10 | 10 quality-requirements | No | No new quality scenario or runtime gate. | |
| 11 | 11 risks-and-technical-debt | Yes | Stale generated explore output and a no-active change default are risks. | Added R-128 and R-129. |
| 12 | 12 glossary | Yes | New entry-routing terms are recorded in the standalone glossary. | Added Start, Direct work, and Proportional routing. |

## Notes

The change adds one policy instruction, one skill template, routing text/JSON, retirement cleanup, and documentation. It adds no dependency, configuration key, manifest artifact, shell command, or runtime classifier.

ADR action: NONE
ADR target: `givn/specs/obtain-effective-guidance`, `givn/specs/initialize-project`, `givn/specs/inspect-change-state`, `givn/specs/fragments`, and `docs/arc42/08-crosscutting-concepts.md`

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/obtain-effective-guidance, givn/specs/initialize-project, givn/specs/fragments, and docs/arc42/08-crosscutting-concepts.md
target_adr: null
replacement_adr: null
```

The decision fixes no component, authority, storage, or deployment boundary; it changes the entry guidance and the retirement of an old generated skill. The use cases, design, and chapter 08 are its canonical homes.

---

## Status

STATUS: DONE
