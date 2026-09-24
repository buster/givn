# Design Review: change-release-note

## Grilling log

The table is the scannable index; the full record for each question is
written in a per-question block below it. The `Question` cell holds the short
title, not the whole record.

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | E2E fidelity / scope | Are the generated command surfaces pinned? | closed 2026-09-24 (developer) — extend the generated-guidance scenario (A) |
| 2 | Domain context | Is the authoring agent a confirmed review persona? | closed 2026-09-24 (developer) — confirm it in the proposal (A) |
| 3 | Testability | Are the acceptance substrings case- and wrap-fragile? | closed 2026-09-24 (developer) — lowercase the assertions (B) |
| 4 | Scope | Heading or labeled line for the release note? | closed 2026-09-24 (developer) — heading only (A) |
| 5 | ADR qualification | Is the existing-ADR check evidenced? | closed by exploration + hardening — evidence recorded |

### Q1 — Are the generated command surfaces pinned by the acceptance check?

- Status: closed 2026-09-24 (developer)
- Blocks: the GREEN definition of the generated implementation guidance
  scenario — whether a partial edit can pass.
- Context: the design edits roughly ten surfaces that state the revision-subject
  rule, but the scenario asserted only the three generated skills and the
  `AGENTS.md` hint. An implementation could update the skills and leave the
  generated opencode commands (the generated `givn-steps`, `givn-implement`,
  and `givn-tasks` commands) with the superseded subject rule and still report
  GREEN — the contradictory guidance this change exists to remove.
- Question: should the acceptance check pin the generated command surfaces
  and the absence of the superseded rule too?
- Recommended: A — extend the same scenario; the command assertions reuse
  existing bindings, one inventory action keeps one `@e2e` scenario.
- Options:
  1. **Extend the scenario** — three `the generated givn-<name> command
     contains "carries the release note as its message subject"` assertions
     plus `generated guidance does not contain "message references the
     scenario title"`.
     In practice: after `givn skills sync`, `.opencode/commands/givn-implement.md`
     must carry the canonical sentence; a leftover old sentence fails the run.
     Gain: a partial edit can no longer report GREEN.
     Cost: five Then lines, no new bindings, no new inventory row.
  2. **Leave the spec unchanged** — commands and instructions enforced only
     by `tasks.md` and manual review.
     In practice: the skills say release note, the command still says scenario
     title, and the runner exits 0.
     Gain: no spec edit.
     Cost: the exact failure this change removes can ship green.
  3. **Add a fourth `@e2e` scenario for `givn instructions tasks`** — a
     separate scenario for the instruction output.
     In practice: a fourth scenario with a second inventory row for the same
     read-the-guidance action.
     Gain: same coverage.
     Cost: over-covers; violates one-E2E-per-action.
- Resolution: **extend the scenario** because the command halves are part of
  the same real action and the reused bindings make it cheap. Rejected:
  leave unchanged because a known silent-drift path is unacceptable; fourth
  scenario because it over-covers one action with two `@e2e` scenarios.

### Q2 — Is the authoring agent a confirmed review persona?

- Status: closed 2026-09-24 (developer)
- Blocks: the persona disposition the design review signs off.
- Context: the proposal confirmed `developer--primary-user` and
  `givn-maintainer--owner`, but the design and the permanent use case also
  name `agent--technical-role`. The agent is the actor that executes the new
  rule: it reads the generated guidance and composes the revision subject, so
  its review lens is genuine, not decorative.
- Question: does the agent persona belong in the proposal's confirmed list?
- Recommended: A — confirm it, matching the use case and design.
- Options:
  1. **Confirm the agent** — add it to the proposal's Confirmed Personas line.
     In practice: `Confirmed Personas: developer--primary-user,
     agent--technical-role, givn-maintainer--owner`.
     Gain: one confirmation list across proposal, use case, and design.
     Cost: one proposal line.
  2. **Demote it in the design** — keep two confirmed personas and record the
     agent as an inherited supporting role.
     In practice: the design contradicts the permanent use-case Personas list.
     Gain: no proposal edit.
     Cost: an unconfirmed review lens on the very actor the rule targets.
- Resolution: **confirm the agent** because the use case already carries it and
  it is the rule's executor. Rejected: demote because it contradicts the
  durable use-case contract.

### Q3 — Are the acceptance substrings case- and wrap-fragile?

- Status: closed 2026-09-24 (developer)
- Blocks: whether a correct edit can fail GREEN for the wrong reason.
- Context: the acceptance checks assert exact substrings case-sensitively and
  line-wise. The scaffold and proposal-guidance checks asserted capital
  `Release note`; the design pinned only lowercase wording. A hand-wrapped
  80-column asset or a different label spelling would fail an otherwise
  correct implementation.
