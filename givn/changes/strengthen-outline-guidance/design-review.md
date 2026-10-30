# Design Review: strengthen-outline-guidance

Review kind: self-review
Author: agent
Reviewer context: agent

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| Q1 | Tech choices | Does the renderer already support an `addons.outline` condition? | Closed — yes; no source change |
| Q2 | Testability | Is the addon-off absence assertion meaningful? | Closed — `outline` appears only in the conditional blocks |
| Q3 | Missing scenarios | Does every required readability outcome have a distinct assertion? | Closed — six step assertions plus five review assertions |
| Q4 | Scope | Is the review template contradiction removed without breaking the existing scenario? | Closed — the modified delta carries the new assertion |
| Q5 | Risk | What is the most likely failure during implementation? | Closed — phrase drift while wrapping; mitigation is exact-phrase assertions |
| Q6 | Use-case and Persona context | Is the owning use case preserved and Personas confirmed? | Closed — obtain-effective-guidance, agent/developer |
| Q7 | Decision ledger | Does every proposal question resolve? | Closed — the proposal records no `D<n>` |
| Q8 | Contracts and components | Does the change introduce or move a contract? | Closed — no new runtime contract |
| Q9 | Interface classification | Could this be a Web UI instead of a terminal interface? | Closed — no browser-rendered surface |
| Q10 | ADR qualification | Does the guidance decision need a new ADR? | Closed — no; feature plus chapter 08 are the canonical homes |
| Q11 | Architecture documentation | Does chapter 08 match the plan? | Closed — hardening updated the stale serving-path text |

### Q1 — Renderer support for the outline condition

- Status: closed (agent, codebase exploration)
- Blocks: whether `src/` needs a change.
- Context: the plan expects conditional rendering with no code change. If the
  renderer did not know `outline`, the wrapping would fail the sync or render
  nothing.
- Question: Does the existing instruction renderer accept
  `{{#if addons.outline}}` and resolve it from project configuration?
- Recommended: use the existing conditional directive; no source change.
- Options:
  1. **Use the existing directive** — the key is already registered.
     In practice: `givn instructions steps` omits the REFACTOR block when
     `addons.outline: false`.
     Gain: no new code, no new failure mode.
     Cost: none.
  2. **Add a new code path** — register a second mechanism.
     In practice: a parallel conditional vocabulary.
     Gain: none.
     Cost: duplicated rendering logic.
- Resolution: **Use the existing directive** because
  `src/skills/render.rs::KNOWN_ADDON_KEYS` contains `outline`,
  `manifest_addon_enabled` maps it, and `src/cli/instructions.rs::addon_flags`
  feeds the same keys to the instruction renderer. Rejected: a new code path,
  because it duplicates an existing mechanism.

### Q2 — Meaningful addon-off assertion

- Status: closed (agent, codebase exploration)
- Blocks: the addon-off scenario's value.
- Context: asserting the absence of "the advisory outline section" could pass
  even when other outline text leaks. The strongest available assertion is the
  absence of the word `outline` when the addon is disabled.
- Question: Does either instruction contain `outline` outside the conditional
  blocks?
- Recommended: assert absence of `outline` for the disabled fixture and
  presence of a distinctive phrase for the enabled fixture.
- Options:
  1. **Assert absence of `outline`** — a whole-word-level leak check.
     In practice: `givn instructions review` with the addon off prints no
     `outline` occurrence.
     Gain: catches leaks from either block.
     Cost: a future legitimate mention outside the blocks would need the
     scenario updated.
  2. **Assert absence of one section heading** — narrower.
     In practice: only `## Advisory outline evidence` is checked.
     Gain: less coupled to wording.
     Cost: misses the steps block and the review-context paragraph.
- Resolution: **Assert absence of `outline`** because a repository grep shows
  every occurrence in `assets/instructions/steps.md` and
  `assets/instructions/review.md` sits inside the blocks being wrapped, so the
  assertion is meaningful today. Rejected: the narrow heading check, because
  it misses two of the three blocks.

