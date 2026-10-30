This project uses **givn** for spec-driven development. This block is
auto-maintained by `givn init` / `givn skills sync` — do not edit it by hand.

- Run `givn instructions` as the first action of every session. Do not
  search, grep, or inspect files to determine what to do next — this command
  reports the current project state and the next lifecycle step, including
  `--change <CHANGE>` for active changes. A reported step applies when
  continuing that change; it does not authorize unrelated work or require every
  task to become a Givn change.
- For new or unclassified work, load `givn-start` to choose answer only, direct
  work without a Givn change, ideation, or a formal change. For a change whose
  goal and scope are already clear, run `givn new <change-id>` directly. Honor
  an explicitly requested, applicable path without another routing conversation.
- For every user-facing question, follow the effective dialog policy from
  `givn instructions dialog`; load it once per context before the first
  question, including clarification and confirmation outside a named
  workflow, and pass the obligation explicitly when work is handed to another
  context. This obligation applies with optional addons disabled and outside
  any named skill; the presenting agent remains responsible for the question.
  Before the first question of a type indexed by the policy, read that type's
  companion examples from `givn instructions dialog-examples`; retained
  examples need no reread. The policy is not loaded automatically.
- Generated guidance is skills-only: Claude Code and OpenCode use their
  native skill invocation, the neutral `.agents` target uses skill discovery,
  OMP uses `/skill:givn-<step>`, and Maki uses an explicit request or an
  agent-selected skill. No generated Givn command files exist.
- Run `givn status --change <CHANGE>` for a full artifact checklist of an
  active change (includes task progress and pending task descriptions).
{{RESIDENT_RULES}}
- The full workflow and available skills are documented in the output of
  these commands — this block is intentionally just a pointer.
