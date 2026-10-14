# Use Case Definitions: rules-through-instructions

Cut from `eventstorming.md` (stable event IDs). Existing permanent use cases
keep their IDs, levels, and contracts; this topic defines their delta plus
the new use cases and fragments. Levels: `!` user goal (change-sized),
`+` summary, `-` subfunction (never becomes a use case), fragment =
reusable behavior with no independent user goal. Every event has exactly one
owner; "Consumes" lists inclusion or observation, not ownership. `[gated]`
marks a definition whose behavior depends on an open question; the marker
names the question. Open questions are never presented as decided.

Vocabulary: Givn's built-in, technology-agnostic rules (workflow and
guardrails) plus the project's extension form the extended rule an agent
follows (confirmed 2026-10-13). Rules are extended, never overridden; the
built-in flow is not configurable.

## Summary table

| ID | Level | Status | Actors / personas | Goal (delta in italics) | Owns events | Consumes |
|---|---|---|---|---|---|---|
| `obtain-effective-guidance` | ! | new, `[gated]` on Q4 | Authoring agent (agent--technical-role), Supporting: Developer (developer--primary-user) | Obtain the rules governing the work at hand, extended with the project's extension. *The request serves the built-in rules plus the project's extension `[Q4]`; the text and JSON paths carry the same content; an unknown id refuses by name.* | A1, A3, A4, E2, F1 | A2, B1-B3 |
| `extend-an-instruction` | ! | new, `[gated]` on Q4 | Developer (developer--primary-user, givn-maintainer--owner) | Extend an instruction's built-in rules with project-specific rules. *The extension is recorded in the extension source `[Q4]`, never inside installed files; it extends, never overrides; the next request serves the extended rule.* | B1-B3, F2, F3 | A3, A4 |
| `initialize-project` | ! | existing, modified | Developer, Supporting: Authoring agent (developer--primary-user, agent--technical-role, givn-maintainer--owner) | Set up a project and its agent integrations. *Generated guidance directs agents to `givn instructions` instead of carrying the rules `[Q1]`; full copies are reconciled to the directing form when that form is resolved `[Q1]`; the always-on set follows Q2's decision (open); the dialog obligation stays under every option.* | C1-C3 | C4 |
| `upgrade-a-project` | ! | existing, modified | Project maintainer, Supporting: Authoring agent (developer--primary-user, agent--technical-role, givn-maintainer--owner) | Cross a release gap with reviewable revisions. *The managed revision refreshes direction-only guidance `[Q1]`; extensions live outside the allowlist and are never staged, merged, or rewritten.* | E1 | C1-C3, C4 |
| `author-change` | + | existing, unchanged | Developer, Authoring agent (developer--primary-user, agent--technical-role) | Set up a Givn project and create changes with guidance. *No direct delta; the delivery delta lives in its child `initialize-project`.* | — | — |
| `maintain-project` | + | existing, unchanged | Project maintainer (developer--primary-user, givn-maintainer--owner) | Keep documentation and managed files current across releases. *No direct delta; the delta lives in `upgrade-a-project`.* | — | — |
| `create-change` | ! | existing, unchanged contract | Developer, Supporting: Authoring agent (developer--primary-user, agent--technical-role) | Create a change ready to carry from intent to executable specification. *No contract delta: its flows consume the extended rules; the invoked guidance file becomes direction-only (`initialize-project`); its capabilities' content assertions on generated guidance move to the instruction output (see Assertion rework).* | — | A1-A4, D1, D2 |
| `inspect-change-state` | ! | existing, unchanged | Developer (developer--primary-user) | Understand the current change and artifact state without reading workflow files by hand. *The session-start guidance still directs to `givn instructions`; no delta.* | — | C3 |
| `verify-change` | ! | existing, unchanged | Developer, Supporting: Reviewer (developer--primary-user) | Check a change for structural validity, completeness, and overlap. *The generated-guidance reference checks grade the direction-only files as before; no delta.* | — | C1-C3, D1 |
| `discover-specifications` | ! | existing, unchanged | Authoring agent (agent--technical-role) | Find existing behavior deterministically before authoring or reviewing a specification. *Its step guidance becomes direction-only like the other steps; no contract delta.* | — | D1 |
| fragment `instruction-integrity` | fragment | existing, modified | included by `obtain-effective-guidance`, `initialize-project`, `upgrade-a-project`, `create-change` | Resolve instructions (cascade order, override precedence, JSON shape, reference integrity). *The resolved content feeds the extended rule when an extension exists `[Q4]`; merge and delivery belong to `obtain-effective-guidance`.* | A2 | B1-B3 |
| fragment `generated-file-form` | fragment | new, `[gated]` on Q1 option 2 | included by `initialize-project` and `upgrade-a-project` | Resolve which rendering sync produces: files that direct the agent to `givn instructions`, or files that carry the rules themselves. *Provisional descriptive name, not a domain term.* | C4 | — |
| `support` — capabilities `refresh-generated-guidance`, `match-guidance-to-enabled-addons` | fragment | existing, modified | included by the setup, upgrade, and authoring flows | Render and refresh generated guidance. *Rule content leaves the generated files: content assertions move to the served instruction, and addon conditions select the instruction's content rather than the file's.* | C1-C3 | — |

