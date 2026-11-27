# Arc42 Documentation Update: givn-self-rebuild-under-the-updated-rules

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change applies an existing `ise` rule to the project's declaration and records trial evidence; it adds no goal, stakeholder, or requirement. | |
| 2 | 02 architecture-constraints | Yes | The addon constraint records the project's declared build inputs and the toolchain pin, which are durable technical constraints on the build. | Section 2.3 names the build manifest and lockfile under `contracts`, the toolchain pin under `limits`, the templates under `structure`, and the served text under `behavior`; the `e2e` package's manifest and lockfile stay the test build's inputs. |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface appears; the declaration and the rebuild commands are already in scope. | |
| 4 | 04 solution-strategy | Yes | The `Clean-room spec copy` and `Self-rebuild` rows are the strategy's home. | The `Self-rebuild` row records that the copy carries the declared build inputs, the updated-rule re-run resolves the five former specification defects, residual implementation defects stay classification evidence, and nothing is promoted. |
| 5 | 05 building-block-view | No | No module, component, or responsibility changes; the declaration and copy components are unchanged. | |
| 6 | 06 runtime-view | No | The clean-room flow and the E2E flow are documented and run unchanged; the change adds data, not a flow. | |
| 7 | 07 deployment-view | No | No packaging, container, runtime, or topology change; the product stays one binary. | |
| 8 | 08 crosscutting-concepts | No | The configuration and declaration concepts already exist; the role set and the loader are unchanged. | |
| 9 | 09 architecture-decisions | No | No new architectural commitment is chosen. ADR-010 owns configuration authority, ADR-076 the clean-room manifest, and ADR-078 the frozen package outside the trial. | |
| 10 | 10 quality-requirements | No | No new quality requirement or scenario; the re-run exercises the existing rebuildability evidence. | |
| 11 | 11 risks-and-technical-debt | Yes | The declared build inputs reduce the undeclared-surface risk, and the toolchain pin adds a new failure mode. | `R-163` records that `assets/**` and the product version are now declared and names the remaining repository-file and test-fixture surfaces; `R-164` records the toolchain-pin setup failure and the `nix build` scope limit. |
| 12 | 12 glossary | No | No new domain term. `Build input` already defines the classes and the role home; chapter 12 stays a link to `docs/glossary.md`. | |

## ADR qualification

```text
Architectural commitment: none. The declaration entries and the toolchain pin are project configuration and one technical constraint; no component boundary, dependency direction, durable contract, deployment boundary, or compatibility boundary changes.
Real alternatives: the toolchain pin has real alternatives (1.97.1, 1.99.1, the stable alias). The declaration split has real alternatives (full partition, five surfaces, one role). Both are bounded by the updated ise rule and reverse with one declaration edit and one trial rerun.
Durable consequence: none beyond the build constraint. Reversal is one file edit plus a toolchain install; no migration, interface, or cross-command coordination follows.
Existing-decision search: ADR-010 owns the configuration authority and the declaration; ADR-076 owns the clean-room manifest; ADR-078 owns the frozen package outside the trial. The pin is a chapter-02 technical constraint, not a separate architecture decision.
ADR action: none. Canonical home: docs/arc42/02-architecture-constraints.md (the declared build inputs and the pin), docs/arc42/04-solution-strategy.md (the self-rebuild rows), and docs/arc42/11-risks-and-technical-debt.md (R-163 and the pin risk).
```

## Notes

The three affected chapters are updated in this change with the same
vocabulary. `docs/glossary.md` and `docs/domain-model.md` stay unchanged: the
change adds no term and no structural relationship. The spec copy, the
manifest, and the rebuilt binary are ephemeral evidence outside the checkout;
the architecture documentation records only the durable declaration, the pin,
and their consequences.

STATUS: DONE
