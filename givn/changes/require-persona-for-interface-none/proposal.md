# Proposal: require-persona-for-interface-none

## Release note

Review now requires a confirmed Persona for every change that declares a visual design contract, including `Interface: none`, while a change that declares no visual design contract stays exempt.

## Use-Case Context

- Use-case ID: `verify-change`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner` — all confirmed in `givn/personas/` and named by the permanent `verify-change` contract. This change tightens the review gate they run, author, and own; no new Persona is proposed.

## Problem / Opportunity

A change can declare `Interface: none` in its visual design contract and skip the Persona disposition entirely. The design guidance reserves `Interface: none` for a change with no rendered screen, and review treats that declaration as a full exemption: no Persona, no visual verdict, no evidence. Terminal, library, and CLI changes take that path, so the changes with no rendered screen carry no stakeholder lens at all — even though they still have consumers whose goals the change can help or harm.

The exemption is also a bypass. A change that visibly adds a screen but declares `Interface: none` is treated as non-visual by review, and the same declaration removes the Persona requirement. The gate cannot tell a legitimately non-visual change from a misclassified one.

The deliberate exemption that should remain is narrower: a change that declares no visual design contract at all — for example a documentation-only change with no design contract — has no interface to reason about and stays exempt.

A `PERSONAS: none — operator confirmed` line currently passes for any change whose interface is `none` or absent. That reasoned-none path is valid only for the no-contract case; for a declared contract it turns a missing stakeholder lens into an accepted record.

## Proposed Solution

The Persona requirement follows the presence of a declared visual design contract, not the rendered interface:

1. **Any declared contract requires a Persona.** When a change's design declares a `## Visual Design Contract` section, review passes only when `review.md` names at least one confirmed Persona that resolves to a file in `givn/personas/`. This includes `Interface: none`.
2. **`PERSONAS: none` never satisfies a declared contract.** Neither the bare value `none` nor `none — <reason>` nor an empty value passes while a contract is declared; the review reports the Persona disposition as the failure.
3. **The visual verdict and evidence stay conditional on the interface.** A `web` or `terminal` contract still requires `VISUAL-REVIEW: PASS` and the matching evidence. An `Interface: none` contract requires neither a visual verdict nor visual evidence.
4. **No contract at all remains exempt.** A change that declares no `## Visual Design Contract` section is exempt from the Persona disposition, the visual verdict, and visual evidence. It may record `PERSONAS: none — <reason + operator confirmation>`; `VISUAL-REVIEW: NOT APPLICABLE` remains valid.
5. **The guidance, review template, and durable documentation state the same rule.** The design guidance distinguishes "declares no contract" from "declares `Interface: none`"; the review guidance and template require a confirmed Persona for any declared contract; the arc42 chapters and glossary carry the invariant.

Example after the change: a library change whose design records `- Interface: none` and whose review records `PERSONAS: developer--primary-user` passes review with no screenshot and no visual verdict. The same change with `PERSONAS: none — operator confirmed` fails review, naming the Persona disposition.

## Capability Routing

`givn spec route --change require-persona-for-interface-none` reports signal for `EXTEND visual-review-gate`, the capability whose permanent spec already owns the verdict, Persona disposition, and evidence scenarios. The recorded decision is the same: the behaviour being changed is one branch of the visual review gate, and no review-diagnostic or use-case-schema behaviour is involved.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| visual-review-gate | EXTEND visual-review-gate (signal) | EXTEND visual-review-gate | No deviation. |

No new capability and no new interaction are introduced. The existing `verify-change` interaction "run the review check on a visual change" remains the interaction owner; the delta adds error and passing cases under the same action.

## Out of Scope

- Changing the visual verdict or evidence requirements for `web` and `terminal` contracts.
- Changing the exemption for a change that declares no visual design contract at all.
- Requiring a Persona in proposal, specs, use-case, or any artifact other than the review disposition.
- Changing the persona collection, persona schema, or how a Persona is confirmed.
- Adding a new consumer action or a new `@e2e` scenario; the review invocation is unchanged.
- Migrating archived changes or rewriting their review records.
- Reclassifying an existing screen, or changing what counts as a Web UI.

## Open Questions

### D1 — Does `Interface: none` require a Persona without requiring visual evidence?

- Question: When a design declares `Interface: none`, should review require a confirmed Persona although there is no rendered interface, no visual verdict, and no visual evidence?
- Options:
  1. **Contract-based requirement** — any declared `## Visual Design Contract`, including `Interface: none`, requires at least one confirmed Persona; the visual verdict and evidence remain limited to `web` and `terminal`; gain: no exemption loophole and every declared interface carries a stakeholder lens; cost: a non-visual change that keeps the contract section must name a Persona; in practice: a library change records `- Interface: none` and `PERSONAS: developer--primary-user`, and review passes with no screenshot.
  2. **Interface-based requirement (status quo)** — only `web` and `terminal` require a Persona, and `Interface: none` is exempt; gain: non-visual changes need no Persona record; cost: terminal, library, and CLI changes carry no stakeholder lens, and a misdeclared `none` bypasses both the Persona and visual gates; in practice: the same library change records `PERSONAS: none — operator confirmed` and review passes.
  3. **Every-change requirement** — every change names a confirmed Persona, including a documentation-only change with no design contract; gain: one rule with no exception; cost: forces a stakeholder lens onto work with no interface or design contract and removes the deliberate no-contract exemption; in practice: a typo-fix change with no `design.md` must name a Persona.
- Disposition: answered — the recorded user request for this change: remove the `Interface: none` exemption, keep a deliberate exemption for a change that declares no visual design contract at all, keep visual evidence and the visual verdict conditional on the interface, and keep `PERSONAS: none` failing. Realized in `design.md` Decisions.
