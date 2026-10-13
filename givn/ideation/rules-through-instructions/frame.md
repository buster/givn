# Frame: rules-through-instructions

## Problem

Givn delivers its rules to an authoring agent through two overlapping
channels:

1. **Generated rule files** — skills and slash commands written as full
   snapshots by `givn skills sync` into the selected agent targets
   (`.opencode/`, `.agents/`, `.claude/`, `.omp/`), plus the fenced blocks
   maintained in `AGENTS.md`.
2. **Printed instructions** — `givn instructions <artifact>` resolved at
   call time through the project-overrides-first cascade.

The two channels carry overlapping content, up to three copies of the same
rules (skill template, command asset, instruction asset). Concrete: the
generated `givn-propose` command's "Artifact-specific rules" section repeats
the release-note, capability-routing, use-case-context, seed-stamping, and
question-disposition rules that also live in the `proposal` instruction; the
`givn-propose` skill carries a third partial copy. The doctrine already says
the instruction is canonical and generated snapshots have no independent
policy authority (arc42 §8.3a; the `prose-consolidation` capability), but
nothing keeps the copies mechanically aligned — and the copies are exactly
what the agent reads first.

Customization is the second half. A project cannot durably add its own rules
to an instruction:

- Under `rules: local-only` (the default), generated rule files are local
  installation output — a repo-side edit has no home and does not survive a
  second machine.
- Under `rules: versioned`, `givn skills sync` and `givn upgrade` overwrite
  the generated files. This repository runs `rules: versioned`
  (self-hosting), so every generated rule here is exactly in that state.
- The supported customization replaces whole facets (cascade:
  `givn/artifacts/<id>/instructions.md`, `givn/skills/<name>/SKILL.md.tmpl`,
  `givn/commands/<name>.md`; `givn eject`) or patches the manifest in
  `givn/config.yaml` (inline `instruction:` text). There is no additive
  "extend this instruction with project-specific rules".

OpenSpec solves the same shape in its project configuration:

```yaml
rules:
  proposal:
    - Keep proposals under 500 words
```

"Extra rules added to one artifact's built-in guidance." Only that mechanism
is the reference: OpenSpec also lets a project define its workflow schema,
which Givn deliberately does not. The Givn flow (propose → spec → design →
design-review → tasks → implement → review → archive) stays built-in, and
Givn is the harness that tracks the workflow and guides the LLM. The key
`rules:` in `givn/config.yaml` is already taken by the local-only/versioned
generated-rules policy (glossary: Rules policy).

Requested outcome (user, 2026-10-13, on branch
`givn-instructions-rules-output-experiment`): run an experiment that makes
all rules and instructions output of `givn instructions`, with the generated
rules and skills only directing the agent to that call, and Givn extending
an instruction's built-in rules with project-specific rules the way OpenSpec
does. Two directions the user set alongside it (2026-10-13): rules are
extended, never overridden, and Givn is not configurable into a completely
custom workflow — the built-in flow is not replaced. The user also suspects
the delivery change removes content that is currently "loaded twice" — the
same rules reaching the agent once as a full copy and again as instruction
output. That suspicion is an analysis task, not an assumption.

## Vision

One authoritative home per rule: the effective instruction served by
`givn instructions`, extended by the project's extension. Everything the
agent needs for a step — orchestration, built-in rules, project additions —
comes from that call. Generated rule files shrink to a trigger and a
direction to the instruction: enough for the harness to auto-load or invoke
the right skill or command, then the instruction carries the content. A
project adds its extension in project configuration; sync and upgrade never
conflict with it, and an upgrade propagates improved built-in rules without
touching the extension. The Givn maintainer edits one asset per rule; every
target sees the change on the next call instead of the next sync.

## Scope boundary

In scope (candidate; confirm):

- The delivery model: what `givn instructions` prints per artifact and what
  the generated skills, commands, and `AGENTS.md` blocks contain (a
  direction to the instruction, or a full copy).
- Extensions: project-specific rule additions to an instruction's
  guidance — where they live, their shape, how they extend the built-in
  rules (position, validation, failure mode), and the boundary to the
  existing whole-facet replacement mechanism (risk R-74: extensions extend,
  they never replace).
