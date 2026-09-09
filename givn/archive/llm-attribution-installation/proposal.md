# Proposal: llm-attribution-installation

## Problem / Opportunity

`givn init` installs commit-attribution plumbing, but its final summary does not
tell the person which Git hook or agent integrations were installed. The result
is invisible unless the person inspects generated files manually.

An existing project upgraded to a newer Givn version can receive the checked-in
hook and selected integrations through the synchronization path, but the
upgrade output does not make that retrofit explicit. Claude's merged settings
file is also not part of the managed upgrade allowlist, so the retrofit is not
fully represented in the managed upgrade commit.

## Proposed Solution

After initialization, the summary explicitly names:

- the shared Git commit-attribution hook;
- whether local Git activation was installed or could not be activated; and
- each selected OpenCode or Claude Code attribution integration.

When an existing project runs `givn upgrade`, the same commit-attribution
plumbing is retrofitted using the project's selected agent targets. The managed
upgrade includes the checked-in hook and the selected integration configuration,
including the Claude Code settings entry, so the upgrade commit contains the
files needed to share the behavior with the project.

The upgrade output identifies that commit-attribution plumbing was refreshed.
Projects without a Git worktree still receive the checked-in shared hook and
agent files, while local Git activation is reported as unavailable rather than
treated as a failure.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| llm-attribution-installation | EXTEND run-command (maintenance) | NEW in support | The route matches shared init/upgrade commands, but installation reporting and attribution retrofit remain a support capability extending the existing attribution behavior. |

## Out of Scope

The attribution protocol, best-effort semantics, committer identity format,
message trailers, OpenCode runtime lookup, and Claude command transformation do
not change. This change does not make local Git activation mandatory, add
server-side provenance, or change unrelated upgrade staging rules.

## Open Questions

None.
