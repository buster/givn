# 05 — adr-qualification-hardening

Status: ready for `/givn-propose`
Source: ADR overuse (T4 options A and C)
Depends on: 04 (use-case model — the product-behavior exclusion references it)

## Problem

The qualification gate was already hardened from OR to strict AND semantics
(shipped in 0.3.1). The remaining problem is that the gate language is
stretchable: "reversal would require migration, broad code or interface
changes" can be honestly claimed by any persisted entity, workflow gate, or
shipped feature. There is no falsification test. The ADR template ships
prefilled with the passing verdict, so the same agent drafts the verdict and
the ADR. Every change must audition an ADR candidate through a mandatory
marker, and the chapter-selection guidance normalizes "a single change
routinely triggers row 9". Since the lightweight decision log was rejected,
the only durable decision home is the ADR — the falsification tests and the
cheaper-home challenge must therefore name an existing canonical home.

## Evidence

kiekste side (23 ADRs, ADR-0001 through ADR-0023, 2026-08-30 to 2026-09-12):

- Genuinely architectural (keep as ADRs): ADR-0001 (agent protocol), ADR-0002
  (repository as only durable store, superseded), ADR-0003 (engine library
  plus thin presentation layers), ADR-0006 (authority split), ADR-0007
  (embedded control store), ADR-0010 (runtime lifecycle boundary), ADR-0011
  (host-controlled execution profiles), ADR-0018 (repository backend), 
  ADR-0019 (verification authority), ADR-0020 (isolated environment bounds
  host damage), ADR-0022 (serve security minimum).
- Misplaced: ADR-0009 (project ID/Name split — a key decision), ADR-0014
  (stop-and-report terminal state value), ADR-0013 (assumption-and-go-ahead
  gate), ADR-0016 (evaluation human gate), ADR-0008 (change request entity).
  Each passes the current gate only by stretching "authority boundary" or
  "reversal requires migration".
- Clustering: `orchestrator-core` minted three ADRs, `publication-cas` four,
  in single changes.

givn side:

- `assets/addons/arc42-docs/instructions.md` lines 100–117: the five
  mandatory dimensions; line 106–108 defines durable consequence as
  "reversal would require migration, broad code or interface changes, …".
- Same file lines 324–328: "a single change routinely triggers rows 3, 5, 6,
  9, and 11 together".
- `assets/addons/arc42-docs/impact-assessment-template.md` lines 44–48: every
  change emits an `ADR action:` marker.
- `assets/addons/arc42-docs/chapters/adr/adr-template.md` lines 35–49:
  prefilled `ADR-QUALIFICATION: PASS`, `qualification: QUALIFIED`, all five
  gates `PASS`, `routing: NEW_ADR`.
- `assets/templates/design.md` lines 41–44: "Justify decisions made here (why
  this approach over alternatives?)" — the shape of an ADR Considered
  Options section.
- `givn/specs/fragments/adr-qualification-gate.feature`: the executable
  contract for the gate.

## Decided scope

1. Three fail-fast falsification tests added to the gate, with kiekste
   misplacements as explicit non-examples. (T4 A)
2. Verdict recorded before any MADR file may be created; mandatory
   name-the-cheaper-home challenge at design-review; blank ADR template.
   (T4 C)

## Required changes

### 1. Falsification tests

Add to `assets/addons/arc42-docs/instructions.md` (Qualification gate
section), immediately after the five dimensions. A candidate fails the gate
when any of these holds:

- **Single-change reversibility.** The decision is fully implementable and
  reversible within one change and touches one component. File count or
  interface breadth does not change this. Example non-example: a project
  identity key split (kiekste ADR-0009) is a data-model decision owned by
  `design.md` and the glossary.
- **No multi-change constraint.** The decision does not constrain at least
  two future changes or two components. A boundary nobody else builds
  against is a local decision. Example non-example: a terminal state value
  plus its presentation filtering (kiekste ADR-0014) is feature behavior
  owned by the capability spec and design.
- **Product-behavior exclusion.** The candidate is observable workflow
  behavior ownable by a use case plus Gherkin (who authorizes what, in which
  order, with which state values). It fails `lower_level_artifact` unless it
  also moves a component, authority, or deployment boundary, and that
  boundary is stated in the verdict. Example non-examples: an
  assumption-and-go-ahead gate (ADR-0013) and a human evaluation gate
  (ADR-0016) are workflow behavior; a change-request entity with
  crash-safety requirements (ADR-0008) is a domain entity.

The tests are about kind, not count: a foundational change may legitimately
mint several genuine ADRs. Do not add a numeric cap.

### 2. Verdict before drafting

