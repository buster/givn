# arc42 Documentation Update: add-java-typescript-outline

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | FR-46 names the supported outline languages. | Added Java and TypeScript. |
| 2 | 02 architecture-constraints | Yes | TC-42 names the parser set and offline constraint. | Added bundled Java/TypeScript Tree-sitter grammars. |
| 3 | 03 context-and-scope | Yes | The outline report interface exposes new language values and schema 4. | Updated the scope boundary. |
| 4 | 04 solution-strategy | Yes | The outline strategy now covers four languages. | Updated the code-structure report row. |
| 5 | 05 building-block-view | Yes | A production adapter module is added. | Added `tree_sitter_adapter` to the outline responsibilities. |
| 6 | 06 runtime-view | Yes | Discovery, parsing, and schema output change. | Updated 6.19. |
| 7 | 07 deployment-view | No | The grammars are compiled into the existing binary; no new service or runtime dependency. | |
| 8 | 08 crosscutting-concepts | Yes | The outline analysis boundary now includes Java/TypeScript adapters and schema 4. | Updated 8.26. |
| 9 | 09 architecture-decisions | No | The decision is language coverage behind the existing neutral port, owned by the use case and design. No new component/authority/storage boundary warrants an ADR. | |
| 10 | 10 quality-requirements | Yes | Q-112 and Q-114 reference the supported set. | Updated the unsupported-scope example and review economy. |
| 11 | 11 risks-and-technical-debt | Yes | Language coverage and grammar supply change. | Updated R-114 and the outline coverage debt row. |
| 12 | 12 glossary | Yes | New language adapter and interface terms. | Updated Outline report, Production source, and added Tree-sitter language adapter. |

## Notes

The change adds two production grammar dependencies and a neutral `interface` type kind. It keeps the Rust/Python adapters, adds no gate, and does not resolve calls across languages.

ADR action: NONE
ADR target: `givn/specs/assess-code-structure`, `docs/outline.md`, and `docs/arc42/08-crosscutting-concepts.md`

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/assess-code-structure and docs/outline.md
target_adr: null
replacement_adr: null
```

The decision extends an existing adapter behind an existing port; it does not fix a new component, authority, storage, or deployment boundary.

---

## Status

STATUS: DONE
