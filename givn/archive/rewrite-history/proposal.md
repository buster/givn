# Proposal: rewrite-history

## Use-Case Context

- Use-case ID: `rewrite-history` (new)
- Ideation topic: `local-only-integrations`
- Seed path: `givn/ideation/local-only-integrations/changes/rewrite-history.md`
- Confirmed Personas: `developer--primary-user`,
  `givn-maintainer--owner`

## Problem / Opportunity

The cleanliness report names generated rules that survive in reachable
history, but nothing in Givn can remove them. A project that committed its
rules before adopting the local-only policy cannot publish a clean history
without leaving the tool and hand-running a destructive rewrite.

## Proposed Solution

One explicit command, never automatic:

1. `givn rewrite-history` requires the `local-only` policy (it refuses under
   `versioned`), a clean checkout, and an explicit confirmation:
   `--yes`, or an interactive prompt; piped input without `--yes` refuses,
   naming the flag.
2. It reports the rule paths found in reachable history and the affected
   revision count before writing anything.
3. It refuses by name when `git-filter-repo` is absent, and refuses by name
   on the Fossil backend.
4. On success, no reachable revision contains a rule, every rule file still
   exists on disk, the new revision identifier is reported, and the developer
   is told that a force-push is required and that remotes and forks are
   outside the guarantee.

## Capability Routing

`givn spec route --change rewrite-history` reports no signal.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `history-rewrite` | no signal | `NEW in rewrite-history` | A new user goal with its own use case and command. |

## Out of Scope

- Automatic rewriting or rewriting as a side effect of another command.
- Pushing, remote repair, or fork cleanup.
- Fossil history rewriting (named refusal).
- Backup policy for the original refs (documented by `git-filter-repo`).

## Open Questions

None blocking. The command name is the working name from the ideation topic
(seed Q21 resolved as `givn rewrite-history` by this change).
