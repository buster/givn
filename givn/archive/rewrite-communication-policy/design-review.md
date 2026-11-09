# Design Review: rewrite-communication-policy

Review kind: self-review
Author: givn-agent
Reviewer context: givn-agent

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Is 11 rules the minimum that closes the documentation gap? | Closed — measured against four repositories |
| 2 | Testability | Can every new Then fail in RED? | Closed — OLD_PRODUCT_RED captured |
| 3 | E2E fidelity | Does the new scenario need its own `@e2e`? | Closed — variant of one normalized action |
| 4 | Interaction coverage | Does `initialize-project` need a new inventory row? | Closed — no interaction table; action already covered |
| 5 | Contracts and components | Is `CommunicationPolicyBlock` one contract with one component? | Closed — asset plus existing renderer |
| 6 | ADR qualification | Does this content decision need an ADR? | Closed — ADR action NONE |
| 7 | Risk | Is the 300-word assertion brittle? | Closed — 21% headroom, single lever |
| 8 | Ubiquitous language | Is the glossary term current? | Closed — term updated in the same batch |
| 9 | Structural model | Does the domain model change? | Closed — no boundary or relationship change |

### Q1 — Block content and scope

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: the size and content of `assets/addons/communication-policy/policy.md`
- Context: the draft rewrite carried 502 words, per-document budgets, named documentation files, a buzzword blacklist, and a restated dialog-example obligation. Each element must earn its place because the addon is default-on and syncs into every project's `AGENTS.md`.
- Question: Which rules belong in the shipped block?
- Recommended: keep the nine existing presentation obligations, add six documentation rules that name formats, and drop everything that names a repository layout or duplicates the dialog policy.
- Options:
  1. **Minimal documentation rules** — 11 rules, two sections, 237 words.
     In practice: `givn addons enable communication_policy` writes a block whose documentation section names tables, quick-start boundaries, and the spec/`--help` split.
     Gain: 47% of the draft's context cost; no repository is declared non-conforming.
     Cost: no README size guard in the block.
  2. **Full draft** — 502 words with budgets and named files.
     In practice: kiekste's 683-line README violates the shipped budget immediately.
     Gain: stronger layout pressure.
     Cost: false non-conformance in three of four projects; duplicated dialog content.
- Resolution: **Minimal documentation rules** because the budget clause contradicts the measured corpus and the dialog duplication has an unconditional owner. Rejected: the full draft because it prescribes layout the addon cannot enforce.

### Q2 — RED provability

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: the task list's RED evidence
- Context: the new assertions read generated text; a step that asserts a substring can pass vacuously if the block is never written.
- Question: Do the new Then-steps fail against the current product?
- Recommended: run the scenario against the pre-change asset and capture the failure.
- Options:
  1. **Assert the rendered block** — extract the fenced section and assert named phrases plus the word count.
     In practice: the pre-change asset fails at `**Tabulate mappings.**`; captured in the task evidence.
     Gain: the failure is a product-content failure, not an undefined step.
     Cost: none.
- Resolution: **Assert the rendered block** because the recorded RED output shows the old 101-word block failing the first phrase assertion.

### Q3 — E2E classification

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: whether the delta adds `@e2e`
- Context: `enable-optional-addons` already carries one `@e2e` scenario for addon enablement through a real CLI subprocess.
- Question: Is the new scenario a distinct user action?
- Recommended: no — it is a content invariant of the same action; keep it regular.
- Options:
  1. **Regular scenario** — asserts the generated block content.
     In practice: `givn addons enable communication_policy` is already driven end-to-end by `Consumer enables the outline addon and it is listed as enabled` (same action, different addon).
     Gain: no duplicate E2E coverage.
     Cost: none.
  2. **Second `@e2e`** — duplicate coverage of one action.
     In practice: two scenarios spawn the same command.
     Gain: none.
     Cost: flagged as over-coverage.
- Resolution: **Regular scenario** because the action is already normalized to one inventory entry.

### Q4 — Interaction inventory

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: `usecase.md` inventory edits
- Context: the inventory lives in the owning use case's `## Interactions` table.
- Question: Does `initialize-project/usecase.md` need a new entry?
- Recommended: no — the file has no `## Interactions` section and the action is already covered by the capability's `@e2e` scenario.
- Options:
  1. **No inventory change** — the delta adds a regular scenario only.
     In practice: `givn lint` reports the corpus clean.
     Gain: no duplicate mapping.
     Cost: none.
- Resolution: **No inventory change** because the enablement action is unchanged and already mapped.

### Q5 — Contracts and components

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: the architecture-impact verdict
- Context: the block is one asset rendered by `cli::addons`/`skills::targets`.
- Question: Is this one contract with exactly one implementing component?
- Recommended: yes — `CommunicationPolicyBlock`, implemented by the asset plus its existing renderer.
- Options:
  1. **One contract** — no split.
     In practice: `skills::targets::sync_communication_policy` reads the embedded asset and upserts one fenced block.
     Gain: no new component or port.
     Cost: none; the split trigger (a second block or per-project variants) is not met.
- Resolution: **One contract** because the consumer surface and implementation path are single.

