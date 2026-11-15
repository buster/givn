---
status: accepted
date: 2026-11-15
decision-makers: buster (project owner)
consulted: none
informed: none

---

# Serve stack mechanics on demand from per-stack instructions under a recorded corpus budget

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

Evidence: the `ise-rebuildability` plan's `S5` and gate `G2` require the core
step instructions to stop growing and the corpus to stay at or below 4,500
lines; the `obtain-effective-guidance` capability states that a core instruction
names the stack instruction it leaves to a separate request. Neither artifact
chooses which stacks receive an instruction, whether the ceiling counts the
stack files, or which instruction owns the runner mechanics. This change's
design resolves those as `D1` to `D3`. ADR-005 owns the facet cascade, ADR-013
owns skills-only generated guidance, and ADR-048 owns the effective-policy and
instruction JSON contract; none owns guidance composition or a size ceiling, so
the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later re-merges six files into two core
      instructions, republishes every generated skill that names a stack id, and
      drops a ceiling the acceptance suite enforces.
- [x] **Shapes quality attributes** - It bounds the guidance an author loads and
      the corpus a release ships, so maintainability and agent ergonomics are
      measured rather than assumed.
- [x] **Structural** - It fixes which instruction owns which rule and the
      project-visible id contract `<id>-<stack>` resolved through the cascade
      and overridable per id.
- [x] **Must be shared** - Every future guidance change and every project
      override is keyed by the same ids; an author who invents a parallel rule in
      a core instruction breaks the composition, and silently grows the corpus.

## Context and Problem Statement

The four core step instructions carried per-runner mechanics inline: the step
definition path, binding form, and empty-body stub for six runners in `steps`,
and the strict-mode mechanism and single-scenario command in `design`. Every
author read every stack's rows. The corpus also crossed its ceiling: 4,528 lines
against 4,500, because the `ise` track's three predecessor changes added 131
lines to a 4,397-line baseline, so any further guidance work would deepen the
overrun that the `v0.9.15` release showed can cost real test volume.

## Decision Drivers

- one owner per rule, so a moved row has exactly one home
- an author loads only the mechanics of the stack in hand
- a measured ceiling, not a remembered principle
- no project-visible break: the same cascade, the same refusal, overridable ids

## Considered Options

- Six stack instructions named by `steps` and `design`, with a corpus ceiling
  counted with the stack files and a four-core ceiling at the post-split total
- Rust only, leaving five stacks' bodies in the core instructions
- The outline's five languages, deleting the Go and Ruby rows
- Two files per stack (`steps-rust`, `design-rust`, …), one per core owner
- The ceiling counted without the stack files, or a recorded measurement only

## Decision Outcome

Chosen option: "Six stack instructions named by `steps` and `design`, with a
corpus ceiling counted with the stack files and a four-core ceiling at the
post-split total", because it makes the split's own rule true for every stack
the corpus names while removing no rule, and because a ceiling that excludes the
stack files can be satisfied by moving text rather than reducing what an author
loads.

### Consequences

- Good, because an author on one stack loads that stack's four or five facts
  instead of reading six runners' rows, and the core text shrinks.
- Good, because the ceiling turns guidance growth into a failing check with a
  measured value, and the four-core ceiling can never be raised above the
  pre-split total.
- Bad, because a future runner now costs a file header inside the same ceiling,
  and meeting the ceiling required deleting duplicated guidance in the same
  change rather than only moving rows.
- Bad, because `design` names ids owned by `steps`, so the id convention carries
  an owner that the pair alone does not express.

### Confirmation

The acceptance suite asserts both ceilings over `assets/instructions/` and
asserts that the four-core ceiling is never above the pre-split total; the
`serve-extended-rules` scenarios assert that `steps` and `design` name every
stack id with its condition and carry no stack body, and that the stack body is
served only on its own request. Review checks that each retained check in the
consolidated `review.md` still exists as an obligation.

## Pros and Cons of the Options

### Six stack instructions, ceiling counted with them

- Good, because no rule is removed and no stack keeps a body in a core
  instruction.
- Good, because the ceiling cannot be gamed by extraction.
- Bad, because six small files need a scope sentence each inside the ceiling.

### Rust only

- Good, because it is the smallest cut and the clearest proof of the mechanism.
- Bad, because five stacks keep their bodies in the core instructions, so the
  split's rule stays false for them and their authors keep reading every row.

### The outline's five languages

- Good, because the set matches the outline's language list.
- Bad, because the Go and Ruby stub rows lose their home and their mechanics are
  deleted, and one `cucumber-js` runner would split across two instructions.

### Two files per stack

- Good, because `<id>-<stack>` pairs literally with the owning core instruction.
- Bad, because twelve files cost about 60 lines of headers that the ceiling
  cannot absorb.

### Ceiling without the stack files, or a recorded measurement only

- Good, because the extraction alone satisfies the budget and nothing must be
  consolidated.
- Bad, because total served text can grow without limit by extraction, which is
  the failure the ceiling exists to catch.

## More Information

Owner evidence: `obtain-effective-guidance` (`serve-extended-rules`),
`instruction-integrity`, `docs/arc42/08-crosscutting-concepts.md` §8.30,
`Q-126`, `R-148`, and `D-29`. Related records: ADR-005 (facet cascade), ADR-013
(skills-only guidance), ADR-048 (effective policy and instruction JSON
contract). The plan reference is `givn/ideation/ise-rebuildability/plan.md`,
phase `S5` and gate `G2`.