### Q3 — Readability outcomes are separately asserted

- Status: closed (agent, design read)
- Blocks: test completeness.
- Context: the readability contract has several independent claims. One
  catch-all assertion would pass while a specific claim regressed.
- Question: Does the delta assert every claim the plan lists?
- Recommended: bind each claim to its own Then.
- Options:
  1. **Separate Thens** — responsibility/call flow, naming vocabulary, generic
     names, metrics never overruling, source confirming, fix-or-finding.
     In practice: a regression in the naming sentence fails exactly one Then.
     Gain: precise diagnosis.
     Cost: more bindings.
  2. **One combined Then** — a single phrase sweep.
     In practice: one failing step for any regression.
     Gain: fewer bindings.
     Cost: poor diagnosis and a weak contract.
- Resolution: **Separate Thens** because instruction regressions are otherwise
  indistinguishable. Rejected: the combined Then, because it hides which rule
  disappeared.

### Q4 — Review template contradiction

- Status: closed (agent, codebase exploration)
- Blocks: the template change and its test.
- Context: the template is ejected raw, so any addon-specific phrase leaks to
  every project. The permanent scenario currently asserts the retired-gate
  phrase.
- Question: How does the existing scenario keep passing while the phrase is
  removed?
- Recommended: modify the scenario in the delta to assert the new sentence and
  the absence of `outline verdict`; leave the old binding unused until the
  cleanup commit.
- Options:
  1. **Modify the scenario in the delta** — the change owns the contract.
     In practice: the merged permanent scenario asserts
     `records evidence and material mismatches` and no `outline verdict`.
     Gain: one forward authority.
     Cost: the permanent scenario is stale until archive.
  2. **Keep both phrases** — leave the retired phrase.
     In practice: addon-disabled projects still read `outline verdict`.
     Gain: no test edit.
     Cost: violates the change.
- Resolution: **Modify the scenario in the delta** because the plan requires
  the template to carry no outline wording. Rejected: keeping the phrase,
  because it defeats the addon-disabled contract.

### Q5 — Most likely implementation failure

- Status: closed (agent, risk assessment)
- Blocks: batch sequencing.
- Context: the assets already contain phrases asserted verbatim. Wrapping and
  rewrapping paragraphs is the most likely place to break one.
- Question: What is the single most likely way this change fails?
- Recommended: phrase drift while wrapping; preserve every listed sentence and
  run the exact-phrase scenarios after each batch.
- Options:
  1. **Preserve listed phrases and run each binding** — surgical edits.
     In practice: the REFACTOR cadence sentence remains byte-identical.
     Gain: no hidden regressions.
     Cost: slightly more careful editing.
  2. **Rewrite the blocks freely** — cleaner prose.
     In practice: existing assertions fail in unrelated places.
     Gain: better prose.
     Cost: couples the change to broad test rewrites.
- Resolution: **Preserve listed phrases and run each binding** because the
  plan requires it and the phrases are already the tested contract. Rejected:
  free rewriting, because it expands scope without behavior gain.

### Q6 — Use case and Persona context

- Status: closed (agent, artifact read)
- Blocks: design sign-off.
- Context: the change serves agents and developers reading guidance.
- Question: Is the owning use case identified and are confirmed Personas
  named without invention?
- Recommended: obtain-effective-guidance with assess-code-structure and
  verify-change as supporting owners; agent and developer Personas.
- Options:
  1. **Reuse the existing use case and Personas** — no new contract.
     In practice: the served guidance remains the same use-case action.
     Gain: no vocabulary or actor drift.
     Cost: none.
  2. **Create a new use case** — a separate guidance use case.
     In practice: duplicate routing for the same action.
     Gain: a narrower name.
     Cost: a second authority for the same behavior.
- Resolution: **Reuse the existing use case and Personas** because the change
  alters served text for an existing action. Rejected: a new use case, because
  it duplicates ownership.

