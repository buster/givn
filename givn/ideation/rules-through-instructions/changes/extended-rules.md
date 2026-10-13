# Change seed: extended-rules

Status: proposed by extended-rules

- Use-case IDs: `obtain-effective-guidance` (new), `extend-an-instruction`
  (new), `initialize-project` (modified), `upgrade-a-project` (modified);
  capabilities `instruction-integrity` (modified), `generated-file-form`
  (new, gated on Q1 option 2), `refresh-generated-guidance` and
  `match-guidance-to-enabled-addons` (modified)
- Topic: `rules-through-instructions`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`
- Suggested start: 1; no dependencies. `/givn-propose` runs on branch
  `givn-instructions-rules-output-experiment`.
- Handoff: ready; the experiment branch exists and the trial is the point.

## Why one change

The trial must exercise the whole story — generated files that direct
instead of copy, and the project's extension served with the built-in
rules — in one coherent experiment. Splitting leaves an intermediate state
where the delivery has landed but the customization answer has not (user
decision, 2026-10-13). The full product of the unchanged flow stays in
scope: the assertion rework that moves rule-content assertions from
generated guidance to the served instruction.

## Accepted decisions

### Extended rule — the vocabulary (user, 2026-10-13)

- Chosen: Givn's built-in, technology-agnostic rules (workflow and
  guardrails) plus the project's extension form the extended rule an agent
  follows; the shape coins `snapshot rule`, `pointer rule`, `project rule`,
  and `rule topic` are rejected and kept only as a rejection record.
  Rejected: naming the delivery shapes as domain terms.

### Extension-only semantics (user, 2026-10-13; Q6 closed)

- Chosen: rules are extended, never overridden; the extension adds to the
  built-in rules and never replaces, suppresses, or reorders them.
  Rejected: override-style semantics for rules.

### No custom workflow (user, 2026-10-13)

- Chosen: Givn is the harness that tracks the workflow and guides the LLM;
  the built-in flow (steps, order, gates) is not configurable and is not
  replaced; an extension adds guidance only.

### One change on a branch (user, 2026-10-13; Q3 closed)

- Chosen: one change, `extended-rules`, on
  `givn-instructions-rules-output-experiment`. Rejected: the two-change
  split (isolated trial, but a half-realized `obtain-effective-guidance`
  contract and two review cycles); deciding at handoff.

### The delivery model core (from the elaborated contracts)

- Chosen: generated skills and commands carry trigger metadata and
  invocation-specific orchestration only and direct the agent to
  `givn instructions <id>`; content is served at call time, so improved
  built-in rules need no synchronization; a full copy from an earlier
  version is reconciled when the directing form is resolved; a disabled
  addon's files are removed; a project-owned replacement stays
  byte-for-byte (R-74).
- Chosen: rule-content assertions on generated guidance move to the served
  instruction (the assertion-rework inventory names every surface).

### Guarantees kept

- Chosen: the dialog obligation stays in the project-start block under
  every always-on option (independent of optional addons); unknown
  instruction ids refuse by name; reference integrity checks stay as they
  are; the no-SCM, migration, allowlist, and one-revision behaviors of
  setup and upgrade are unchanged.

## Open questions

### Q1 — How is the experiment run and compared?

Run and compared: should the experiment change the generated output
directly on the branch, or add a setting that selects the generated-file
form (a file that directs the agent to `givn instructions` versus a file
that carries the rules)? It blocks the experiment's shape and the scope of
the setting-gated `generated-file-form` fragment. Options: **(1) branch-only
delivery change** — smallest change, no configuration surface, but no
side-by-side comparison; **(2) selectable generated-file form** — a config
key selects directing or full copy, A/B comparison in one checkout, but a
new product surface to specify, test, and possibly deprecate; **(3) staged**
— branch-only first, the setting only if the trial needs it. Givn
deliberately avoids configurable workflows; a file-form setting is not
workflow configuration, but the minimal-surface direction favors the
smallest change that answers the question. Example under (1): `givn skills
sync` on this branch rewrites `.opencode/skills/givn-propose/SKILL.md` to
the directing form and the previous full copy is seen by checking out the
base revision.

### Q2 — Which rules stay always available?

Which rules must remain always available, and which may move fully behind
`givn instructions`? It blocks what the generated files keep carrying and
the initialize-project delta. Today's always-on surface: the
`givn-project` block (a direction plus the resident task-checkoff and
commit-message obligations), the tone block when enabled, and skill
descriptions; the block's dialog obligation stays under every option. The
resident obligations restate what the tasks instruction owns — an overlap
site this change removes. Options: **(1) keep today's always-on set** —
smallest change; the commit rule keeps two homes and its drift risk;
**(2) generate the resident obligations from the canonical source** — one
source even for always-on rules; sync must render `AGENTS.md` rule content;
**(3) directions only** — minimal always-on surface; an agent that never
pulls guidance has no commit rule. Example under (1): a fresh agent reads
the commit rule at session start but learns the proposal rules only when
it pulls them.

### Q4 — Where does an extension live, and how does it extend a Givn rule?

Where should an extension live and how should it extend the built-in rules?
It blocks the `extend-an-instruction` contract and the `obtain-effective-
guidance` merge position. The key `rules:` already means the generated-rules
policy (`local-only`/`versioned`), so it cannot carry the extension map.
Options: **(1) new config-map key** (name open, e.g. `extensions:`) mapping
artifact id to rule strings, appended under a marked section — one file,
OpenSpec-like, but multi-line prose is awkward; **(2) extension file per
artifact** (e.g. `givn/artifacts/<id>/extension.md`), appended to the
resolved instruction — prose-capable and cascade-consistent, but a second
mechanism to discover; **(3) both** — config list for short rules, file for
prose; two mechanisms to specify and test. Example under (1): `extensions:
{proposal: ["Keep proposals under 500 words"]}` and `givn instructions
proposal` prints the built-in rules followed by the marked section.

### Q5 — Which steps receive an instruction id?

Which steps receive an instruction id served by `givn instructions`, and
which keep resident content in their generated files? It blocks the
generated-file contract for every skill and command. Several skills have no
artifact or policy id today (`givn-explore`, `givn-ideation`,
`givn-steps`, `givn-reverse-engineer`, `givn-characterize`, `grill-me`,
`grill-with-docs`). Options: **(1) every generated skill and command gets
an id** — complete single-source coverage; a second asset family to
maintain; **(2) only artifact- and policy-aligned steps** — smaller change;
duplication persists for the rest; **(3) staged** — the artifact chain now,
per-step coverage later with usage evidence. Example under (2):
`/givn-propose` directs to the instruction while `/givn-ideation` keeps its
content.

### Q7 — Does an extension apply on top of a whole-facet replacement?

When a project owns a whole-facet replacement for an id and also records an
extension for it, does the extension apply on top of the replacement? It
blocks one rule and one guarantee in `obtain-effective-guidance`. Options:
**(1) the extension applies on top** — one consistent rule, but a replacing
project must know the extension still appends; **(2) the replacement stands
alone; the extension is shadowed and reported** — R-74 stays literal, but
two customization paths cancel each other; **(3) refuse the combination by
name** — no ambiguity, but refuses a possibly intended configuration.

## Scope note

Makes the extended rule true end to end: the built-in rules served at call
time, generated guidance that directs instead of copying, the reconciliation
of full copies on sync and upgrade, the extension source and its merge, the
assertion rework, and the mapping updates in `fragments/fragment.md`.
Introduces the permanent leaves `obtain-effective-guidance` (capability
`serve-extended-rules`) and `extend-an-instruction` (capability
`instruction-extension`); modifies `initialize-project` and
`upgrade-a-project` and the `instruction-integrity`,
`refresh-generated-guidance`, and `match-guidance-to-enabled-addons`
capabilities; adds the `generated-file-form` capability only under Q1
option 2. Amends arc42 §8.3a/§8.7, the glossary (`Extended rule`, the
built-in-content sense of `Givn rule`), and the content assertions of the
named scenarios when the change lands.
