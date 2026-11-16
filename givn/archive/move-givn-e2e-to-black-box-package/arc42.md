# Arc42 Documentation Update: move-givn-e2e-to-black-box-package

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change moves the acceptance suite and adds no goal, stakeholder, or quality goal; the product contract is unchanged. | |
| 2 | 02 architecture-constraints | Yes | The repository's acceptance authority becomes a build and dependency constraint. | `TC-52` records the standalone `e2e` package, its exclusion from the published crate, the binary-only reach, and the declaration role. |
| 3 | 03 context-and-scope | No | The product's external interface is unchanged; the package is a repository test artifact, not a new external system. | |
| 4 | 04 solution-strategy | Yes | The test-execution strategy now names two tiers. | The test-execution row names the package; a new acceptance-package row records the binary-only reach and the tag selection. |
| 5 | 05 building-block-view | Yes | A repository test building block is added beside the product. | The crate diagram gains the excluded `e2e` package; 5.3 gains its responsibility row and renames the component tier. |
| 6 | 06 runtime-view | Yes | The repository's acceptance flow now runs two packages. | 6.14 records the component tier and the package runner, including `GIVN_BIN` and the result contract. |
| 7 | 07 deployment-view | Yes | The browser assets move, and the package must not reach an installed product. | The browser tier path becomes `e2e/browser/` and the package is recorded as absent from an installed product. |
| 8 | 08 crosscutting-concepts | Yes | The runner contract and the acceptance boundary are cross-cutting. | 8.31 records the package shape, the runner variables, the strictness rule, and the failure mode it closes. |
| 9 | 09 architecture-decisions | Yes | A durable acceptance-authority and dependency-direction boundary is decided. | ADR-079 is accepted; the register count becomes 31. |
| 10 | 10 quality-requirements | No | No new quality scenario or metric is introduced; the qualification script is verification evidence for the existing acceptance guarantee. | |
| 11 | 11 risks-and-technical-debt | Yes | The move creates coverage, drift, evidence, and qualification costs. | `R-86` names the new browser path; `R-155` to `R-159` record the instrumented-binary path, the mirrored formatting, the archived-evidence rewrite, the qualification cost, and the static reader's registry blindness. |
| 12 | 12 glossary | Yes | The acceptance package is a new domain term. | `docs/glossary.md` gains `E2E package`; chapter 12 stays a link and needs no edit. |

## ADR qualification

```text
Architectural commitment: the repository's acceptance authority becomes a standalone workspace package with no product dependency, and the E2E verification scope gains a binary-path contract that every future @e2e step inherits.
Real alternatives: a standalone excluded package, a member of the product workspace, a consumer artifact against a published API, and retagging the whole suite as @component (proposal D3 and the seed's S13 options).
Durable consequence: every future @e2e step lands in the package; reversal moves the suite, changes the receipt and trial identities, and re-couples the acceptance evidence to the implementation.
Existing-decision search: active and archived registers checked. ADR-069 owns the single feature-free binary, ADR-070 owns the E2E-bound change receipt, ADR-078 owns the frozen package outside a trial, and ADR-015 owns the declared test runner; none owns the repository's acceptance package.
ADR action: NEW_ADR
ADR target: ADR-079
Cheaper home if not qualified: `docs/arc42/02-architecture-constraints.md` (TC-52) and `docs/reference.md` would own the package layout and the runner contract; they carry the current-architecture facts, while ADR-079 owns the commitment and its alternatives.
```

Rejected candidates: the runner's `GIVN_BIN` and `GIVN_READY_FIXTURE` variables
(an implementation contract owned by `design.md` and `docs/reference.md`); the
package-layout inventory (design evidence); the seeded defect and the
qualification script (verification evidence, not architecture); the delta
scenarios (Gherkin); the declaration file (owned by `declare-the-specification`
and the planning documents).

## Notes

Every affected chapter named above is updated in this change, and
`docs/reference.md`, `docs/glossary.md`, ADR-079, and `docs/arc42/adr/README.md`
carry the same vocabulary. The implementation batches create the `e2e` package,
the project runner wiring, the declaration, and the qualification script; the
product's `e2e_boundary` resolver and receipt code are unchanged.

STATUS: DONE
