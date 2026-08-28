# Design Review: fix-domain-modeling-skill-trigger

## Grilling

Branches walked against proposal, specs, design, and source:

- **Scope.** Spec matches the proposal: skill trigger rework + session router
  line at the specs/design phases. Deferred enforcement is explicitly out of
  scope in both documents. "Done" is unambiguous: ten scenarios green.
- **Tech choices.** Two text touchpoints plus a two-entry pure helper; no
  dependency, no manifest/graph change. No simpler alternative satisfies the
  trigger (must live in the skill frontmatter) and the route (must be emitted
  by `givn instructions`).
- **Missing scenarios.** Checked: addon-off contract (covered, specs variant),
  JSON contract (covered, next-step variant), design-phase route (covered,
  e2e), generated-skill content (covered, e2e after sync).
- **Testability / RED capability.** Verified each non-e2e content assertion
  fails against the current template (new trigger phrases and the removed
  lifecycle string are absent/present today in the failing direction). Verified
  the e2e assertions fail because `givn instructions` emits no
  "Skill to load now" line today. One caveat recorded: the addon-off scenario
  ("the guidance does not mention the domain-modeling skill") passes both
  before and after implementation — it is a regression guard for the negative
  contract, not a RED-failing scenario. Accepted.
- **Risk.** Highest risk is breaking the `givn instructions --json` contract by
  adding a key. Mitigation: the key is added conditionally (only when the
  addon is on and the phase applies) and the existing contract tests run with
  the addon off, so they stay green. Schema-wise the JSON object grows, never
  shrinks or renames.
- **ADR qualification.** Non-qualifying: the change refines existing guidance
  text and output wording; no ownership boundary, no alternatives tradeoff with
  architectural impact. Canonical destination: this change's specs/design and
  the two touched assets. No ADR.
- **arc42.** Addon enabled; walked all 12 rows — all "No" with per-row reasons
  (pure guidance-text change; no architecture impact). No chapter edits
  required; `arc42.md` records the assessment with `STATUS: DONE`. All 12
  chapter files exist and are non-placeholder (previously edited by prior
  changes); no ASCII-art diagrams introduced.

## Hardening

No decisions changed during grilling. No missing scenarios found. No edits to
proposal needed. No `tasks.md` written (next artifact).

## Verdict

All questions resolved. Proceed to tasks.

DESIGN-REVIEW: PASS
