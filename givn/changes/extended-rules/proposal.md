# Proposal: extended-rules

## Release note

Givn serves every step's rules through `givn instructions` — installed skills
and commands direct agents there instead of carrying rule copies — and a
project can extend any served instruction with its own rules, which
synchronization and upgrades never fight.

## Use-Case Context

- Use-case ID: `obtain-effective-guidance` (new), `extend-an-instruction`
  (new), `initialize-project` (modified), `upgrade-a-project` (modified).
- Ideation topic: `rules-through-instructions`.
- Seed path: `givn/ideation/rules-through-instructions/changes/extended-rules.md`
  — already stamped `Status: proposed by extended-rules`; its original text is
  unchanged.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`. The change has CLI text output as a user-visible
  surface (the served instruction and the synchronization report); the
  existing confirmed project records are reused and no Persona is created or
  promoted.

## Problem / Opportunity

The rules an agent follows reach it through two overlapping channels, and
nothing keeps them aligned:

- Generated rule files — each installed skill and slash command carries a
  full, hand-maintained copy of the rules for its step, and the
  `givn-project` block in `AGENTS.md` restates the task-checkoff and
  commit-message obligations that the tasks instruction also owns.
- Printed instructions — `givn instructions <id>` resolves the canonical
  rules at call time, extended by nothing: a project cannot durably add its
  own rules to an instruction.

The copies are what the agent reads first, and they drift. The
`prose-consolidation` capability had to repair exactly that kind of drift by
hand, and the same rule today lives in up to three files (skill template,
command asset, instruction asset). Customization makes it worse: a project's
own rules have no additive home. Replacing a whole facet (an instruction
file, a skill template, a command asset) is all-or-nothing; under
`rules: local-only` a repo-side edit to a generated file has no home at all,
and under `rules: versioned` the next `givn skills sync` or `givn upgrade`
overwrites it. A team convention or domain rule either owns an entire file
or is lost.

The opportunity: one authoritative home per rule. The instruction served at
call time carries the built-in rules and the project's extension; installed
files shrink to a trigger, invocation orchestration, and a direction to the
instruction; improved built-in rules then arrive at the next request without
any synchronization, and project rules survive every sync and upgrade.

## Proposed Solution

- The rules an agent follows for a step come from one place: the extended
  rule served by `givn instructions <id>` — Givn's built-in, technology-
  agnostic rules resolved at call time, plus the project's extension when one
  exists. Text output and machine-readable output carry the same extended
  content.
- Every installed skill and command carries its trigger metadata,
  invocation-specific orchestration (change context, subagent boundaries,
  stop conditions, reporting), and a direction to the instruction it needs —
  no copy of built-in rule text.
- Each direction names an instruction id that resolves in the same Givn
  version; every step of the workflow receives such an id.
- When the directing form is resolved, the next `givn skills sync` or
  `givn upgrade` rewrites a generated file installed by an earlier version
  that still carries a full copy; a disabled addon's files are removed; a
  project-owned replacement stays byte-for-byte and receives no injected
  text.
- A project records an extension for an instruction in project-owned
  configuration (the source and shape are D2). The extension adds rules; it
  never overrides, suppresses, or reorders the built-in rules, and it never
  changes steps, order, or gates. It is effective at the next request,
  survives every upgrade untouched, and never enters generated files or the
  managed upgrade revision. Removing it leaves the built-in rules standing
  alone.
- An extension that names an unknown instruction or is malformed is dropped
  with a warning naming the problem; nothing changes, and the built-in rules
  are served alone.
- The always-on project-start block keeps its direction to
  `givn instructions` and its dialog-policy obligation under every option;
  whether its resident task-checkoff and commit-message obligations stay
  resident, are generated from the canonical source, or are dropped is D1.
- Improved built-in rules are served at the next request after an upgrade —
  no synchronization, no committed change, no project action.
- An unknown instruction id keeps today's named refusal listing the known
  ids; a request never modifies project state.

## Capability Routing

`givn spec route --change extended-rules` reported no signal (the top
candidates tied at 16.00), so the recommendation column records no signal for
every row; the decisions below are the recorded declarations. The column is
kept because the archive reads the decision from the third table cell (a
three-column table would not be parseable).

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `serve-extended-rules` | none — route reported no signal | `NEW in obtain-effective-guidance` | New capability for the new serving goal: the request serves the built-in rules plus the project's extension, identical on text and machine-readable paths. |
| `instruction-extension` | none — route reported no signal | `NEW in extend-an-instruction` | New capability for the developer's own goal: record, change, and remove a project's rules for one instruction. |
| `initialize-a-project` | none — route reported no signal | `EXTEND initialize-a-project` | Existing capability; synchronization renders the directing form and reconciles full copies. |
| `apply-a-managed-upgrade` | none — route reported no signal | `EXTEND apply-a-managed-upgrade` | Existing capability; the managed revision refreshes the directing form and never carries extensions. |
| `instruction-integrity` | none — route reported no signal | `NEW in fragments` | Fragment capability: it lives in the reusable `support` fragment (`givn/specs/fragments/fragment.md`); fragments carry no use-case directory, so the delta lives under `specs/fragments/`. |
| `refresh-generated-guidance` | none — route reported no signal | `NEW in fragments` | Fragment capability of the `support` fragment; generated guidance loses its rule content. |
| `match-guidance-to-enabled-addons` | none — route reported no signal | `NEW in fragments` | Fragment capability of the `support` fragment; addon conditions select the served instruction's content. |
| `generate-ideation-guidance` | none — route reported no signal | `NEW in fragments` | Fragment capability of the `support` fragment; the ideation step gains a served instruction. |
| `prose-consolidation` | none — route reported no signal | `EXTEND prose-consolidation` | Existing capability (create-change); its content assertions move to the served instruction. |
| `change-release-note` | none — route reported no signal | `EXTEND change-release-note` | Existing capability (create-change); the shared release note is asserted on the served implementation instruction. |
| `visual-design-contract` | none — route reported no signal | `EXTEND visual-design-contract` | Existing capability (create-change); the design, design-review, and steps assertions move to the served instructions. |
| `visual-review-gate` | none — route reported no signal | `EXTEND visual-review-gate` | Existing capability (verify-change); the review block moves to the served review instruction. |

## Out of Scope

- The generated-file setting (seed Q1 option 2) and the gated
  `generated-file-form` fragment: the trial is branch-only, `givn skills
  sync` renders the directing form directly, and no new configuration key
  selects a file form. If the trial later needs side-by-side comparison, the
  setting returns as a follow-up seed.
- Custom workflow configuration: steps, order, gates, and the artifact list
  stay built-in and are not configurable; an extension adds guidance only
  (Q6, closed).
- Override semantics for rules: rules are extended, never overridden (Q6,
  closed).
- The `local-only` / `versioned` rules policy semantics are unchanged.
- The whole-facet replacement mechanism (an instruction, skill template, or
  command asset replaced by a project) is unchanged and is never written to;
  how an extension interacts with it is D4.
- No new agent target, no harness internals, no artifact-catalog change; the
  hand-written rules a project keeps beside Givn's (for example a personal
  skill) are untouched.

## Open Questions

Seed question dispositions before specs — nothing intersecting the seed is
left undispositioned:

| Seed question | Disposition |
|---|---|
| Q1 — how the experiment is run and compared | answered: branch-only (user, 2026-10-13). `givn skills sync` renders the directing form directly; no setting selects a file form. Rejected: the selectable setting in this change (a product surface to specify, test, document, and possibly deprecate for a need the trial has not shown) and the staged variant (it collapses into the chosen option — if the trial shows the need, the setting returns as a follow-up seed). |
| Q3 — one change or two for delivery and extensions | answered: one change on the experiment branch (user, 2026-10-13), because the trial must exercise the whole story; the two-change split was rejected. |
| Q6 — whole-facet override versus extension | answered: rules are extended, never overridden (user, 2026-10-13); override-style semantics for rules are rejected. |
| Q2 — which rules stay always available | carried → D1 |
| Q4 — where an extension lives and how it extends a rule | carried → D2 |
| Q5 — which steps receive an instruction id | carried → D3 |
| Q7 — an extension on top of a whole-facet replacement | carried → D4 |

Each carried question below names its owner and the artifact that decides it:
the design author, decided in the decisions ledger of `design.md` as
`Decides D<n> (seed Q<n>)`.

### D1 — Which rules stay always available (seed Q2)

- Question: which rules must remain always available in a project — present
  without any `givn instructions` call — and which may move fully behind the
  call? The dialog-policy obligation stays in the project-start block under
  every option; the choice is about the block's resident task-checkoff and
  commit-message obligations.
- Options:
  1. **Keep today's always-on set.** Meaning: the block keeps the resident
     obligations as hand-written text, beside the direction and the dialog
     obligation. Gain: smallest change; the safety-critical commit rule
     survives an agent that never calls Givn. Cost: the commit rule keeps two
     homes (the block and the tasks instruction), so its drift risk survives
     the experiment. In practice: a fresh agent reads the commit rule at
     session start but learns the proposal rules only when it pulls them.
  2. **Generate the resident obligations from the canonical source.**
     Meaning: the block's rule text is rendered at synchronization time from
     the same asset `givn instructions` serves, and the dialog obligation
     stays as today. Gain: one source even for always-on rules — the block
     and the tasks instruction can never disagree. Cost: synchronization
     gains a rendering surface to specify, test, and keep honest; the block
     becomes a computed artifact. In practice: the block shows the commit
     rule, and its text is the tasks instruction's own words.
  3. **Directions only.** Meaning: the block keeps only the directions and
     drops the resident obligations. Gain: one home for every rule; the
     always-on surface is minimal. Cost: an agent that never pulls the
     implementation guidance has no commit rule — today's safety net is
     removed. In practice: a session-start agent that skips the call commits
     with no resident correction.
- Disposition: carried — owner: the design author; decided in the decisions
  ledger of `design.md` (`Decides D1 (seed Q2)`).

### D2 — Where an extension lives and how it extends a rule (seed Q4)

- Question: where should a project's extension live, and how should it
  extend an instruction's built-in rules — including the marked section's
  position and the rejection mode for an unknown instruction id or a
  malformed entry? The key `rules:` already means the generated-rules policy
  (`local-only`/`versioned`), so it cannot carry the extension map.
- Options:
  1. **A new config-map key in `givn/config.yaml`.** Meaning: an instruction
     id maps to a list of rule strings, appended to the served instruction
     under a marked section. Gain: one file, mirroring the reference model;
     short rules stay one line each. Cost: multi-line prose is awkward in a
     YAML list; the new key needs a name that cannot collide with `rules:`.
     In practice: `extensions: {proposal: ["Keep proposals under 500
     words"]}` and `givn instructions proposal` prints the built-in rules
     then the marked section with that line.
  2. **An extension file per instruction** (for example
     `givn/artifacts/<id>/extension.md`). Meaning: the project's rules as
     Markdown, appended to the resolved instruction, sibling to the existing
     whole-facet replacement. Gain: prose-capable; sits in the existing
     cascade location; diffs review like normal project content. Cost: a
     second discovery mechanism on top of the config overlay. In practice:
     `givn/artifacts/proposal/extension.md` holds the rules; the printed
     instruction appends it.
  3. **Both**: config for short rules, file for prose. Gain: each
     granularity gets a fitting home. Cost: two mechanisms to specify, test,
     document, and explain — the larger the surface, the larger the
     experiment.
- Disposition: carried — owner: the design author; decided in the decisions
  ledger of `design.md` (`Decides D2 (seed Q4)`).

### D3 — Which steps receive an instruction id (seed Q5)

- Question: which generated skills and commands receive an instruction id
  served by `givn instructions`, and which keep resident content? Several
  steps have no id today: `givn-explore`, `givn-ideation`, `givn-steps`,
  `givn-reverse-engineer`, `givn-characterize`, `givn-archive`,
  `givn-implement`, the addon skills `givn-domain-modeling` and
  `givn-dev-principles`, and the addon commands `grill-me` and
  `grill-with-docs`.
- Options:
  1. **Every generated skill and command gets an instruction id.** Meaning:
     a step-asset family joins artifacts and policies in the call. Gain:
     complete single-source coverage — one mechanism for all rules, and the
     extension mechanism applies to every step. Cost: new instruction assets
     to name and maintain; the instruction id namespace grows beyond
     artifacts; the assertion rework moves the non-artifact steps' content
     assertions to the served instruction. In practice: `givn instructions
     ideation` prints the ideation guidance and the skill file directs to it.
  2. **Only artifact- and policy-aligned steps.** Meaning: proposal, specs,
     design, design-review, tasks, review, and dialog move behind the call;
     the other steps keep resident files. Gain: smaller change; the
     highest-traffic path becomes single-source. Cost: duplication persists
     for the remaining steps, the direction-only trial does not cover them,
     and the extension mechanism cannot reach their rules. In practice:
     `/givn-propose` directs to the instruction while `/givn-ideation` still
     carries its content.
  3. **Staged: the artifact chain now, per-step coverage later.** Meaning:
     the experiment covers artifacts and policies; the remaining steps
     become a named follow-up. Gain: no premature asset family;
     evidence-driven coverage. Cost: the single-source claim stays partial
     until the follow-up lands. In practice: the trial proves the directing
     form on the chain only.
- Disposition: carried — owner: the design author; decided in the decisions
  ledger of `design.md` (`Decides D3 (seed Q5)`).

### D4 — Does an extension apply on top of a whole-facet replacement (seed Q7)

- Question: when a project owns a whole-facet replacement for an instruction
  id and also records an extension for it, does the extension apply on top of
  the replacement?
- Options:
  1. **The extension applies on top of the replacement.** Meaning: the
     replacement is the base the extension extends. Gain: one consistent
     rule — the extension always extends the effective base; no recorded
     configuration is silently shadowed. Cost: a replacing project must know
     that a recorded extension still appends. In practice: the request
     serves the project's replacement text followed by the marked extension
     section.
  2. **The replacement stands alone; the extension is shadowed and
     reported.** Meaning: a whole-facet replacement is full control. Gain:
     "replacement = full control" stays literal. Cost: two project-owned
     customization paths cancel each other unless reported; the project must
     move its rules into the replacement. In practice: recording an
     extension for a replaced id changes nothing visible but the report.
  3. **Givn refuses the combination by name.** Meaning: replacement plus
     extension for one id is rejected. Gain: no ambiguity at all. Cost:
     refuses a configuration that could be intended. In practice: the
     request fails naming both sources.
- Disposition: carried — owner: the design author; decided in the decisions
  ledger of `design.md` (`Decides D4 (seed Q7)`).
