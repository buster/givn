# Design: adr-qualification-hardening

## Technical Approach

Guidance and template text only. No runtime code, gate, or dependency change.
The AND semantics, routing values, and verdict schema stay as they are; the
change adds falsification tests, moves the verdict ahead of drafting, and
blanks the template prefill.

## Falsification Tests

`assets/addons/arc42-docs/instructions.md` gains a "Falsification tests"
block immediately after the five mandatory dimensions. A candidate fails the
gate when any holds:

- **Single-change reversibility.** Fully implementable and reversible within
  one change, touching one component; file count and interface breadth do not
  change this. Non-example: a project identity key split is a data-model
  decision owned by `design.md` and the glossary.
- **No multi-change constraint.** Does not constrain at least two future
  changes or two components; a boundary nobody builds against is local.
  Non-example: a terminal state value plus its presentation filtering is
  feature behavior owned by the capability spec and design.
- **Product-behavior exclusion.** Observable workflow behavior ownable by a
  use case plus Gherkin (who authorizes what, in which order, with which state
  values) fails `lower_level_artifact` unless it also moves a component,
  authority, or deployment boundary, and that boundary is stated in the
  verdict. Non-examples: an assumption-and-go-ahead gate and a human
  evaluation gate are workflow behavior; a change-request entity with
  crash-safety requirements is a domain entity.

The block states the tests classify by kind, not count, and that no numeric
cap exists: a foundational change may legitimately mint several genuine ADRs.

## Row-9 Pressure Language

The chapter-selection paragraph in the same file is reworded: correlated rows
prompt a double-check rather than normalizing a routine set; an affected
chapter does not by itself qualify an ADR. The existing asserted phrase "does
not by itself qualify an ADR" survives verbatim.

## Verdict Before Drafting

`assets/addons/arc42-docs/impact-assessment-template.md`'s ADR action block
gains blank verdict field placeholders (qualification, the five gate results,
supporting-only shared evidence, routing, and the three route targets) with a
placement rule: record the structured verdict here, in the change assessment,
before creating or editing any MADR file. The block carries no allowed-value
lists, no AND formula, and no evidence-array schema — the canonical
instruction owns those, and the derived source stays thin. The block keeps its
existing asserted phrases: `ADR action: NONE`, `ADR target: none`, "A change
may affect chapter 09 without qualifying", and the two deference sentences.

The canonical instruction's own description of the ADR template is updated in
the same change: the sentence that says the template embeds the passing gate
marker becomes a description of the blank qualification section filled from
the recorded verdict.

## Blank ADR Template

`assets/addons/arc42-docs/chapters/adr/adr-template.md` loses every pre-passed
value: no `ADR-QUALIFICATION: PASS`, no `qualification: QUALIFIED`, no
pre-passed gate values, and no default `routing: NEW_ADR`. The fields become
angle-bracket placeholders with a one-line instruction: fill every value from
the structured verdict recorded in the change's `arc42.md` before creating the
file. `NOT_QUALIFIED` is not prefilled either; blanks force authorship.

Template-scaffold note: the template is copied into projects. New projects
receive the blank version; existing project copies keep their old prefill
until re-copied. This change does not migrate project-local templates; the
addon instruction owns the rule, so behavior converges at the next refresh.

## Cheaper-Home Challenge

`assets/instructions/design-review.md`'s ADR branch, the matching branch in
`assets/commands/givn-design-review.md`, and a short pointer in
`assets/skills/givn-design-review/SKILL.md.tmpl` require that every proposed
`NEW_ADR` survives an explicit cheaper-home challenge: name the specific
canonical artifact (feature specification, `design.md`, Arc42 chapter, process
documentation, project documentation, or code) that would own the rationale
instead. A missing challenge for a proposed `NEW_ADR` is a finding. The
derived sources stay thin: they point at the canonical Arc42 procedure and do
not copy the verdict schema. The delta scenario asserts the phrase in each of
the three files individually, and the change runs `givn skills sync` before
review so the checked-in generated copies carry the pointer.

## Design Template Prompt

`assets/templates/design.md` Technology Decisions states that justification
belongs in `design.md` and is not promoted to ADR candidacy unless the
falsification tests pass.

## Spec Coverage

