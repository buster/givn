# Proposal: split-stack-instructions

## Release note

Every core step instruction now names the stack instruction for the project's
runner and carries only the universal rules, while the per-stack mechanics are
served on request by id and the shipped guidance corpus stays within its
recorded line budget.

## Use-Case Context

- Use-case IDs: `obtain-effective-guidance` (`serve-extended-rules`) and the
  `support` fragment (`instruction-integrity`)
- Ideation topic: `ise-rebuildability`
- Seed path: `givn/ideation/ise-rebuildability/changes/split-stack-instructions.md`
- Confirmed Personas: `agent--technical-role`, `givn-maintainer--owner`,
  `developer--primary-user`

## Review Context

- Review kind: independent
- Reason: major change. It restructures the guidance every consumer of Givn
  reads, moves the per-stack mechanics out of the four core step instructions
  that the lifecycle skills direct authors to, adds the instruction ids a
  project may request, and turns the `ise-rebuildability` plan's `G2` gate into
  a permanent budget check.

## Problem / Opportunity

The four core step instructions carry per-stack mechanics inline: the step
definition path, the binding form, and the empty-body stub for each supported
runner in `steps`, and the runner's strict-mode mechanism and single-scenario
command in `design`. An author writing a Rust change reads the Python, Java,
Node, Go, and Ruby rows as well; an author on any stack reads every runner's
strict-mode row. The measured corpus is already over the budget the
`ise-rebuildability` plan set: `assets/instructions/` holds 22 files and 4,528
lines against a 4,500-line cap, because the three predecessor changes of the
`ise` track added 131 lines to a 4,397-line baseline. The four core step
instructions total 1,630 lines (373 `specs`, 261 `steps`, 386 `design`, 610
`review`).

Impact: the plan's `G2` gate cannot pass by adding guidance; every further
`ise` phase would push the corpus further over the cap that exists to prevent
the `v0.9.15` collapse, in which added defensive prose cut Catopia scenario
volume from 130 to 76 while the hidden grade stayed at 25/25. A project on one
stack also pays for every other stack's lookup tables on every request, and
nothing detects a core instruction that names an instruction id nobody serves.

## Proposed Solution

Observable behaviour, from the authoring agent's and the maintainer's side:

- **The core step instruction names what it does not carry.** The served text
  of `steps` and `design` names the stack instruction id for the project's
  runner with the condition for loading it, and carries no stack body: no
  per-runner step-definition path table, no per-language empty-body stub table,
  per-runner strict-mode table, and no per-stack single-scenario command
  list. `givn instructions specs` and `givn instructions review` serve no stack
  body either and therefore name no stack instruction; `givn instructions
  tasks` and `givn instructions tasks-reverse` point at `steps` for the mapping
  and carry no stack body.
- **A stack instruction is served only on its own request.** The mechanics
  named by the core instruction arrive only when the caller requests that id:
  the Rust instruction carries the Rust step file, binding form, empty-body
  stub, strict-mode mechanism, and single-scenario command. A core request never
  includes the stack body.
- **An unknown stack id is refused by name.** A request for an id that is not
  served keeps the existing refusal and lists the known ids, including the
  stack instruction ids.
- **The ise instruction's directions resolve.** The served `ise` instruction
  names `steps` as the mapping for the change's stack mechanics and one stack
  instruction id as a worked example, plus the two record owners; every id it
  names is served while `ise` is effective.
- **A dangling id is a named defect.** The guidance reference check resolves
  every instruction id a served instruction names, and fails naming the
  unresolved id. A core instruction that names `steps-rust` while no such
  instruction resolves is a failure, not a silent gap.
- **The corpus stays within its recorded budget.** The shipped instruction
  corpus stays at or below 4,500 lines with the stack instructions counted, and
  the four core step instructions stay at or below the post-split total recorded
  by this change. Both numbers are permanent checks and both are recorded.

## Capability Routing

