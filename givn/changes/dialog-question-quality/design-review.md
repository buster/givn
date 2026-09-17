# Design Review: dialog-question-quality

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Tech choices | Must the dialog policy be retrievable without an active change? | agreed — fallback before change resolution |
| 2 | Missing scenarios | Does the ejected design-review template need an executable assertion? | agreed — eject scenario added |
| 3 | Risk | Does the local AGENTS.md absolute-mode block refresh after the policy edit? | agreed — re-apply plus scenario |
| 4 | Missing scenarios | Is the unknown-id error path of the instruction command covered? | agreed — error scenario added |
| 5 | Missing scenarios | Is the generated design-review command pointer asserted? | agreed — assertion added |
| 6 | Scope | Should review and archive guidance gain dialog-policy pointers? | agreed — handover list stays, boundary recorded |
| 7 | Testability | Which phrase proves the cross-reference gloss rule? | agreed — "one-line gloss on first use" |
| 8 | Architecture documentation | Is chapter 10 (quality requirements) affected? | agreed — row corrected to No |
| 9 | Architecture documentation | Is the existing-ADR check against ADR-048 recorded? | agreed — chapter 8.3b names it |

### Q1 — Dialog policy availability in a project with no active change

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether `givn instructions dialog` requires an active change, and
  therefore what the retrieval scenario can assert.
- Context: the instruction command resolves the change context before it looks
  at the manifest artifact list, so in a freshly initialized project with no
  change, `givn instructions dialog` would fail with "No active changes"
  even though the policy asset is resolvable. The delta scenario created a
  change first, which hid that behavior.
- Question: should the dialog policy be retrievable in any initialized
  project, or only while a change is active?
- Options:
  1. **Fallback before change resolution.**
     Meaning: a non-artifact id resolves the instruction first; an active
     change is required only for manifest artifacts.
     In practice: in a fresh project, `givn instructions dialog` prints the
     policy; `givn instructions specs` still says "No active changes".
     Gain: the policy is always one command away; no fixture change needed
     for a reader who just wants the rule.
     Cost: the fallback branch runs before the change check, so the ordering
     in `run_artifact` must be deliberate and documented.
  2. **Keep change resolution first.**
     Meaning: the policy is retrievable only with an active change or an
     explicit `--change`.
     In practice: a fresh project answers `givn instructions dialog` with
     "No active changes"; the reader must create a change first.
     Gain: no reordering; one code path.
     Cost: the canonical policy is unreachable exactly when a reader has no
     change context, which is the common case for consulting a rule.
- Resolution: **Option 1** because the policy is not change-scoped and
  requiring a change would make the canonical source unreachable for the
  reader who most needs it. Rejected: Option 2 because it preserves an
  ordering artifact, not a requirement.

### Q2 — Executable assertion for the design-review template shape

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the template's new per-question record shape is proven by
  a scenario or only by review inspection.
- Context: the permanent suite already proves the persona disposition table
  by ejecting `design-review:template` and asserting its phrases. The new
  index/per-question shape had no equivalent assertion, so a template edit
  could silently keep the old table.
- Question: should the ejected template's new shape be asserted?
- Options:
  1. **Add the eject scenario.**
     Meaning: a scenario ejects the template and asserts the index header
     words plus the per-question record phrases.
     In practice: `givn eject design-review:template` followed by assertions
     on "Branch", "Outcome", "In practice:", "Recommended:", "Rejected:".
     Gain: the template edit fails in RED when the shape is missing.
     Cost: one more scenario and five assertions.
  2. **Leave it to review inspection.**
     Meaning: no scenario; the review scan verifies the template.
     In practice: a reviewer opens the ejected file and compares it by hand.
     Gain: no new scenario.
     Cost: the shape has no executable guard, unlike its sibling table.
- Resolution: **Option 1** because the repository already proves template
  shape this way and an unasserted template edit is the exact failure this
  change removes. Rejected: Option 2 because it leaves the record shape
  unprotected against regression.

### Q3 — Absolute-mode policy refresh in this repository

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the local AGENTS.md block is refreshed and covered when the
  policy asset changes.
