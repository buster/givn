# Proposal: mask-rule-names

## Use-Case Context

- Use-case ID: `initialize-project` (modified)
- Ideation topic: none — follow-up to `local-only-rules`
- Seed path: none
- Confirmed Personas: `developer--primary-user`,
  `external-contributor--open-source-consumer`, `givn-maintainer--owner`

## Problem / Opportunity

The local-only ignore block enumerates every generated rule path, for example
`.opencode/skills/givn-propose/SKILL.md`. A published repository therefore
leaks the names of Givn's skills and commands in its tracked `.gitignore`,
even though the rules themselves stay local. The ignore block must not name
them.

## Proposed Solution

The fenced ignore entries become prefix patterns instead of per-file paths:

- `.opencode/skills/givn-*/`, `.agents/skills/givn-*/`,
  `.claude/skills/givn-*/`
- `.opencode/commands/givn-*.md`, `.opencode/commands/grill-*.md`
- the fixed single-file integrations (`.githooks/commit-msg`, the OpenCode
  plugin, the Claude hook, `.claude/settings.local.json`)

Observable outcomes:

1. A published `.gitignore` contains no individual skill or command name.
2. The transition and the cleanliness check still find tracked rules
   (including orphans from a disabled addon) through the existing path
   predicate, so nothing becomes untracked-invisible.
3. Under `versioned` the block is still removed.

## Capability Routing

`givn spec route --change mask-rule-names` reports no signal.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `initialize-a-project` | no signal | `EXTEND initialize-a-project` | The ignore entries belong to the setup capability. |

## Out of Scope

- The discovery predicate and the cleanliness scan.
- The ignore destination files themselves.
- Changing the patterns' reserved namespace (`givn-*`, `grill-*`).

## Open Questions

None.
