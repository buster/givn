# Ideation session: rules-through-instructions

## Phase

Handoff complete 2026-10-13.

## Progress

- 2026-10-13: Topic id confirmed by the user: `rules-through-instructions`
  (the branch `givn-instructions-rules-output-experiment` already exists in
  this worktree).
- 2026-10-13: Frame research complete. Read `givn status`, `givn spec tree`,
  README.md, arc42 ch. 3/5/6/8/10/11/12 (including the glossary and
  ADR-013), the `prose-consolidation`, `visual-design-contract`,
  `change-release-note`, `report-change-status`, `givn-workflow-enforcement`
  spec surfaces, `initialize-a-project`, `apply-a-managed-upgrade`, the
  generated rule files in `.opencode/` and `.agents/`, the skill and
  command assets, `src/cli/instructions.rs`,
  `src/config/cascade.rs`, `src/skills/targets.rs`, `src/skills/rules.rs`,
  `src/skills/render.rs`, `givn/config.yaml`, and the OpenSpec project
  configuration documentation the user referenced (`rules` map, `context`).
- 2026-10-13: Frame written to `frame.md`; four unknowns parked in
  `questions.md` (Q1 experiment shape, Q2 always-available boundary,
  Q3 one change or two, Q4 project-rule location and merge).
- 2026-10-13: Candidate terms recorded in `domain-terms.md` with collision
  checks (effective instruction, snapshot rule, pointer rule, project rule,
  the `rule` collision record, the `rules:` key collision).
- 2026-10-13: Persona set confirmed by the user: three reuses with topic
  stances, no new segment; record-only topic files written under `personas/`.
- 2026-10-13: Event Storming draft written to `eventstorming.md`: chain
  A1–F4 with actor/command/aggregate/policy/read model per event, six mermaid
  canvases (single declared aggregate `Project`), the reverse narrative, the
  duplicate-loading analysis with measured overlaps (propose, implement,
  tasks), invariants, and the question-dependent behavior table. Q5 (which
  rule topics receive an instruction id) and Q6 (whole-facet override vs
  project-rule merge) parked.
- 2026-10-13: Event Storming boundary quality review run (fresh context).
  All ten findings applied: the duplicated guard merged into F2, the
  persona-confirmation misattribution fixed in the frame and the storming,
  the boundary/invariant/analysis-result made conditional on Q1/Q2, the
  resident obligations of the `givn-project` block added to the overlap
  inventory (and Q2 rewritten around them: keep, generate, or remove), the
  term first-use cells changed to file:line, the D2 command field added, the
  command quotation corrected, `[Q5]` markers added to C1/D1, the
  delivery-setting fallback added to C4, and the session phase refreshed.
  The reviewer also corrected one frame detail: `skill_to_load` requires the
  domain-modeling and arc42 addons together.
- 2026-10-13: User correction on the domain vocabulary. The shape coins
  `snapshot rule`, `pointer rule`, `project rule`, and `rule topic` are
  rejected as too many and too specific. The vocabulary is now: Givn's
  built-in, technology-agnostic rules (workflow and guardrails) + the
  project's extension = the extended rule. All topic files swept
  (frame, storming, questions, personas, term record).
- 2026-10-13: User direction on scope and semantics: rules are extended,
  never overridden — unlike OpenSpec, Givn does not aim at completely custom
  workflows or a replaced built-in flow; Givn is the harness that tracks the
  workflow and guides the LLM. Q6 closed by this direction; the storming
  invariants carry the two constraints (extended, never overridden; the
  extension adds guidance, never workflow).
- 2026-10-13: Term check confirmed by the user: `extended rule` is
  `confirmed` in `domain-terms.md`; the shape coins stay rejected. All six
  mermaid canvases rendered successfully with `mmdc` after the rewrite.
- 2026-10-13: Use Case Definition written to `use-case-definitions.md`:
  two new `!` use cases (`obtain-effective-guidance`,
  `extend-an-instruction`), `initialize-project` and `upgrade-a-project`
  modified, summaries and unchanged interactions examined, the
  `instruction-integrity` fragment modified, a gated
  `generated-file-form` fragment for Q1 option 2, rejected candidates
  recorded.