| Delta scenario | Asserted phrases (implementation detail) |
|---|---|
| The guidance carries the three falsification tests | "Single-change reversibility", "No multi-change constraint", "Product-behavior exclusion", "project identity key split", "terminal state value", "assumption-and-go-ahead gate", "human evaluation gate", "change-request entity", "still qualifies", "classify by kind, not count", "no numeric ADR cap" |
| The design template keeps decision justification in the design | "belongs in `design.md`", "falsification tests pass" |
| The impact assessment records the verdict before drafting | "before creating or editing any MADR file" |
| The ADR template ships with no pre-passed qualification values | the template does NOT contain "ADR-QUALIFICATION: PASS" or "qualification: QUALIFIED", and contains "Fill every value from the structured verdict" |
| Design-review requires the cheaper-home challenge for a new ADR | "cheaper home" and "NEW_ADR" in the instruction, "cheaper home" in the command, "cheaper home" in the skill template |
| The chapter-selection guidance does not normalize row 9 | "Treat correlated rows as a prompt to double-check", "does not by itself qualify an ADR", and the absence of "a single change routinely triggers rows 3, 5, 6, 9, and 11" |

Every new Then-step asserts at least one phrase that exists only after this
change; the tasks record the failing RED run against the unedited asset for
each scenario, and the Spec Coverage table is the single source of the
asserted phrases.

## Use-Case Traceability

| Contract element | Source | Technical decision | Evidence |
|---|---|---|---|
| One qualified decision per ADR; non-qualifying choices route to one canonical home | `givn/specs/fragments/adr-qualification-gate.feature` | Falsification tests and the cheaper-home challenge | The falsification and cheaper-home scenarios |
| The verdict is a strict Boolean contract | same | Verdict recorded before drafting; blank template | The verdict-placement and blank-template scenarios |
| Arc42 impact does not by itself require an ADR | same | Row-9 wording change | The chapter-selection scenario |

Persona impact: `agent--technical-role` consumes the Arc42 and design-review
guidance; `givn-maintainer--owner` owns the rule corpus and the ADR register.
No Persona is invented or promoted; Gherkin actors stay the author,
maintainer, and reviewer.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| adr-qualification-gate | `tests/steps/adr_qualification_gate_steps.rs` | Direct asset reads, matching the existing steps in that file; adds one Given for the ADR template and the new When/Then steps |

## Runner and Strict Mode

- Runner: Rust `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()`; proven by the existing suite.
- Not-implemented stub: `unimplemented!()`.
- Single-scenario command:

```text
GIVN_FEATURES=givn/changes/adr-qualification-hardening/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

## E2E Infrastructure and Local Runnability

- Real interface: direct asset reads for the guidance and template scenarios;
  the existing `@e2e` CLI scenarios stay unchanged.
- Local run command: `./run-tests.sh`; e2e scope: `./run-tests.sh --e2e`.
- Digital twins: none required; no external dependency.
- Anticipated interface obstacle: none.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| adr-qualification-gate: request the Arc42 ADR guidance (existing) | An author requests the strict ADR guidance through the Givn CLI (existing) | CLI | Real `givn instructions arc42-docs` subprocess |
| adr-qualification-gate: request the design-review guidance (existing) | A reviewer requests design-review guidance through the Givn CLI (existing) | CLI | Real `givn instructions design-review` subprocess |

No new `@e2e` scenario is added: the actions are already proven end-to-end,
and the new scenarios are guidance-content variants. The fragment declares no
interactions of its own.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | the Arc42 instruction renderer path |

## Version Freshness

No version-bearing technology is introduced. The Rust toolchain, `cucumber-rs`
0.23.0 (already pinned in `Cargo.lock`), and the existing scripts are
unchanged.

## Architecture Impact

Embedded addon guidance, one project template, and the design-review
guidance change. The qualification gate, routing values, and verdict schema
are unchanged; no parser, gate, or runtime behavior changes. Existing project
copies of the ADR template keep their prefill until re-copied, as recorded
above.

## Risks

- The three tests must not read as a count cap. The block states kind-not-count
  explicitly; no numeric cap is introduced.
- Removing the template prefill may briefly confuse agents that copied it. The
  one-line instruction and the recorded-verdict requirement cover this; the
  change design records the no-migration decision.