- Context: `givn skills sync` refreshes only the unconditional `givn-project`
  block; the absolute-mode policy block is written by
  `givn addons enable absolute_mode`. After the asset edit, the local
  AGENTS.md would keep the old wording, and no scenario covers the addon
  policy today.
- Question: should this change re-apply the block and assert it?
- Options:
  1. **Re-apply plus scenario.**
     Meaning: run `givn addons enable absolute_mode` after the asset edit and
     add an install-extras scenario asserting the generated AGENTS.md block
     contains the new line.
     In practice: the new sentence appears in this repository's AGENTS.md in
     the same change, and the scenario fails in RED before the asset edit.
     Gain: the change ships whole; the addon policy gains its first
     executable assertion.
     Cost: one scenario and one task step.
  2. **Asset edit only.**
     Meaning: the block refreshes at some future init.
     In practice: sessions in this repository keep the weaker rule.
     Gain: smallest diff.
     Cost: the change ships half-applied with no guard.
- Resolution: **Option 1** because the unrefreshed local block is the most
  likely half-done outcome and the scenario is cheap. Rejected: Option 2
  because it leaves the repository's own guidance stale.

### Q4 — Unknown-id error path after the fallback

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether an unresolvable instruction id keeps a clear error.
- Context: the fallback adds a branch before the known-artifact error. If it
  swallows the error or reports the wrong one, a typo becomes confusing; the
  existing unknown-artifact scenarios cover `givn check` and `givn eject`,
  never `givn instructions`.
- Question: should the error path be asserted?
- Options:
  1. **Add the error scenario.**
     Meaning: a scenario requests a non-artifact, non-policy id and asserts
     the known-artifact error.
     In practice: `givn instructions not-an-artifact` fails with "unknown
     artifact 'not-an-artifact'. Known artifacts: …".
     Gain: the fallback cannot silently change the error contract.
     Cost: one scenario.
  2. **Rely on sibling commands.**
     Meaning: no scenario for `givn instructions`.
     In practice: a regression in the error message ships unnoticed.
     Gain: no new scenario.
     Cost: the code path the change introduces is untested.
- Resolution: **Option 1** because the fallback is the changed path and its
  error behavior is user-visible. Rejected: Option 2 because sibling
  scenarios never execute this command.

### Q5 — Generated design-review command pointer

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the command surface carries an asserted policy pointer.
- Context: the design edits both the generated design-review skill and its
  slash command. The scenario asserted the skill pointer but not the command;
  the existing generic command regex already matches `givn-design-review`.
- Question: should the command pointer be asserted?
- Options:
  1. **Add the assertion.**
     Meaning: one more Then in the pointer scenario.
     In practice: the generated `.opencode/commands/givn-design-review.md`
     must contain "givn instructions dialog".
     Gain: both generated surfaces are guarded with no new step code.
     Cost: one line.
  2. **Skill only.**
     Meaning: leave the command unasserted.
     In practice: a command edit could drop the pointer silently.
     Gain: no change.
     Cost: an asymmetric guard on two generated twins.
- Resolution: **Option 1** because the proposal lists the command as a
  surface and the assertion is free. Rejected: Option 2 because it weakens
  the "every listed surface" criterion.

### Q6 — Review and archive guidance pointers

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the pointer edits stay within the handover's file list.
- Context: the policy's scope line names review dispositions and archive
  confirmations, but the handover's decided target list contains no review or
  archive asset. Adding pointers there would grow the change beyond its
  decided scope.
- Question: should review and archive guidance gain pointers now?
- Options:
  1. **Keep the handover list; record the boundary.**
     Meaning: no pointer edit in review/archive guidance; design.md records
     that those surfaces are governed but unpointed.
     In practice: the review instruction stays as-is; the boundary is
     explicit so a later change can extend it.
     Gain: scope stays as decided; the gap is visible.
     Cost: a reader of the review instruction does not see the pointer yet.
  2. **Extend the pointers.**
     Meaning: add the reference to the review and archive guidance and
     assert them.
     In practice: two more asset edits and assertions.
     Gain: uniformity.
     Cost: scope expansion beyond the handover's decided list.