Events without a single Givn owner: D1 (a workflow step was invoked) is
realized by every step flow (`create-change`, `discover-specifications`,
`verify-change`, and the supporting flows); the delivery delta inside it —
the direction-only file — is owned by `initialize-project`. D2 (a fresh
delegated context was prepared) is the existing dialog obligation carried by
the delegating flows; unchanged. F4 (generated guidance without Givn
installed) is a documented consequence; the system has no behavior and no
owner.

## `obtain-effective-guidance` (new, `[gated]` on Q4)

- Goal: obtain the rules governing the work at hand, extended with the
  project's extension.
- Level: `!`; Actors: Authoring agent (primary), Supporting: Developer.
- Precondition: an initialized Givn project; for an artifact instruction, a
  resolvable change context.
- Boundary: serving the effective content for one instruction id. It owns
  the request path (A1), the extension merge (A3), the delivery parity (A4),
  the post-upgrade freshness (E2), and the unknown-id refusal (F1). The
  cascade resolution mechanics stay the `instruction-integrity` fragment's
  capability (A2); the content of the built-in rules stays with the
  canonical assets; the extension content stays with `extend-an-instruction`.
- Behavior to elaborate:
  - The id resolves as an artifact, a policy, or a step `[Q5]`; the change
    context is explicit or the single active change, otherwise a refusal
    naming the boundary (A1).
  - The built-in rules resolve through the cascade — project replacement
    wins entirely, then embedded, then addon (A2, unchanged).
  - The project's extension for the id is added to the built-in rules in
    the position Q4 decides (open) under a marked section; the built-in
    rules stay unmodified; the extension never overrides, suppresses, or
    reorders them (A3; Q4 open, Q6 closed).
  - The text and JSON paths carry the same effective content, extensions
    included (A4).
  - Improved built-in rules are served at the next request after an upgrade,
    with no project action (E2).
  - An unknown id refuses with the known list (F1, unchanged).
- Stakeholders: agent--technical-role (deterministic, discoverable rules),
  developer--primary-user (reads manually).

## `extend-an-instruction` (new, `[gated]` on Q4)

- Goal: extend an instruction's built-in rules with project-specific rules.
- Level: `!`; Actors: Developer.
- Precondition: an initialized Givn project.
- Boundary: recording, changing, and removing the project's extensions and
  their rejection behavior. Q4 decides the extension source and shape
  (open: config map, extension file, or both); the request-time merge is
  `obtain-effective-guidance`; the whole-facet replacement mechanism is
  untouched; the built-in flow is out of scope (no custom workflow).
- Behavior to elaborate:
  - The extension is recorded in the extension source Q4 decides (open); it
    never enters generated files or the managed revision (B1-B3).
  - An extension adds project-specific rules; it never changes steps,
    order, or gates, and never overrides the built-in rules (Q6 closed).
  - An unknown artifact id or a malformed entry is rejected per Q4's
    decision (open) — warn and drop, or a named refusal (F2).
  - A textual contradiction between the extension and a built-in rule is
    visible in the served content; no semantic conflict detection exists
    (F3; Q4).
  - Removing an extension leaves the built-in rules standing alone; nothing
    else needs cleanup (B3).
- Stakeholders: developer--primary-user (guidance that fits the project),
  givn-maintainer--owner (truthful specs and an upgrade path that never
  fights project configuration).

## `initialize-project` (existing, modified)