### Q7 — Decision ledger

- Status: closed (agent, proposal read)
- Blocks: design sign-off.
- Context: the design must resolve every proposal `D<n>`.
- Question: Does any proposal question remain unresolved?
- Recommended: none; the proposal records no open question.
- Options:
  1. **Record none** — the operator plan fixed the decisions.
     In practice: the design's `## Decisions` lists the operative choices.
     Gain: honest ledger.
     Cost: none.
  2. **Invent a carried question** — a place for future wording changes.
     In practice: a fake open item.
     Gain: none.
     Cost: an unresolved design item.
- Resolution: **Record none** because the proposal states no open question.
  Rejected: inventing a carried question, because it would block sign-off for
  no reason.

### Q8 — Contracts and components

- Status: closed (agent, design read)
- Blocks: the mandatory Contracts and Components table.
- Context: the architecture-principles addon is enabled, so the table is
  mandatory even though the change adds no runtime module.
- Question: Does the change introduce or move a contract?
- Recommended: no; the existing effective-guidance contract and its embedded
  asset component are unchanged.
- Options:
  1. **Record the existing contract** — honest, minimal.
     In practice: one row names effective guidance, its use case, and its
     embedded-asset implementation.
     Gain: satisfies the component check.
     Cost: none.
  2. **Declare a new component** — model the guidance blocks separately.
     In practice: a component with no consumer and no boundary.
     Gain: none.
     Cost: speculative abstraction.
- Resolution: **Record the existing contract** because no boundary moves.
  Rejected: a new component, because it has no independent consumer.

### Q9 — Interface classification

- Status: closed (agent, design read)
- Blocks: the Visual Design Contract.
- Context: the change alters CLI text and an ejected Markdown template.
- Question: Is this a Web UI?
- Recommended: no; it is a terminal interface.
- Options:
  1. **Terminal** — stdout and file output read by a human or agent.
     In practice: the E2E asserts ejected Markdown and a captured transcript.
     Gain: correct evidence kind.
     Cost: none.
  2. **Web** — require a browser driver.
     In practice: no page exists to render.
     Gain: none.
     Cost: a false browser requirement.
- Resolution: **Terminal** because no HTML, CSS, or JavaScript is rendered and
  no HTTP surface exists. Rejected: Web, because the opposite classification
  does not hold.

### Q10 — ADR qualification

- Status: closed (agent, arc42 assessment)
- Blocks: chapter 09.
- Context: the decision is guidance policy about advisory evidence and review
  findings.
- Question: Does the decision fix a lasting architectural boundary?
- Recommended: no; the feature specification and chapter 08 own it.
- Options:
  1. **No ADR** — canonical artifact routing.
     In practice: `arc42.md` records `NOT_QUALIFIED` with the feature and
     chapter 08 as the canonical homes.
     Gain: no duplicated rationale.
     Cost: none.
  2. **New ADR** — durable decision record.
     In practice: a record with no component, storage, or deployment boundary.
     Gain: a formal record.
     Cost: a record that duplicates the feature contract.
- Resolution: **No ADR** because the decision is observable workflow behavior
  owned by an existing use case plus Gherkin. Rejected: a new ADR, because the
  qualification's architectural-commitment question fails.

### Q11 — Architecture documentation

- Status: closed (agent, review finding and hardening)
- Blocks: design sign-off.
- Context: chapter 08 described the previous serving path (one conditional
  step sentence) and did not carry the comprehension test.
- Question: Does chapter 08 match the design?
- Recommended: update 8.26a now as hardening; the content is documentation,
  not runtime behavior.
- Options:
  1. **Update chapter 08 in hardening** — the review verifies real content.
     In practice: 8.26a names both conditional blocks, the review-template
     phrase removal, the comprehension test, and the advisory/finding
     distinction.
     Gain: no stale architecture claim.
     Cost: the doc edit lands before the behavior commit.
  2. **Defer to implementation** — leave a known contradiction.
     In practice: design-review would fail on a stale chapter.
     Gain: strict batch ordering.
     Cost: an unreviewed contradiction.
