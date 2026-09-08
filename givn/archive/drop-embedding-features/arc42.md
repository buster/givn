# arc42 impact: drop-embedding-features

STATUS: DONE

Assessment walked all 12 chapters against `proposal.md`, `specs/`, and
`design.md`.

| # | Chapter | Affected | Change |
|---|---|---|---|
| 1 | 01-introduction-and-goals | Yes | Engine FR/NFR rows deleted (IDs stay stable); NFR-1 rewritten to the single feature-free build; secondary-goal engine bullets removed. |
| 2 | 02-architecture-constraints | Yes | TC-3 rewritten to the feature-free build contract; engine constraints and §2.4 deleted; acceptance boundary rewritten (single pass, two coverage reports). |
| 3 | 03-context-and-scope | Yes | Model-artifact external entity removed; CLI interface row now lists the deterministic spec tools only; engine in/out-of-scope bullets replaced. |
| 4 | 04-solution-strategy | Yes | Engine strategy rows deleted; §4.3 and §4.5 removed; installability rewritten. |
| 5 | 05-building-block-view | Yes | Mermaid diagram de-scoped to the deterministic toolchain; `spec::retrieval` building block §5.4 deleted; engine dependency rows removed. |
| 6 | 06-runtime-view | Yes | check-review and archive flows lose the semantic stages; engine runtime sections (§6.8–§6.11) deleted. |
| 7 | 07-deployment-view | Yes | §7.1 rewritten to one feature-free binary; §7.2 retrieval artifact section deleted. |
| 8 | 08-crosscutting-concepts | Yes | Directory layout loses engine state files; stub-env / model-cache / rulebook / semantic-review concepts removed; duplicate audit rewritten deterministic-only. |
| 9 | 09-architecture-decisions | Yes | New ADR-069 supersedes the fifteen engine records; register, archive index, relationships, and evidence rows updated in `adr/`; chapter text points at the register. |
| 10 | 10-quality-requirements | Yes | Quality tree loses the semantic subtree and feature-kind split; engine quality rows deleted. |
| 11 | 11-risks-and-technical-debt | Yes | Engine risks and debts removed as resolved-by-removal; tombstone note credits this change and ADR-069; historical model measurements section deleted. |
| 12 | 12-glossary | Yes | Engine vocabulary rows deleted; deterministic, delta, migration, run-declaration, ideation, and ADR vocabulary kept. |

Verification: `grep` over `docs/arc42` (excluding `adr/`) finds no live engine
capability statements; remaining mentions are the chapter 9/11 tombstones and
non-engine uses of the word "semantic".