- Goal unchanged; the generated guidance changes form.
- Capability status: `initialize-a-project` modified (directing rendering
  `[Q1]`, reconciliation); `enable-optional-addons` possibly modified only
  through the always-on set `[Q2]`.
- Changed observable behavior:
  - `givn init` and `givn skills sync` render each selected target's skill
    and command as trigger metadata plus a direction to
    `givn instructions <id>`, where a served instruction exists
    `[Q1][Q5]` (C1). Which form is rendered is Q1's answer (open).
  - When the resolved form is the directing form `[Q1]`, a full copy
    installed by an earlier version is rewritten to it; a disabled addon's
    files are removed; a project-owned replacement stays byte-for-byte and
    receives no injected text (C2; R-74).
  - The always-on `givn-project` block keeps its direction to
    `givn instructions` and its dialog-policy obligation under every Q2
    option. Whether it also keeps or generates the resident task-checkoff
    and commit-message obligations is Q2's answer (open; C3).
  - If the experiment adds the generated-file setting, init and sync render
    per the resolved form; an absent key defaults to today's full-copy
    form, and an unrecognized value warns and continues with the default
    (C4; `generated-file-form` fragment, gated).
- Boundary: setup and local installation of generated guidance. The
  request path is `obtain-effective-guidance`; the upgrade's managed
  revision is `upgrade-a-project`.
- Stakeholders: developer--primary-user (a repository and installation
  without duplicated rules), agent--technical-role (files that reliably
  lead to the rules), givn-maintainer--owner (one canonical asset per
  rule).

## `upgrade-a-project` (existing, modified)

- Goal unchanged; the managed revision refreshes the resolved guidance
  form.
- Capability status: `apply-a-managed-upgrade` modified (allowlist and
  revision content).
- Changed observable behavior:
  - The managed revision refreshes the scaffold and the installed
    direction-only guidance `[Q1]`; when the directing form is resolved, a
    full copy from an earlier version is rewritten (E1; C1-C2).
  - The project's extensions live outside the managed allowlist and are
    never staged, merged, or rewritten (E1).
  - A project-owned replacement stays authoritative (R-74, unchanged).
  - Improved built-in rules need no revision at all: they are served at the
    next request (E2).
- Boundary: crossing a release gap with reviewable revisions. Rendering is
  `initialize-project`; serving is `obtain-effective-guidance`.
- Stakeholders: developer--primary-user (extensions survive every upgrade),
  agent--technical-role (deterministic revision content),
  givn-maintainer--owner (specs and migration path truthful).

## `author-change` and `maintain-project` (summaries, unchanged)

- Both are `+` summaries whose children carry this topic's delta;
  `author-change` covers `initialize-project` (modified),
  `maintain-project` covers `upgrade-a-project` (modified). Their own
  contracts and coverage rules are untouched.

## `create-change`, `inspect-change-state`, `verify-change`, `discover-specifications` (existing, unchanged contracts)

- `create-change`: no contract delta. Its authoring flows read the extended
  rules through `obtain-effective-guidance`; the invoked guidance file is
  direction-only, which `initialize-project` owns; the delegation
  obligation (D2) is carried as today. Its capabilities' content assertions
  on generated guidance are reworked (see below).
- `inspect-change-state`: no delta; the session-start guidance still directs
  to `givn instructions` (C3).
- `verify-change`: no delta; the reference checks grade whatever guidance is
  installed (C1-C3).
- `discover-specifications`: no contract delta; its step guidance becomes
  direction-only like the other steps (D1).

## Fragment `instruction-integrity` (existing, modified)

- No independent user goal; included by the guidance, setup, upgrade, and
  authoring flows.
- Capability: `instruction-integrity` — modified.
- Behavior to elaborate: the resolution cascade, override precedence, JSON
  shape (`id`, `generates`, `requires`, `instruction`, `skill_to_load`), and
  reference integrity are unchanged; the resolved content served to the
  caller is the extended rule when an extension exists `[Q4]` (A2;
  cross-reference `obtain-effective-guidance` A3-A4 for merge and delivery,
  which belong to that use case, not this fragment).
- Mapping note: the permanent `fragment.md` interaction row
  "instruction-integrity | request effective instructions by id" moves under
  `obtain-effective-guidance`; the capability remains for resolution and
  machine-contract mechanics.
- Boundary: resolution and machine contract mechanics. Content ownership
  stays with the canonical assets and `extend-an-instruction`.