- The always-available boundary: which content must stay resident (skill
  trigger descriptions, the project-start block, the tone addon) and which
  content moves behind the call.
- The machine contract: `givn instructions --json` shape for automation
  callers; what a fresh delegated context receives.
- The experiment itself: what is compared, in which checkout, and what
  counts as evidence that direction-only generated files work (including
  the duplicate-loading analysis).
- Interaction with the rules policy (`local-only`/`versioned`) and with the
  managed upgrade.

Out of scope (candidate; confirm):

- Changing the `local-only`/`versioned` policy semantics itself.
- Custom workflow configuration or replacing the built-in Givn flow:
  project-defined steps, order, or gates. Givn is the harness that tracks
  the workflow and guides the LLM; the extension adds guidance, never
  workflow.
- Agent-harness internals: how opencode, Claude Code, omp, or another
  harness internally decides to auto-load a skill or expand a command.
- The artifact catalog: which steps exist, in what order (`design-review`
  stays).
- Non-Givn rule files (for example the hand-written `gh-stack` skill beside
  the generated skills in this repository).

## Stakeholders

| Stakeholder type | Interest |
|---|---|
| Consumer-project owner (developer / maintainer) | Wants project-specific extensions that survive `givn skills sync` and `givn upgrade` without forking or owning whole files; expects the upgrade path to stay clean and the project to stay readable. |
| Authoring agent (technical role / automation caller) | Consumes the guidance; needs each rule discoverable at the moment of the step, stable between calls, and machine-readable where a contract exists. |
| Givn maintainer (owner) | Owns up to three copies per rule today (skill template, command asset, instruction asset); wants one canonical source, truthful specs and quality requirements, and releases that propagate rules without drift. |

## Research protocol

### What exists

- **Instruction delivery.** `givn instructions <artifact>`
  (`src/cli/instructions.rs`) resolves the instruction through the cascade
  (`src/config/cascade.rs::resolve_instruction`): a project override
  `givn/artifacts/<id>/instructions.md` wins entirely over the embedded
  `instructions/<id>.md`; addon instructions fall back as
  `addons/<id>/instructions.md`. A policy id (for example `dialog`)
  resolves the same way without a lifecycle stage. `--json` emits `id`,
  `generates`, `requires`, `instruction`, and `skill_to_load` when the
  domain-modeling and arc42 addons are both active (arc42 §8.3b).
- **Context-aware delivery.** `givn instructions` with no artifact inspects
  project/change state and names the next step, its shell command, and the
  skill to load (`src/cli/instructions.rs::run_generic`).
- **Generated rule files.** `givn skills sync` (`src/skills/targets.rs::sync`)
  renders skill templates (`assets/skills/*/SKILL.md.tmpl`, project override
  `givn/skills/<name>/SKILL.md.tmpl`) and command assets
  (`assets/commands/*.md`, project override `givn/commands/<name>.md`) into
  the configured targets; writes attribution integrations; and refreshes the
  `AGENTS.md` fenced blocks (`givn-project` direction to `givn instructions`,
  `absolute-mode` when enabled).
- **Fixed skill/command catalog.** Skills and commands are a fixed list
  (`default_skill_names`, `default_command_names`); a custom artifact added
  by manifest overlay receives no skill and is reached only through
  `givn instructions <id>`. `dialog` is likewise instructions-only. An
  instructions-only delivery already exists for policy and custom content;
  the experiment extends the shape to the core workflow steps.
- **Duplication, concrete.** `assets/commands/givn-propose.md` (345 lines)
  carries an "Artifact-specific rules" section repeating rules of
  `assets/instructions/proposal.md` (release note, capability routing,
  use-case context, seed stamping, question disposition, question IDs);
  `assets/skills/givn-propose/SKILL.md.tmpl` (53 lines) carries a third
  partial copy, including the persona-confirmation rule that `proposal.md`
  also owns. `assets/commands/givn-implement.md`
  (285 lines) and `assets/skills/givn-implement/SKILL.md.tmpl` (180 lines)
  overlap the `tasks` instruction the same way.
