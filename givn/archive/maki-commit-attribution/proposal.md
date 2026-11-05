# Proposal: maki-commit-attribution

## Release note

Maki commits now carry the same best-effort model, reasoning, and harness attribution as the other supported harnesses.

## Use-Case Context

- Use-case ID: `initialize-project`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`

## Review Context

- Review kind: self-review
- Reason: extends the generated Maki integration and its permission manifest; it adds no lifecycle gate, record schema, external boundary, or product data contract. The permission change is narrow and visible in the generated manifest.

## Problem / Opportunity

Commits made through Maki receive no commit attribution even when the project has the shared commit-attribution hook installed. The reported commit `52c3b1f` in `buster/passuff` carries no attribution trailer, and the local checkout at `/home/buster/projects/gh-notify` has the hook and the Maki reporter installed but no Maki attribution preparation. The other supported harnesses prepare attribution context for the commands they issue; Maki does not.

The impact is incomplete history. A reader of a Maki-authored revision cannot tell which model or reasoning mode produced it, cannot distinguish it from a human commit, and cannot rely on the same attribution contract the project already applies to OpenCode, Claude Code, and omp commits. The same gap affects `givn commit` and raw Git commits issued through Maki.

Maki exposes a hook point before a shell command runs, and the generated Maki permission manifest currently revokes the permission that hook requires. The opportunity is to close the gap without weakening the best-effort contract: human commits remain untouched, unknown facts remain unavailable instead of invented, and a non-revision command is never rewritten.

## Proposed Solution

Initializing, synchronizing, or upgrading a Maki project installs one attribution integration beside the Maki development reporter, and the installation report names it. The integration prepares the attribution context for a revision command issued through Maki: the active model identity, the reasoning mode, and the harness name `maki`. When the shell command creates a revision, the managed co-author attribution and the committer identity appear exactly as they do for the other supported harnesses.

Attribution remains best effort. When the model identity or reasoning mode is unavailable, the command still carries the harness name and records the missing facts as unavailable; the revision is not blocked. When the command creates no revision, Maki leaves it unchanged. A command issued outside a prepared Maki command path remains a human command and receives no attribution.

The generated Maki permission manifest grants the permission the integration requires. A project whose manifest is the unmodified generated revision receives that grant during initialization, synchronization, or upgrade. A project whose manifest the developer edited is never widened silently: the existing permissions remain authoritative, and the report names the missing grant and the file that needs it.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `attribute-commits` | `EXTEND initialize-a-project` | `EXTEND attribute-commits` | The attribution behavior is owned by the support fragment that `initialize-project` includes; the delta extends that capability's scenarios. |
| `enable-commit-attribution` | `EXTEND initialize-a-project` | `EXTEND enable-commit-attribution` | Installation and retrofit behavior for the same support contract lives in this capability. |
| `initialize-a-project` | `EXTEND initialize-a-project` | `EXTEND initialize-a-project` | The generated Maki integration and permission manifest are written during initialization and synchronization. |

## Out of Scope

- Changing the attribution protocol, trailer wording, or version stamping shared by the supported harnesses.
- Adding attribution preparation to any harness other than Maki.
- Per-subagent or per-task attribution when Maki exposes only one session model; the integration never invents a model identity.
- Headless Maki runs that expose no model identity; they may still stamp the harness with unavailable facts.
- Revision paths that never spell a revision verb, such as a merge or cherry-pick that does not name `commit`.
- Broadening any other Maki permission; network, environment, agent, session-settings, and chat-input access remain revoked.
- Changing the development record, the history intake, or the reporter's activity mapping.

## Open Questions

### D1 — Which model identity does Maki attribution name?

- Question: When a revision command is issued by a subagent whose model differs from the focused session's model, should attribution name the focused session's model, the issuing agent's resolved model, or mark the model unavailable when the two disagree?
- Options:
  1. **Focused session model** — use the model the developer sees for the focused session; gain: one stable source, matching the existing timeline reporter's model observation; cost: a subagent committing under a different model is attributed to the focused session; in practice: a parent session on model A spawns a subagent on model B, and the subagent's commit names A.
  2. **Issuing agent's resolved model with a focused-session fallback** — ask the harness for the current agent's model and fall back to the focused session; gain: the subagent's own model is named when the harness exposes it; cost: an extra harness lookup whose failure must remain best effort; in practice: the same commit names B, or A when the lookup is unavailable.
  3. **Unavailable on disagreement** — name neither model when the two disagree; gain: no misleading identity; cost: loses usable attribution in the common single-model case where the lookup is merely incomplete; in practice: the same commit records `unavailable` even though the developer selected a model.
- Disposition: carried — design.md decides; the owner is the design step, and the chosen source must preserve best-effort behavior and never invent a model identity.
