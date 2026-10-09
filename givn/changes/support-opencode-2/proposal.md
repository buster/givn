# Proposal: support-opencode-2

## Release note

OpenCode 2 sessions attribute their commits to the model that produced them.

## Use-Case Context

- Use-case ID: support
- Ideation topic: none
- Seed path: none
- Confirmed Personas: developer--primary-user — the developer who runs OpenCode and reads commit attribution.

## Problem / Opportunity

`givn init --agents opencode` generates the OpenCode attribution integration into
`.opencode/plugins/givn-attribution.js`. The generated file targets the plugin
contract of the previous OpenCode generation. OpenCode 2 loads plugins through
a different contract, so the generated integration takes no effect there: a
project that selected the OpenCode target still receives no commit attribution.

Observed consequence: in an OpenCode 2 session, a shell command — including the
command that creates a commit — receives no attribution context. The revision
is then recorded with unknown LLM and reasoning mode even though the session
has a selected provider and model, and even though the project asked for the
OpenCode integration. The repository's own generated integration has the same
problem, so the maintainer's own OpenCode 2 sessions lose attribution.

## Proposed Solution

When OpenCode is among the selected agent targets, givn generates an
attribution integration that OpenCode 2 loads, and that keeps the
commit-attribution contract already in effect:

- The integration observes the provider, the model identity, and the reasoning
  mode selected for a session, including the sessions OpenCode runs for a
  subagent.
- When a session creates a shell command, the integration supplies that
  session's attribution context to the command's environment, so the
  commit-attribution hook and `givn commit` can name the issuing model. The
  command is never blocked, delayed, or altered when the context is missing.
- A shell command is attributed to the session that issued it: a subagent's
  command carries the subagent's model, not the last model observed anywhere
  in the run.
- When no model has been observed for the session or for the run, attribution
  stays marked unavailable, exactly as the existing best-effort contract
  requires.
- Regeneration follows the existing paths: initialization, upgrade, and
  synchronization overwrite the generated integration. Exactly one OpenCode
  integration file is generated; there is no companion file for the previous
  generation and no runtime version selection.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| attribute-commits | no signal — the top candidates tie at 16.00 | `NEW in fragments` | The capability is permanently stored in the reusable support fragment (`givn/specs/fragments/`); the fragment route requires the delta under `specs/fragments/`, and `fragment.md` declares `attribute-commits`. |

## Out of Scope

- OpenCode 1 support. The generated integration targets OpenCode 2 only: no V1
  companion file, no runtime version detection, and no fallback to the
  previous plugin contract.
- The Claude Code and omp integrations, the checked-in commit hook, the
  SCM-neutral commit path, and the installation report are unchanged.
- Integration-internal edge cases that the attribution contract does not
  expose (rejected hook registrations, malformed tool events, a bounded
  configured-default lookup) stay implementation details and gain no separate
  scenarios.

## Open Questions

### D1 — Compatibility window for the generated OpenCode integration

- Question: should the generated attribution integration support OpenCode 1
  and OpenCode 2 at the same time, or target OpenCode 2 only?
- Options:
  1. **OpenCode 1 plus OpenCode 2** — the generated file carries both plugin
     contracts; gain: a project that regenerates keeps attribution under an
     older OpenCode 1 checkout; cost: one integration carries two hook
     vocabularies and the acceptance suite must pin and exercise both
     generations; in practice: the same file serves an OpenCode 1 user who has
     not upgraded and an OpenCode 2 user who has.
  2. **OpenCode 2 only** — the generated file carries the current contract
     only; gain: one small integration, one verification surface, no version
     detection; cost: an OpenCode 1 user loses attribution after regenerating
     until they upgrade OpenCode; in practice: the regenerated file is loaded
     by OpenCode 2 and ignored by OpenCode 1.
- Disposition: answered — the operator chose OpenCode 2 only (planning
  discussion, 2026-10-09). Rejected: the dual contract, because it doubles the
  integration and its verification surface for a superseded runtime.

### D2 — How a shell command is linked to its issuing session

- Question: should a shell command be attributed to the session that issued
  it, or to the last model observed in the run?
- Options:
  1. **Attribute to the issuing session** — the integration associates the
     shell command with its session while the command is requested and matches
     it when the command's environment is prepared; gain: a subagent's commit
     carries the subagent's model; cost: the integration must keep and retire
     pending requests; in practice: a subagent that runs `git commit` is
     attributed to the subagent's model while the main session keeps its own.
  2. **Attribute to the last observed model** — the integration stamps
     whichever model it saw most recently, whichever session it came from;
     gain: the smallest integration, closest to the previous generation's
     shape; cost: a commit can be attributed to another session's model; in
     practice: with a main session on one model and a subagent on another, the
     main session's commit can be stamped with the subagent's model.
- Disposition: answered — the operator chose attribution to the issuing
  session (planning discussion, 2026-10-09). Rejected: the last observed
  model, because the shell environment does not name its session when the
  integration must act, so the fallback mislabels concurrent sessions.

### D3 — Verification surface for the OpenCode 2 integration

- Question: should the acceptance suite simulate the OpenCode 2 plugin
  lifecycle, or exercise a pinned OpenCode 2 runtime?
- Options:
  1. **Simulated plugin lifecycle** — the acceptance test replays the
     OpenCode 2 registration and events against the generated integration;
     gain: no runtime downloads, no dependency tree, no container or Nix
     runtime wiring; cost: the simulation is only as faithful as the events it
     replays; in practice: the test selects a model for a session, requests a
     shell command, and reads the attribution the integration contributes.
  2. **Pinned real runtime** — the acceptance suite downloads and drives
     pinned OpenCode 2 releases; gain: real payload fidelity; cost: pinned
     binaries, dependency installs, provider stubs, isolation, and
     mutation/probe harnesses; in practice: the suite provisions a temporary
     project and drives a real OpenCode 2 server.
- Disposition: answered — the operator chose the simulated lifecycle
  (planning discussion, 2026-10-09). Rejected: the pinned real runtime,
  because its cost is disproportionate to one integration and it duplicates
  the Gherkin scenarios.
