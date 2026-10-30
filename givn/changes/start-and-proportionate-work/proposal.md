# Proposal: start-and-proportionate-work

## Release note

New and unclassified work now starts with a proportionate recommendation, while an existing change remains an explicit continuation rather than an assignment.

## Use-Case Context

- Use-case ID: `obtain-effective-guidance` — supporting owners: `initialize-project`, `inspect-change-state`, and the reusable `fragments` document.
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `agent--technical-role`, `developer--primary-user`, `givn-maintainer--owner` — their existing authority and goals apply; no Persona is invented or reconfirmed.
- Source: `docs/improvements/0.9.13/start-and-proportionate-work.md`, revalidated against the 0.9.12 checkout. The operator resolved the no-active guidance shape: load `givn-start` to let the agent decide, or run `givn new <change-id>` directly for an already-clear change.

## Review Context

- Review kind: self-review
- Reason: served guidance, generated skill surface, and CLI text/JSON routing; no persistent data, security boundary, or architecture boundary changes. The regression surface is covered by executable guidance, generation, and CLI scenarios.

## Problem / Opportunity

A user must choose among three overlapping entry points before the problem is understood:

- `givn-explore` investigates without artifacts.
- `givn-ideation` also starts from vague ideas but performs persistent domain work.
- `givn-propose` starts a formal change.

This requires knowing Givn's internal distinctions before deciding what work is justified. The session-start command reinforces the confusion: with no active change it prints `Next: run givn new <change-id>` and its JSON carries `next_command: "givn new <change-id>"`. It therefore reads as an instruction to create a change even when nothing has been investigated yet. With an active change it reports a lifecycle step without saying that the step applies only when the user is continuing that change.

There is no explicit path for a bounded correction that does not deserve a proposal, specification, design, task list, and archive. The formal lifecycle can cost more than the work contributes. The opposite failure also exists: a vague request can be pushed into ideation when inspecting the code would have produced a clear, bounded answer.

User impact: users pay workflow overhead they did not choose, or skip durable work they actually needed, and an unrelated active change can hijack the current request.

## Proposed Solution

Givn replaces `givn-explore` with `givn-start`. The new entry point investigates the problem, question, or improvement and recommends one of four proportionate outcomes:

| Outcome | When it applies | Example |
|---|---|---|
| Answer and stop | Investigation answers the question or shows that no modification is justified. | Explain an existing configuration option; reject a change whose cost exceeds its benefit. |
| Direct work without a Givn change | The work is understood and bounded; the formal lifecycle costs more than it contributes. | Correct a README typo; restore specified behavior with a focused regression test. |
| `givn-ideation` | Domain understanding needs durable, structured development before selecting changes. | Establish actors, rules, terminology, and boundaries for an approval domain. |
| `givn-propose` | A concrete improvement warrants the specification-driven lifecycle. | Introduce a new archive recovery guarantee with observable behavior. |

`givn-start` creates no workflow artifacts and makes no application changes. It preserves the useful exploration behavior: inspect the project, compare approaches, challenge assumptions, and allow the conclusion that no modification is warranted. It hands off once the user authorizes a destination.

Proportionality is a judgment about what the formal workflow buys and what it costs, not a size threshold. Answering without modification and modifying without a formal change are distinct outcomes. A label such as "bugfix" is not an exemption; a one-line permissions change can need more scrutiny than a larger documentation correction.

Direct work skips formal change administration, not correctness. It still reads the relevant contract, keeps existing specifications and documentation accurate, adds proportionate regression coverage, runs proportionate checks, reports the modification and limits, and does not treat authorization to edit as authorization to commit, push, archive, or expand scope.

The session-start command separates state discovery from work selection:

- `givn instructions` reports project state and the next step of an existing change.
- The reported step applies when continuing that change; it does not authorize unrelated work.
- With no active change, the text recommends loading `givn-start` so the agent decides the next action. It also names `givn new <change-id>` as the direct alternative for a change whose goal and scope are already clear.
- JSON carries `next_skill: "givn-start"` and keeps `next_command: "givn new <change-id>"` as the direct executable alternative. The summary must present the command as optional, not mandatory.
- Direct entry to `givn-ideation` and `givn-propose`, existing change continuation, and clearly authorized direct work remain available without a mandatory start gate.

