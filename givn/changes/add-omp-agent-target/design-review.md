# Design Review: add-omp-agent-target

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Testability | Can `node` import the generated `.js` hook without a module context? | closed — fixture pinned in design |
| 2 | Tech choices | Is the generated hook's factory/handler/event contract pinned? | closed — contract pinned in design |
| 3 | Testability | Does the omp E2E assert the reasoning mode it sets? | closed — assertion added |
| 4 | Interaction coverage | Does the `attribute-commits` inventory entry the matrix cites exist? | closed — matrix reworded |
| 5 | Scope | Does `givn upgrade` really report the omp integration? | closed — proposal narrowed |
| 6 | Architecture documentation | Are arc42 rows 4, 9, and 12 and the chapter edits consistent? | closed — chapters 04, 06, 09 updated |
| 7 | Scope | Do `reconcile_addon_artifacts` and `managed_generated_paths` cover every `.omp` path? | closed — design names each path |
| 8 | Risk | Is the external omp extension contract validated or assumed? | closed — operator chose a live run |

### Q1 — Can the generated hook be imported by the test driver at all?

- Status: closed 2026-10-05 (operator)
- Blocks: the first RED/GREEN run of the omp scenarios.
- Context: the design declared the generated asset as a plain `.js` ECMAScript
  module and the E2E as a `node`-driven harness, but never said how `node`
  decides the module kind for that file. `node` resolves a `.js` file's module
  type from the nearest `package.json`, so without a fixture the import fails
  with a syntax error before any behaviour is exercised — a RED for the wrong
  reason.
- Question: must the scenario create a module-context fixture before importing
  the generated hook?
- Recommended: yes — write `package.json` = `{"type":"module"}` beside the
  project, exactly as the existing OpenCode step does.
- Options:
  1. **Module fixture** — the Given writes `package.json` with
     `{"type":"module"}` into the temporary project before the harness imports
     the asset. In practice: `node invoke-omp-hook.mjs` resolves the
     `export default` factory. Gain: the harness drives the real generated
     file unchanged. Cost: one fixture file in the temporary project.
- Resolution: **module fixture** because the alternative would require copying
  the asset to `.mjs`, which would stop the test from exercising the exact
  deployed filename. The design's E2E section now names the fixture.

### Q2 — Is the omp extension contract pinned tightly enough to implement twice?

- Status: closed 2026-10-05 (operator)
- Blocks: writing the asset and the E2E harness against one contract.
- Context: the design said "default-exports the factory", "collects the
  `tool_call` handler", and "reads `ctx.models.current()`", but never fixed how
  the API object reaches the factory, the event shape, or what the handler
  returns. The asset and the harness would have been invented independently.
- Question: what exactly is the generated hook's shape?
- Recommended: pin the factory signature, the event fields, and the return
  value in the design.
- Options:
  1. **Pin the full contract** — `export default function givnAttribution(pi)`,
     `pi.on("tool_call", handler)`, `handler(event, ctx)` reading
     `event.toolName`/`event.input.command` and returning
     `{ input: { ...event.input, command } }` or `undefined`. In practice: the
     harness fakes `pi` and `ctx` and asserts the returned `input.command`.
     Gain: RED is deterministic and the two implementations cannot drift.
     Cost: none.
- Resolution: **pin the full contract** because the design is the only source
  both implementations read. The design's technology-decisions section now
  carries the pinned shape and cites the omp expansion documentation.

### Q3 — Does the omp E2E prove the reasoning mode it configures?

- Status: closed 2026-10-05 (operator)
- Blocks: the proposal's promise that the reasoning mode is recorded.
- Context: the scenario's Given set the reasoning mode to `high`, but the Then
  steps asserted only the model and the committer. A hook that dropped the
  reasoning value entirely would still have passed.
- Question: should the scenario assert the reasoning value?
- Recommended: yes — assert the managed trailer contains the reasoning mode.
- Resolution: **assert it** because a configured-but-unasserted value is a
  weaker claim than the proposal makes. The delta now adds
  "its omp message identifies reasoning mode \"high\"".

### Q4 — Does the interaction-coverage matrix cite a real inventory entry?

- Status: closed 2026-10-05 (operator)
- Blocks: the coverage claim in the design.
- Context: the matrix claimed the change extends the `attribute-commits`
  inventory, but the interaction walk skips `givn/specs/fragments/`
  (`src/gherkin/interaction.rs`), and `fragment.md`'s `## Interactions` table
  lists only `instruction-integrity`. The claimed entry does not exist.
- Question: how should the coverage claim be stated?
- Recommended: state that fragments carry no inventory and justify the omp
  `@e2e` by the existing per-harness precedent.
- Resolution: **reworded** because the permanent `attribute-commits.feature`
  already holds one attribution `@e2e` per harness (OpenCode, Claude) and the
  omp scenario is the third of the same kind; claiming a table row the tooling
  ignores would be a false statement.

### Q5 — Does the managed upgrade really report the omp integration?

- Status: closed 2026-10-05 (operator)
- Blocks: the proposal's upgrade bullet.
- Context: `src/cli/upgrade.rs` prints one generic sentence about refreshed
  attribution plumbing; it does not enumerate integrations. The proposal
  promised a per-integration upgrade report.
- Question: change the code or narrow the promise?
- Recommended: narrow the promise — the upgrade refreshes the omp files when
  `omp` is a configured target, which is what the shared sync path does.
- Resolution: **narrowed** because enumerating integrations in the upgrade
  report is new scope the trigger phases would not otherwise need, and the
  generic sentence stays true.

### Q6 — Do the rebuilt ignore and allowlist helpers cover every `.omp` path?

