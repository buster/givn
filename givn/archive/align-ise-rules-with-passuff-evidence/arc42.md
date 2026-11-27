# arc42 Impact Assessment: align-ise-rules-with-passuff-evidence

This change aligns the served `ise` and `review` rules with the three Passuff
trials and the Givn self-rebuild. It edits guidance text, reference rows,
glossary terms, and the scenarios that assert them. It changes no module,
runtime flow, command behavior, deployment, or quality scenario.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | No | No project goal or quality attribute changes; the edit is guidance text. |
| 2 | Architecture Constraints | No | No legal, platform, or organizational constraint is added; the rule states which existing declaration roles own the build inputs' content classes. |
| 3 | Context and Scope | No | No command, interface, or user-facing surface changes; the served text is the only changed output. |
| 4 | Solution Strategy | No | No technical strategy changes; the spec-copy partition, command set, and declaration schema stay as they are. |
| 5 | Building-Block View | No | No component, module, or responsibility changes; no `src/**` file is edited. |
| 6 | Runtime View | No | No command flow or sequence changes. |
| 7 | Deployment View | No | No deployment process or environment changes. |
| 8 | Crosscutting Concepts | No | No error handling, security, configuration, or logging concept changes; the registry scan is a trial obligation in guidance, not a new mechanism. |
| 9 | Architecture Decisions | No | No ADR: the decisions have canonical homes in the `ise` instruction, `docs/reference.md`, and `docs/glossary.md`, and none fixes a component, dependency, or compatibility boundary. |
| 10 | Quality Requirements | No | No quality scenario changes; the delta scenarios assert served guidance text through the CLI. |
| 11 | Risks and Technical Debt | No | No new project risk or technical debt; the permanent scenario bodies are written in place by the implementation batch, so the review preflight runs green before archive. |
| 12 | Glossary | Yes | `Build input`, `Neighbor record`, and `Replaced unit` are added to `docs/glossary.md`; chapter 12 links that file, so no chapter file is edited. |

No chapter file under `docs/arc42/` changes, and `docs/arc42/README.md` stays as
the index.

## Status

STATUS: DONE
