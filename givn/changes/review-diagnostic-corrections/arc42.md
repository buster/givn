# arc42 Impact Assessment: review-diagnostic-corrections

This change corrects two defects in the batched use-case diagnostics emitted by
`givn check review`. It adds no capability, command, interface, or dependency.
The review command keeps its exit codes and strict schema; only the parser's
diagnostic decision flow, the correction suggestion, and the glossary change.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | No | No goal, stakeholder, or quality attribute changes; the existing review report is corrected, not re-scoped. |
| 2 | Architecture Constraints | No | No legal, technology, organizational, or operational constraint changes; no dependency or toolchain change. |
| 3 | Context and Scope | No | No new external system, command, interface, or user-facing surface; `givn check review` is unchanged as an entry point. |
| 4 | Solution Strategy | No | No technical strategy or implementation approach changes; the parser's private scan flow changes within the existing module. |
| 5 | Building-Block View | No | No module, component, file boundary, or dependency changes; `usecase_md.rs` keeps its public API and consumers. |
| 6 | Runtime View | Yes | Updated section 6.7's `givn check review` flow: root diagnostics (invalid title line or Level) are reported first, dependent completeness findings are withheld, and independent findings still appear in the same pass. |
| 7 | Deployment View | No | No deployment target, process, or environment change; the same single CLI binary runs locally. |
| 8 | Crosscutting Concepts | Yes | Updated the mandatory-check description in section 8.12: the schema/routing gate reports in one pass but withholds dependent completeness findings while the title or Level is invalid. |
| 9 | Architecture Decisions | No | No new ADR. The design's five decisions (canonical suggestion, suppression boundary, no Level fallback, parser-owned gate, coverage reuse) are single-change, single-component choices whose complete rationale is owned by `design.md`; the qualification verdicts are recorded below. |
| 10 | Quality Requirements | No | No new quality scenario or measurable requirement; existing review testability and strictness are preserved. |
| 11 | Risks and Technical Debt | Yes | Added R-119 (a canonical slug can differ from the intended identifier; advisory only) and R-120 (exact-prefix invalid identifiers no longer produce a usable document) to `docs/arc42/11-risks-and-technical-debt.md`. |
| 12 | Glossary | Yes | Added **root diagnostic**, **dependent diagnostic**, **title line**, and **section schema** to `docs/arc42/12-glossary.md`. |

## ADR qualification

No candidate qualifies for a new ADR. Each design decision fails the mandatory
dimensions outside `design.md`:

- `D1` canonical title suggestion — reversible within one parser change, one
  component, no multi-change constraint; canonical artifact `design.md`.
- `D2` suppression boundary — feature behavior owned by the capability spec and
  `design.md`; no component, authority, or deployment boundary moves.
- `D3` no Level fallback — parser-local behavior, one component, reversible in
  one change; canonical artifact `design.md`.
- `D4` parser-owned root validation — a local responsibility placement inside
  one module; existing public `Option<UsecaseDoc>` contract is unchanged.
- `D5` E2E coverage reuse — project test/coverage policy, owned by the feature
  specification and `design.md`, not an architecture boundary.

Existing ADR registers were searched: none records use-case diagnostic
ordering, title recovery, or Level fallback. Routing: `CANONICAL_ARTIFACT`
(`design.md`) for all five; no `NEW_ADR`, `AMEND_ADR`, or `SUPERSEDE_ADR`.

## Status

STATUS: DONE
