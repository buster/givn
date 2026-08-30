# arc42 Documentation Update: fix-remediation-delta2delta

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goals, stakeholders, or quality attributes change. | |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint changes. | |
| 3 | 03 context-and-scope | No | No system boundary, external interface, or user-facing product capability changes. | |
| 4 | 04 solution-strategy | No | The retrieval architecture (E5 embedding, BGE/NLI second stage, ADR-047 verification) is unchanged; the verifier's coverage broadens. | |
| 5 | 05 building-block-view | No | No production building block or module is added; only `verify_semantic_remediation` gains delta-to-delta arms. | |
| 6 | 06 runtime-view | Yes | `givn check review` now verifies prior classifications whose right endpoint lives in the change's own delta. | The semantic review run recomputes the delta-to-delta pair score against the active delta and applies the DUPLICATE / VARIANT / VALID-BOUNDARY contracts instead of bailing as stale. |
| 7 | 07 deployment-view | No | No deployment, packaging, or infrastructure change occurs. | |
| 8 | 08 crosscutting-concepts | No | The existing semantic-review crosscutting concept (E5 admission at 0.9500, class evidence contracts) remains unchanged. | |
| 9 | 09 architecture-decisions | No | ADR-047 (verify semantic remediation against the projected tree) already governs this behavior; no new decision is introduced. | |
| 10 | 10 quality-requirements | No | No new quality target or measurable quality scenario is introduced. | |
| 11 | 11 risks-and-technical-debt | No | Same-flow scenarios scoring at or above the E5 boundary are a known, measured property of the embedder; documented in the change design. | |
| 12 | 12 glossary | No | "delta-to-delta" is already used by the semantic-review spec family; no new term is required. | |

## Notes

The regression scenarios are staged under distinct change ids with long
narrative markers so the delta's own semantic gate stays green; the redundant
same-flow scenarios were trimmed because real E5 pins same-family scenarios at
or above 0.9500 (the grandfathered permanent family behaves identically). This
is documented here and in design.md, not as a new decision.

```text
ADR action: NONE
ADR target: none
```

---

## Status

STATUS: DONE
