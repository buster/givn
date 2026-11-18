# Proposal: make-ise-guidance-self-contained

## Release note

The served ise instruction defines its rebuild vocabulary inline, so an ise-enabled consumer needs no Givn-repository reference, and two scenarios pin the ise-enabled reference check and the ise-off guidance sweep.

## Use-Case Context

- Use-case ID: `obtain-effective-guidance`; the `instruction-integrity` capability is owned by the `support` fragment.
- Ideation topic: `ise-rebuildability`.
- Seed path: none.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.

## Review Context

- Review kind: self-review
- Reason: a two-scenario correction to served vocabulary and its regression coverage; the earlier independent review recorded the finding, and the change has no architecture or runtime decision.

## Problem / Opportunity

The independent isolation review (`agent://IseAddonIsolation`) confirmed the default-off path is clean, but found that the opt-in instruction is not self-contained:

- `assets/instructions/ise.md:13-18` says `Spec copy`, `spec-copy manifest`, and `checksum` are defined by `docs/reference.md` and the project glossary, and that `Rebuild impact record` and `Replaced unit` are defined by the project glossary.
- A consumer that runs `givn init` then `givn addons enable ise` gets no `docs/reference.md`; its `docs/glossary.md` is the empty scaffold. The served instruction therefore delegates vocabulary to a Givn-repository document.
- The repository reference checker would classify `docs/reference.md` as unresolved in that consumer.
- No scenario requests `proposal`, `design`, `design-review`, `review`, `archive`, `steps`, or the startup guidance in an ise-off project and asserts the ise-only phrases are absent. Moving an `{{/if}}` boundary would pass the suite.

## Proposed Solution

- Define the five rebuild terms inline in `ise.md`: `spec copy`, `spec-copy manifest`, `checksum`, `Rebuild impact record`, and `Replaced unit`. Remove the `docs/reference.md` delegation.
- Add `The ise instruction's file references resolve in a consumer project`: with `ise` enabled, request `givn instructions ise` and run the reference checker over the served text.
- Add `Printed guidance omits the opt-in ise rules`: with `ise` off, request every printed instruction plus the startup guidance and assert that `Ise rebuild decision`, `Rebuilt code`, `Spec gap`, `Rebuild impact record`, `Specification Manifest`, and `Ise Test` are absent.
- Keep the default-off posture and every command, flag, field, and gate unchanged.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `instruction-integrity` | no signal | `EXTEND instruction-integrity` | The served-text scenarios are owned by the `support` fragment; the change adds two variants and does not change another capability. |

## Out of Scope

- No scaffold of a consumer `docs/reference.md`; the inline definitions are smaller and leave the default project unchanged.
- No change to `givn ise rebuild-*`, the `ise-manifest/v1` scope, report labels, or the site.
- No change to the existing ise-off gates; the new sweep is regression coverage, not a new rule.

## Open Questions

### D1 — Inline definitions or a scaffolded reference

- Question: Should the instruction define the five terms itself, or should enabling `ise` scaffold a consumer `docs/reference.md`?
- Options:
  1. **Inline definitions** — gain: no new scaffolded file; the instruction is self-contained; cost: the terms are defined twice (instruction and glossary); in practice: `ise.md` defines `spec copy` in one sentence and the later section expands it.
  2. **Scaffold a reference** — gain: one durable home; cost: a new materialization for every ise project and a second copy of maintainer prose; in practice: `givn addons enable ise` writes `docs/reference.md`.
- Disposition: answered — inline, because the instruction is the served surface and the scaffold would add a second copy of Givn-repository prose.

### D2 — Test shape for the off path

- Question: Should the off-path protection be one sweep or per-instruction scenarios?
- Options:
  1. **One sweep** — gain: one scenario catches a moved `{{/if}}` boundary across every printed instruction; cost: a failure names the phrase, not the instruction; in practice: the Then lists the absent phrases.
  2. **Per-instruction** — gain: precise failure; cost: seven scenarios for one gate; in practice: one scenario per instruction id.
- Disposition: answered — one sweep, because the failure is a boundary regression and the message names the phrase.
