# Design Review: harden-event-storming

## Grilling

A fresh-context grilling subagent (Task tool, 2026-10-03) read the proposal,
design, delta spec, arc42 marker, the four target assets, the
domain-modeling skill, the permanent fragment spec, and the shared
content-assertion steps, and returned verdict **pass** with one should-fix
and four notes.

Required branches covered:

- **Scope** — the delta covers the proposal; the one requirement without an
  assertion (8, the two durable rows) is a recorded, precedent-backed
  review-verification choice.
- **Tech choices** — guidance text through the skill template plus one
  command guardrail line; no runtime code exists to change.
- **Missing scenarios** — no permanent scenario contradicts the delta, so no
  `@givn.modified` or `@givn.removed` entry is needed; findings 1 and 2 below
  close the two gaps.
- **Testability** — all 22 asserted phrases are contiguous on one line of the
  design's literal wording and absent from the current assets, so RED is real
  and every phrase discriminates the rule it pins.
- **E2E fidelity** — CLI text-output capability; the existing `@e2e` scenario
  drives a real `givn skills sync` subprocess; content assertions are
  variants, not new inventory entries.
- **Visual contract** — no screen is introduced or changed; the opposite
  classification does not hold because nothing renders HTML, CSS, or JS.
- **Interface classification** — CLI text output, tested against the
  opposite: no browser is involved at any point.
- **Persona disposition** — recorded below.
- **Deferrals** — none.
- **Interaction coverage** — the fragment carries no inventory
  (`fragment.md`: `Interactions: none`); the single matrix row names the
  capability's one real action and its real driver.
- **Risk** — the failure mode is RED never turning GREEN through a line-wrap
  or case mismatch, or an assertion that also matches pre-existing text;
  mechanically checked, both are excluded.
- **Domain context** — fragment `support` owns `generate-ideation-guidance`;
  no capability moves; the aggregate discipline the guidance points at
  exists in `givn-domain-modeling`.
- **ADR qualification** — independently judged: no boundary moves, reversal
  is a text edit, process guidance owns the rationale; the structured verdict
  needed the qualification value corrected (finding 3).
- **Architecture documentation** — the reviewer re-derived all 12 rows and
  agreed with `arc42.md`: only chapter 12 changes.
- **Ubiquitous language** — "aggregate" and "canvas" stay phase-element nouns
  defined in place, matching the command/policy/read-model convention; the
  addon-conditional glossary reference is finding 1.

## Findings and resolutions

1. Should-fix: the aggregate bullet named the arc42 glossary unconditionally
   in an always-on skill, whose only other arc42 reference is wrapped in
   `{{#if addons.arc42}}` with a generic fallback; a project without the
   addon would be pointed at a file that does not exist. Resolved: the
   glossary parenthetical is a Handlebars conditional mirroring the
   term-collision bullet — the arc42 branch renders the proposal's wording
   verbatim, the else-branch names the project's durable documentation, and
   the asserted reuse phrase sits outside the branch.
2. Note: the assertion `(the consistency boundary the command changes)` did
   not discriminate requirement 3 (every event line names its aggregate).
   Resolved: the assertion now reads `aggregate (the consistency boundary the
   command changes)`, contiguous on the bullet's own line.
3. Note: `arc42.md` recorded `qualification: NOT_APPLICABLE`; the contract
   defines `QUALIFIED` or `NOT_QUALIFIED`. Resolved: the value is now
   `NOT_QUALIFIED`, matching the block's failed mandatory dimensions.
4. Note: requirement 8 has no executable assertion; the two durable rows are
   review-verified. Accepted and recorded — this matches the
   `change-release-note` and `term-introduction-discipline` precedent where
   project documents are not read by any step.
5. Note: the proposal's incident is a user report; the named kiekste topic
   `interface-abilities` is not present on disk, and no artifact claims it as
   repository evidence. The countable claims were independently confirmed:
   eleven `eventstorming.md` records across the four repositories, ten
   without a diagram, and the single diagram (givn's `multi-scm-support`)
   charts events only. No artifact change: the incident stays attributed to
   the user report.

## Persona disposition

- `developer--primary-user` — relevance: reads the guidance and reviews the
  canvas; the rules are what makes that review possible. Evidence: the
  eleven observed records cited in the proposal.
- `agent--technical-role` — relevance: runs the phase and draws the canvas;
  the rules are addressed to it. Evidence: the guidance is the agent's
  operating instruction.
- `givn-maintainer--owner` — relevance: owns the guidance corpus and the
  durable records amended here. Evidence: the template, command, workflow
  document, and glossary are the corpus.

No Persona was invented or auto-promoted.
PERSONAS: developer--primary-user, agent--technical-role, givn-maintainer--owner

## Deferrals

None. No mandatory check is deferred.

## Hardening applied

- `design.md`: the aggregate bullet records the addon-conditional glossary
  reference and its rationale.
- `specs/fragments/generate-ideation-guidance.feature`: the per-event
  assertion now names the aggregate clause.
- `arc42.md`: qualification value corrected to `NOT_QUALIFIED`.
- `givn lint --change harden-event-storming` exits 0 (advisory only).

DESIGN-REVIEW: PASS
