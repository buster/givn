---
status: accepted
date: 2026-11-15
decision-makers: buster (project owner)
consulted: none
informed: none

---

# Serve stack mechanics on demand from per-stack instructions

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

Evidence: the `ise-rebuildability` plan's `S5` requires the core step
instructions to stop carrying per-runner mechanics, and the
`obtain-effective-guidance` capability states that a core instruction names the
stack instruction it leaves to a separate request. Neither artifact chooses
which stacks receive an instruction, which id set carries them, or which
instruction owns the runner mechanics. This change's design resolves those as
`D1` and `D3`. ADR-005 owns the facet cascade, ADR-013 owns skills-only
generated guidance, and ADR-048 owns the effective-policy and instruction JSON
contract; none owns guidance composition, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later re-merges six files into two core
      instructions and republishes every generated skill that names a stack id.
- [x] **Shapes quality attributes** - It bounds the guidance an author loads, so
      agent ergonomics and maintainability are properties of the composition.
- [x] **Structural** - It fixes which instruction owns which rule and the
      project-visible id contract `<id>-<stack>` resolved through the cascade
      and overridable per id.
- [x] **Must be shared** - Every future guidance change and every project
      override is keyed by the same ids; an author who invents a parallel rule in
      a core instruction breaks the composition.

## Context and Problem Statement

The four core step instructions carried per-runner mechanics inline: the step
definition path, binding form, and empty-body stub for six runners in `steps`,
and the strict-mode mechanism and single-scenario command in `design`. Every
author read every stack's rows, and a row that moved had no single owner. No
check detected a core instruction that named an instruction id nobody served.

## Decision Drivers

- one owner per rule, so a moved row has exactly one home
- an author loads only the mechanics of the stack in hand
- no project-visible break: the same cascade, the same refusal, overridable ids
- instruction length is not capped; a rule that states valid behavior is never
  deleted to satisfy a line count

## Considered Options

- Six stack instructions named by `steps` and `design`, one per runner the core
  instructions name
- Rust only, leaving five stacks' bodies in the core instructions
- The outline's five languages, deleting the Go and Ruby rows
- Two files per stack (`steps-rust`, `design-rust`, …), one per core owner
- Runner mechanics staying in `design`, with only the step mechanics split

## Decision Outcome

Chosen option: "Six stack instructions named by `steps` and `design`", because
it makes the split's own rule true for every stack the corpus names while
removing no rule and keeping the id convention `<id>-<stack>`.

The contract:

- one instruction per runner: `steps-rust` (`cucumber-rs`), `steps-python`
  (`behave`), `steps-node` (`cucumber-js`), `steps-java` (`cucumber-jvm`),
  `steps-go` (`godog`), and `steps-ruby` (`cucumber-ruby`);
- `steps` owns the id mapping, names every id with its load condition, and
  carries the universal rules; `design` names the same ids with the same
  condition;
- a stack instruction is a policy instruction id: it resolves through the same
  cascade with no change context, a project may override it per id, and its
  body reaches the caller only on a request for its own id;
- `specs` and `review` carry no stack body and name no stack instruction;
  `tasks` and `tasks-reverse` point at `steps` for the mapping.

### Corpus line budget withdrawal

The first accepted version of this record also decided a corpus line budget:
a 4,500-line ceiling over `assets/instructions/` and a four-core ceiling at the
measured post-split total, both asserted by the acceptance suite. The operator
withdrew that budget on 2026-11-15. Instruction length carries no cap; prose
that states no rule is the defect, not a rule-bearing sentence. Meeting the
ceiling had required deleting Step 0 obligations, the E2E coverage section, the
sign-off E2E block, and the spot-check section from `review.md`, which the
operator overruled. The budget constants, the budget check, and the two budget
scenarios are deleted; the composition rule above is asserted as text presence
and absence.

### Consequences

- Good, because an author on one stack loads that stack's four or five facts
  instead of reading six runners' rows.
- Good, because a moved row has one owner, and an id that does not resolve
  fails the reference check by name.
- Bad, because `design` names ids owned by `steps`, so the id convention carries
  an owner that the pair alone does not express.
- Bad, because six small files cost a scope sentence each; a stack file is a
  lookup, not a document.

### Confirmation

The `serve-extended-rules` scenarios assert that `steps` and `design` name every
stack id with its condition and carry no stack body, and that the stack body is
served only on its own request. The `instruction-integrity` scenarios resolve
every instruction id a served instruction names, including effective addon
instructions. Review checks that each retained `review.md` obligation still
exists as a check.

## Pros and Cons of the Options

### Six stack instructions, one per runner

- Good, because no rule is removed and no stack keeps a body in a core
  instruction.
- Good, because the mapping has one owner and the load condition is explicit.
- Bad, because six small files need a scope sentence each.

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
- Bad, because twelve files cost about 60 lines of headers for no additional
  rule.

### Runner mechanics staying in `design`

- Good, because fewer files exist and the pairing rule is untouched.
- Bad, because `design` keeps the six-runner strict-mode table, so the core
  instruction still carries a stack body.

## More Information

Owner evidence: `obtain-effective-guidance` (`serve-extended-rules`),
`instruction-integrity`, `docs/arc42/08-crosscutting-concepts.md` §8.30, and
`Q-126`. Related records: ADR-005 (facet cascade), ADR-013 (skills-only
guidance), ADR-048 (effective policy and instruction JSON contract). The plan
reference is `givn/ideation/ise-rebuildability/plan.md`, phase `S5` and gate
`G2`.
