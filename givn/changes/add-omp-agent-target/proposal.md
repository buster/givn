# Proposal: add-omp-agent-target

## Release note

> Givn installs the workflow skills, the workflow commands, and a commit
> attribution hook for the omp coding agent when `omp` is a selected agent
> target.

## Use-Case Context

- Use-case ID: none
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `agent--technical-role` (loads the generated omp skills
  and commands and runs the attribution hook), `developer--primary-user`
  (selects the agent target and reads the installation report)

## Problem / Opportunity

The omp coding agent reads project guidance from its own directory: workflow
skills from `.omp/skills/`, workflow commands from `.omp/commands/`, and
extension hooks from `.omp/hooks/pre/`. Givn offers three agent targets —
`.opencode`, `.agents`, and `.claude` — and no `.omp` target.

A developer working with omp therefore has no honest choice. Selecting
`agents` delivers only skills, no commands and no attribution. Selecting
`.opencode` or `.claude` installs guidance into a harness the developer is not
running. The result is visible in the commit history: a commit created inside
an omp session carries no LLM or harness attribution at all, because no
generated integration supplies the attribution context to the commit. A
reviewer reading `git log` cannot tell which model produced a change, while
the same project under OpenCode or Claude Code does show it.

## Proposed Solution

- During project initialization a developer can select the omp agent target,
  alone or alongside the existing targets.
- A project with the omp target receives, in omp's own locations, the Givn
  workflow skills (loaded on demand) and the workflow commands (invoked as
  `/givn-<step>`), plus a commit-attribution behavior.
- When a commit is created inside an omp session, the commit records the
  current LLM identity, reasoning mode, and the harness name `omp` in its
  committer identity and message — exactly the behavior OpenCode and Claude
  Code projects already receive. A commit created without that context stays
  an ordinary commit.
- The initialization report names the omp attribution integration, next to the
  OpenCode and Claude Code integrations.
- A managed upgrade refreshes the omp integration files when `omp` is a
  configured target, exactly as it refreshes the other configured integrations.

## Capability Routing

`givn spec route` reports no signal for this change (the top candidates tie),
so the decision is recorded directly.

| Proposed capability | Decision | Rationale |
|---|---|---|
| Commit attribution and agent-target generation | `EXTEND fragments` | The change adds one more agent target to the existing target-generation support fragment and one more generated integration implementing the existing attribution contract. No new protocol, message format, or permanent capability is introduced. |

## Out of Scope

- The attribution message format, the managed trailer text, and the
  environment-variable protocol are unchanged.
- The existing `.opencode`, `.agents`, and `.claude` integrations are
  unchanged.
- No configuration outside the project's `.omp/` directory is written; Givn
  does not install into the user-level omp directory.
- No change to how any harness discovers models or reasoning levels.
- The `.githooks/commit-msg` hook and the local Git hook launcher are
  unchanged and remain unconditional.

## Open Questions

### D1 — Which Bash commands does the generated omp hook rewrite?

- Question: When the omp attribution hook intercepts a Bash tool call, should
  it prepend the attribution exports to every command, or only to commands
  that create a revision?
- Options:
  1. **Only revision commands** — the hook rewrites a command only when its
     text names a revision operation (`git commit`, `givn commit`); gain: every
     other command is displayed and persisted verbatim, so the transcript and
     the tool-call display stay readable; cost: a revision path that does not
     spell "commit" (for example `git merge --no-edit` or `git cherry-pick`)
     receives no attribution; in practice: `git status` shows as `git status`,
     while `git merge feature` records no LLM.
  2. **Every Bash command** — the hook rewrites every command, matching what
     the current Claude Code integration does; gain: no revision path is
     missed; cost: omp persists the revised call, so every command line in the
     transcript carries the export prefix; in practice: `ls` is shown and
     recorded as `export GIVN_ATTRIBUTION_ACTIVE='1' …; ls`.
- Disposition: carried — decided in `design.md` (D1).

### D2 — How does omp load the generated attribution integration?

- Question: Which omp extension mechanism should the generated integration
  target?
- Options:
  1. **`pre` hook factory** — a JavaScript factory under
     `.omp/hooks/pre/` that omp's extension runner loads and binds to the
     tool-call event; gain: it is the documented project-local hook location
     and intercepts Bash before execution; cost: it depends on omp's extension
     loader accepting an input rewrite; in practice: the factory returns the
     revised command and omp executes it.
  2. **Project `.env` file** — write the attribution variables into
     `.omp/.env`; gain: no hook code; cost: the model and reasoning level are
     per-commit, not per-project, so the values would be wrong or absent; in
     practice: switching models mid-project would keep the old attribution.
- Disposition: carried — decided in `design.md` (D2).
