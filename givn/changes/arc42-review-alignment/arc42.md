# arc42 Documentation Update: arc42-review-alignment

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change improves lifecycle assurance but does not change givn's goals or stakeholders. | |
| 2 | 02 architecture-constraints | No | No legal, organisational, or technology constraint is introduced. | |
| 3 | 03 context-and-scope | No | The review assets do not add an external system or user-facing product boundary. | |
| 4 | 04 solution-strategy | Yes | The lifecycle strategy now includes a post-implementation Arc42 conformance comparison when the addon is enabled. | Added the architecture-conformance review strategy alongside the existing design-review gate. |
| 5 | 05 building-block-view | No | No production module, service, or durable architecture building block is added. | |
| 6 | 06 runtime-view | No | The change adds review guidance but does not change a product runtime flow. | |
| 7 | 07 deployment-view | No | No packaging, process, or deployment topology changes. | |
| 8 | 08 crosscutting-concepts | Yes | Review guidance becomes a cross-cutting documentation-consistency control for Arc42-enabled changes. | Documented the conditional review block, evidence table, and remediation loop. |
| 9 | 09 architecture-decisions | Yes | Choosing a human post-implementation conformance review is an architecture/process decision. | Added an ADR explaining why the check is prompt-layer and evidence-based rather than automatic source inference. |
| 10 | 10 quality-requirements | Yes | The change adds a quality scenario requiring architecture documentation, design, tasks, and implementation to agree. | Added an Arc42 conformance quality scenario. |
| 11 | 11 risks-and-technical-debt | Yes | The change addresses the risk of stale durable architecture documentation and silent implementation drift. | Added the stale-Arc42/implementation-drift risk and its review mitigation. |
| 12 | 12 glossary | Yes | The workflow gains the domain term "Arc42 implementation conformance". | Added definitions for the conformance review and architecture fact. |

## Notes

The implementation is prompt-layer only. The durable documentation records the
new review obligation; it does not attempt to infer architecture automatically
from source code.

Diagrams in chapter content remain Mermaid-only.

---

## Status

STATUS: DONE
