# arc42 Documentation Update: local-only-rules

> Change-level arc42 impact assessment. Not an ADR template.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal or stakeholder change; the repository becomes an explicit `versioned` consumer. | |
| 2 | 02 architecture-constraints | Yes | Constraint 2.7 owns the managed migration allowlist and force-staging rule. | 2.7 now forces only under `versioned`; under `local-only` the revision carries scaffold, ignore-block changes, and removals, and never `AGENTS.md`. |
| 3 | 03 context-and-scope | No | No external system or interface change. | |
| 4 | 04 solution-strategy | No | No strategy change. | |
| 5 | 05 building-block-view | Yes | A new production block and updated responsibilities. | Added `skills::rules` and updated `cli::check`, `cli::commit` rows. |
| 6 | 06 runtime-view | No | No new runtime flow beyond the existing command paths. | |
| 7 | 07 deployment-view | No | No deployment change. | |
| 8 | 08 crosscutting-concepts | Yes | Section 8.7 owns generated rule generation. | Documented the policy scalar, both ignore destinations, the local-only transition, and the never-staged `AGENTS.md` rule. |
| 9 | 09 architecture-decisions | No | No qualified ADR; the policy applies existing guidance and SCM boundaries. | |
| 10 | 10 quality-requirements | Yes | Q-57 owns the release-safety expectation for generated files. | Q-57 now states the policy-aware revision content. |
| 11 | 11 risks-and-technical-debt | Yes | R-33 owns the force-staging risk. | R-33 now scopes force-staging to the `versioned` policy and names the `AGENTS.md` exclusion. | |
| 12 | 12 glossary | No | The policy terms were merged during the `local-only-integrations` handoff; this change introduces no new term. | |

## Notes

The durable records that named force-staging unconditionally (arc42 constraint
2.7, Q-57, R-33) are amended in this change. The managed-upgrade scenario text
keeps its title and now asserts the policy-appropriate revision shape through
its step implementation.

```text
ADR action: CANONICAL_ARTIFACT
ADR target: docs/arc42/08-crosscutting-concepts.md section 8.7

qualification: NOT_APPLICABLE
alternatives: FAIL
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/08-crosscutting-concepts.md section 8.7
target_adr: none
replacement_adr: none
```

The policy is workflow configuration applied through existing boundaries; the
qualification procedure routes it to the generated guidance and its arc42
description instead of an ADR.

---

## Status

STATUS: DONE
