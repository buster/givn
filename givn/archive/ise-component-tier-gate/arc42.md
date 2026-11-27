# arc42 Impact Assessment: ise-component-tier-gate

The change adds a generated verification contract to the spec copy, a
read-only comparison label, and the `givn ise verify-trial` execution gate. It
changes the ISE command surface, the spec-copy manifest, the trial result
record, and the served trial rules.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal or quality attribute changes; the gate enforces an existing Ise goal. |
| 2 | Architecture Constraints | Yes | TC-53 records the contract, exact-count gate, coverage checks, and result record. |
| 3 | Context and Scope | Yes | The verifier entity and the command table gain `verify-trial` and the input-only comparison meaning. |
| 4 | Solution Strategy | Yes | The clean-room strategy now separates frozen-input equality from the executed trial result. |
| 5 | Building-Block View | Yes | `src/ise/contract.rs` and `src/ise/verify.rs` are new modules; `cli::ise` and `ise` gain responsibilities. |
| 6 | Runtime View | Yes | Section 6.21 gains the contract write steps and the `verify-trial` flow. |
| 7 | Deployment View | No | No deployment process, artifact, or environment changes. |
| 8 | Crosscutting Concepts | Yes | §8.29 records the contract field, its optional manifest record, and the trial result record. |
| 9 | Architecture Decisions | Yes | ADR-076 is amended: one generated contract, `compare` read-only, `verify-trial` execution, optional `verification` field. |
| 10 | Quality Requirements | No | No quality scenario changes; the delta and acceptance tests prove the gate behavior. |
| 11 | Risks and Technical Debt | Yes | A runner whose scenario accounting differs from expanded Gherkin refuses the exact gate; an older manifest cannot claim a trial result. |
| 12 | Glossary | Yes | `Verification contract`, `Execution unit`, and `Trial result` are added to `docs/glossary.md`. |

## ADR qualification

ADR-QUALIFICATION: PASS

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: null
target_adr: docs/arc42/adr/ADR-076-clean-room-trial-record.md
replacement_adr: null

Evidence: ADR-076 already owns the clean-room command set, the
`ise-manifest/v1` shape, and the additive-field rule. This change refines that
decision with one generated contract, one execution command, and the optional
`verification` record. The comparison/execution split has a real alternative
(execute inside `compare`) and a durable consequence (untrusted copies and the
meaning of exit 0), so the existing ADR is amended rather than left to
`design.md`. The full rationale and the rejected alternative are in the
amended ADR.

## Status

STATUS: DONE
