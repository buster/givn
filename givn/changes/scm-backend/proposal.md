# Proposal: scm-backend

## Use-Case Context

- Use-case ID: `scm-backend` (ideation fragment; realized as capability
  `scm-backend-selection` in the permanent `support` fragment)
- Ideation topic: multi-scm-support
- Confirmed Personas: developer--primary-user, agent--technical-role,
  givn-maintainer--owner

## Problem / Opportunity

givn assumes Git everywhere. The archive boundary and the managed upgrade each
run their own Git checks, so the same assumption exists twice and can drift. A
project kept in Fossil cannot use either path, and what its maintainer sees is
a message about Git rather than about the project's own version-control state.

No command answers "which version-control system owns this project?". There is
no single named refusal when the answer is "none", no recognition when the
answer is "Fossil", and no place that states what the active system can do:
witness a clean state, name a revision, publish a chosen subset of paths, or
reject a revision through a hook.

## Proposed Solution

givn detects the version-control backend of the project root and speaks about
it in one vocabulary.

- A Git worktree selects Git. The archive behaves exactly as it does today.
  The managed upgrade keeps its dirty-checkout refusal and gains the same
  nested-root refusal the archive already has, so a project nested inside a
  larger repository is no longer upgraded against the enclosing one.
- A Fossil checkout is recognized as a version-controlled project. Until the
  Fossil backend is available, commands that need repository state refuse by
  name: the refusal names the Fossil checkout, the project root, and the fact
  that the backend is not available in this version. Nothing is changed.
- Both a Git and a Fossil marker at the project root: refused as ambiguous,
  naming both markers. Git is never silently preferred.
- Neither marker: the project has no backend. Commands that need repository
  state refuse by name, naming the project root and the command that would
  have run; `givn init` and `givn status` keep working. Nothing is changed.
- A selected backend whose executable is missing is refused by name, naming
  the executable, before anything is changed.
- No configuration key selects the backend: the project's own markers decide.
  A forced backend can be added later if mixed setups demand it.

Repository-state questions are answered in one place, so no command can assume
a different backend than another. The managed upgrade moves its checks behind
that place; the archive keeps its current Git boundary and moves onto the seam
with the change that implements the Fossil backend.

## Capability Routing

`givn spec route` returned a flat 7.50 ranking whose top entry was
`EXTEND archive-proof`, driven by shared `givn status` and `givn init`
mentions rather than by detection behavior. The ranking carries no distinct
signal, so the decision below is made on capability boundaries.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| scm-backend-selection | EXTEND archive-proof (top of a flat 7.50 ranking) | `NEW in fragments` | No existing capability owns backend detection or refusal; `archive-proof` only consumes it. Detection is reusable behavior with no independent user goal, so it belongs to the `support` fragment. |
| release-0-3-0 | EXTEND release-0-3-0 | `EXTEND release-0-3-0` | The managed upgrade's no-version-control refusal changes from "requires a Git repository" to the backend-neutral named refusal; the protected-migration scenario records the new wording. |
| adr-qualification-gate | EXTEND adr-qualification-gate | `NEW in fragments` | ADR-071 is a new accepted record, so the consolidated-register scenario's accepted count moves from 26 to 27. The capability lives in the reusable support fragment, which the EXTEND route cannot resolve. |

## Out of Scope

- The Fossil backend itself: selecting it, witnessing a Fossil checkout,
  publishing a Fossil revision, or committing through Fossil. This change
  recognizes a Fossil checkout only to refuse it by name; the archive change
  that first needs Fossil adds the backend.
- Moving the archive publication path onto the seam: the archive keeps its
  current Git boundary until the change that implements the Fossil backend.
- Initialization, generated guidance, and `givn commit` for a Fossil project;
  `givn status` reporting the active backend.
- A configuration key that forces a backend.
- Installing or configuring Git or Fossil; remote and network operations.
- Kiekste hosting.

## Open Questions

- None blocking. The vocabulary (`revision`, `checkout`, `clean state`,
  capability facts) is fixed in the project glossary; the test strategy is a
  design decision.
