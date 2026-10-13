# Questions: rules-through-instructions

Every parked unknown. One entry per question; never debate an opinion in
place of a missing answer. Status is `open` or `closed <date> (<decider>)`.

## Q1 — How is the experiment run and compared?

- Status: closed 2026-10-13 (user)
- Blocks: the shape of every change seed (whether Givn gains a delivery
  setting or only changes what `givn skills sync` writes) and the evidence
  the trial must produce.
- Context: the user wants to see how Givn works when all rules and
  instructions are output of `givn instructions` and the generated rules
  only direct the agent there. Two generated-file forms can coexist in one
  checkout only if something selects between them; without a selector the
  trial is the branch itself. The answer decides whether the experiment adds
  a product surface (a config key) or stays a branch-only trial with usage
  as its evidence, and how a regression is isolated: by switching forms and
  diffing, or by checking out the base revision. Givn deliberately does not
  offer configurable workflows, but a generated-file form is not workflow
  configuration — the minimal-surface direction still argues for the
  smallest change that answers the question.
- Question: should the experiment change the generated output directly on
  the branch, or add a setting that selects the generated-file form — a
  file that directs the agent to `givn instructions`, versus a file that
  carries the rules itself?
- Options:
  1. **Branch-only delivery change.**
     Meaning: sync renders every generated skill and command as a
     trigger plus a direction to `givn instructions`; no new
     configuration.
     In practice: on this branch, `givn skills sync` rewrites
     `.opencode/skills/givn-propose/SKILL.md` to the short directing form;
     the previous full copy is seen by checking out the base revision.
     Gain: smallest change; no configuration surface that could outlive the
     experiment.
     Cost: no side-by-side operation; the evidence is "the branch was used
     for a while", and a behavioral regression is bisected by checkout.
  2. **Selectable generated-file form.**
     Meaning: a `givn/config.yaml` key selects what sync writes — the
     directing form or the full-copy form.
     In practice: synchronize here in the directing form, keep a second
     project on the full copy, and compare `git diff` and agent behavior
     directly.
     Gain: A/B comparison in one checkout; a gradual-adoption path if the
     experiment succeeds.
     Cost: a new product surface to specify, test, document, and possibly
     deprecate again.
  3. **Staged: branch-only first, setting only if the trial needs it.**
     Meaning: start with option 1; add the setting only when a concrete
     need appears (for example isolating an agent-behavior difference).
     In practice: the first two weeks run the branch; if a comparison is
     needed, the setting arrives as a follow-up seed.
     Gain: no premature configuration key; comparison cost paid only when
     wanted.
     Cost: early feedback is coarse; the setting, if needed later, starts
     from zero.
- Resolution: **branch-only** because the trial does not need a second
  configuration surface to answer "do directions work", and the smallest
  change is the most honest experiment. Rejected: the selectable setting in
  this change (a product surface to specify, test, document, and possibly
  deprecate for a need the trial has not shown); the staged variant
  collapses into the chosen option — if the trial shows the need, the
  setting returns as a follow-up seed.

## Q2 — Which rules stay always available?

- Status: open
- Blocks: what the generated files keep carrying; shapes the generated-rule
  contract and the use cases.
- Context: the user noted it may be good that some rules are always
  available. Today the always-on surface is the `givn-project` block in
  `AGENTS.md` — a direction to `givn instructions` that also carries the
  resident task-checkoff and commit-message obligations — the tone block
  when the `absolute-mode` addon is enabled, and the skill frontmatter
  descriptions (trigger metadata the harness reads before any content is
  loaded). The block additionally carries the dialog-policy obligation — a
  direction to `givn instructions dialog`, required independently of
  optional addons and covered by a permanent rule and an end-to-end
  scenario; it stays under every option and is not part of this choice.
  Everything else is loaded when a skill triggers or an instruction is
  pulled. The `givn-project` block's resident obligations restate what
  `assets/instructions/tasks.md` owns, so they are an overlap site of
  exactly the kind this experiment removes.
