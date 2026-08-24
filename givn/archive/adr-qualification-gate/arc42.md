# Arc42 Impact Assessment: ADR Qualification Gate

This change tightens the distinction between architecture rationale, current
Arc42 facts, executable specifications, implementation design, and ordinary
project documentation. It changes documentation and authoring governance, not
the Givn runtime.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---:|---|:---:|---|---|
| 1 | 01 introduction-and-goals | Yes | The change adds a documentation-quality goal: the active architecture contract must remain small, current, and free of duplicate or non-ADR records. | Update the quality-goal and acceptance-ownership text to state the current-register and canonical-source objective. |
| 2 | 02 architecture-constraints | Yes | The repository authoring constraints change: qualification is a strict five-dimension AND gate, one ADR covers one independent decision, and non-ADR material has one canonical home. | Add the strict gate, one-decision-per-ADR, canonical-single-source, project-override, and non-runtime-enforcement constraints. |
| 3 | 03 context-and-scope | Yes | Existing user-facing CLI guidance changes: `givn instructions` must expose the effective qualification and routing policy without adding a runtime interface. | Update the CLI guidance interface and identify the live source cascade and project override boundary. |
| 4 | 04 solution-strategy | No | The implementation strategy and product architecture remain unchanged; only documentation ownership is clarified. | |
| 5 | 05 building-block-view | No | No production component, module, or responsibility boundary changes. | |
| 6 | 06 runtime-view | No | No command execution flow, lifecycle transition, or runtime sequence changes. | |
| 7 | 07 deployment-view | No | No binary, feature build, model artifact, host, or deployment topology changes. | |
| 8 | 08 crosscutting-concepts | Yes | ADR qualification and single-source documentation ownership are cross-cutting authoring and maintenance rules. | Document strict Boolean routing, supporting-only `Must be shared`, source ownership, project overrides, and the non-runtime-enforcement boundary. |
| 9 | 09 architecture-decisions | Yes | The qualification gate changes and the existing ADR register receives the deterministic ID-by-ID migration. | Apply the manifest, retain independent decisions, amend ADR-062 into ADR-059, remove non-ADR records, preserve only genuine superseded history, and leave ADR-063 proposed. |
| 10 | 10 quality-requirements | Yes | Documentation quality now includes preventing false ADR qualification, unsupported evidence, duplicate records, and guidance drift. | Add executable scenarios for every gate, strict verdict/routing, amendment/supersession, source/register integrity, and effective/generated parity. |
| 11 | 11 risks-and-technical-debt | Yes | ADR inflation, source drift, migration loss, and the lack of semantic register validation are risks addressed or bounded by this change. | Record the migration and parity mitigations and state that current integrity hooks do not semantically validate the register. |
| 12 | 12 glossary | Yes | The change introduces shared terms for mandatory dimensions, verdicts, routing, amendment, supersession, and canonical ownership. | Define the new qualification, routing, register, and source-ownership terms once in the durable glossary. |

## Notes

- Arc42 chapter impact is not treated as an automatic ADR requirement.
- The MADR checklist is evidence for the strict mandatory dimensions, not an
  at-least-one escape hatch; `Must be shared` is supporting evidence only and
  never qualifies a candidate by itself.
- The qualification and canonical-source rules are default authoring guidance;
  project overrides may replace the effective policy, and no runtime validator
  enforces it.
- An ADR contains the rationale for one durable architectural decision. Current
  architecture facts are maintained in the relevant Arc42 chapter, observable
  behavior in Gherkin, and implementation details in code or `design.md`.
- ADR action: NONE
- ADR target: none
- Non-ADR records are not copied into an ADR archive. Useful information is
  moved once to its canonical location; redundant records are removed.
- `STATUS: DONE` records that this change-level impact assessment is complete;
  the durable chapter, register, source, and ADR edits listed above are present
  in the working tree and are verified by the change scenarios and final review.
- No new diagram was introduced.

## Status

STATUS: DONE
