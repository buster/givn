# Design Review: validate-generated-references

## Grilling

A fresh-context grilling subagent (read-only, 2026-10-11) read the proposal,
both delta feature files, `design.md`, `arc42.md`, the touched assets, the
runner and step files, the config cascade, and the canonical specs policy. It
built consumer-like projects under `/tmp` (all targets with all addons;
`--no-addons`; all seven artifact instructions) and re-implemented the
design's extraction and resolution algorithm against both corpora. It
returned four design questions, fifteen findings, an independent arc42
12-row read, and an independent ADR qualification verdict.

The operator delegated the full lifecycle ("Propose and implement until
archived", 2026-10-11). Every question resolved from repository evidence or
from the operator's earlier scope decision, so none required operator
arbitration. Resolutions:

- Q1 — which printed instructions are graded. The subagent showed the
  characterize fix creates a route to `givn instructions review-reverse`
  while the designed corpus covered manifest artifacts only, leaving the
  route's own target ungraded. Resolved: the printed corpus is every
  instruction Givn prints — the manifest artifacts plus `review-reverse`,
  `specs-reverse`, `tasks-reverse`, and `dialog`.
- Q2 — does the change own the bare `review-reverse.md` pointer in the
  characterize skill (`assets/skills/givn-characterize/SKILL.md.tmpl:63`).
  Resolved: yes; the skill is reworded like the command, and the scenario
  asserts both surfaces.
- Q3 — arc42 rows 10 and 11. The independent read marked chapter 10 Yes (the
  change establishes and measures a new guidance-integrity property, and
  chapter 10 already carries Q-91/Q-94 for guidance content) and chapter 11
  Yes (the check's graded surface is a heuristic with known limits).
  Resolved: rows corrected to Yes; Q-97, the 10.3 verification signal, and
  deliberate debt D-20 are recorded.
- Q4 — overlap dispositions for the new persona-schema scenario. `givn lint`
  reports it as a subset of five permanent scenarios and shape-sharing with
  several more. Resolved: keep the scenario as `@givn.added`; the review
  carries a disposition row per reported pair (the schema headings
  `Stable core`, `Success criterion`, `Running record`, and
  `Topic stances` are content no permanent scenario asserts).

Required branches covered:

- **Scope** — the four proposal outcomes map to the seven delta scenarios;
  the reference classes match the rendered corpus the subagent walked.
- **Tech choices** — test support in `tests/common/` plus text edits; no new
  dependency or version; the narrow table over a prefix allowlist and
  test-support over a `givn check` subcommand are recorded decisions.
- **Missing scenarios** — the printed-corpus negative and the installed
  negative were missing (findings 1 and 10); the asset-removal scenario was
  removed from the plan (finding 2).
- **Testability** — every added scenario can fail RED: the schema scenario
  asserts strings the current guidance lacks (capitalized schema headings),
  the check scenarios fail undefined before implementation and, after it,
  fail on the three dead pointers as the subagent's walk showed.
- **E2E fidelity** — CLI text-output capability; no new consumer action; the
  existing `@e2e` scenarios drive real `givn skills sync` and
  `givn instructions` subprocesses.
- **Visual contract** — no screen is introduced; the opposite classification
  (Web UI) does not hold because nothing renders HTML, CSS, or JavaScript;
  no new evidence is due because no delta `@e2e` scenario is added.
- **Interface classification** — terminal; tested against the opposite and
  recorded.
- **Persona disposition** — recorded below; all three Personas exist as
  project records.
- **Deferrals** — none.
- **Interaction coverage** — the matrix now uses the fragment inventory's two
  `instruction-integrity` rows and annotates the sync action its existing
  `@e2e` covers. The review instruction's step 11a wording (a
  `# User Interaction Inventory:` block in the `.feature` file) contradicts
  the canonical policy (the inventory lives in the use-case/fragment
  document); no feature in the repository carries such a block, the
  discrepancy predates this change, and it stays out of scope — recorded
  here for the review's cross-reference note.
- **Risk** — the most likely failure is a classifier false positive on a
  valid guidance form; trailing-slash normalization (finding 4) and the
  rule-owned ADR rows (finding 3) are the mitigation, and the limits are
  registered as D-20.
- **Domain context** — fragment `support` owns both capabilities; actors stay
  distinct from Personas; no Persona is invented or auto-promoted.
- **ADR qualification** — independently re-derived as `NOT_QUALIFIED` with
  routing `CANONICAL_ARTIFACT` to
  `docs/arc42/08-crosscutting-concepts.md` section 8.7; the three
  falsification-test outcomes are now recorded in `arc42.md`.
- **Architecture documentation** — the reviewer re-derived all 12 rows
  before opening `arc42.md`. Rows 8 and 12 matched; rows 10 and 11 were the
  mismatches (finding 3/Q3) and are corrected with real chapter content.
  Chapter 09 needs no MADR entry because no candidate qualifies.
- **Ubiquitous language** — the vocabulary is now consistent: "consumer
  project" and "generated guidance" are glossary rows, and "named creation
  step" is the single phrase for a creator in proposal, specs, design, and
  chapter 8.7.

## Findings and resolutions

1. Note (blocking for GREEN): the printed scenario used the installed-corpus
   Then step, so it never graded printed text and contradicted D2. Resolved:
   the scenario is "The printed instructions reference only resolvable
   files" with the When step "the author requests every instruction Givn
   prints" and the printed Then step.
2. Note: deleting the repository persona template (and the retired steps)
   would break the permanent scenario that reads it during every pre-archive
   full-suite run, because removal deltas merge only at archive. Resolved:
   the asset and its scenario stay; the guidance carries the schema inline;
   no `@givn.removed` entry exists.
3. Note: the arc42-docs instruction prints `docs/arc42/adr/README.md`,
   `docs/arc42/adr/ADR-NNNN-short-slug.md`, and the archive path, none of
   which exist after `givn init`. Resolved: the rule-owned table gains
   `docs/arc42/adr` / `docs/arc42/adr/**` owned by the arc42-docs ADR
   workflow.
4. Note: trailing-slash candidates (`givn/personas/`,
   `givn/changes/<change-id>/`, `givn/ideation/<topic-id>/`) are valid
   guidance but would fail a literal matcher; `givn/personas/` has no
   directory before first use. Resolved: the design normalizes the trailing
   slash and states that a root row matches the bare directory candidate;
   the no-addons scenario exercises the case.
5. Note: the characterize skill still named the distribution artifact
   `review-reverse.md` without a slash, which the extractor cannot see.
   Resolved by Q2.
6. Note: the printed corpus omitted non-artifact instructions. Resolved by
   Q1.
7. Note: the Interaction Coverage Matrix cited entries and a shape that did
   not match the fragment inventory. Resolved: the matrix carries the two
   fragment rows plus the annotated sync row; the step 11a discrepancy is
   recorded as out of scope (the review branch above).
8. Note: the proposal said three dead references while the design fixes a
   fourth site (the `docs/adr/` example). Resolved: the proposal's fix list
   now names four items and the counts are consistent.
9. Note: the negative scenarios' override bodies were under-specified.
   Resolved: the Give steps name the exact file paths and bodies (valid
   frontmatter for the skill override; a one-line backticked reference).
10. Note: no negative scenario proved fail-closed behaviour for the installed
    corpus. Resolved: "An undeclared reference in an installed skill fails
    the check" was added; the installed override produces one finding per
    configured target.
11. Note: the D1 rationale overstated that the spec skill carries the
    placement test. Resolved: the rationale now says the skill delegates to
    the mandated instruction, which carries the test.
12. Note: vocabulary drifted between "named creation step", "named workflow
    step", and "consumer view". Resolved: unified to "named creation step"
    and "consumer project" across proposal, specs, design, and chapter 8.7.
13. Note: the ADR record omitted the three falsification tests. Resolved:
    `arc42.md` now records each outcome; the verdict is unchanged.
14. Note: the design instruction's fenced example line
    (`docs/design/design-system.md`) is not backticked and therefore outside
    the graded surface. Resolved: recorded as deliberate debt D-20.
15. Note: the Visual Design Contract overstated existing evidence.
    Resolved: it now states that no new evidence is required because the
    change adds no delta `@e2e` scenario.

## Persona disposition

- `developer--primary-user` — relevance: owns the consumer project whose
  generated guidance pointed at files it did not have; the change makes that
  guidance self-contained. Evidence: the observed Fossil-project session in
  the proposal.
- `agent--technical-role` — relevance: follows the guidance's file
  references and is the reader the dead pointers sent hunting. Evidence: the
  reference check and the guidance texts it loads.
- `givn-maintainer--owner` — relevance: owns the guidance corpus, the
  resolution tables, and the acceptance suite that enforces the rule.
  Evidence: the reference check module, the delta scenarios, and chapter 8.7.

No Persona was invented or auto-promoted.
PERSONAS: developer--primary-user, agent--technical-role, givn-maintainer--owner

## Deferrals

None. No mandatory check is deferred.

## Hardening applied

- `proposal.md`: the four-item fix list; "named creation step" vocabulary;
  the no-marker resolution of the use-case-model pointers.
- `specs/fragments/instruction-integrity.feature`: printed corpus and both
  fail-closed scenarios; the characterization scenario asserts the skill and
  the command.
- `specs/fragments/generate-ideation-guidance.feature`: no removal entry; the
  persona schema stays an addition while the repository template and its
  permanent scenario stay.
- `design.md`: kept asset decision; printed-corpus enumeration; trailing-slash
  normalization; the `docs/arc42/adr` rule-owned rows; override bodies; the
  corrected rationale; the vocabulary alignment.
- `docs/arc42/08-crosscutting-concepts.md`: 8.7 and 8.22 wording aligned to
  the final texts.
- `docs/arc42/10-quality-requirements.md`: Q-97 and the 10.3 verification
  signal.
- `docs/arc42/11-risks-and-technical-debt.md`: D-20.
- `arc42.md`: rows 10 and 11 corrected to Yes; the three falsification-test
  outcomes recorded.
- `givn lint --change validate-generated-references` exits 2 (the expected
  `@wip` finding); the `[SHAPE]`/`[SUBST]` overlap advisories are recorded
  for review-stage disposition.

DESIGN-REVIEW: PASS
