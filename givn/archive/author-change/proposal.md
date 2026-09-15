# Proposal: author-change

## Use-Case Context

- Use-case ID: `author-change` (existing, modified), including the
  `support` / `llm-attribution` fragment capabilities
- Ideation topic: multi-scm-support
- Confirmed Personas: developer--primary-user, agent--technical-role,
  givn-maintainer--owner

## Problem / Opportunity

Authoring support assumes Git. `givn init` installs Git attribution plumbing
and reports it unconditionally, a project without any SCM gets no word about
which lifecycle commands can work, and there is no SCM-neutral way to create
an attributed revision — on Fossil an agent's commits carry no attribution at
all. The generated guidance likewise stays silent about backend limits.

## Proposed Solution

- `givn commit -m <message> [paths...]` creates exactly one attributed
  revision through the selected backend (Git or Fossil), stamping the same
  managed trailer the Git hook uses when attribution context is active.
- `givn init` asks the seam: on Fossil it installs no hook and reports the
  `givn commit` attribution path; without any backend it scaffolds, generates
  skills, warns by name that archive and managed upgrade are unavailable, and
  exits 0 (Q7).
- The generated guidance names `givn commit` (valid on every backend) and
  states that a raw `fossil commit` carries no attribution.
- The `llm-attribution-installation` capability is registered in the support
  fragment narrative (resolving the seed's open question).

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| llm-attribution | EXTEND llm-attribution | `NEW in fragments` | The capability lives under the support fragment; the routing check resolves only use-case-owned capabilities, so fragment deltas record `NEW in fragments` (A1). |
| llm-attribution-installation | EXTEND llm-attribution-installation | `NEW in fragments` | Same fragment rule; the capability's permanent feature already exists. |
| init-agent-target-selection | EXTEND init-agent-target-selection | `EXTEND init-agent-target-selection` | The existing capability owns init behavior; the delta adds backend-aware scenarios. |
| prose-consolidation | EXTEND prose-consolidation | `EXTEND prose-consolidation` | The existing capability owns generated-guidance behavior. |

## Out of Scope

- Rewriting existing history or attributing raw Fossil commits that bypass
  `givn commit`.
- Archive publication and the managed upgrade (their own seeds).
- Backend-conditional template rendering (the guidance note is static, A15).

## Open Questions

- None blocking.
