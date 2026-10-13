# Rules through instructions

Topic: `rules-through-instructions` — ideation handoff, 2026-10-13.

A project's agents follow one rule set per step: Givn's built-in,
technology-agnostic rules extended by the project's own rules, served by
`givn instructions`. Installed skills and commands stop carrying copies and
direct the agent there. This document is the readable record; the working
state lives under `givn/ideation/rules-through-instructions/`.

## Why

Givn delivers its rules through two overlapping channels: generated rule
files — skills and slash commands written as full copies, plus the fenced
blocks in `AGENTS.md` — and printed instructions (`givn instructions <id>`,
resolved at call time through the project-overrides-first cascade). The
channels duplicate content: up to three copies of the same rules (skill
template, command asset, instruction asset), and the copies are exactly
what an agent reads first. Nothing keeps them aligned — the
`prose-consolidation` capability had to fix one such drift by hand.

Customization has no durable home either. Under `rules: local-only` (the
default) generated files are local installation output, so a repo-side edit
has no home; under `rules: versioned`, `givn skills sync` and `givn upgrade`
overwrite them — this repository runs `versioned` and self-hosts exactly
that state. The supported customization replaces whole facets or patches
the manifest; there is no additive way to extend an instruction with
project-specific rules. OpenSpec solves that shape with extra rules per
artifact in its project configuration; only that mechanism is the
reference. Givn deliberately does not aim at custom workflows or a replaced
built-in flow — Givn is the harness that tracks the workflow and guides the
LLM.

## Vision

One authoritative home per rule: the effective instruction served by
`givn instructions`, extended by the project's extension. Generated rule
files shrink to a trigger and a direction; the project's rules live in
project configuration and never conflict with sync or upgrade; the Givn
maintainer edits one asset per rule instead of up to three copies.

## The model

**Built-in rules.** Givn's technology-agnostic rules — workflow and
guardrails — live in the canonical instruction assets and resolve through
the cascade (project replacement wins, then embedded, then addon). Content
is served at call time, so an upgrade's improved rules arrive at the next
request without any synchronization.

**Directions instead of copies.** In the directing form, a generated skill
or command carries trigger metadata and invocation-specific orchestration
only; it directs the agent to `givn instructions <id>`. A full copy
installed by an earlier version is reconciled when the directing form is
resolved; a disabled addon's files are removed; a project-owned replacement
stays byte-for-byte.

**Extensions.** A project extends one instruction's built-in rules with
project-specific rules. Extensions add; they never override, suppress, or
reorder the built-in rules, and they never change the workflow — steps,
order, and gates stay built-in. They live in project configuration (shape
open: Q4), survive every upgrade untouched, and are merged into the served
content (position open: Q4).

**The extended rule.** The rule an agent follows for a step: Givn's
built-in rules plus the project's extension, identical on the text and
machine-readable (`--json`) paths.

**Guarantees stay.** Unknown instruction ids refuse by name; the
dialog-policy obligation stays in the project-start block under every
always-on option, independent of addons; the generated-guidance reference
checks stay as they are; a project-owned replacement stays authoritative;
setup and upgrade keep their no-SCM, migration, allowlist, and
one-revision behaviors.

## Stakeholders and personas

| Persona | Interest |
|---|---|
| Primary user (developer / maintainer) | Extensions in project-owned configuration; sync and upgrade never conflict with them; the fresh-clone workflow stays recoverable. |
| Technical role (authoring agent / automation caller) | Every rule discoverable at the moment of the step; the direction suffices to load it; text and JSON carry the same content. |
| Givn maintainer (owner) | One canonical asset per rule; truthful specs and quality requirements; the experiment proves the directing form before it becomes a default. |

## Change program

One change on the experiment branch
(`givn-instructions-rules-output-experiment`), cut by deliverable
guarantee: the trial must exercise the whole story — directions plus the
customization answer — in one coherent experiment; a split would leave an
intermediate state where the delivery has landed but the customization
story has not.

| Order | Change | Covers |
|---|---|---|
| 1 | `extended-rules` | use cases `obtain-effective-guidance` (new), `extend-an-instruction` (new), `initialize-project` (modified), `upgrade-a-project` (modified); capabilities `instruction-integrity` (modified), `generated-file-form` (new, gated on Q1 option 2), `refresh-generated-guidance` and `match-guidance-to-enabled-addons` (modified) |

Permanent homes: `obtain-effective-guidance` and `extend-an-instruction`
become new permanent leaf use cases with capabilities `serve-extended-rules`
and `instruction-extension`; `initialize-project` and `upgrade-a-project`
keep their IDs and gain the delta; the resolution mechanics stay the
`instruction-integrity` capability; the generated-file setting, when it
exists, becomes a support capability.

Existing durable records the change amends when it lands: the content
assertions on generated guidance in `refresh-generated-guidance`,
`match-guidance-to-enabled-addons`, `instruction-integrity`,
`generate-ideation-guidance` (Q5-dependent), `prose-consolidation`,
`change-release-note`, `visual-design-contract`, `visual-review-gate`, and
`initialize-a-project`; the interaction mapping rows in
`givn/specs/fragments/fragment.md`; arc42 §8.3a and §8.7; and the glossary
(the `Extended rule` term and the built-in-content sense of `Givn rule`).

## Open questions

Q1 experiment shape (branch-only or a setting), Q2 the always-available
set, Q4 the extension source and merge, Q5 instruction-id coverage, Q7 the
extension on top of a whole-facet replacement. Full records live in
`givn/ideation/rules-through-instructions/questions.md`; the change seed
carries the same treatments with every option's meaning, gain, cost, and a
concrete example.

## What was rejected

- Naming the delivery shapes as domain terms (`snapshot rule`, `pointer
  rule`, `project rule`, `rule topic`): too many and too specific; the
  vocabulary is built-in rules + extension = extended rule (user,
  2026-10-13).
- Override semantics for rules: rules are extended, never overridden (Q6,
  user).
- Custom workflow configuration or replacing the built-in Givn flow: Givn
  is the harness (user).
- Splitting the work into two changes, and deciding the cut at handoff:
  one change on the experiment branch (Q3, user).