- Stakeholders: agent--technical-role, givn-maintainer--owner.

## Fragment `generated-file-form` (provisional name; new, `[gated]` on Q1 option 2)

- No independent user goal; included by `initialize-project` and
  `upgrade-a-project` when the experiment adds the setting.
- The name is descriptive and provisional — not a domain term; the term
  record rejects shape coins, and the setting's user-facing name is Q1's
  and the proposal's to decide. The fragment exists only under Q1 option 2.
- Behavior to elaborate:
  - The resolved value selects whether sync renders a direction to
    `givn instructions` or a full copy (C4).
  - An absent key resolves to today's form; an unrecognized value warns and
    continues with the default (C4).
  - The value never changes the workflow, the selected targets, or the
    installed file set (C4).
- Boundary: one resolved value consumed by the writers of generated
  guidance. It writes nothing itself.
- Stakeholders: developer--primary-user, givn-maintainer--owner (if the
  setting exists at all, it is a product surface to keep honest).

## Assertion rework in existing capabilities

The delivery change moves rule content out of generated files, so existing
scenarios that assert rule content inside a generated skill or command must
move their assertion to the served instruction (or establish the
direction-plus-orchestration split). Named surfaces:

- `support` — `refresh-generated-guidance`: "the generated givn-review
  guidance contains *Overlap dispositions*", "the generated givn-design
  guidance contains *Black-Box-First*", and similar content assertions move
  to `givn instructions review` / `givn instructions design` output.
- `support` — `match-guidance-to-enabled-addons`: addon-conditional content
  ("the generated review skill contains the Arc42 implementation
  conformance section") becomes a condition on the served instruction's
  content; the generated file keeps trigger metadata and orchestration.
- `support` — `instruction-integrity`: the "synchronized agent guidance for
  the specs workflow …" content assertions move to the specs instruction
  output.
- `support` — `generate-ideation-guidance`: its content assertions are
  Q5-dependent — the ideation step either gains an instruction id (the
  assertions move to the served instruction) or keeps resident content
  (the assertions stay on the file), per Q5's answer.
- `create-change` — `prose-consolidation` ("the generated givn-propose
  command contains *proposed by*") and `change-release-note` move their
  content assertions to the instruction output.
- `create-change` — `visual-design-contract`: "the generated
  givn-design-review guidance contains *Visual Design Contract*", "the
  generated givn-steps guidance contains *evidence/visual*", and "the
  generated givn-design guidance contains *renders HTML, CSS, or
  JavaScript in a browser*" move to the design, design-review, and steps
  instruction output.
- `verify-change` — `visual-review-gate`: "the generated givn-review
  guidance contains *VISUAL-REVIEW: PASS*" and its neighbouring assertions
  move to the review instruction output.
- `initialize-project` — `initialize-a-project` content assertions that
  read rules from generated guidance (for example the commit-path statement)
  move likewise; its target-selection, reporting, and block assertions stay.
- `verify-change` — the generated-guidance reference checks stay as they
  are: they grade references, not content.

This rework is part of the delivery change's scope; the affected permanent
scenarios are amended by the implementing change, not by this ideation.

## Rejected candidates

| Candidate | Level proposed | Why rejected |
|---|---|---|
| `configure-the-extension` as a use case | - | Recording an extension is the developer's goal itself (`extend-an-instruction`); splitting configuration from the goal adds a use case without independent value. |
| `select-the-generated-file-form` as a use case | - | A configuration value consumed by setup and upgrade; the developer's goal is setup or refresh, not selecting an internal contract (same reasoning as `configure-rules-policy` in the local-only topic). |
| `deliver-guidance` as a use case | ! | Component/channel-shaped name and no independent goal; delivery is the observable outcome inside `obtain-effective-guidance`. |
| `merge-the-extension` as a use case | - | A step inside `obtain-effective-guidance`'s flow (A3); no independent goal. |
| `customize-the-workflow` as a use case | - | Out of scope by user direction: Givn has no custom workflow configuration and the built-in flow is not replaced; the extension adds guidance only. |
| Keeping instruction delivery as only the `instruction-integrity` capability | fragment | The delivery change makes obtaining the extended rules the primary path with its own actor goal; keeping it capability-only would hide the agent's goal. The capability remains for resolution mechanics within the new use case. |
