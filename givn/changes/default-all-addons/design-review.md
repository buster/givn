# Design Review: default-all-addons

Review kind: self-review
Author: agent
Reviewer context: agent

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | How does a project opt out? | An explicit `false` is the only opt-out; `givn init --no-addons` writes it for every known add-on. |
| 2 | Tech choices | What does a partial `addons:` block mean? | Missing keys resolve to enabled through the serde default; only named keys change. |
| 3 | Risk | What happens when Arc42 is disabled? | The overlay removes the `arc42-docs` artifact and its dependencies; the default is enabled, so this path is now live. |
| 4 | Testability | Does the outline gate fire on archive-only changes? | The gate skips when no production path under `src` changed; fixtures that exercise other gates disable the outline addon explicitly. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Scope | `AddonSpec.default_enabled` drives `givn init`; `parse_enabled_addons` reads only explicit config keys. Both are the registry seam. |
| Tech choices | `Addons` serde fields and the manual `Default` impl are the effective-default source; the embedded manifest carries the same values. |
| Risk | `config::overlay` already expanded Arc42 on enable; the disable branch removes the artifact and `apply.requires` / design-review dependency. |
| Testability | The archive-proof and review fixtures use `make_init_root`; the outline gate is disabled in those fixtures where production changes are synthetic. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `initialize-project` owns `enable-optional-addons`; `assess-code-structure` owns `outline-report`. | Accepted from the proposal's route. |
| Typed Includes / Extends relationships | Unchanged. | No action. |
| Confirmed Persona coverage | `developer--primary-user` and `agent--technical-role` are the CLI consumers. | Proposal records both. |
| Actor and Persona distinction | The Gherkin actor stays the developer; Personas are review lenses. | No collapse. |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| `developer--primary-user` | Runs `givn init`, `givn addons list`, and the opt-out commands. | Init and add-on scenarios. |
| `agent--technical-role` | Reads the generated guidance that follows the add-on defaults. | Addon-aware generation scenarios. |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | No goal change. |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | TC-45 matches the default posture. |
| 3 | 03 context-and-scope | No | No | Yes | The existing init/add-on surface is reused. |
| 4 | 04 solution-strategy | No | No | Yes | Existing manifest and registry. |
| 5 | 05 building-block-view | Yes | Yes | Yes | `cli::init` and `cli::addons` rows updated. |
| 6 | 06 runtime-view | Yes | Yes | Yes | Init writes explicit opt-outs and reads effective defaults. |
| 7 | 07 deployment-view | No | No | Yes | No deployment change. |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | 8.11 and 8.26 match the default posture. |
| 9 | 09 architecture-decisions | No | No | Yes | Configuration policy; no ADR. |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Q-121 matches the init scenarios. |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | R-126 states the activation cost. |
| 12 | 12 glossary | Yes | Yes | Yes | Addon and add-on default terms match. |

- [x] All 12 rows assessed independently against proposal.md/design.md before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md saying "No" or omitting the row.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry in chapter 09; the change records `ADR action: NONE` with a canonical target.
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in chapter 11; no new ADR means no new consequence.
- [x] Every ADR candidate records the structured qualification verdict and its route-specific target; the verdict routes to `CANONICAL_ARTIFACT`.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `src/config/overlay.rs` | Arc42 enable now ensures the artifact exists; disable removes it and its dependencies. |
| `src/outline/gate.rs` | A change with no touched production path has no outline obligation. |
| Test fixtures | Archive/review fixtures disable the outline addon explicitly; the readme and resolve fixtures do the same. |
| Permanent specs | Old default-off scenarios now assert the enabled default until the delta merge replaces them. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D1`) resolves in the design's `## Decisions` ledger.
- [x] All questions are resolved.
- [x] `design.md` reflects decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the default and explicit-disable scenarios.
- [x] `givn lint --change default-all-addons` exits 0 or 2.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
