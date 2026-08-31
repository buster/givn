# Arc42 impact assessment: release-override-hardening

| Chapter | Affected | Reason |
|---|---|---|
| 01 Introduction & Goals | No | No new goal; hardening an existing enforcement guarantee. |
| 02 Constraints | No | No new technical, legal, or organisational constraint. |
| 03 Context & Scope | No | No new user-facing surface; the refusal policy is internal enforcement. |
| 04 Solution Strategy | No | The strategy rows already state installed-build strictness; this change completes and documents it. |
| 05 Building Block View | No | No module boundary changes; the completeness net is a test inside `spec::retrieval`. |
| 06 Runtime View | No | No runtime stage changes; the refusal path is unchanged behaviour. |
| 07 Deployment View | No | No deployment boundary changes. |
| 08 Cross-Cutting Concepts | Yes | § 8.5a gains the mechanically complete refusal policy: registry + exemptions, the request-class table, and the fail-safe rule. |
| 09 Architecture Decisions | No | The predecessor change routed installed-build strictness `CANONICAL_ARTIFACT` (spec + design + glossary, no ADR); this change refines that same decision and its canonical home stays ch. 08 § 8.5a. ADR-028 is cited only for the non-deactivatable-gate property. |
| 10 Quality Requirements | No | Existing quality scenarios already pin the installed-build behaviour; the new scenarios live in the permanent spec. |
| 11 Risks & Technical Debt | No | The model-swap exception is accepted and documented in ch. 08; no new debt entry (the fail-safe keeps the gate alive). |
| 12 Glossary | No | The policy reuses established terms (fixture request, installed build, model-location request); no new aggregate. |

STATUS: DONE
