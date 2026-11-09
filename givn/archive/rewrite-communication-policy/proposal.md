# Proposal: rewrite-communication-policy

## Release note

> The communication policy addon states documentation rules with named formats and a bounded block, and stops prescribing repository-specific documentation layout.

## Use-Case Context

- Use-case ID: initialize-project
- Ideation topic: none
- Seed path: none
- Confirmed Personas: developer--primary-user, agent--technical-role — the generated block is CLI text read by the developer and the authoring agent.

## Review Context

- Review kind: self-review
- Reason: one managed text block plus its assertion; no new interface, lifecycle stage, or security surface.

## Problem / Opportunity

The `communication_policy` addon ships a 101-word block that declares itself "Presentation rules" and says nothing about documentation. Agents that read it produce prose directory layouts, narrate keybindings, and copy Gherkin state rules into `docs/`.

A draft rewrite of the block (502 words, five sections) closes that gap but overshoots: it prescribes `README.md` at 100–150 lines and names `docs/workflow.md`, `docs/architecture.md`, and `docs/reference.md`. Measured against the addon's default-on audience, three of four givn projects already break the budget (kiekste 683, shoppinglist-070 210, watn 167 README lines) and `docs/architecture.md` exists in none of them. The draft also restates the dialog example obligation that the unconditional dialog policy owns, and 6 of its 14 bullets violate its own "2–4 word" lead-in rule.

## Proposed Solution

The managed block emits two bounded sections, `Tone` and `Documentation`, with 11 rules:

- Five tone rules carry the existing presentation obligations unchanged: no emojis or filler, high-perception reader, blunt phrasing, named costs, examples over labels.
- Six documentation rules bind formatting to named artifacts: point to artifacts instead of describing layouts; tabulate command, flag, keybinding, config-key, and state mappings; bold lead-ins; keep Quick Start runnable; keep Gherkin state rules in specs and `--help`; update user docs on scope change, not per edge-case fix.
- The block names no documentation file, no README line budget, and no dialog-example content, and stays at or below 300 words.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| enable-optional-addons | EXTEND givn-workflow-enforcement (score 17.00) | `EXTEND enable-optional-addons` | The addon block, its enablement scenario, and its generated file belong to `initialize-project`; route matched incidental `givn check` text in the proposal's review rules. |

## Out of Scope

- Dialog policy content and the `givn instructions dialog*` assets.
- Documentation layout rules for givn's own `docs/` tree; those belong to repository guidance, not a default-on addon.
- Addon enablement defaults, the addon name, and the `absolute_mode` retirement path.

## Open Questions

### D1 — README line budget in the shipped block

- Question: Should the managed block bound `README.md` length at roughly 100–150 lines?
- Options:
  1. **No budget** — the block stays layout-agnostic; gain: no repository is declared non-conforming by a sync; cost: guidance carries no size guard; in practice: kiekste's 683-line README remains valid.
  2. **Keep the budget** — gain: forces concise READMEs; cost: three of four existing givn projects violate it on the next sync; in practice: kiekste must cut 500+ lines to conform.
- Disposition: answered — measured READMEs (givn 132, watn 167, shoppinglist-070 210, kiekste 683) show the budget contradicts the addon's default-on audience; the block omits it.