- Question: how should the checks and the design handle case and line breaks?
- Recommended: A — pin the capital anchor and make each asserted phrase
  unbroken on one line.
- Options:
  1. **Pin capitals and the line rule** — extend the canonical-wording block
     with `Release note` and a single-line requirement.
     In practice: the asset carries `Release note` in the rule and the whole
     53-character sentence on one unwrapped line.
     Gain: removes both failure modes.
     Cost: two design lines.
  2. **Lowercase the assertions** — assert `release note` in the scaffold and
     instruction checks.
     In practice: the label spelling stops mattering; the sentence still must
     not wrap.
     Gain: case-tolerant checks.
     Cost: drops the field-name capitalization from the assertion.
  3. **Make the shared contains steps case-insensitive** — over-broad; changes
     assertion semantics for the whole suite.
- Resolution: **lowercase the assertions** because the field capitalization is
  a presentation choice, not the behaviour under test. The single-line
  requirement stays in the design as an implementation constraint so the
  phrase assertions cannot fail on a wrap. Rejected: capital pin because it
  couples the assertion to presentation; case-insensitive steps because they
  change shared semantics.

### Q4 — Heading or labeled line for the release note?

- Status: closed 2026-09-24 (developer)
- Blocks: the scaffold shape this change introduces and its first instance.
- Context: the design promised a `Release note: <sentence>` line while this
  change's own proposal used a `## Release note` heading with a prose sentence.
  The acceptance check cannot distinguish the shapes, so the first change
  under the rule would not model the template it introduces.
- Question: heading, labeled line, or heading plus a second label?
- Recommended: A — heading only, one term, one label.
- Options:
  1. **Heading only** — `## Release note` with a guidance quote carrying the
     rule and the authored sentence below.
     In practice: the scaffold renders the heading, the required-sentence
     guidance, and a placeholder sentence; no second label.
     Gain: one term, one label, matches this change's own proposal.
     Cost: the sentence itself is not machine-labeled; the heading is.
  2. **Labeled line only** — no section, a `Release note: <sentence>` line
     under the title.
     In practice: the proposal structure starts with an unlabeled quote block.
     Gain: greppable line.
     Cost: loses the section shape.
  3. **Heading plus `Changelog summary:` line** — a second label for the same
     sentence.
     In practice: `## Release note` followed by `Changelog summary: ...`.
     Gain: names the derived artifact.
     Cost: the glossary records "change summary" and "changelog entry" as
     rejected anti-terms; a changelog line is derived, never authored.
- Resolution: **heading only** because the heading already names the term and a
  second label reintroduces the anti-term drift the glossary guards against.
  Rejected: labeled line because it drops the section; heading plus
  `Changelog summary` because the release note is the source, not the derived
  changelog line.

### Q5 — Is the existing-ADR check evidenced?

- Status: closed 2026-09-24 (reviewer, by codebase exploration + hardening)
- Blocks: the auditability of the `existing_adr_check: PASS` verdict.
- Context: `arc42.md` asserted `existing_adr_check: PASS` without evidence.
  The search covered the active register and the archived records under
  `docs/arc42/adr/`; no ADR owns commit-subject composition or
  generated-guidance policy.
- Question: does anything need to be amended or superseded?
- Resolution: no. The evidence sentence is recorded in `arc42.md` under
  `## Notes`; the route stays `CANONICAL_ARTIFACT` with `design.md` as the
  canonical home.

## Resolved by codebase exploration

Items answered without user input by exploring the existing codebase.

| Branch | Finding |
|---|---|
| Scope | The delta adds one rule, one capability, and three inventory rows to the permanent `create-change` contract; nothing else changed (`diff givn/specs/create-change/usecase.md` against the delta). |
| Testability | Every Given/When/Then in the three scenarios is bound by an existing step (`tests/steps/mod.rs`, `tests/steps/prose_consolidation_steps.rs:35,97,106,128`, `tests/steps/run_execution_e2e_steps.rs:231`). A delta run before the asset edits fails each scenario at its first new Then — genuine RED, no production code needed. |
| Testability | No permanent scenario pins the superseded commit wording; `givn lint --change change-release-note` reports only WIP findings plus cross-delta shape/subset advisories, exit 2. |
| E2E fidelity | The use-case Interactions rows and the design matrix match one-to-one; the driver is the real `givn` subprocess and `verify.e2e_command` (`./run-tests.sh --e2e`) is distinct from `verify.command`. |
| Interface classification | Opposite test: nothing renders HTML/CSS/JS in a browser. Two archived CLI-only guidance changes passed review with the same "visual block N/A" disposition. Not a deferral. |
| Decision ledger | D1 and D2 are answered in the proposal and realized in the design; D3 was carried to the design and is decided there. No unresolved `D<n>`. |
| Risk | Most likely failure: hand-wrapped canonical sentences across ~13 edited surfaces. Mitigation: the canonical-wording block, the pinned single-line rule, and the generated-guidance assertions now covering the command surfaces. |
| Allowlist | Repo-wide search for the old rule (`scenario title`, `feat(<capability>)`, `test(e2e)`) finds every remaining surface in the design's file-change table; the root `AGENTS.md` is regenerated by the mandated self-dogfood and is never staged automatically under `rules: versioned`. |
| Ubiquitous language | `docs/arc42/12-glossary.md` already holds the **Release note** term with the scenario-title contrast and anti-terms before sign-off. No changed asset uses a competing synonym. |

