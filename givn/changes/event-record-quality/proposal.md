# Proposal: event-record-quality

## Release note

Make development records unambiguous by recording one usage representation per request, explicit answers to questions, bounded failure text and known model identity.

## Use-Case Context

- Use-case ID: inspect-development-history
- Ideation topic: none — derived from analysis of a real record
- Seed path: none
- Confirmed Personas: none — operator confirmed; this is operator and analyst tooling without a rendered product surface; the operator is a Gherkin actor.

## Problem / Opportunity

A real 13,795-event record exposed four ambiguities:

- 2,653 usage events duplicated inline turn usage;
- eight questions had text and wait time but no recorded answer;
- failed commands often carried only an exit code, not a bounded error line;
- some usage and subagent records lacked model identity or reported usage as unavailable.

The analyzer handles these conservatively, but the raw record makes attribution and cost reconciliation harder than necessary. Duplicate representations invite double counting. Missing answers invite false assumptions about approval. Missing error text weakens failure analysis. Missing identity weakens provenance.

## Proposed Solution

Every request has one authoritative usage representation. If a linked usage event is still emitted, it is explicitly marked as derived or duplicate rather than looking independent.

Question records include the selected option, decision source and time when an answer exists. Silence is never recorded as approval.

Failed commands include a bounded first error line or stderr excerpt with provenance. Unavailable values remain explicitly unavailable; they are not omitted or replaced with zero.

Usage and subagent records include provider, model and variant when the producer knows them.

Old records remain readable. New fields are additive and the analyzer tolerates their absence.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| development-record | `EXTEND prose-consolidation` (create-change, 14.00) | `EXTEND development-record` | The route matched generic words such as “source” and “selected”; the behaviour is recorder provenance, owned by development-record. |

## Out of Scope

- Changing the event schema version in a breaking way.
- Adding an LLM judge or semantic scoring.
- Reconstructing answers or error text that was never recorded.
- Changing billing semantics or estimating prices.
- Changing the analyzer's conservative accounting rules.

## Open Questions

### D1 — Omit duplicate usage or mark it derived?

- Question: Should the recorder stop emitting the linked usage event entirely, or keep it with an explicit derived marker?
- Options:
  1. **Omit** — one usage object on the turn. Gain: simplest record. Cost: older consumers of linked usage break. In practice: a turn carries usage once.
  2. **Mark derived** — keep the linked event with `derived: true` or `usage_of: <turn>`. Gain: compatibility and explicit provenance. Cost: larger record. In practice: the analyzer can skip derived records by rule.
- Disposition: carried to `design.md` — choose compatibility versus minimality.

### D2 — What answer fields are required?

- Question: When an answer is recorded, what minimum fields make it attributable?
- Options:
  1. **Selected option and source** — record the chosen option, who/what selected it, and when. Gain: auditable approval. Cost: more fields. In practice: `selected`, `source`, `answered_at`.
  2. **Free-text answer only** — record only the text. Gain: simple. Cost: cannot tell automated approval from human approval. In practice: a text blob with no provenance.
- Disposition: answered — selected option, source and time.

### D3 — How much failure text is safe to record?

- Question: Should failed commands record full stderr or only the first bounded error line?
- Options:
  1. **Bounded first line** — record a short error line and its provenance. Gain: safe, small. Cost: may lose stack context. In practice: `error: archive blocked: ...`.
  2. **Full stderr** — record everything. Gain: diagnosis. Cost: secrets, noise and size. In practice: a full build log in every event.
- Disposition: answered — bounded first line by default; opt-in fuller capture remains a project setting.