Route reports one recommendation: `EXTEND serve-extended-rules (use case
'obtain-effective-guidance', score 24.50) — confidence: signal`. It ranked no
candidate for the reference-integrity capability. The table records the
recommendation where it applies and records `no signal` where it does not; the
decision column records the capability placement the use-case contracts and the
seed assign.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `serve-extended-rules` | `EXTEND serve-extended-rules` (24.50) | `EXTEND serve-extended-rules` | Route agrees. Serving a stack instruction by id is the same request action with another id, and the seed assigns the split's serving behaviour, the named ids, and the text/JSON equality to `obtain-effective-guidance`. |
| `instruction-integrity` | `no signal` | `EXTEND instruction-integrity` | Resolution and reference integrity stay with the `support` fragment: the dangling id, the `ise` directions, and the corpus budget are checks over served guidance, not new serving behaviour. Route ranked no candidate because the proposal's checks name no command surface. |
| `adr-qualification-gate` | `no signal` | `EXTEND adr-qualification-gate` | The change mints `ADR-077`, whose accepted record changes the count the permanent register scenario pins; the scenario is `@givn.modified` with the new count and no behavior of the capability changes. Route ranked no candidate because the proposal's subject is guidance composition, not ADR qualification. |

## Out of Scope

- Enabling `ise` on this repository, moving Givn's own `@e2e` steps, and the
  self-rebuild (`enable-ise-on-givn-and-run-a-dry-run`).
- The `givn ise replace` and `givn ise verify` commands
  (`add-ise-clean-room-commands`).
- Any change to the `@e2e` policy: the package boundary, the real-interface
  rules, the interaction inventory, and the visual evidence rules keep their
  current wording and their pinned scenarios.
- Addon-bound serving beyond `ise`: no other addon instruction gains a gate.
- The stack bodies' content: the split moves mechanics; it does not add a
  runner, a language, or a step convention, and it does not change the Rust
  archive binding tokens.
- A per-project budget: the cap describes Givn's shipped corpus, not a consumer
  project's overrides.

## Open Questions

### D1 — Which stacks receive an instruction in the first cut

- Question: The four core step instructions name six runners today
  (`cucumber-rs`, `cucumber-js`, `cucumber-jvm`, `behave`, `godog`,
  `cucumber-ruby`). Which of them receive a stack instruction in this change,
  and what happens to the rows of the ones that do not? The answer fixes which
  ids the core instructions name, which scenarios can assert, and how the
  4,500-line corpus cap is reached.