## Open questions

Questions that could not be resolved and were flagged explicitly.

| # | Question |
|---|---|
| — | None. |

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `create-change` owns the canonical guidance set; `change-release-note` is `NEW in create-change`, matching the proposal's routing. | Accepted (no objection raised). |
| Typed Includes / Extends relationships | Unchanged: includes the `support` fragment, extends none. | Accepted. |
| Confirmed Persona coverage | Proposal now confirms `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`, matching the use case and design. | Confirmed (Q2, option A). |
| Actor and Persona distinction | Gherkin actors are Developer and the supporting Authoring agent; Personas are review lenses only and are not used as actors. | Accepted. |

### Persona disposition

One row per confirmed persona, recording the relevance and the evidence; when
no persona is confirmed, record the reason and the operator confirmation.

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Authors the release note in the proposal and reads it in the changelog. | Scaffold field and proposal instruction carry the rule; CLI text output. |
| agent--technical-role | Executes the rule: reads the generated guidance and composes every revision subject. | Generated skills, commands, and `AGENTS.md` hint carry the canonical sentence. |
| givn-maintainer--owner | Keeps one canonical wording across all guidance surfaces. | One sentence reused in the assets; glossary term with anti-terms. |

## Architecture documentation (arc42) check

The 12 rows below were assessed from `proposal.md`/`specs/`/`design.md`
before reading `arc42.md`'s table.

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | N/A |
| 2 | 02 architecture-constraints | No | No | Yes | N/A |
| 3 | 03 context-and-scope | No | No | Yes | N/A |
| 4 | 04 solution-strategy | No | No | Yes | N/A |
| 5 | 05 building-block-view | No | No | Yes | N/A |
| 6 | 06 runtime-view | No | No | Yes | N/A |
| 7 | 07 deployment-view | No | No | Yes | N/A |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | 8.7 now records the release-note rule and that `givn commit` neither composes nor validates. |
| 9 | 09 architecture-decisions | No | No | Yes | No qualified ADR; verdict routed to `design.md`. |
| 10 | 10 quality-requirements | No | No | Yes | N/A |
| 11 | 11 risks-and-technical-debt | No | No | Yes | No new ADR consequence; D3 trade-off stays in `design.md`. |
| 12 | 12 glossary | Yes | Yes | Yes | **Release note** term with contrast and anti-terms present. |

- [x] All 12 rows assessed independently against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md
      saying "No" or omitting the row — any such row is a finding,
      not a note.
- [x] Every qualified ADR candidate in design.md has a corresponding MADR entry
      in chapter 09; non-qualifying decisions name exactly one canonical target.
      (No qualified candidate; `design.md` named once.)
- [x] Every MADR "Bad, because..." consequence has a counterpart entry in
      chapter 11 (risks-and-technical-debt). (No MADR.)
- [x] Every ADR candidate records the structured qualification verdict and its
      route-specific target; `Must be shared` is supporting evidence only.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

Edits applied to existing artifacts as a result of grilling.

| Artifact | Change summary |
|---|---|
| `specs/create-change/change-release-note.feature` | Scaffold and guidance checks assert lowercase `release note` (Q3); the generated-guidance scenario gains the three generated-command assertions and the absence check for `message references the scenario title` (Q1). |
| `proposal.md` | Confirmed Personas line adds `agent--technical-role` (Q2). |
| `design.md` | Template row and canonical-wording block describe the heading-only shape, the lowercase anchor, the single-line rule, and the command/absence assertions. |
| `arc42.md` | `## Notes` records the existing-ADR search evidence for the `existing_adr_check: PASS` verdict (Q5). |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger or is carried with a named owner.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated if any missing scenarios were identified.
- [x] `givn lint --change change-release-note` exits 0 or 2 (no parse errors) — exit 2, WIP only.
- [x] Architecture documentation (arc42) check completed above (or marked
      N/A because the addon is not enabled) — never left blank.

**DESIGN-REVIEW: PASS**