- **Doctrine today.** arc42 §8.3a: the resolved `specs` instruction is the
  single normative source for interaction inventories and E2E evidence;
  generated skills and commands are snapshots with no independent policy
  authority. The `prose-consolidation` capability asserts derived guidance
  points at canonical policies and that detailed dialog rules stay in the
  dialog instruction rather than competing copies. Risk R-25 names drift
  between canonical and derived guidance; risk R-74 forbids silently
  injecting defaults into project-owned overrides.
- **Customization surface.** Cascade replacement per facet; `givn eject`
  materializes an embedded asset into the project; the manifest overlay in
  `givn/config.yaml` patches an artifact, adds a custom artifact with inline
  `instruction: |` text, reorders, or removes steps. No extension mechanism
  exists; the key `rules:` is taken by the Rules policy scalar.
- **Rules policy.** `rules:` in `givn/config.yaml` (`local-only` default /
  `versioned`), implemented in `src/skills/rules.rs` (ignore destinations,
  tracked-rule transition, cleanliness report). This repository runs
  `rules: versioned` and tracks `.opencode/skills/givn-*`,
  `.opencode/commands/givn-*`, `.agents/skills/givn-*`, and `AGENTS.md`.
- **OpenSpec model (reference).** `openspec/config.yaml`: `context` free
  text injected into every artifact's instructions; `rules` an
  artifact-id → list-of-strings map, "extra rules added to one artifact's
  built-in guidance"; `operations` advisory guidance for apply and archive.
  Invalid fields never fail a command; a bad value is dropped with a
  warning. Only the additive-rules mechanism is the reference — the custom
  workflow schemas are not.

### What the permanent specs already own

- `initialize-project/initialize-a-project.feature` — target selection,
  persisted `skills.targets`, synchronization writes, the managed
  `AGENTS.md` block, non-interactive refusal.
- `create-change/prose-consolidation.feature` — canonical instructions vs
  derived guidance, dialog-obligation placement, project override
  precedence, replacement preservation.
- `create-change/change-release-note.feature` — the release note in the
  proposal guidance and in the generated implementation guidance.
- `inspect-change-state/report-change-status.feature` — context-aware
  instructions and status output; `validate-change-ids.feature` — the
  `givn instructions --change` id boundary.
- `verify-change/givn-workflow-enforcement.feature` and the quality
  requirements — instruction integrity: every reference generated guidance
  carries resolves (Q-97, D-20, D-21).
- `upgrade-a-project/apply-a-managed-upgrade.feature` — the managed upgrade
  and guidance refresh; `maintain-project` holds the goal summary.
- `create-change/visual-design-contract.feature` and
  `verify-change/visual-review-gate.feature` — instruction-driven design and
  review guidance; examples of one rule split across instruction, skill, and
  command.

### Gaps

| ID | Gap |
|---|---|
| G1 | No extension mechanism: customization replaces a whole facet or patches the manifest; there is no "extend this instruction with project-specific rules". |
| G2 | Rule content lives in up to three copies (skill template, command asset, instruction asset) aligned only by discipline; nothing detects drift mechanically. |
| G3 | No direction-only delivery shape: every generated skill and command carries a full copy; there is no mode where the file carries a trigger and directs the agent to `givn instructions`. |
| G4 | No extension contract: position, validation, conflict, and failure behavior are undefined. |
| G5 | The key `rules:` is taken by the Rules policy; an OpenSpec-style artifact → rules map needs a different key, a different shape, or a scoped change to the existing key. |
| G6 | The always-available boundary is undefined: no recorded decision says what must stay resident (skill descriptions, project-start block, tone addon) and what may move behind the call. |
| G7 | The duplicate-loading suspicion is unanalyzed: no picture exists of what each harness loads when (auto-loaded skill, invoked command, always-on `AGENTS.md` block, pulled instruction) and where the same content is read twice. |
| G8 | The experiment has no defined shape or evidence: no switch, no comparison mode, no success criterion for "direction-only generated files work". |
| G9 | Self-hosting: this repository tracks its generated rules; the experiment will regenerate its own tracked guidance, so the trial's effect on tracked rule files and on the upgrade path must be decided, not incidental. |

## Open questions

See `questions.md` (Q1–Q7).
