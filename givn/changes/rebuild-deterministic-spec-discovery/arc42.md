# arc42 assessment: rebuild-deterministic-spec-discovery

## Status

STATUS: DONE

## 12-row assessment

| # | Chapter | Affected | Reason / summary |
| --- | --- | --- | --- |
| 1 | 01 introduction-and-goals | No | No new goals. |
| 2 | 02 architecture-constraints | No | No new constraints. |
| 3 | 03 context-and-scope | Yes | The discovery subcommands' specified surface becomes deterministic-only; retrieval stays opt-in. |
| 4 | 04 solution-strategy | No | Continues the deterministic direction (ADR-037 lineage). |
| 5 | 05 building-block-view | Yes | `spec::tree`, `spec::route`, `spec::routing_table`, `spec::cluster` now own the discovery behaviour the specs describe. |
| 6 | 06 runtime-view | Yes | `givn spec duplicates/tree/find/route` flows documented in ch.6; duplicates gains tolerant missing-corpus and directed subset matching. |
| 7 | 07 deployment-view | No | Single binary unchanged. |
| 8 | 08 crosscutting-concepts | No | No new crosscutting concepts. |
| 9 | 09 architecture-decisions | No | No new ADR; deterministic-only spec surface follows the retirement plan. |
| 10 | 10 quality-requirements | No | No new quality scenarios. |
| 11 | 11 risks-and-technical-debt | Yes | Removes the last spec-level coupling of discovery to embeddings; unblocks the default-feature drop. |
| 12 | 12 glossary | No | Terms already recorded (Group, Use case, Fragment). |
