# Arc42 Impact: fix-opencode-llm-attribution

| Chapter | Affected | Reason |
|---|---|---|
| 01 Introduction and Goals | No | The existing commit-attribution goal is refined without adding a new project goal or stakeholder. |
| 02 Architecture Constraints | Yes | OpenCode payload shape, Claude transcript model lookup, and full-command environment scope are explicit integration constraints. |
| 03 Context and Scope | Yes | The OpenCode plugin and Claude hook boundaries are clarified as external integration interfaces, including shell-wrapper propagation. |
| 04 Solution Strategy | No | The existing generated-adapter and shared Git protocol strategy remains unchanged. |
| 05 Building Block View | Yes | The existing attribution CLI now owns Claude transcript model extraction, full-command environment export, and managed co-author stamping. |
| 06 Runtime View | Yes | The OpenCode hook sequence and Claude transcript-to-PreToolUse-to-RTK-to-Git sequence are documented. |
| 07 Deployment View | No | Generated paths and deployment footprint remain unchanged. |
| 08 Crosscutting Concepts | Yes | The process-scoped attribution protocol now includes managed co-author trailer normalization and full Bash-command scope. |
| 09 Architecture Decisions | No | The payload extraction is a local implementation correction owned by the feature and design; no ADR qualifies. |
| 10 Quality Requirements | Yes | Selected-model attribution receives executable OpenCode and Claude quality scenarios. |
| 11 Risks and Technical Debt | Yes | OpenCode payload drift, Claude transcript/model-switch drift, and shell-wrapper environment loss are recorded with regression coverage. |
| 12 Glossary | Yes | `Model identity` and the managed co-author attribution are defined as precise domain terms. |

The change is a correction to an existing agent integration, not a new
architecture boundary.

## ADR Qualification

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/fix-opencode-llm-attribution/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Read the current nested model payload or retain the obsolete top-level lookup."
    ],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "The complete rationale is local plugin implementation detail documented in design.md and covered by the feature."
    ],
    "existing_adr_check": [
      "No active or archived ADR governs this local OpenCode payload extraction."
    ]
  }
}
```

## Status

STATUS: DONE
