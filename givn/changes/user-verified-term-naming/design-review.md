# Design Review: user-verified-term-naming

## Grilling

A fresh-context grilling subagent (read-only, 2026-10-11) read the proposal,
`design.md`, the `arc42.md` marker, both delta feature files, the four target
assets, the `givn-domain-modeling` skill, the permanent fragment specs, the
shared content-assertion steps, and the runner, and returned verdict **pass**
with one blocking finding and five corrections. Every question was answerable
from repository evidence, so none was put to the operator; under the standing
"go ahead until archived" authorization each resolution is recorded below.

Required branches covered:

- **Scope** — requirements 1-8 map to the delta scenarios; requirement 6
  (the dialog outside ideation) is carried by the domain-modeling skill body
  plus the permanent trigger scenarios. The one uncovered sentence
  (requirement 2's revisit outcome) is closed by finding 5.
- **Tech choices** — text-only edits to three existing assets; no new
  dependency, version, or Rust path. The D2 alternatives (all-in-ideation,
  all-in-skill, split) were genuinely weighed; the split is realized.
- **Missing scenarios** — no permanent scenario contradicts the delta, so no
  `@givn.modified` or `@givn.removed` entry is needed; findings 4 and 5 close
  the two gaps (the addon-off pin for the term check; the revisit outcome).
- **Testability** — live RED run: 7 tests, 7 failed, 0 skipped. Every Then
  asserts a concrete one-line substring, each checked against the design's
  wording blocks; `Propose it and justify it` is absent from the replacement.
  The 8 new steps are absent from `tests/` today, so the `@wip` scenarios
  fail via `fail_on_skipped` before implementation.
- **E2E fidelity** — CLI text-output capability; the existing `@e2e`
  scenarios drive the real `givn skills sync` subprocess. The change adds no
  consumer action, so no new `@e2e` is due; the matrix rows carry the real
  driver.
- **Visual contract** — no screen is introduced or changed; the opposite
  classification does not hold because nothing renders HTML, CSS, or
  JavaScript. The contract's missing fields are finding 2.
- **Interface classification** — CLI text output, tested against the
  opposite: no browser is involved at any point; the explicit why-not
  sentence was missing (finding 2).
- **Persona disposition** — recorded below; all three Personas exist as
  project records.
- **Deferrals** — none.
- **Interaction coverage** — the fragment carries no inventory rows for
  these capabilities; the matrix cited non-existent entries (finding 3).
- **Risk** — the most likely failure is durable records disagreeing
  (`08-crosscutting-concepts` §8.20 against the glossary) and a review-time
  visual-gate ambiguity; findings 1 and 2 are the mitigation.
- **Domain context** — fragment `support` owns both capabilities; nothing
  moves; the naming-dialog shape lands in `givn-domain-modeling`, the
  binding in the ideation rules and command.
- **Decision ledger** — D1 and D2 resolve in `design.md`'s `## Decisions`;
  no seed exists, so no seed reference is due.
- **ADR qualification** — independently re-derived: `NOT_QUALIFIED` with
  routing `CANONICAL_ARTIFACT` to
  `assets/skills/givn-domain-modeling/SKILL.md.tmpl`; the existing-ADR
  evidence sentence was corrected (finding 6).
- **Architecture documentation** — the reviewer re-derived all 12 rows
  before opening `arc42.md`: rows 8 and 12 Yes, the rest No. The row-8
  mismatch was the blocking finding (1). Chapter file integrity: all 12
  files exist, carry decision-specific content, and no pseudo-diagram
  characters were found. No candidate qualifies as an ADR, so no MADR entry
  is required.
- **Ubiquitous language** — the change carries its own vocabulary: the
  user's verified name, the phase term check, the naming question, and the
  anti-term, all defined in place; the durable records now agree (finding 1).

## Findings and resolutions

1. Blocking: `arc42.md` marked row 8 "No", but section 8.20 Term
   Introduction Discipline owns the term-record convention and this change
   amends the `confirmed` definition, so the independent derivation was
   "Yes"; the durable chapter would otherwise contradict the amended
   glossary. Resolved: row 8 is now Yes; the §8.20 first paragraph adds the
   user's verified name; the design's file table and Arc42 impact section
   cover the amendment. (R-68 in chapter 11 stays untouched; row 11 stays
   No — no new risk class.)
2. Note: the terminal Visual Design Contract was declared without
   viewports, a capture/evidence note, or the opposite-classification
   sentence, which would force review-time improvisation. Resolved: the
   contract now records `Viewports: not applicable` (no Web UI; no
   HTML/CSS/JavaScript; Markdown guidance files written, stdout contract
   unchanged), the evidence path (no new `@e2e`; the unchanged
   synchronize-skills transcripts), and the explicit why-not.
3. Note: the Interaction Coverage Matrix cited inventory entries that do
   not exist — the fragment's `## Interactions` table has no rows for these
   capabilities and neither delta carries an inventory block. Resolved:
   both rows are labeled "Existing coverage (no new action)" and cite the
   existing `@e2e` scenarios; the fragment is not edited in this change.
4. Note: the unconditional term-check decision was not pinned by any
   scenario — an implementation that wrapped the new section in the addon
   conditional would leave all delta scenarios green while hiding the term
   check from addon-free projects. Resolved: the addon-free scenario now
   also asserts `Term check — <phase>`.
5. Note: requirement 2's revisit outcome ("the other terms stand and
   nothing else re-opens") was carried by neither wording nor scenario.
   Resolved: the wording block carries the clause and the phase-close
   scenario asserts "the other terms stand".
6. Note: the ADR verdict's existing-ADR evidence overstated chapter 09 and
   ADR-013. Resolved: the sentence now says ADR-013 owns the
   generated-guidance destination boundary and no existing record decides
   the naming-dialog content rule.

## Persona disposition

- `developer--primary-user` — relevance: answers the naming question and
  the phase term check; the change hands the name choice to the user.
  Evidence: the proposal's Meeting→Handshake incident and the term-record
  gate.
- `agent--technical-role` — relevance: asks the naming question, runs the
  phase term check, and records the outcome. Evidence: the guidance texts
  it loads.
- `givn-maintainer--owner` — relevance: owns the guidance corpus and the
  durable records amended here. Evidence: the two skill templates, the
  command guardrails, `docs/workflow.md`, and the arc42 chapters.

No Persona was invented or auto-promoted.
PERSONAS: developer--primary-user, agent--technical-role, givn-maintainer--owner

## Deferrals

None. No mandatory check is deferred.

## Hardening applied

- `arc42.md`: row 8 corrected to Yes; the existing-ADR sentence tightened.
- `docs/arc42/08-crosscutting-concepts.md` §8.20: the `confirmed`
  definition now includes the user's verified name.
- `design.md`: Arc42 impact updated; the Visual Design Contract completed;
  the Interaction Coverage Matrix reworded to the existing coverage.
- `specs/fragments/generate-ideation-guidance.feature`: the addon-free
  scenario asserts `Term check — <phase>`; the phase-close scenario asserts
  "the other terms stand".
- `givn lint --change user-verified-term-naming` exits 2 (the two expected
  `@wip` findings); the `[SHAPE]`/`[SUBST]` overlap advisories are recorded
  for review-stage disposition.
- Scoped RED run re-confirmed after the edits: 7 tests, 7 failed, 0 skipped.

DESIGN-REVIEW: PASS
