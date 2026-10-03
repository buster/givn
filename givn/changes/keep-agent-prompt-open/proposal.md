# Proposal: keep-agent-prompt-open

## Release note

`givn init` keeps its interactive agent-target prompt open with OpenCode
pre-selected and reports an error when the confirmation marks no target,
instead of closing the dialog.

## Use-Case Context

- Use-case ID: initialize-project
- Ideation topic: none
- Seed path: none
- Confirmed Personas: developer--primary-user

## Problem / Opportunity

The previous change (`require-agent-target`) made an unmarked confirmation
refuse the whole initialization and removed the pre-marked `.opencode` entry.
The operator wants the dialog to stay open instead:

- a bare confirmation should accept the default target set — `.opencode`
  marked — so the common path stays one keystroke;
- a confirmation with nothing marked should not end the dialog, print an
  error naming the requirement, and show the list again.

Today an unmarked confirmation ends the prompt with a refusal, so the
developer loses the dialog, sees an error meant for a scripted path, and must
run `givn init` from the start.

## Proposed Solution

- The interactive list opens with `.opencode` marked; confirming a marked
  selection installs to exactly those targets.
- Confirming with nothing marked prints an error that states at least one
  agent target must be marked and names `.opencode`, `.agents`, and
  `.claude`; the list is shown again, and initialization proceeds only after
  a target is marked.
- Cancelling the prompt with Ctrl-C (SIGINT) still aborts with a non-zero
  exit and no writes; the dialog ignores Esc, matching `dialoguer`'s
  `MultiSelect::interact()`.
- The non-interactive selection rules stay as archived: an empty or
  unknown-only answer and EOF without `--agents` refuse with a non-zero exit
  and write nothing; `--agents` states the selection explicitly. A piped
  answer is not re-asked — a loop could hang automation.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| initialize-a-project | no signal — the top candidates tie at 16.00 | `EXTEND initialize-a-project` | The change refines the interactive prompt behaviour inside the existing capability; no new capability is introduced. |

## Out of Scope

- The non-interactive refusal rules and the `--agents` flag semantics.
- `givn skills sync`'s OpenCode fallback for a config without a skills block.
- The set of known agent targets.
- The pinned-prompt rendering (colors, ordering).

## Open Questions

### D1 — Does the re-ask also apply to a piped answer?

- Question: when stdin is not a terminal and the answer line marks nothing,
  should the command re-read a line until something is marked, like the
  interactive dialog?
- Options:
  1. **Keep the refusal** — a piped empty/unknown-only answer exits
     non-zero with the error; gain: deterministic automation and no read
     loop that could block on a closed or silent stdin; cost: a script that
     passes a bad value must be fixed and rerun; in practice:
     `echo nope | givn init` fails with the error, `givn init --agents
     opencode` works.
  2. **Re-read lines** — a piped answer loop mirrors the dialog; gain:
     symmetry; cost: automation that never sends another line hangs, and the
     error/loop semantics cannot be scripted deterministically; in practice:
     `givn init < /dev/null` would loop on EOF.
- Disposition: answered — the operator asked for the dialog to stay open
  (interactive terminal); the archived scripted-path decision stands, so a
  missing selection fails fast instead of looping.