- Options:
  1. **Rust only** — `givn instructions steps-rust` serves the Rust mechanics
     and the other five runners' rows stay in `steps` and `design`; gain: the
     smallest cut and the clearest proof of the rule; cost: the core
     instructions still carry five stacks' bodies, so the split's rule ("the
     core instruction carries no stack body") is false for them, and a Python
     author still reads every runner's row; in practice: `givn instructions
     steps` names `steps-rust` and still shows the Java stub table.
  2. **Every runner the core instructions name** — `steps-rust`,
     `steps-python`, `steps-node`, `steps-java`, `steps-go`, and `steps-ruby`
     each hold their own four to five facts, and the core instructions keep the
     universal rule plus the id and load condition; gain: no rule is removed,
     the core carries no stack body, and every stack's author loads one
     instruction; cost: six new small files, and the corpus must shrink about
     90 lines in this change to stay under the cap; in practice: `givn
     instructions steps` names all six ids, and a Go change loads
     `steps-go`.
  3. **Every stack the outline supports** — Rust, Python, Java, TypeScript, and
     JavaScript receive instructions; gain: one uniform set with the outline's
     language list; cost: the Go and Ruby rows have no home and are deleted,
     which removes their stub and runner mechanics, and the outline's
     TypeScript/JavaScript pair splits one `cucumber-js` runner across two
     instructions; in practice: a Ruby project's stub form is no longer served
     anywhere.
- Disposition: carried — the design's Decisions ledger decides it; the deciding
  artifact is `design.md`, and the recorded ids and load conditions are the
  contract the specs and the reference check use.

### D2 — Where the durable budget check lives and what it counts

- Question: The plan's `G2` gate measures both `wc -l` on the four core step
  instructions and `wc -l assets/instructions/*.md`. Is the budget a permanent
  mechanical check over the shipped assets, and does the 4,500-line total count
  the stack instructions or exclude them? The answer fixes whether a later
  change can add corpus lines while passing, and whether this change must
  delete guidance or may only move it.
- Options:
  1. **Cap counted with the stack instructions, checked permanently** — the
     total over every file in `assets/instructions/` stays at or below 4,500 and
     the four core files stay at or below their post-split total, both checked
     by a scenario; gain: the cap is the plan's own gate and cannot be gamed by
     moving text, and the four core files cannot regrow to where they were;
     cost: this change must delete about 90 lines of duplicated guidance to fit,
     so the split is a consolidation as well as an extraction; in practice: a
     later change adding 40 lines to `review` fails the budget scenario even if
     it moves 40 lines into a stack instruction.
  2. **Cap counted without the stack instructions** — the 4,500-line total
     excludes the stack instruction files, which are reported separately;
     gain: the extraction alone satisfies the budget and no existing guidance
     must be removed; cost: total served text can grow without limit by
     extraction, which is the failure the gate exists to prevent; in practice:
     moving 40 lines into a new stack instruction keeps the cap satisfied while
     a consumer on that stack still loads the same text.
  3. **Recorded measurement only** — the before and after counts are recorded in
     the change's evidence and no permanent check exists; gain: nothing to
     maintain in the test suite; cost: the gate stops gating at archive, and the
     next phase's growth is detected by nobody; in practice: `assets/
     instructions/` grows past 4,500 again with no failing check.
- Disposition: carried — the design's Decisions ledger decides it; the deciding
  artifact is `design.md`, whose budget section records the exact counts and the
  check the specs pin.

### D3 — Who owns the runner mechanics the design instruction serves

- Question: `design` serves the runner's strict-mode mechanism and the
  single-scenario command. Does a stack instruction pair with the core
  instruction that used to carry the body (`design` to `design-rust`), or does
  each stack get one instruction that both `steps` and `design` name? The answer
  fixes the id set and how many files the corpus pays for.
- Options:
  1. **One instruction per stack, named by both core files** — `steps-rust`
     holds the Rust step and runner mechanics, and `design` names it with the
     load condition "the change's runner is Rust"; gain: six files instead of
     twelve, one place per stack, and the runner rows keep one home; cost: the
     id convention `<id>-<stack>` pairs the file with `steps` while `design`
     also names it, so the pairing rule needs the stated exception; in
     practice: a Rust change reads `steps` and `design`, then loads `steps-rust`
     once for both the step file and the strict-mode mechanism.
  2. **One instruction per core instruction and stack** — `steps-rust` and
     `design-rust` and their five counterparts, twelve files; gain: the pairing
     rule stays literal and each core instruction owns its own payload; cost:
     twelve files of three to five lines each, with about 60 lines of file
     headers and scope sentences, which the 4,500-line cap cannot absorb; in
     practice: a Rust change loads two instructions whose content a reader could
     hold in one.
  3. **Runner mechanics stay in `design`, only step mechanics split** — gain:
     the pairing rule is untouched and fewer files exist; cost: `design` keeps
     the six-runner strict-mode table, so the core instruction still carries a
     stack body and D1's rule is only half true; in practice: `givn instructions
     design` still shows the Java and Go strict-mode rows to a Rust change.
- Disposition: carried — the design's Decisions ledger decides it; the deciding
  artifact is `design.md`, and the resulting id set is what the reference check
  resolves.

### Seed questions carried into this change

- **`Q15` (the name for an on-demand language or runner instruction)**:
  answered — the topic record rejects a new noun and keeps `Instruction` plus
  the id convention (`<id>-examples`, `<id>-<stack>`); source
  `givn/ideation/ise-rebuildability/questions.md`, `Q15` Resolution, and the
  seed's accepted decision.
- **Which stack instructions ship in the first split**: carried — it is `D1`
  above.
- **Whether the `ise` instruction belongs to the addon seed or to this split**:
  answered — it belongs to `add-ise-addon-and-specification`, which shipped
  `assets/instructions/ise.md` and its serving gate; source the archived change
  and the permanent `serve-extended-rules` scenarios. This change only checks
  that the directions it serves resolve.