- The structured verdict (the five gates, `must_be_shared`, `qualification`,
  `routing`, and route targets) must be recorded in the change's `arc42.md`
  impact assessment **before** any MADR file is created. Update
  `assets/addons/arc42-docs/impact-assessment-template.md` so the ADR action
  block records the verdict and its placement order.
- `assets/instructions/design-review.md` and
  `assets/commands/givn-design-review.md`: the grilling must
  confirm-or-downgrade every proposed `NEW_ADR` with an explicit
  "name the cheaper home" challenge — the specific canonical artifact
  (feature spec, `design.md`, arc42 chapter, process doc, project
  documentation, or code) that would own the rationale instead. A missing
  challenge for a proposed `NEW_ADR` is a finding.
- `assets/skills/givn-design-review/SKILL.md.tmpl`: same requirement, short
  pointer, no duplicated schema.

### 3. Blank ADR template

Remove the prefilled passing verdict from
`assets/addons/arc42-docs/chapters/adr/adr-template.md`: no
`ADR-QUALIFICATION: PASS`, no `qualification: QUALIFIED`, no pre-passed gate
values, no default `routing: NEW_ADR`. Leave the fields blank with a one-line
instruction that the author fills them from the recorded verdict. Do not
prefill `NOT_QUALIFIED` either; blanks force authorship.

Note for the design: the template is a scaffold copied into projects. New
projects receive the blank version; existing project copies keep their old
prefill until re-copied. State this in the change design; do not attempt a
migration of project-local templates.

### 4. Design template prompt

`assets/templates/design.md` Technology Decisions: state that justification
belongs in `design.md` and must not be promoted to ADR candidacy unless the
falsification tests pass.

### 5. Per-change pressure language

Reword `assets/addons/arc42-docs/instructions.md` lines 324–328 so row 9 is
not normalized as a routine companion of rows 3, 5, 6, and 11. The sentence
must say: correlated rows prompt a double-check, and an affected chapter does
not by itself qualify an ADR.

## Files to change

| Path | Change |
|---|---|
| `assets/addons/arc42-docs/instructions.md` | Falsification tests; row-9 wording |
| `assets/addons/arc42-docs/impact-assessment-template.md` | Verdict recorded before drafting |
| `assets/addons/arc42-docs/chapters/adr/adr-template.md` | Blank verdict fields |
| `assets/templates/design.md` | Technology Decisions prompt |
| `assets/instructions/design-review.md` | Cheaper-home challenge requirement |
| `assets/commands/givn-design-review.md` | Same, in the ADR branch |
| `assets/skills/givn-design-review/SKILL.md.tmpl` | Pointer |

## Executable-spec impact

- `givn/specs/fragments/adr-qualification-gate.feature`: add scenarios for
  each falsification test (single-change reversibility, multi-change
  constraint, product-behavior exclusion), for verdict-before-drafting, for
  the cheaper-home challenge, and for the blank template contract (no
  prefilled PASS). Keep the existing scenarios green — the AND semantics and
  routing values do not change.
- `givn/specs/fragments/instruction-integrity.feature`: update if it asserts
  design-review guidance phrases.
- Check `givn/specs/fragments/arc42-review-alignment.feature` for
  assumptions about the impact-assessment marker shape.

## Acceptance criteria

- [ ] A state-machine value (ADR-0014 shape) fails the product-behavior
      exclusion or single-change reversibility test.
- [ ] An entity key split (ADR-0009 shape) fails single-change
      reversibility.
- [ ] A workflow approval gate (ADR-0013/0016 shape) fails the
      product-behavior exclusion.
- [ ] A genuine boundary decision (ADR-0006/0010/0018 shapes) still passes.
- [ ] No MADR file may be created before the structured verdict is recorded
      in `arc42.md`; design-review rejects a missing cheaper-home challenge
      as a finding.
- [ ] The ADR template contains no pre-passed qualification values.
- [ ] The row-9 pressure wording no longer normalizes ADR-per-change.

## Out of scope

- T4 option B (lightweight decision log). Because it was rejected, the
  cheaper-home challenge must name an existing canonical home; do not
  introduce a new decision-record artifact in this change.
- T4 option D (one-ADR-per-change budget).
- Migrating existing misplaced ADRs in kiekste or elsewhere. Leave them in
  place; a historical-scope note is a project decision, not part of this
  change.
- Any runtime enforcement. The qualification gate remains authoring
  guidance.

## Risks and notes

- The three tests must not read as a count cap. A greenfield foundational
  change may legitimately carry several ADRs; the tests classify by kind.
- Removing the template prefill may briefly confuse agents that relied on
  copying it. The one-line instruction and the recorded-verdict requirement
  cover this.
- Existing projects keep their project-local ADR template copy; the addon
  instruction still owns the rule, so behavior converges at the next
  template refresh. Document that in the design.