### Q6 — ADR qualification

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: chapter 09 and the ADR directory
- Context: the change fixes no component boundary, dependency direction, or compatibility surface.
- Question: Does any decision qualify for an ADR?
- Recommended: no — record ADR action `NONE` in `arc42.md`.
- Options:
  1. **No ADR** — rationale stays in `arc42.md` and the asset.
     In practice: `docs/arc42/adr/` gains no file; chapter 09 is untouched.
     Gain: no duplicate record for a content choice.
     Cost: none.
- Resolution: **No ADR** because the qualification records no durable architectural commitment.

### Q7 — Word-budget brittleness

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: the `at most 300 words` assertion
- Context: a hard count can fail on a legitimate edit.
- Question: Is 300 the right bound?
- Recommended: yes — the new block is 237 words (including the begin marker line), leaving 21% headroom while rejecting the 502-word draft.
- Options:
  1. **300-word bound** — one assertion.
     In practice: a 502-word block fails; a 237-word block passes with room for one added rule.
     Gain: protects resident context from silent re-expansion.
     Cost: a future four-rule addition needs a deliberate bound change.
- Resolution: **300-word bound** because the asset is the single lever and the bound is the only automated guard against regrowth.

### Q8 — Ubiquitous language

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: glossary consistency
- Context: the addon's scope changed from presentation to presentation and documentation.
- Question: Does the glossary define the term as the block now behaves?
- Recommended: yes — update the row in the same batch.
- Resolution: **Glossary row updated** (`docs/glossary.md`, "Communication policy") because the definition must match the shipped block.

### Q9 — Structural model

- Status: closed 2026-11-09 (codebase exploration)
- Blocks: `docs/domain-model.md`
- Context: no aggregate, boundary, or relationship changes.
- Question: Does the domain model need an edit?
- Resolution: **No edit** because the change alters text content inside an existing component; a no-op edit would not be maintenance.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Scope | The draft's README budget contradicts the measured corpus (givn 132, watn 167, shoppinglist-070 210, kiekste 683 lines). |
| Tech choices | The asset is embedded through `rust_embed`; a change to `assets/` rebuilds the binary, so the RED run exercised the old text. |
| Missing scenarios | Only the documentation-content contract is new; enablement, rename refusal, and dialog independence already have scenarios. |
| Testability | The new step bodies assert named phrases in the extracted fence and fail with the old asset. |
| E2E fidelity | The capability's existing `@e2e` scenario drives a real `givn addons enable` subprocess. |
| Interaction coverage | No `## Interactions` table exists in `initialize-project/usecase.md`; `givn lint` reports clean. |
| Risk | The most likely failure is a step asserting a phrase the asset does not contain; the RED run proves the binding. |
| Contracts | `CommunicationPolicyBlock` maps to one asset and one renderer. |
| Decision timing | D1 is answered with measurements; no carried decision and no speculative abstraction. |

## Open questions

| # | Question |
|---|---|
| — | None. |

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `initialize-project` owns `enable-optional-addons`; the block is written during init/sync and addon enablement. | none needed |
| Typed Includes / Extends relationships | The use case declares no typed includes or extends for this capability. | none needed |
| Confirmed Persona coverage | `developer--primary-user` and `agent--technical-role` are confirmed in the use case. | none needed |
| Actor and Persona distinction | Gherkin actors stay the developer; Personas remain review lenses. | none needed |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Reads the generated block and the project docs it governs. | The block is written into `AGENTS.md`; the assertion runs through the real CLI. |
| agent--technical-role | Applies the block's formatting rules when authoring docs. | The block's documentation section names the formats the agent must produce. |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | N/A — unchanged |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | Yes — addon-scope constraint names presentation and documentation |
| 3 | 03 context-and-scope | Yes | Yes | Yes | Yes — fenced-block description updated |
| 4 | 04 solution-strategy | No | No | Yes | N/A — unchanged |
| 5 | 05 building-block-view | No | No | Yes | N/A — unchanged |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — sync step names the addon scope |
| 7 | 07 deployment-view | No | No | Yes | N/A — unchanged |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — guidance-ownership section updated |
| 9 | 09 architecture-decisions | No | No | Yes | N/A — no ADR; register unchanged |
| 10 | 10 quality-requirements | No | No | Yes | N/A — unchanged |
| 11 | 11 risks-and-technical-debt | No | No | Yes | N/A — unchanged |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — chapter links the standalone glossary, whose term is updated |

- [x] All 12 rows assessed independently against proposal.md/design.md before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md saying "No" or omitting the row.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry in chapter 09; non-qualifying decisions name exactly one canonical target (`arc42.md`, ADR action `NONE`).
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in chapter 11 — no new MADR was created.
- [x] Every ADR candidate uses the concise qualification recorded once in the change assessment.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text (all 12 files exceed the scaffold; chapter 12 links the standalone glossary by design).

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `design.md` | No change — grilling confirmed the planned content and the 300-word bound. |
| `specs/initialize-project/enable-optional-addons.feature` | `@wip` removed after the step definitions landed. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D1`) resolves in the design's `## Decisions` ledger.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated if any missing scenarios were identified.
- [x] `givn lint --change rewrite-communication-policy` exits 0 (clean).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