- Resolution: **Option 1** because the handover's target list is binding and
  the boundary is now recorded. Rejected: Option 2 because it expands a
  decided scope without operator need.

### Q7 — Phrase proving the cross-reference gloss rule

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the gloss assertion can fail in RED.
- Context: the record-shape scenario asserts concrete phrases for the option
  treatment and rejected alternatives, but its gloss step named no phrase,
  so it could not fail against a policy without the rule.
- Question: which phrase proves the gloss rule?
- Options:
  1. **"one-line gloss on first use".**
     Meaning: the policy must contain the phrase verbatim.
     In practice: the assertion fails until the policy carries the rule.
     Gain: a concrete RED condition.
     Cost: the policy must keep this exact wording.
  2. **Drop the gloss step.**
     Meaning: cover the gloss rule only through the retrievable policy
     content.
     In practice: the gloss rule has no dedicated assertion.
     Gain: one less wording constraint.
     Cost: the rule loses its executable guard.
- Resolution: **Option 1** because the handover's Rule 3 is part of the
  contract and a phrase-bound assertion is the established pattern here.
  Rejected: Option 2 because it drops a required rule from the checks.

### Q8 — Chapter 10 (quality requirements) impact

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the arc42 assessment is accurate.
- Context: the assessment marked chapter 10 affected because the delta
  scenarios verify behavior. Those are functional specs, not quality
  attribute scenarios, and the existing quality tree already covers one
  normative authoring policy.
- Question: is chapter 10 affected?
- Options:
  1. **Mark it No.**
     Meaning: no chapter 10 change; correct the assessment reason.
     In practice: chapter 10 stays untouched; arc42.md row 10 reads No.
     Gain: the assessment matches the actual document.
     Cost: none.
  2. **Add a quality scenario.**
     Meaning: invent a quality attribute row for the dialog policy.
     In practice: a new Q-number with no measured requirement.
     Gain: none.
     Cost: fabricated quality requirement.
- Resolution: **Option 1** because no quality attribute changes and the
  delta scenarios are functional. Rejected: Option 2 because it would
  invent a requirement the change does not establish.

### Q9 — Existing-ADR check for the policy-instruction lookup

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the non-qualifying routing names the closest existing
  decision.
- Context: the lookup boundary was routed to chapter 8.3b, but the
  existing-ADR check never named the instruction-JSON-contract decision
  (ADR-048), which already fixes the one-policy-source and JSON-shape
  contract the fallback applies to a new id.
- Question: record the ADR-048 check, or amend ADR-048?
- Options:
  1. **Record the check; do not amend.**
     Meaning: chapter 8.3b names ADR-048 as checked and unchanged.
     In practice: the fallback applies ADR-048's contract to a new id
     instead of refining the decision.
     Gain: the qualification verdict is complete without touching the ADR.
     Cost: one sentence.
  2. **Amend ADR-048.**
     Meaning: extend the record with the non-artifact lookup.
     In practice: a new amendment paragraph for a single-function change.
     Gain: the register mentions the fallback.
     Cost: an amendment for a reversal that is a small code change.
- Resolution: **Option 1** because the candidate fails architectural impact
  and durable consequence, and the closest decision is unchanged. Rejected:
  Option 2 because it inflates the ADR contract for a non-qualifying choice.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Tech choices | Version freshness error: the design claimed `cucumber-rs` 0.16.0; `Cargo.toml` and `Cargo.lock` pin 0.23.0. Corrected in design.md. |
