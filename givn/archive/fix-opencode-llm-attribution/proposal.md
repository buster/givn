# Proposal: fix-opencode-llm-attribution

## Use-Case Context

> Optional. When this change comes from ideation or an existing permanent
> contract, record the stable use-case ID, source topic, and confirmed Persona
> IDs. Do not copy Persona biographies into the proposal.

- Use-case ID: support
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

OpenCode- and Claude-created commits can identify the harness but not the active
LLM, even when each harness has supplied model metadata. The commit therefore
reports partial or misleading attribution and loses the model identity that
users need when reviewing commit history.

## Proposed Solution

When an OpenCode or Claude session has an active model selection, a commit
created from that session should expose the model identity in the committer
identity and in the managed co-author attribution. OpenCode provider/model
composition, Claude transcript model identity, reasoning mode, and harness
context should continue to be recorded as before. The commit message contains
contains a managed co-author attribution in the form
`Co-Authored-By: <model>/<reasoning>@<harness> with Givn <version>`. The
attribution status is complete when all expected values are available.

When OpenCode has no active model selection, commits should retain the existing
best-effort behavior: the harness remains identifiable, missing model and
reasoning values are marked unavailable, and the commit is not blocked.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| llm-attribution | `EXTEND llm-attribution` (legacy fragment capability) | `NEW in fragments` | The existing capability is permanently stored in the reusable support fragment; fragment deltas use the special fragment route. |

## Out of Scope

Human commits without attribution context and installation reporting are not
changing. Legacy Givn attribution trailers are removed from stamped messages;
the managed co-author attribution is the only Givn-owned message output.

## Open Questions

None.
