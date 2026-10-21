# Proposal: add-independent-rationale-review

## Release note

Givn requires an independent fresh-context reviewer for every readability rationale, without adding a human approval gate.

## Use-Case Context

- Use-case ID: `verify-change` (extends `readability-review-gate`)
- Ideation topic: `readability-experiment`
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`

## Problem / Opportunity

The gate's rationale floor accepted a self-attested `Owner:` line. A concrete-sounding rationale written by the same agent could pass without anyone else reading it. That is the easy escape the gate was meant to prevent.

## Proposed Solution

A rationale now records `Author:`, `Reviewed-by:`, and `Review verdict:`. The reviewer must differ from the author and must record `accept`; a `reject` verdict fails. The reviewer is a fresh-context subagent that reads the rationale and the finding evidence. No human approval gate is introduced: the owner remains the accountable persona, but the independent check is a separate agent context.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `readability-review-gate` | `EXTEND givn-workflow-enforcement` | `EXTEND readability-review-gate` | This is a refinement of the existing gate's rationale validation, not a new capability or use case. |

## Out of Scope

- Human approval, signatures, or cryptographic reviewer identity.
- A new CLI command; the gate remains inside `givn check review`.
- Changing the finding, FIXED/JUSTIFIED, or PASS/FAIL vocabulary.

## Open Questions

### D1 — Who counts as an independent reviewer?

- Status: answered 2026-10-20 (user)
- Question: must the reviewer be a human, or can it be a fresh-context subagent?
- Options:
  1. **Fresh-context subagent** — the orchestrator spawns a reviewer that did not write the rationale; it records `Reviewed-by` and a verdict. Gain: independent check without a forced human gate. Cost: procedural, not cryptographic. In practice: the reviewer reads the rationale and says accept or reject.
  2. **Human approval** — a person signs every rationale. Gain: stronger accountability. Cost: a forced human gate on every finding.
- Disposition: answered — option 1. The gate checks that `Reviewed-by` differs from `Author` and that the verdict is `accept`; it does not require a human.
