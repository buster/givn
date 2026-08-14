# arc42 Impact Assessment: prose-consolidation

This change consolidates the normative authoring policy and changes the
generated guidance surface. It does not change givn command behavior or the
runtime system under test.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | Yes | Agent-authoring consistency becomes an explicit quality goal for generated guidance. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | Yes | `givn instructions` exposes one canonical authoring-policy output consumed by agents. |
| 4 | Solution Strategy | No | The implementation reuses the existing asset cascade and renderer; no system strategy changes. |
| 5 | Building-Block View | Yes | Instruction, skill, and command asset responsibilities are clarified and the existing instruction/cascade blocks are updated. |
| 6 | Runtime View | No | Existing command execution flows remain unchanged; only emitted guidance content changes. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | Normative authoring guidance and project override resolution are consolidated as a cross-cutting concern. |
| 9 | Architecture Decisions | Yes | ADR-038 records the single normative authoring-policy source decision. |
| 10 | Quality Requirements | Yes | Executable scenarios verify canonical policy output, generated-guidance parity, and override behavior. |
| 11 | Risks and Technical Debt | Yes | R-17 records the remaining drift risk and its verification mitigation. |
| 12 | Glossary | Yes | `Canonical authoring policy` is a new project term. |

## Status

STATUS: DONE
