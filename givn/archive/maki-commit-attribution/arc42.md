# arc42 Documentation Update: maki-commit-attribution

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goal, stakeholder class, or success guarantee changes; attribution coverage extends to an already supported harness. | |
| 2 | 02 architecture-constraints | Yes | TC-36 enumerates what the Maki target receives; the attribution integration and its required `session_write` grant belong in that constraint. | TC-36 names the attribution module, the second managed fence, and the attribution grant. |
| 3 | 03 context-and-scope | Yes | The technical-context interface table names each harness boundary; Maki's boundary currently names only the reporter. | The Maki plugin boundary gains the attribution module and fence; the runtime trust and reporter rows stay unchanged. |
| 4 | 04 solution-strategy | Yes | The "Commit attribution" strategy row enumerates the adapters. | The row names Maki alongside OpenCode, Claude, and omp. |
| 5 | 05 building-block-view | Yes | The embedded attribution-assets row enumerates the generated adapter assets. | The row names the Maki attribution asset; `skills::targets` already owns the merge path. |
| 6 | 06 runtime-view | Yes | 6.1a documents the commit-attribution ordering per harness and explicitly states Maki has no attribution asset; 6.12 documents the generated integrations. | 6.1a gains the Maki ordering and the `session_write` condition; 6.12 names the attribution module beside the reporter. |
| 7 | 07 deployment-view | No | No topology or artifact change; the single binary and the pinned Maki 0.7.0 test binary are unchanged. | |
| 8 | 08 crosscutting-concepts | Yes | 8.12a lists the attribution integrations and 8.25 describes the four reporters and their generated files. | 8.12a names Maki; 8.25 names the attribution asset and the attribution grant beside the reporter. |
| 9 | 09 architecture-decisions | Yes | ADR-013 owns the generated agent-integration boundary, and its Consequences sentence enumerates what a selected `maki` target receives. | Amend ADR-013's Consequences sentence to include the Maki attribution integration; update the chapter-09 register note. |
| 10 | 10 quality-requirements | Yes | The commit-attribution quality scenarios enumerate the harnesses. | Add Q-124 for a Maki session that prepares and runs a revision command. |
| 11 | 11 risks-and-technical-debt | Yes | Maki's attribution surface is an external contract with permission and model-source limitations. | Add R-134 for hook/model API drift, the focused-session model, the edited-manifest residual, and the headless unavailable case. |
| 12 | 12 glossary | Yes | The "Harness" term enumerates supported runtimes, and `session_write` is a new attribution-specific grant. | Name Maki in "Harness"; add "Maki attribution grant". |

## Notes

The change adds one generated adapter under ADR-013 and reuses the
commit-attribution environment protocol. It adds no new lifecycle gate, data
store, external service, or dependency. The exact-prior-manifest migration is
the only new generated-file rewrite; it is bounded to the byte-identical
generated revision and never widens a user-edited manifest.

```text
ADR action: AMEND_ADR
ADR target: docs/arc42/adr/ADR-013-agent-integration.md

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: docs/arc42/adr/ADR-013-agent-integration.md
target_adr: ADR-013
replacement_adr: none
evidence:
  alternatives:
    - "reuse the reporter module and its existing fence vs. a separate attribution module and fence; the separate module keeps the reporter's bounded intake independent from the command-rewrite hook, and the two fences let a user disable one without editing the other"
    - "auto-migrate the exact generated manifest vs. only warn on every pre-attribution manifest; migrating the byte-identical generated revision removes the silent breakage for existing projects, while the warning path preserves user-edited files"
  architectural_impact:
    - "ADR-013 fixes the generated destinations and the user-owned merge discipline; the Maki target's received-file set changes, and the permission manifest gains a harness-required grant"
  durable_consequence:
    - "reversal would remove a generated integration path and a permission grant, and every new project and synchronization would again leave Maki commits unattributed; the permanent attribute-commits and initialize-a-project specs assert the file set and the grant"
  lower_level_artifact:
    - "the hook behavior, environment protocol, and migration algorithm are owned by design.md, the Gherkin delta, assets/agent-integrations/maki/givn-attribution.lua, and src/skills/targets.rs; ADR-013 owns the durable generated-destination boundary that this change amends"
  existing_adr_check:
    - "searched the active register and docs/arc42/adr/ for agent-integration, attribution, and Maki boundaries; ADR-013 is the owning record, and its Consequences sentence is amended in place"
```

Version freshness: Maki 0.7.0 (verified with `maki --version` on 2026-11-05;
the project E2E harness pins the binary). No new Rust or JavaScript
dependency; the Lua module uses Maki's embedded plugin runtime.

---

## Status

STATUS: DONE
