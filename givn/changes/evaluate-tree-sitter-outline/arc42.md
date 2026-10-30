# arc42 Documentation Update: evaluate-tree-sitter-outline

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goal or stakeholder class changes. | |
| 2 | 02 architecture-constraints | No | The evaluation adds test-only dependencies; the production constraint is unchanged. | |
| 3 | 03 context-and-scope | No | No public interface or command changes. | |
| 4 | 04 solution-strategy | No | The production outline strategy is unchanged. | |
| 5 | 05 building-block-view | No | No production module is added; the evaluation module is test-only. | |
| 6 | 06 runtime-view | No | The outline command path is unchanged. | |
| 7 | 07 deployment-view | No | The release binary does not link Tree-sitter. | |
| 8 | 08 crosscutting-concepts | Yes | The outline analysis boundary now records a test-only evaluation and a retain decision. | Updated 8.26: evaluation scope, dev dependencies, and the decision record. |
| 9 | 09 architecture-decisions | No | The decision is evaluation evidence owned by the outline use case and the evaluation record; it fixes no production boundary. | |
| 10 | 10 quality-requirements | No | No new runtime quality scenario; the regression scenario protects the existing report. | |
| 11 | 11 risks-and-technical-debt | Yes | Grammar/toolchain supply and mistaking evaluation for parity are risks. | Added R-130 and R-131. |
| 12 | 12 glossary | Yes | The evaluation term is recorded. | Added Tree-sitter evaluation. |

## Notes

The change adds dev dependencies, a test-only comparison module, one regression scenario, and a durable evaluation record. It does not add a production adapter, CLI flag, language, or schema change.

ADR action: NONE
ADR target: `givn/specs/assess-code-structure`, `docs/outline-tree-sitter-evaluation.md`, and `docs/arc42/08-crosscutting-concepts.md`

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/assess-code-structure and docs/outline-tree-sitter-evaluation.md
target_adr: null
replacement_adr: null
```

The decision is evaluation evidence for the existing advisory report, not a
production component or deployment boundary.

---

## Status

STATUS: DONE
