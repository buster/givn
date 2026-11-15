# Proposal: split-stack-instructions

## Release note

Every core step instruction now names the stack instruction for the project's
runner and carries only the universal rules, while the per-stack mechanics are
served on request by id.

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
  project may request, and makes the composition rule a permanent text
  assertion.

## Problem / Opportunity

The four core step instructions carry per-stack mechanics inline: the step
definition path, the binding form, and the empty-body stub for each supported
runner in `steps`, and the runner's strict-mode mechanism and single-scenario
command in `design`. An author writing a Rust change reads the Python, Java,
Node, Go, and Ruby rows as well; an author on any stack reads every runner's
strict-mode row.

Impact: a project on one stack pays for every other stack's lookup tables on
every request, and nothing detects a core instruction that names an instruction
id nobody serves. The split is approved on its own merits: one owner per rule
and one load per stack. Instruction length is not capped; prose that states no
rule is the defect, not a rule-bearing sentence.

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
- **The composition rule is asserted as text.** A core step instruction carries
  the universal rules and the id mapping, per-stack mechanics are served only on
  a request for that stack's id, and a core instruction that regrows a stack
  body fails a scenario that names the row. Instruction length carries no
  numeric cap.

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
| `instruction-integrity` | `no signal` | `EXTEND instruction-integrity` | Resolution and reference integrity stay with the `support` fragment: the dangling id, the `ise` directions, and the composition rule are checks over served guidance, not new serving behaviour. Route ranked no candidate because the proposal's checks name no command surface. |
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
- A per-project instruction-length rule: the composition rule describes Givn's
  shipped instructions, not a consumer project's overrides.

## Open Questions

### D1 — Which stacks receive an instruction in the first cut

- Question: The four core step instructions name six runners today
  (`cucumber-rs`, `cucumber-js`, `cucumber-jvm`, `behave`, `godog`,
  `cucumber-ruby`). Which of them receive a stack instruction in this change,
  and what happens to the rows of the ones that do not? The answer fixes which
  ids the core instructions name and which scenarios can assert.
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
     instruction; cost: six new small files; in practice: `givn
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

### D2 — Whether instruction length is capped

- Question: The plan's `G2` gate measured `wc -l` on the four core step
  instructions and on `assets/instructions/*.md`. Does instruction length carry
  a durable numeric cap and a permanent check, and does the cap count the stack
  instructions or exclude them? The answer fixes whether a later change is
  judged by a line count.
- Options:
  1. **No numeric cap; padding is the defect** — a core step instruction
     carries the universal rules and the id mapping, per-stack mechanics are
     served only on request, and prose that adds no rule is removed; gain: no
     valid rule is ever deleted to satisfy a number, and one owner per rule is
     checked as text; cost: no mechanical size signal, so review has to read
     the text for padding and divided ownership.
  2. **A corpus line cap with a permanent check** — the shipped total and the
     four core files stay at or below recorded ceilings; gain: a mechanical
     size signal; cost: valid guidance is deleted to satisfy the number, which
     is what happened to the `review.md` obligations, and the operator withdrew
     this option on 2026-11-15.
  3. **A recorded measurement with no check** — counts are recorded and no
     scenario asserts them; gain: nothing to maintain; cost: a core instruction
     that regrows a stack body is detected by nobody.
- Disposition: answered by the operator decision of 2026-11-15 — option 1. The
  numeric cap, its constants, and its check are withdrawn; the design records
  `Operator decision: no corpus line cap`, and the composition rule is asserted
  as text presence and absence by the load-condition and body scenarios.

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
     headers and scope sentences for no additional rule; in practice: a Rust
     change loads two instructions whose content a reader could hold in one.
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