- 2026-10-13: Use Case Definition boundary review run (fresh context); all
  findings applied: Q2/Q4-dependent wording changed to open-question
  phrasing, `[gated]` markers added (`extend-an-instruction` on Q4;
  reconciliation steps on Q1), the dialog-policy obligation added to the
  always-on set (C3 and Q2), the `instruction-integrity` fragment
  de-double-claimed (merge and delivery belong to
  `obtain-effective-guidance`, mapping note for `fragment.md`), the
  Consumes columns corrected (C4, D1, D2; `discover-specifications` added),
  the `generated-file-form` name marked provisional, and an "Assertion
  rework" section added naming the existing content assertions on generated
  guidance that move to the served instruction (refresh-generated-guidance,
  match-guidance-to-enabled-addons, prose-consolidation,
  change-release-note, initialize-a-project).
- 2026-10-13: Use Case Elaboration written: `use-cases/
  obtain-effective-guidance.md`, `extend-an-instruction.md`,
  `initialize-project.md`, `upgrade-a-project.md`,
  `instruction-integrity.md`, `generated-file-form.md` (Q7 parked for the
  replacement interplay).
- 2026-10-13: Use Case Elaboration boundary review run (fresh context); all
  fourteen findings applied: Q7 qualifiers on the obtain success guarantee
  and override example, `[Q5]` markers on step-id resolution, the
  flow-established precondition removed from `extend-an-instruction`, the
  `instruction-integrity` goal aligned (feeds the extended rule; merge and
  delivery belong to `obtain-effective-guidance`), Q2's three options named
  in the initialize flow, the no-SCM branch and unchanged-report rule
  restored, `[Q1]` qualifiers on the gated examples and interactions, the
  extension success guarantee reduced to the serving outcome, the
  `fragment.md` interaction mapping reflected (request row under obtain,
  session-start row with `inspect-change-state`), the "project's rules"
  wording swept to the extension vocabulary, event references corrected,
  and the assertion-rework inventory completed (visual-design-contract,
  visual-review-gate, instruction-integrity, generate-ideation-guidance
  added; Q5-dependent entries marked).

## Next steps

No change was started. On branch `givn-instructions-rules-output-experiment`,
run `/givn-propose extended-rules` (as a skill) to start the single change
from the seed; the proposal flow stamps the seed's status to
`proposed by extended-rules`.

## Completion (2026-10-13)

- Q3 answered by the user: one change on the experiment branch.
- Handoff records confirmed by the user: the change seed, the
  use-case-to-permanent mapping, the term merge, and the persona promotion
  choices.
- Change seed written: `changes/extended-rules.md` with `Status: ready`,
  no dependencies, suggested start 1.
- Topic document published: `docs/ideation/rules-through-instructions.md`.
- Term merged into `docs/arc42/12-glossary.md`: `Extended rule` (with the
  `Givn rule` disambiguation of the built-in-content sense); the rejected
  coins stay in the topic record only.
- Personas promoted: the dated 2026-10-13 stances appended to
  `givn/personas/developer--primary-user.md`,
  `agent--technical-role.md`, and `givn-maintainer--owner.md`; the topic
  files stay as historical record.

## Decisions already taken by the user

- Topic id (2026-10-13): `rules-through-instructions`.
- Frame confirmed as written (2026-10-13): problem, vision, scope, and
  stakeholders stand; later revisions must name the affected canvas, use
  cases, and seeds.
- Persona set confirmed (2026-10-13): three reuses with topic stances, no
  new segment.
- Vocabulary direction (2026-10-13): built-in rules + extension = extended
  rule; no snapshot-rule, pointer-rule, project-rule, or rule-topic
  coinages.
- Semantics direction (2026-10-13): rules are extended, never overridden;
  no custom workflow configuration and no replacement of the built-in Givn
  flow — Givn is the harness that tracks the workflow and guides the LLM.
- Requested experiment (recorded in the frame, confirmed): all rules and
  instructions become output of `givn instructions`; generated files only
  direct the agent to that call; Givn extends an instruction's built-in
  rules with project-specific rules; the duplicate-loading suspicion is
  analyzed rather than assumed.
