This project uses **givn** for spec-driven development. This block is
auto-maintained by `givn init` / `givn skills sync` — do not edit it by hand.

- Run `givn instructions` as the first action of every session. Do not
  search, grep, or explore files to determine what to do next — this command
  inspects the current project state and tells you exactly what command or
  skill to run next, including `--change <CHANGE>` for active changes.
- For every user-facing question, follow the effective dialog policy from
  `givn instructions dialog`; load it once per context before the first
  question, including clarification and confirmation outside a named
  workflow, and pass the obligation explicitly to fresh delegated contexts.
  This obligation applies with optional addons disabled and outside any named
  skill; the presenting agent remains responsible for the question.
- Run `givn status --change <CHANGE>` for a full artifact checklist of an
  active change (includes task progress and pending task descriptions).
{{RESIDENT_RULES}}
- The full workflow, available commands, and skills are documented in the
  output of these commands — this block is intentionally just a pointer.
