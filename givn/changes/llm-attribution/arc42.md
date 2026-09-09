# Arc42 Impact: llm-attribution

| Chapter | Affected | Reason |
|---|---|---|
| 01 Introduction and Goals | Yes | Adds commit-attribution visibility and non-blocking usability requirements. |
| 02 Architecture Constraints | Yes | Adds checked-in Git hook, local activation, and process-scoped runtime constraints. |
| 03 Context and Scope | Yes | Adds Git and LLM harnesses as external interfaces. |
| 04 Solution Strategy | Yes | Adds the shared-hook plus harness-adapter strategy. |
| 05 Building Block View | Yes | Adds attribution CLI behavior and embedded integration assets. |
| 06 Runtime View | Yes | Adds initialization, hook dispatch, stamping, and commit sequencing. |
| 07 Deployment View | Yes | Adds the checked-in and generated project footprint plus local activation boundary. |
| 08 Crosscutting Concepts | Yes | Defines best-effort attribution, trailer normalization, identity, and hook ownership. |
| 09 Architecture Decisions | Yes | Records that the choice is routed to the feature/design canonical artifacts rather than a new ADR. |
| 10 Quality Requirements | Yes | Adds non-blocking, compatibility, and attribution visibility scenarios. |
| 11 Risks and Technical Debt | Yes | Adds custom-hook, harness-payload, and clone-activation risks. |
| 12 Glossary | Yes | Adds attribution and Git-hook vocabulary. |

The durable chapters were updated surgically. No ADR was created: the
qualification route is `CANONICAL_ARTIFACT` with `design.md` as the complete
technical rationale and the feature file as the observable contract.

## Status

STATUS: DONE
