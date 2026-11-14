# Arc42 Documentation Update: enforce-ise-e2e-boundary

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change adds enforced behavior for an existing `ise` capability; it introduces no goal, stakeholder, or quality goal. | |
| 2 | 02 architecture-constraints | Yes | `TC-39` names the receipt binding, and the declared E2E package is a new change-level acceptance constraint. | `TC-39` gains the E2E identity and E2E scope outcome; `TC-45` records the blocking boundary and its advisory counterpart. |
| 3 | 03 context-and-scope | No | The declared `e2e` role, `verify.e2e_command`, and the `GIVN_RESULT_SCOPE` values already cross the boundary; no new external interface appears. | |
| 4 | 04 solution-strategy | Yes | The change-verification and addon strategy rows now describe the `ise`-effective boundary and the second change scope. | The change-verification and addons rows and the mandatory-check list are updated. |
| 5 | 05 building-block-view | Yes | A new shared boundary resolver and changed consumer responsibilities. | `e2e_boundary` is added; `cli::lint`, `cli::check`, `proof`, and `config::lock` responsibilities are updated; the crate diagram gains the resolver. |
| 6 | 06 runtime-view | Yes | The tasks check gains the boundary preflight, the E2E scope, and the receipt identity; freshness gains the E2E stale reasons. | 6.6 records the new mandatory-check order; 6.7.1 records the E2E scope and the extended receipt and freshness. |
| 7 | 07 deployment-view | No | No packaging, container, runtime, or topology change; the product stays one binary. | |
| 8 | 08 crosscutting-concepts | Yes | The mandatory-check contract and the receipt concept change for `ise`-effective changes. | 8.12 records the boundary, the E2E scope, and the E2E-bound receipt. |
| 9 | 09 architecture-decisions | Yes | The durable receipt compatibility boundary changes. | ADR-070 is amended with the E2E identity; the register and `adr/README.md` rows name the amendment. |
| 10 | 10 quality-requirements | No | The boundary is functional behavior asserted by Gherkin; no new quality scenario or metric is introduced. | |
| 11 | 11 risks-and-technical-debt | Yes | The textual runner-target rule, the import reader's depth, and the receipt change are deliberate limits; the change needs one rerun. | `R-138` to `R-141` record the runner-target limit, the import-reader limit, the in-flight rerun, and the broad-entry weakening. |
| 12 | 12 glossary | Yes | The receipt term changes; the E2E identity term was recorded by the predecessor and stays as it is. | `docs/glossary.md` extends `Change verification receipt` with the E2E identity and the delta-aware scope; chapter 12 stays a link and needs no edit. |

## ADR qualification

```text
Architectural commitment: the declared E2E package becomes the change-level acceptance boundary, and its identity becomes a durable receipt field that review and archive consume; the receipt scope is a compatibility boundary.
Real alternatives: additive fields under change-proof/v1 versus a new change-proof/v2 scope (proposal D1); a Cargo-only static reader versus additional stack readers or a project-declared dependency command (proposal D2).
Durable consequence: receipts are durable project data; a later identity change needs a new scope, and an ise-effective change refuses a receipt without the identity. Reversal touches review, archive, and every ise project's evidence.
Existing-decision search: active and archived ADR registers checked. ADR-070 owns the change and archive receipt, ADR-010 owns the configuration authority and the declaration, ADR-028 owns the mandatory-check boundary, and ADR-075 owns projected interaction integrity.
ADR action: AMEND_ADR
ADR target: ADR-070
```

Rejected candidates: the `e2e_boundary` module extraction (an internal refactor
whose reversal is one change and whose consumers are the same two CLI commands,
so it fails the durable-consequence question); the boundary condition list
(observable behaviour owned by the `verify-change` capability and its Gherkin);
the runner-target textual rule (an implementation limit recorded as R-138, not a
durable architecture choice); the `@e2e` policy amendment (owned by
`create-change` / `prose-consolidation`).

## Notes

Every affected chapter named above is updated in this change, and
`docs/reference.md`, `docs/glossary.md`, ADR-070, and the two ADR indexes are
updated with the same vocabulary. The `assets/instructions/specs.md`
conditional amendments, the `assets/instructions/ise.md` two-layer boundary
correction, and the `prose-consolidation` scenarios land in the implementation
batch that enables the blocking boundary, as the seed's landing rule requires.

STATUS: DONE
