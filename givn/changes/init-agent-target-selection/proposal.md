# Proposal: init-agent-target-selection

## Problem / Opportunity

`givn init` and `givn skills sync` write the workflow skills and commands to
every agent directory unconditionally: `.agents/`, `.claude/`, and
`.opencode/`. Most projects are used from a single agent. A user who only
works in opencode gets `.agents/` and `.claude/` directories full of files
they never read and never asked for, polluting the repo and confusing
contributors about which directory is authoritative.

There is no way to choose which agents receive the generated skills. The only
opt-out is manual deletion, which `givn skills sync` silently undoes on the
next run.

## Proposed Solution

When a user runs `givn init`, givn asks which agents it should install the
skills and commands to. The choice is a multi-select list of the three agent
targets: **opencode**, **.agents**, and **.claude**. By default only
**opencode** is selected.

Only the selected targets receive generated files. The choice is stored in
the project so that later `givn skills sync` runs install to exactly the same
targets — a user's selection is not silently undone.

When init runs without an interactive terminal (e.g. in CI or a script), it
does not block on a prompt and falls back to the default: opencode only.

## Out of Scope

- Changing what skills/commands are generated. The skill and command set is
  unchanged; only the set of target directories they are written to changes.
- Adding or removing the three agent targets themselves. The options are fixed
  as opencode, .agents, and .claude.
- A dedicated command to edit the target selection after init. The selection is
  made at init; later re-runs of `givn skills sync` respect it.
- The addon system. Addon enable/disable behaviour is unchanged.

## Open Questions

- None — the target set, the default, and the persistence requirement are
  resolved in the design artifact.
