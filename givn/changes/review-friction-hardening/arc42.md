# arc42 Documentation Update: review-friction-hardening

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No new stakeholder, product goal, or quality attribute; the change removes avoidable iterations inside existing goals. | — |
| 2 | 02 architecture-constraints | Yes | The change verification receipt constraint gains the tested-content binding and the reuse rule. | TC-39 now binds the receipt to the tested content and records that review and archive reuse it across a documentation-only revision. |
| 3 | 03 context-and-scope | No | No external system, integration, or new user-facing surface; only existing CLI output text changes. | — |
| 4 | 04 solution-strategy | Yes | The change-verification strategy now shares one freshness rule and batches the schema/routing diagnosis. | The strategy table records tested-content binding, documentation-only reuse, and complete review diagnostics in one pass. |
| 5 | 05 building-block-view | Yes | Parser, proof, SCM, and CLI responsibilities change. | `gherkin::usecase_md` collects diagnostics; `proof` owns schema 2, the tested-content digest, and the freshness result; `scm`/`git`/`fossil` expose content paths; `cli::check` and `cli::archive` share the freshness result. |
| 6 | 06 runtime-view | Yes | The review and archive receipt flows change. | §6.7.1 describes the tested-content digest, the current/documentation-only/stale statuses, and the reuse message; archive reuses the proof without a change-suite run. |
| 7 | 07 deployment-view | No | The product remains one native binary with the same distribution channels. | — |
| 8 | 08 crosscutting-concepts | Yes | The mandatory review-check inventory gains the batched routing diagnostics and the receipt-freshness gate. | §8.12 names the complete review check order and the single-pass routing report. |
| 9 | 09 architecture-decisions | Yes | The durable receipt contract is refined. | ADR-070 is amended with the tested-content receipt binding and the schema-1 compatibility rule; the register records the amendment. |
| 10 | 10 quality-requirements | Yes | The change-verification quality scenario changes. | Q-105 now treats an unchanged tested content under a documentation-only revision as reusable and tested-content changes as stale. |
| 11 | 11 risks-and-technical-debt | Yes | The documentation allowlist is a new trust boundary, and the digest adds review-time I/O plus a schema-1 rerun. | R-104 and R-105 record the boundary and compatibility costs with their mitigations. |
| 12 | 12 glossary | Yes | A new revision category and a refined receipt definition enter the domain vocabulary. | The change verification receipt definition gains the tested-content digest and scope; "Documentation-only revision" is defined. |

## Notes

ADR action: AMEND_ADR
ADR target: ADR-070

The tested-content binding is a refinement of ADR-070's durable receipt
contract, not a new independent boundary. The archive publication boundary,
the candidate exactness rule, and the fail-closed scope results are unchanged.

```text
qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: null
target_adr: ADR-070
replacement_adr: null
```

- alternatives: proposal D2 (closed allowlist vs all Markdown vs SCM diff),
  D3 (tested-content digest vs history diff vs refreshed head), and D6
  (schema 2 with schema 1 fallback vs permissive fallback vs reject all).
- architectural_impact: the durable receipt contract now fixes what review and
  archive may reuse; the compatibility rule spans both commands and both SCM
  backends.
- durable_consequence: receipts are durable project data; the scope name and
  schema version are compatibility commitments, and changing the boundary
  later requires a new scope rather than a silent reinterpretation.
- lower_level_artifact: the observable current/documentation-only/stale
  behaviour is owned by the delta specs and the design; ADR-070 gains only the
  durable binding and compatibility boundary.
- existing_adr_check: ADR-070 owns the archive proof and receipt contract and
  is amended; ADR-028 owns the mandatory-check boundary and is unchanged.

*[Architecture documentation index](../../../docs/arc42/README.md)*

## Status

STATUS: DONE
