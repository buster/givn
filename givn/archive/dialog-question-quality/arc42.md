# arc42 Impact Assessment: dialog-question-quality

This change adds one canonical dialog-policy instruction, references it from
the interactive surfaces, and adds a policy-instruction fallback to the
`givn instructions` CLI. It does not change the artifact graph, archive gates,
or runtime services.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal or quality goal changes; the dialog policy is authoring guidance, not a system goal. |
| 2 | Architecture Constraints | No | No legal, organizational, or platform constraint changes. |
| 3 | Context and Scope | Yes | `givn instructions dialog` is a new externally visible CLI output; the interface table gains its row. |
| 4 | Solution Strategy | No | The change reuses the existing asset cascade and renderer; no strategy changes. |
| 5 | Building-Block View | Yes | The `cli::instructions` responsibility now includes non-artifact policy instructions. |
| 6 | Runtime View | Yes | Section 6.3 documents the non-artifact fallback branch in the `givn instructions` flow. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | Yes | Section 8.3a records the dialog policy as a normative authoring source; 8.3b records the policy-instruction JSON shape and the lookup boundary. |
| 9 | Architecture Decisions | No | The policy-instruction lookup is not a qualified ADR candidate: it fixes no component boundary with a durable migration consequence, and its reversal is a single-function change. Canonical destination: chapter 8.3b. |
| 10 | Quality Requirements | Yes | The executable scenarios verify CLI retrieval, policy content, and the generated-guidance pointers. |
| 11 | Risks and Technical Debt | Yes | R-67 records the dialog-rule drift risk and its pointer/scan mitigation. |
| 12 | Glossary | Yes | `Dialog policy` is a new project term. |

## Status

STATUS: DONE
