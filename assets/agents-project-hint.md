This project uses **givn** for spec-driven development. This block is
auto-maintained by `givn init` / `givn skills sync` — do not edit it by hand.

- Run `givn instructions` as the first action of every session. Do not
  search, grep, or explore files to determine what to do next — this command
  inspects the current project state and tells you exactly what command or
  skill to run next, including `--change <CHANGE>` for active changes.
- Run `givn status --change <CHANGE>` for a full artifact checklist of an
  active change (includes task progress and pending task descriptions).
- Tick off each task in `tasks.md` immediately after completing it —
  an unchecked box means that scenario is not done. Never batch-check
  boxes at the end.
- One atomic `git commit` per scenario (RED+GREEN+REFACTOR together).
  No commit, or a commit touching only the spec, means the work is not
  done. Record the commit hash in tasks.md.
- The full workflow, available commands, and skills are documented in the
  output of these commands — this block is intentionally just a pointer.
