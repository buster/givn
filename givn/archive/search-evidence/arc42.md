# Arc42 Assessment: search-evidence

| # | Chapter | Affected | Assessment |
|---:|---|---|---|
| 1 | Introduction and Goals | No | No new stakeholder or quality goal; the feature is an opt-in diagnostic extension of the existing retrieval goal. |
| 2 | Architecture Constraints | No | No legal, technical, or organizational constraint change. |
| 3 | Context and Scope | Yes | New user-facing surface: `givn spec search --evidence` appends optional BGE/NLI verdicts; the table row in `03-context-and-scope.md` is extended. |
| 4 | Solution Strategy | No | Reuses the established E5-then-second-stage advisory strategy; no new strategic approach. |
| 5 | Building Block View | No | No new module or component: the search path reuses `bge_explore_run`/`nli_explore_run` inside `spec::retrieval`. |
| 6 | Runtime View | Yes | New flow: search takes the five displayed results, wraps the raw query, and runs BGE/NLI evidence; §6.8.2 updated. |
| 7 | Deployment View | No | No packaging, topology, service, or artifact-distribution change. |
| 8 | Crosscutting Concepts | Yes | Renders the search surface in the index/second-stage section of `08-crosscutting-concepts.md`. |
| 9 | Architecture Decisions | Yes | No new MADR (not qualified): the evidence contract is already owned by ADR-052/ADR-058/ADR-063; their surface enumerations are amended to include `spec search --evidence`. |
| 10 | Quality Requirements | No | No new quality scenario; evidence latency risk captured in chapter 11. |
| 11 | Risks and Technical Debt | Yes | Adds R-38: per-invocation second-stage model load behind the opt-in flag, mitigated by XDG caches and the unavailable fallback. |
| 12 | Glossary | Yes | Adds the "Search evidence" term. |

## Updated Durable Chapters

- `docs/arc42/README.md`
- `docs/arc42/03-context-and-scope.md`
- `docs/arc42/06-runtime-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`
- `docs/arc42/adr/ADR-052-bge-advisory-evidence.md`
- `docs/arc42/adr/ADR-058-nli-evidence.md`

## Status

STATUS: DONE