- Question: which rules must remain always available in a project, and
  which may move fully behind `givn instructions`?
- Options:
  1. **Keep today's always-on set.**
     Meaning: the project-start block as it is today — the direction, the
     dialog obligation, and the resident task-checkoff and commit-message
     obligations — the tone block when enabled, and skill descriptions.
     In practice: a fresh agent reads the commit rule at session start but
     learns the proposal rules only when it pulls them.
     Gain: smallest change; the safety-critical commit rule survives an
     agent that never calls Givn.
     Cost: the commit rule keeps two homes (the block and the `tasks`
     instruction), so its drift risk survives the experiment.
  2. **Generate the resident obligations from the canonical source.**
     Meaning: the block's rule text is rendered at sync time from the same
     asset that `givn instructions` serves; the resident set may grow by
     confirmation; the dialog obligation stays as today.
     In practice: the block shows the commit rule, and the block's text and
     the `tasks` instruction can never disagree.
     Gain: one source even for the always-on rules; drift becomes
     impossible for them too.
     Cost: sync must render `AGENTS.md` rule content — a new rendering
     surface to specify, test, and keep honest; the project-start block
     becomes a computed artifact.
  3. **Directions only.**
     Meaning: the block keeps only the directions — `givn instructions`
     and the dialog obligation — and drops the resident task-checkoff and
     commit-message text; every other rule arrives through the call.
     In practice: an agent that never pulls the implementation guidance
     has no commit-message rule.
     Gain: one home for every rule; the always-on surface is minimal.
     Cost: removes today's safety net — an agent that misses the direction
     commits wrongly with no resident correction.

## Q3 — One change or two: delivery and extensions?

- Status: closed 2026-10-13 (user)
- Blocks: seed cutting at handoff (a change whose guarantee is true end to
  end and ships alone) and the use-case boundaries.
- Context: the requested experiment bundles two separable outcomes: (a) all
  rules and instructions delivered through `givn instructions`, generated
  rules as directions; (b) extensions — project-specific rules that extend
  an instruction's built-in rules. Outcome (a) removes the duplication by
  itself; outcome (b) removes the customization conflict. OpenSpec ships
  the equivalent of both in one configuration story, but Givn's constraints
  differ (the `rules:` key is taken by the rules policy; the built-in flow
  is not configurable).
- Question: should the delivery change and the extension mechanism be one
  change or two sequential changes?
- Options:
  1. **One change.**
     Meaning: one seed covering delivery and extensions together.
     In practice: one proposal, one branch/PR, one archive; the trial
     exercises both halves at once.
     Gain: one coherent experiment; no intermediate state where
     directions exist but extensions still require whole-facet
     replacements.
     Cost: harder to attribute a regression to one half; the archive
     guarantee covers both.
  2. **Two changes.**
     Meaning: a delivery change first, an extension change second.
     In practice: the first seed rewrites generated guidance as directions
     and is archived alone; the second adds the extension mechanism on top.
     Gain: each guarantee ships alone; the riskier delivery change is
     isolated and can be abandoned without leftover configuration surface.
     Cost: two review cycles; the second depends on the first, so the order
     is fixed.
- Resolution: **one change on the experiment branch** because the trial
  must exercise the whole story — directions plus the customization answer
  — in one coherent experiment; splitting it would leave an intermediate
  state where the delivery has landed but the customization story has not.
  Rejected: the two-change split (isolated experiment, but two review
  cycles and a half-realized `obtain-effective-guidance` contract across
  changes); deciding at handoff (an extra round-trip for a cut that was
  already clear).

## Q4 — Where does an extension live, and how does it extend a Givn rule?

- Status: open
- Blocks: the use-case rules and guarantees for extending an instruction;
  the key name and shape.