- Status: closed 2026-10-05 (operator)
- Blocks: the local-only policy and the upgrade staging allowlist.
- Context: `ignore_entries()` and `is_rule_path()` in `src/skills/rules.rs`
  enumerate the `.opencode`/`.agents`/`.claude` namespaces, and
  `managed_generated_paths` adds the hook only for the OpenCode target. A
  missing entry means a tracked `.omp` rule stays in the repository under
  `local-only`, or the upgrade never stages the refresh.
- Question: which paths must the design name explicitly?
- Recommended: name `.omp/skills/givn-*/`, `.omp/commands/givn-*.md`,
  `.omp/commands/grill-*.md`, and `.omp/hooks/pre/givn-attribution.js`.
- Resolution: **named** — the design's production-files section now lists each
  path and the per-target reconcile/command loops.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Scope | `--agents all` ordering: `AgentTarget::ALL` is `[Opencode, Agents, Claude]`; appending `Omp` yields `opencode, agents, claude, omp`, matching the delta and `add_targets`. |
| Scope | Command writing is opencode-only today (`sync` gates on `AgentTarget::Opencode`; `write_opencode_command` is hardcoded); the design's generalization claim is accurate. |
| Scope | `KNOWN_AGENT_TARGETS` is `[&str; 3]`, so `omp` would be silently dropped from the selection without the overlay change. |
| Testability | `@givn.modified` merge titles: all three delta titles match a scenario in the corresponding permanent feature file. |
| Testability | The interactive init PTY fixture toggles indices 0–2, so appending `.omp` at index 3 does not break it. |
| Use-case/Persona | `initialize-project` confirms `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`; the `fragments` reuse has no use case of its own and needs no Persona record. |

## Open questions

None. Every finding was resolved by an artifact edit or by the operator
decision recorded in Q8.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `initialize-a-project` gains a target variant; `attribute-commits` and `enable-commit-attribution` extend the reusable `fragments` home. No new use case. | accepted |
| Typed Includes / Extends relationships | `initialize-project` already includes `fragment: support`; no relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` and `developer--primary-user` are the two relevant confirmed Personas; the other two have no lens. | accepted |
| Actor and Persona distinction | Gherkin actors stay `the consumer` / `the user` / `omp`; no Persona is used as an actor. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Consumes the generated `.omp` skills/commands and executes the attribution hook. | "The omp target receives its own guidance and attribution integration"; "omp attributes a commit using its current model" |
| developer--primary-user | Selects the `omp` target and reads the installation report. | "Consumer sees the commit attribution installation report"; the modified `--agents all` scenarios |
| givn-maintainer--owner | Not relevant: no contributor-facing contract changes. | — |
| external-contributor--open-source-consumer | Not relevant: no published-interface change. | — |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | n/a |
| 2 | 02 architecture-constraints | No | No | Yes | n/a |
| 3 | 03 context-and-scope | Yes | Yes | Yes | yes — target-directories row and the new omp hook-boundary row |
| 4 | 04 solution-strategy | Yes | Yes | Yes | yes — Agent integration and Commit attribution rows name `.omp`/omp |
| 5 | 05 building-block-view | Yes | Yes | Yes | yes — `cli::skills` and the embedded-assets rows |
| 6 | 06 runtime-view | Yes | Yes | Yes | yes — 6.1 step 7 and the 6.1a omp ordering |
| 7 | 07 deployment-view | No | No | Yes | n/a |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | yes — 8.7 paths, target set, and ignore prefixes |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | yes — ADR-013 Consequences and the chapter-09 register entry |
| 10 | 10 quality-requirements | Yes | Yes | Yes | yes — Q-5 and the new Q-96 |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | yes — R-77 and R-78 |
| 12 | 12 glossary | Yes | Yes | Yes | yes — Agent target, Harness, Slash command |

- [x] All 12 rows assessed independently against proposal.md/design.md before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md saying "No" or omitting the row.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry in chapter 09; non-qualifying decisions name exactly one canonical target.
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in chapter 11.
- [x] Every ADR candidate records the structured qualification verdict and its route-specific target; `Must be shared` is supporting evidence only.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

Chapter 09 has no MADR entry for this change by design: the decision refines
ADR-013 (`AMEND_ADR`), and the register records the amendment rather than a new
record. The cheaper home for the rationale is ADR-013 plus chapter 8.7, which is
what the verdict names.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `design.md` | Pinned the generated hook's factory/handler/event contract and the `package.json` module fixture; named every `.omp` path in `managed_generated_paths`, `reconcile_addon_artifacts`, `rules.rs`, and the CLI report sites; fixed the D1 wording and the interaction-coverage claim; added the Personas section. |
| `proposal.md` | Narrowed the managed-upgrade bullet to "refreshes the omp integration files when `omp` is a configured target". |
| `specs/fragments/attribute-commits.feature` | Added the reasoning-mode assertion to the omp `@e2e` scenario. |
| `arc42.md` | Row 4 now Yes (chapter 04), row 9 names the chapter-09 register entry, row 11 covers R-78, row 12 now Yes (glossary terms). |
| `docs/arc42/03,04,05,06,08,10,11,12`, `adr/ADR-013` | Chapter updates for the new target and integration. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D1`, `D2`) resolves in the design's `## Decisions` ledger or is carried with a named owner.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated if any missing scenarios were identified.
- [x] `givn lint --change add-omp-agent-target` exited 0 at sign-off; the tasks phase adds `@wip` to each delta scenario and removes it at that scenario's RED, so lint exits 2 (WIP) until the last scenario lands.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
