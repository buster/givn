# Proposal: apply-readability-independent-review

## Release note

Apply the readability gate's independent-review rule to the seven earlier justified findings by extracting five long methods and re-justifying two structural units with reviewed rationales.

## Use-Case Context

- Use-case ID: `verify-change` (existing; the change extends the
  `readability-review-gate` capability)
- Ideation topic: `readability-experiment`
- Seed path: none — this change applies an already confirmed rule to recorded
  findings; it does not introduce a new idea.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner` — confirmed for `readability-experiment` on
  2026-10-19. Personas are review lenses, not Gherkin actors.

## Problem / Opportunity

The readability review gate now refuses a justified finding unless its
rationale records an independent reviewer. The seven rationales recorded for
the gate's own earlier findings predate that rule. A test-only change proved
the rejection but changed no production source, so the gate still cannot run
green on the branch that carries the rule.

The underlying findings are also real. Five units are over the method-length
threshold, and two units are structural modules that are over the readable
size because they aggregate cross-file facts or own one shared registry. The
five long methods should be split; the two structural units should keep their
shape and carry a reviewed rationale.

## Proposed Solution

Apply the independent-review rule to this branch:

- Split the five long methods into smaller named helpers without changing
  their observable behavior or their tests.
- Keep the two structural modules as they are, touch each with one small
  improvement, and record a rationale that names the unit, its classification,
  its finding reason, concrete evidence, the responsibility, and a rejected
  alternative.
- Record the gate decisions in the change's review record: a fixed marker for
  each split unit and a justified marker for each structural unit.

When the independent reviewer signs the two rationales, the readability review
passes with no findings left over the threshold.

## Capability Routing

`givn spec route --change apply-readability-independent-review` recommended
`EXTEND givn-workflow-enforcement` from the otherwise-empty scaffold. The
decision deviates because this change touches only the readability gate's
rationale handling; the gate's observable scenarios belong to the
`readability-review-gate` capability.

| Proposed capability | Route's recommendation | Decision | Rationale (if deviated from route) |
|---|---|---|---|
| `readability-review-gate` | `EXTEND givn-workflow-enforcement` | `EXTEND readability-review-gate` | The proposal's subject is the readability gate, not the wider workflow-enforcement capability that merely contains the `givn check` command. |

## Out of Scope

- No new command, flag, or user-facing output.
- No change to the analyzer's thresholds or classification rules.
- No change to the gate's decision vocabulary or rationale floor.
- No arc42 chapter for this demonstration change.

## Open Questions

### D1 — How should the false long-method measurement on `find_block_end` be removed?

- Question: The skills renderer's `find_block_end` function is 20 code lines,
  but the analyzer reports 116 because its span walker counts braces inside
  the directive string literals. Should the span walker become string-aware,
  or should the function stop carrying those literals in its body?
- Options:
  1. **Make the span walker string-aware** — ignore braces inside string
     literals; gain: every future measurement is more accurate; cost: the
     analyzer's span semantics change for every unit and the gate's base
     measurements shift; in practice: a function containing `"{{#if "` would
     measure its real length, but so would every other function with braces
     in strings, so the change would need its own baseline analysis.
  2. **Move the directive literals to module constants and use small
     predicate helpers** — gain: the measurement reflects the function's real
     shape without touching analyzer semantics; cost: the renderer grows a
     few constants and predicates; in practice: `find_block_end` calls
     `is_open_condition`, `is_close_condition`, and `is_else_condition`, and
     stops at its own closing brace.
- Disposition: answered — option 2 is recorded as the design decision. It is
  a local extraction, preserves behavior, and does not weaken the gate or the
  analyzer for every other unit.