Givn generates `givn-start` for every selected agent target, removes recognizable generated `givn-explore` output, preserves project-authored files and overrides, and recognizes old generated explore command files for cleanup. No callable explore alias remains.

Onboarding, README, reference, workflow, the workflow diagram, the managed project-start block, and the ideation comparison all describe the same entry behavior.

## Capability Routing

`givn spec route --change start-and-proportionate-work` reports no signal (the top candidates tie). The decisions below are recorded with `no signal` in the recommendation column.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `serve-extended-rules` | no signal | `EXTEND serve-extended-rules` | The start instruction and the no-active text/JSON guidance are served guidance owned by `obtain-effective-guidance`. |
| `refresh-generated-guidance` | no signal | `EXTEND refresh-generated-guidance` | Generating `givn-start` and removing recognizable generated explore output is skill synchronization. |
| `initialize-a-project` | no signal | `EXTEND initialize-a-project` | Onboarding text and fresh-init skill generation name the new entry point. |
| `generate-ideation-guidance` | no signal | `EXTEND generate-ideation-guidance` | The ideation comparison and its permanent scenario describe start rather than explore. |
| `instruction-integrity` | no signal | `EXTEND instruction-integrity` | The startup guidance and generated-skill reference checks cover the new instruction id. |

## Out of Scope

- A new shell subcommand named `givn start`; `givn-start` remains a skill, not a CLI command.
- A compatibility alias for `givn-explore`, or automatic conversion of project-owned explore overrides.
- A mandatory intake questionnaire, a numerical routing score, an exemption registry, a lightweight-change artifact, or a second miniature lifecycle.
- A runtime classifier that decides the outcome for the user.
- Redesigning ideation's artifacts or proposal's downstream lifecycle.
- Weakening lifecycle gates for work already conducted as a formal change.
- Changing the `givn run` product-start command.

## Open Questions

### D1 — What should the no-active guidance recommend?

- Status: answered 2026-10-30 (user)
- Question: should no-active guidance load `givn-start`, run `givn new <change-id>`, or both?
- Options:
  1. **Load `givn-start` as the recommendation, with `givn new` as the direct alternative** — the agent investigates and chooses among the four outcomes; a clear change can skip the conversation. In practice: new text names start first and `givn new` second.
  2. **Load `givn-start` only** — every new path goes through investigation. In practice: a user with a fully specified change still starts with a routing conversation.
  3. **Run `givn new` as before** — the machine path keeps its current default. In practice: the no-active output still implies that a change must be created.
- Disposition: answered — option 1. Text recommends `givn-start`; `givn new <change-id>` remains the direct alternative. JSON carries `next_skill: "givn-start"` and `next_command: "givn new <change-id>"` as the direct alternative. Rejected: option 2 adds a gate for already-clear work; option 3 contradicts the confirmed decision that no active change does not imply a new one must be created.

### D2 — How does old generated explore output disappear?

- Status: answered by the improvement verification expectations
- Question: should sync leave old generated explore skills in place, delete every explore skill directory, or delete only recognizable generated output?
- Disposition: answered — sync removes only recognizable generated `givn-explore` skills and legacy generated command files, preserves project-authored files and overrides, and surfaces a manual transition for anything it cannot recognize. Rejected: leaving stale generated files preserves a callable alias; deleting by name alone can destroy user content.

### D3 — Are existing project-owned explore overrides converted?

- Status: answered by the improvement scope
- Question: should project-owned explore instructions, extensions, or skill template overrides be converted automatically?
- Disposition: answered — no. The override bytes are preserved and no callable explore alias is added. A manual transition notice points to the start instruction. Rejected: automatic conversion can silently rewrite project-owned guidance.

### D4 — Is `givn-start` available with optional addons disabled?

- Status: answered by the improvement verification expectations
- Question: should start depend on an optional addon?
- Disposition: answered — no. Start is a default skill and served instruction, generated and usable with every optional addon disabled. The general dialog obligation is unchanged.