- Context: OpenSpec puts extra rules in its project configuration under
  `rules:` — "extra rules added to one artifact's built-in guidance". In
  Givn, the key `rules:` already means the generated-rules policy
  (`local-only`/`versioned`, glossary: Rules policy), so the same name
  cannot carry a second meaning. And unlike OpenSpec, Givn does not aim at
  custom workflows: an extension adds project-specific guidance to a
  built-in instruction; it never changes steps, order, or gates. The
  location and shape decide what a project owner edits, what the extension
  can express (one-line rules vs prose), and how the extended rule is
  separated from the built-in text (position, heading, validation, failure
  mode on an unknown artifact id).
- Question: where should an extension live and how should it extend an
  instruction's built-in rules?
- Options:
  1. **New config-map key in `givn/config.yaml`.**
     Meaning: a key (name open, e.g. `extensions:`) mapping artifact id →
     list of rule strings, appended to the instruction output under a
     marked section.
     In practice: `extensions: {proposal: ["Keep proposals under 500
     words"]}` and `givn instructions proposal` prints the built-in rules
     followed by a marked section with that line.
     Gain: one file, mirrors the OpenSpec model the user referenced; short
     rules stay one line each.
     Cost: multi-line prose is awkward in a YAML list; the new key needs a
     name that cannot collide with `rules:` (or with the use-case
     contract's `Extensions` vocabulary).
  2. **Extension file per artifact.**
     Meaning: e.g. `givn/artifacts/<id>/extension.md`, appended to the
     resolved instruction, sibling to the existing whole-facet replacement
     (`instructions.md` stays the replace mode).
     In practice: `givn/artifacts/proposal/extension.md` holds the
     project's rules as Markdown; `givn instructions proposal` prints the
     built-in rules plus that file.
     Gain: prose-capable; sits in the existing cascade location; diffs
     review like normal project content.
     Cost: a second mechanism on top of the config overlay; more discovery
     for the project owner.
  3. **Both: config list for short rules, extension file for prose.**
     Meaning: the config map covers one-line rules; the file covers longer
     project guidance.
     In practice: small conventions go to the config, a project-specific
     checklist goes to the file.
     Gain: covers both granularities with fitting homes.
     Cost: two mechanisms to specify, test, document, and explain — the
     larger the surface, the larger the experiment.

## Q5 — Which steps receive an instruction id?

- Status: open
- Blocks: the generated-rule contract for every skill and command; the
  use-case boundaries for step delivery.
- Context: `givn instructions <id>` serves artifact instructions (proposal,
  specs, design, design-review, tasks, review, arc42-docs, …) and policy
  instructions (`dialog`). Several generated skills and commands have no
  artifact or policy id: `givn-explore`, `givn-ideation`, `givn-steps`,
  `givn-reverse-engineer`, `givn-characterize`, and the addon commands
  `grill-me` and `grill-with-docs`. If every generated file must direct the
  agent to a served instruction, these need ids; if only artifact-aligned
  steps move behind the call, the others keep resident content and the
  single-source goal stays partial. The answer shapes the use cases and the
  asset layout.
- Question: which steps receive an instruction id served by
  `givn instructions`, and which keep resident content in their generated
  files?
- Options:
  1. **Every generated skill and command gets an instruction id.**
     Meaning: a new step-asset family joins artifacts and policies in the
     cascade.
     In practice: `givn instructions ideation` prints the ideation guidance;
     the skill file directs to it.
     Gain: complete single-source coverage; one mechanism for all rules.
     Cost: a second asset family to name, maintain, and version; the
     instruction id namespace grows beyond artifacts.
  2. **Only artifact- and policy-aligned steps move behind the call.**
     Meaning: proposal, specs, design, design-review, tasks, review, and
     dialog resolve through `givn instructions`; explore, ideation, steps,
     reverse-engineer, characterize, and the grill commands keep resident
     files.
     In practice: `/givn-propose` directs to the instruction; `/givn-ideation`
     still carries its content.
     Gain: smaller change; the highest-traffic path (the artifact chain)
     becomes single-source.
     Cost: duplication persists for the non-artifact steps; two mental
     models coexist.
  3. **Staged: artifact chain now, per-step coverage later.**
     Meaning: cover the artifact and policy ids in the experiment; record
     the remaining steps as a named follow-up with its own seeds.
     In practice: the trial proves the directing form on the chain, and the
     coverage question is answered per step with usage evidence.
     Gain: no premature asset family; evidence-driven coverage.
     Cost: the single-source claim stays partial until the follow-up lands.

## Q6 — Whole-facet override versus extension

- Status: closed 2026-10-13 (user)
- Blocks: the extension contract's interaction with the existing
  whole-facet replacement mechanism.
- Context: the cascade lets a project own a whole-facet replacement
  (`givn/artifacts/<id>/instructions.md`). If the project also records an
  extension for the same id, the two mechanisms meet. The question presumed
  an override-style rules mechanism whose merge behavior had to be decided.
- Question: when a project owns a whole-facet override and records an
  extension for the same id, does the override replace the canonical part
  only (the extension still merges) or the merged result (the extension has
  no effect)?
- Options:
  1. **The extension merges on top of the override.**
     Meaning: the override replaces the canonical asset; the extension is
     still appended.
     In practice: an overriding project sees its replacement text followed
     by its marked extension section.
     Gain: one predictable place for small rules even when a file is
     overridden.
     Cost: "override = full control" stops being literally true; the
     project must know the extension still appends.
  2. **The override suppresses the extension.**
     Meaning: a whole-facet replacement is full control; an extension for
     that id has no visible effect.
     In practice: recording an extension for an overridden id changes
     nothing, and `givn instructions` reports that it is shadowed.
     Gain: one authority model for replacements (R-74), simple to explain.
     Cost: two customization paths cancel each other; the project must move
     its rules into the replacement file.
  3. **Givn refuses the combination by name.**
     Meaning: a whole-facet replacement plus an extension for the same id
     is rejected.
     In practice: the request fails naming both sources and the resolution.
     Gain: no silent cancellation, no ambiguity.
     Cost: refuses a configuration that could be intended (organization
     replacement plus team extensions).
- Resolution: **the extension adds to the built-in rules; rules are
  extended, never overridden** (user, 2026-10-13). The question's premise —
  an override-style rules mechanism whose merge had to be decided — is
  ruled out by that direction; the extension semantics are additive only.
  Rejected: all three options as framed, because they presume that an
  override path for rules exists; the existing whole-facet asset
  replacement stays a separate ownership mechanism, and the built-in Givn
  flow itself is not configurable or replaceable (Givn is the harness).

## Q7 — Does an extension apply on top of a whole-facet replacement?

- Status: open
- Blocks: one rule and one guarantee in `obtain-effective-guidance`; the
  interplay between the additive extension and the existing replacement
  mechanism.
- Context: the cascade lets a project own
  `givn/artifacts/<id>/instructions.md`, replacing the built-in
  instruction entirely (unchanged mechanism; R-74 keeps it authoritative
  and untouched). The extension mechanism is additive by direction. When
  both exist for one id, whether the extension still applies is not settled
  by the direction alone; Q6 ruled out override semantics for rules, not
  the replacement interplay.
- Question: when a project owns a whole-facet replacement for an id and
  also records an extension for it, does the extension apply on top of the
  replacement?
- Options:
  1. **The extension applies on top of the replacement.**
     Meaning: the replacement is the base the extension extends.
     In practice: the request serves the project's replacement text
     followed by the marked extension section.
     Gain: one consistent rule — the extension always extends the effective
     base; no configuration is silently shadowed.
     Cost: a project that replaced the file must know the extension still
     appends.
  2. **The replacement stands alone; the extension is shadowed.**
     Meaning: a whole-facet replacement is full control.
     In practice: recording an extension for a replaced id changes nothing,
     and the request reports that it is shadowed.
     Gain: "replacement = full control" stays literal (R-74).
     Cost: two customization paths cancel each other unless reported; the
     project must move its rules into the replacement.
  3. **Givn refuses the combination by name.**
     Meaning: a replacement plus an extension for the same id is rejected.
     In practice: the request fails naming both sources and the resolution.
     Gain: no ambiguity at all.
     Cost: refuses a configuration that could be intended.
