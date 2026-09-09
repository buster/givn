# Proposal: llm-attribution

## Use-Case Context

> Optional. When this change comes from ideation or an existing permanent
> contract, record the stable use-case ID, source topic, and confirmed Persona
> IDs. Do not copy Persona biographies into the proposal.

- Use-case ID: none
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

Commits created while working with an LLM do not reliably show which LLM,
reasoning mode, harness, or Givn version produced them. The information is
especially difficult to recover when a change switches models or harnesses, and
it is absent when a human initializes the project before an agent session begins.

This makes commit history less useful for reviewing how a change was produced.
It also makes it difficult for tools that display the commit message or
committer identity to expose LLM involvement.

## Proposed Solution

When a project is initialized for Givn, it receives shared commit-attribution
behavior without requiring the person who initializes it to identify an LLM or
harness.

When a commit is created while an LLM harness exposes attribution context, the
commit should, on a best-effort basis:

- identify the LLM and harness in the committer identity;
- append the current Givn version, LLM, reasoning mode, and harness to the end
  of the commit message; and
- reflect the context active for that commit, even when a later commit uses a
  different LLM or harness.

When attribution context is unavailable or incomplete, the commit must still
be allowed to complete. The system must not invent an LLM identity or prevent
normal human editing and committing. Human-created commits without attribution
remain ordinary commits.

The behavior must be available for OpenCode and Claude Code projects through
their generated agent integrations. The Git-side behavior must be checked into
the project so that every checkout receives the same attribution plumbing.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| llm-attribution | EXTEND spec-explore-change (support) | NEW in support | The route is a heuristic match on the support fragment; commit attribution is a separate cross-cutting support capability. |

## Out of Scope

This change does not require attribution to be complete, blocking, or
tamper-proof. It does not change the selected LLM, provider, reasoning policy,
or harness configuration. It does not infer missing model metadata from commit
content or from a human's identity. It does not add server-side enforcement,
signature verification, or remote provenance storage.

## Open Questions

The exact fallback text for partially known attribution remains a design
decision; missing values must remain explicit rather than being fabricated.
