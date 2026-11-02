# arc42 Documentation Update: add-javascript-outline

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | FR-46 names the supported outline languages. | Added JavaScript. |
| 2 | 02 architecture-constraints | Yes | TC-42 names the parser set and offline constraint. | Added the bundled JavaScript Tree-sitter grammar. |
| 3 | 03 context-and-scope | Yes | The outline report interface exposes the language value set and schema version. | Updated to schema v5 and added `javascript`. |
| 4 | 04 solution-strategy | Yes | The code-structure report strategy now covers five languages. | Updated the code-structure report row. |
| 5 | 05 building-block-view | Yes | The production Tree-sitter adapter gains a JavaScript responsibility. | Added JavaScript to `tree_sitter_adapter`. |
| 6 | 06 runtime-view | Yes | Discovery, parsing, and schema output change. | Added `.js`/`.mjs`/`.cjs` discovery, grammar selection, and schema v5. |
| 7 | 07 deployment-view | No | The grammar is compiled into the existing binary; no new service, runtime dependency, or deployment unit. | |
| 8 | 08 crosscutting-concepts | Yes | The outline analysis boundary now includes a JavaScript adapter and schema v5. | Updated the analysis-boundary paragraph and evaluation-scope sentence. |
| 9 | 09 architecture-decisions | No | Language coverage behind the existing neutral port is owned by the use case and design. No new component, authority, storage, or deployment boundary qualifies an ADR. | |
| 10 | 10 quality-requirements | Yes | Q-112 uses an unsupported-language example that named JavaScript. | Changed the example to Go or JSX. |
| 11 | 11 risks-and-technical-debt | Yes | R-114, R-115, and the outline coverage backlog row name the supported set and schema. | Added JavaScript, advanced the schema to 5, and removed JavaScript from unsupported coverage. |
| 12 | 12 glossary | Yes | Outline report, Production source, and Tree-sitter language adapter name the supported languages and schema. | Updated the three glossary entries. |

## Notes

The change adds one production grammar dependency and one language value behind
the existing `SourceAnalyzer` port. It keeps the existing adapters, adds no
gate, does not resolve calls across languages, and leaves JSX and Go
unsupported.

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

The decision extends an existing adapter behind an existing port; it does not
fix a new component, authority, storage, or deployment boundary.

---

## Status

STATUS: DONE
