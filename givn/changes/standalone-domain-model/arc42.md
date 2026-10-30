# arc42 Documentation Update: standalone-domain-model

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change adds no product goal or stakeholder class; it changes where durable domain documentation lives. | |
| 2 | 02 architecture-constraints | No | No new platform, dependency, or compatibility constraint. The fixed documentation paths are ordinary project files. | |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing channel is added. | |
| 4 | 04 solution-strategy | No | The change stays inside the embedded-asset cascade and addon-conditional generation strategy. | |
| 5 | 05 building-block-view | Yes | `cli::addons` gains documentation scaffolds; `cli::instructions` and `skills::targets` change their domain-skill routing. | Updated 5.3: `cli::addons` scaffolds `docs/glossary.md` and `docs/domain-model.md`; domain-skill routing depends only on `domain_modeling`. |
| 6 | 06 runtime-view | Yes | `givn init`, `givn addons enable`, and `givn instructions` change their file writes and routing. | Updated 6.1 step 6, 6.3 routing, and 6.12 addon enable behavior. |
| 7 | 07 deployment-view | No | No packaging, deployment, or runtime dependency changes; the documents are plain files. | |
| 8 | 08 crosscutting-concepts | Yes | Domain documentation ownership and addon-conditional serving are cross-cutting concepts. | Added 8.17: standalone glossary and model ownership, scaffold triggers, migration, and Mermaid host limitation. |
| 9 | 09 architecture-decisions | No | The decision is documentation ownership and guidance behavior owned by the use cases, design, and chapter 08. It adds no component, authority, storage, or deployment boundary requiring an ADR. | |
| 10 | 10 quality-requirements | No | No new quality scenario; the review check is existing guidance. | |
| 11 | 11 risks-and-technical-debt | Yes | The migration leaves an interval where chapter 12 content and the new glossary can coexist. | Added R-44: preservation-first migration can temporarily leave two locations; the guidance requires one authority after migration. |
| 12 | 12 glossary | Yes | The chapter becomes a link, and the vocabulary moves to `docs/glossary.md`. New terms are recorded there. | Chapter 12 now links to `docs/glossary.md`; the term table moved unchanged. |

## Notes

The change adds embedded Markdown scaffolds, two scaffold writes in
`cli::addons`, routing decoupling in `cli::instructions` and
`skills::targets`, and guidance content. It adds no dependency, configuration
key, manifest artifact, or command.

ADR action: NONE
ADR target: `docs/arc42/08-crosscutting-concepts.md`, `docs/glossary.md`, and the effective-guidance, initialize-project, and support features

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/obtain-effective-guidance, givn/specs/initialize-project, givn/specs/fragments, and docs/arc42/08-crosscutting-concepts.md
target_adr: null
replacement_adr: null
```

The decision fixes no component, authority, storage, or deployment boundary; it
changes which documentation owns vocabulary and structure. The use cases, the
design, and chapter 08 are its canonical homes.

---

## Status

STATUS: DONE