- Resolution: **Update chapter 08 in hardening** because sign-off requires the
  chapter to match the design. Rejected: deferral, because the review would be
  signing off a known contradiction.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Tech choices | `outline` is a known conditional key in `src/skills/render.rs` and is mapped by `src/cli/instructions.rs`; no source change is needed. |
| Testability | `tests/features_runner.rs` uses `.fail_on_skipped()`; a missing or undefined step fails. |
| Testability | `batch_guidance`, `normalized_guidance`, `served_instruction`, and `served_artifact_instruction` already exist and cover the fixtures the new scenarios need. |
| Missing scenarios | The review template's existing `@e2e` scenario keeps its transcript step, so the changed template scenario keeps its evidence. |
| Risk | `tests/steps/serve_extended_rules_steps.rs` already asserts the listed REFACTOR phrases; preserving them avoids broad test rewrites. |
| Scope | The generated skills currently contain no `outline` occurrence, so the no-skill-guidance assertion is meaningful. |

## Open questions

None. Every question was answered from the plan, the design, or the existing
codebase; no user decision was required.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `obtain-effective-guidance` owns effective guidance; `assess-code-structure` and `verify-change` own outline use and review findings; the fragments document owns addon serving and generated skills. | Not required |
| Typed Includes / Extends relationships | The fragments are included by the owning use cases; no new relationship is introduced. | Not required |
| Confirmed Persona coverage | `agent--technical-role` and `developer--primary-user` are confirmed and affected. | Not required |
| Actor and Persona distinction | Gherkin actors stay domain actors; Personas remain review lenses only. | Not required |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Primary consumer of served guidance and generated skills. | `givn instructions steps/review/tasks` assertions and the `givn-implement` routing scenario |
| developer--primary-user | Reads the outline guide and the review guidance. | `docs/outline.md`, README addon bullet, review readability scenario |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | N/A — not affected |
| 2 | 02 architecture-constraints | No | No | Yes | N/A — not affected |
| 3 | 03 context-and-scope | No | No | Yes | N/A — not affected |
| 4 | 04 solution-strategy | No | No | Yes | N/A — not affected |
| 5 | 05 building-block-view | No | No | Yes | N/A — not affected |
| 6 | 06 runtime-view | No | No | Yes | N/A — not affected |
| 7 | 07 deployment-view | No | No | Yes | N/A — not affected |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.26a now states both conditional blocks, the review-template phrase removal, the comprehension and naming tests, and the ordinary-review-finding distinction |
| 9 | 09 architecture-decisions | No | No | Yes | N/A — no new ADR; canonical homes recorded |
| 10 | 10 quality-requirements | No | No | Yes | N/A — not affected |
| 11 | 11 risks-and-technical-debt | No | No | Yes | N/A — the existing advisory-outline risk remains documented |
| 12 | 12 glossary | No | No | Yes | N/A — existing terms reused |

- [x] All 12 rows assessed independently against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md
      saying "No" or omitting the row.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry
      in chapter 09; non-qualifying decisions name exactly one canonical target.
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in
      chapter 11 (no new MADR was created).
- [x] Every ADR candidate uses the concise qualification recorded once in the
      change assessment and names its route-specific target; `Must be shared`
      is supporting evidence only.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `docs/arc42/08-crosscutting-concepts.md` | Rewrote 8.26a: both outline blocks render behind `{{#if addons.outline}}`, the review template carries no outline verdict phrase, and the comprehension, naming, and ordinary-review-finding rules are recorded. |
| `design.md` | Added the interface-classification rationale and the transcript evidence path. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger or is carried with a named owner.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated if any missing scenarios were identified.
- [x] `givn lint --change strengthen-outline-guidance` exits 0 or 2 (no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
