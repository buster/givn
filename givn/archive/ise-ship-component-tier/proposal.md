# Proposal: ise-ship-component-tier

## Release note

An Ise change that ships rebuilt code now executes the retained component
corpus before review and archive, and the served guidance names the
whole-system working-tree ship and both verification tiers.

## Use-Case Context

- Use-case ID: verify-change; prepare-an-ise-test; obtain-effective-guidance
- Ideation topic: none
- Seed path: none
- Confirmed Personas: none — the change alters command gates and served
  instruction text; it declares no visual design contract.

## Review Context

- Review kind: self-review
- Reason: the default kind; no independent review was requested for this
  change.

## Problem / Opportunity

A whole-system Ise rebuild can reach `REVIEW: PASS` without ever executing the
retained component Gherkin corpus. The change records `Behavior change: none`
and skips delta specs, so `givn check tasks`, `givn check review`, and
`givn archive` each take a "not applicable" branch and return before any
runner executes. A runner that only runs unit tests therefore satisfies the
gate, and a green frozen End-to-end suite is presented as the ship result.

The served `ise` instruction already states the component obligation, but the
working-tree ship sequence is titled `Single-Component Rebuild with Rebuilt
code: ship` and the practice table maps working-tree ship only to that
decision. The design instruction's `Whole-system rebuild` row omits the
coupled step definitions and fakes under the replaced source roots, and the
review instruction treats a missing tier as a non-critical finding for the
next change.

The impact: a rebuild can land with the Specification's executable corpus
unexecuted, so the only proof is the black-box suite that never binds
component step definitions.

## Proposed Solution

- **A ship carries component evidence.** An ise-effective change whose design
  records `Rebuilt code: ship` must produce a fresh runner result for the
  retained component corpus under `givn/specs/**`. `givn check tasks` executes
  that corpus and refuses a missing result or a result whose executed count
  differs from the declared component execution units, naming
  `component_executed=<executed>/<declared>`.
- **Review and archive require the evidence.** `givn check review` and
  `givn archive` refuse an Ise ship whose receipt lacks the component result or
  whose recorded corpus digest no longer matches, even when the change records
  `Behavior change: none` and skips delta specs.
- **The delta still runs.** A ship that also carries delta scenarios runs the
  delta as the change-scope subject and the retained corpus as the component
  subject; both must be green and both are bound into the receipt.
- **The guidance names the whole-system ship.** The served `ise` instruction
  gains a `Whole-system rebuild with Rebuilt code: ship` sequence that requires
  the component runner, one step file per capability, the fakes and fixture
  builders, and both tier results. The practice table maps working-tree ship to
  both rebuild decisions.
- **The authoring instructions carry the same rule.** The design instruction's
  whole-system row names the coupled step definitions and fakes and requires
  both tier commands and counts; a design without a component runner is a
  design-review finding; the tasks instruction says `.givn-skip specs` removes
  only the delta batch; the review instruction classifies a missing declared
  tier as critical.

## Capability Routing

`givn spec route` reported `EXTEND givn-workflow-enforcement (use case
verify-change)` with signal.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| givn-workflow-enforcement | EXTEND givn-workflow-enforcement (verify-change) | `EXTEND givn-workflow-enforcement` | none |
| serve-extended-rules | no signal | `EXTEND serve-extended-rules` | the served `ise`, design, design-review, tasks, and review text is asserted by this capability |

## Out of Scope

- No new CLI command; the existing change-scope command runs the retained
  corpus.
- `Rebuilt code: discard` and spec-copy trials keep the existing
  `ise-trial-result/v1` gate.
- The `@e2e` package boundary, product binding, and coverage contract are
  unchanged.
- The Passuff rebuild that exposed the gap stays a separate change; this
  change fixes Givn's guidance and gates.

## Open Questions

### D1 — Which decision triggers the component tier?

- Question: must the component tier apply to every `Rebuilt code: ship`, or
  only to `Whole-system rebuild`?
- Options:
  1. **Every ship** — both rebuild decisions adopt the replacement into the
     change and rewrite step definitions or fakes under a replaced source
     root; gain: one rule, no classification escape; cost: a single-component
     ship runs the retained corpus too; in practice: a one-component ship runs
     its delta and the retained corpus.
  2. **Whole-system only** — only a whole-system rebuild runs the corpus;
     gain: smaller rerun for single-component ships; cost: a single-component
     ship can still rewrite step definitions and skip the unchanged retained
     scenarios; in practice: the reported failure is fixed, a sibling gap
     stays.
- Disposition: answered — option 1; `Rebuilt code: discard` is excluded
  because the replacement is not adopted.

### D2 — Exact executed count or a non-zero floor?

- Question: must the component result execute exactly the declared component
  execution units, or is any non-zero green count acceptable?
- Options:
  1. **Exact declared count** — the result must equal the plain scenarios plus
     Scenario Outline example rows under `givn/specs/**`, excluding `@e2e`;
     gain: a partially executed corpus cannot pass; cost: the project runner
     must honour `GIVN_FEATURES=givn/specs` exactly; in practice: 185 declared
     units require `total: 185`.
  2. **Non-zero floor** — any green result passes; gain: fewer runner
     constraints; cost: a runner that executes one scenario proves nothing; in
     practice: `total: 1` would pass a 185-unit corpus.
- Disposition: answered — option 1, matching the existing trial gate's exact
  count rule.

### D3 — One runner invocation or two when a delta exists?

- Question: a ship with delta scenarios and a retained corpus needs both
  subjects; does the change scope run one command over both, or one command per
  subject?
- Options:
  1. **Two invocations** — the delta stays the existing change-scope subject
     and the retained corpus runs into a second result; gain: no change to
     `GIVN_FEATURES` semantics and both subjects are bound; cost: two runner
     runs; in practice: `givn check tasks` runs the delta, then
     `givn/specs`.
  2. **One invocation over both** — pass a path list in `GIVN_FEATURES`; gain:
     one run; cost: every project runner must accept a list, and the documented
     single-path contract breaks.
- Disposition: answered — option 1.

### D4 — Does the gate apply to an empty corpus?

- Question: when `givn/specs/**` declares no component execution unit, does an
  Ise ship still require a component result?
- Options:
  1. **No tier to execute** — the requirement applies only when at least one
     component unit is declared; gain: a project with no component scenarios is
     not blocked by an impossible result; cost: an empty corpus is trusted; in
     practice: zero declared units keeps today's not-applicable branch.
  2. **Refuse an empty corpus** — a ship with no component units is refused;
     gain: no ship without an executable Specification; cost: a small project
     with only End-to-end scenarios cannot ship.
- Disposition: answered — option 1; the trial gate already treats zero declared
  component units as no component tier.