| Tech choices | The fallback is minimal: `run_artifact` bails on unknown ids before resolving, and `resolve_instruction` already resolves `instructions/<id>.md` from the embedded tree, so the new asset plus the reordered lookup is sufficient. |
| Testability | The single-scenario command is valid: on the plain `cargo test` path the runner falls through to cucumber's CLI parsing, so `--name` filters by name and `GIVN_FEATURES` selects the change specs path. |
| Testability | `givn lint --change` exits 2 (WIP) as required and reports subset findings against permanent scenarios; review must record overlap dispositions. |
| E2E fidelity | The `grill-me` and `grill-with-docs` command assertions need a new `grill-<name>` regex step because the existing generic step only matches `givn-`-prefixed commands; design.md names the file. |
| E2E fidelity | `givn init` enables all addons by default, so the grill commands exist in the pointer scenario's fixture. |
| Scope | The Capability Routing table passes the review check: fragment deltas under `specs/fragments/`, `EXTEND prose-consolidation` and `EXTEND install-extras` under `specs/author-change/`. |
| Architecture documentation | Chapter 11 collision: the new risk reused R-64, which already exists; renumbered to R-67 and the assessment updated. |
| Architecture documentation | The claimed chapter updates in 03, 05, 06, 08, and 12 are real and reflect design.md; chapter 10 was the only mismatch. |
| Architecture documentation | The lookup fallback fails architectural impact and durable consequence, so `CANONICAL_ARTIFACT` is the correct route; chapter 8.3b is the single destination. |
| Missing scenarios | `assets/instructions/design-review.md` contains no dialog-discipline text, so no pointer edit is required there. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Fragment capabilities (`instruction-integrity`, `ideation-skill`) keep their fragment ownership; `prose-consolidation` and `install-extras` extend the author-change use case. | accepted |
| Typed Includes / Extends relationships | `author-change` includes the `support` fragment; no typed relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` (generated guidance and CLI output) and `givn-maintainer--owner` (rule corpus) are confirmed permanent Personas. | accepted |
| Actor and Persona distinction | Gherkin keeps Developer and Authoring agent as actors; Personas stay review lenses. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | The generated guidance and the policy instruction are its primary consumption surface; the pointer and retrieval scenarios assert them. | `Generated interactive guidance points to the canonical dialog policy`; `The canonical dialog policy is retrievable through the Givn CLI` |
| givn-maintainer--owner | Owns the rule corpus, the instruction cascade, and the decision to keep one canonical policy. | `The canonical dialog policy demonstrates the full question record`; chapter 8.3a/8.3b updates |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | No | No | Yes | — |
| 3 | 03 context-and-scope | Yes | Yes | Yes | Yes — the `givn instructions dialog` row is present |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — `cli::instructions` responsibility updated |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — 6.3 documents the fallback branch |
| 7 | 07 deployment-view | No | No | Yes | — |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.3a/8.3b record the policy and lookup boundary |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — no qualified ADR; destination 8.3b |
| 10 | 10 quality-requirements | No | No (corrected from Yes during hardening) | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-67 recorded (renumbered from the colliding R-64) |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Dialog policy` term present |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate exists in design.md; the non-qualifying
      lookup boundary names chapter 8.3b as its canonical destination and
      records the ADR-048 existing-ADR check.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/dialog-question-quality/design.md` | Reordered the CLI fallback before change resolution; added the gloss phrase; recorded the review/archive pointer boundary; added the new scenarios and step-definition rows; corrected the cucumber version. |
| `givn/changes/dialog-question-quality/specs/fragments/instruction-integrity.feature` | Added three scenarios: no-active-change retrieval, unknown-id error, ejected template shape. |
| `givn/changes/dialog-question-quality/specs/author-change/prose-consolidation.feature` | Added the generated design-review command pointer assertion. |
| `givn/changes/dialog-question-quality/specs/author-change/install-extras.feature` | New delta: the absolute-mode policy block carries the dialog context rule. |
| `givn/changes/dialog-question-quality/proposal.md` | Added the `install-extras` routing row. |
| `givn/changes/dialog-question-quality/arc42.md` | Corrected row 10 to No; renumbered the risk to R-67; named the ADR-048 check. |
| `docs/arc42/11-risks-and-technical-debt.md` | Renumbered the dialog drift risk to R-67 (R-64 was taken). |
| `docs/arc42/08-crosscutting-concepts.md` | 8.3b names the ADR-048 existing-ADR check for the lookup boundary. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change dialog-question-quality` exits 2 (WIP, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
