# Proposal: archive-change

## Use-Case Context

- Use-case ID: `archive-change` (existing, modified; core capability
  `archive-proof`)
- Ideation topic: multi-scm-support
- Confirmed Personas: developer--primary-user, agent--technical-role,
  givn-maintainer--owner

## Problem / Opportunity

The archive proof is Git-only. `givn archive` opens a Git worktree and refuses
any project that is not one, so a project kept in Fossil cannot publish a
completed change at all. The selection seam added for the managed upgrade
(`scm-backend` change) knows a Fossil checkout when it sees one, but the
archive still talks to Git directly.

Fossil has no staging index, so the two-revision proof cannot be copied
literally: a Fossil check-in names the paths it contains, new files must be
added before they can be checked in, and ignored files stay invisible unless
they are force-added. The proof contract — candidate, unbound pending receipt,
one run per verification scope, proven receipt, evidence-only finalization,
forward repair — must hold on both backends without weakening either.

## Proposed Solution

Archiving a completed change runs the same proof on the backend that owns the
project.

- In a Git worktree, the archive behaves as it does today.
- In a Fossil checkout, the clean witness reports `EXTRA` files as dirty and
  blocks the archive; the candidate check-in contains exactly the merged
  corpus paths plus the pending receipt; each verification scope runs once;
  the proven receipt binds the tested revision; the finalization check-in
  contains only the archive evidence (and the README when the readme addon is
  on). The receipt records the revision identifier the backend shows.
- A project without a usable backend is refused by name before anything is
  merged or written.
- A backend hook that rejects the candidate leaves the archive unproven and
  repairable: on Git the merge stays staged, on Fossil the working tree is
  unchanged. The archive is never reported as proven.
- A failed verification scope leaves the published candidate in place; the
  developer repairs the committed state and reruns the archive. The merge is
  not repeated and nothing is rolled back.
- The existing refusal wording becomes backend-neutral: "Archive requires Git
  version control" and "the project root must be the repository root" are
  restated in the shared vocabulary (no version-controlled project; the
  project root must be the worktree top level or the checkout root).

The archive publication path moves onto the selection seam, so the two
backends share the proof pipeline and differ only in their operations.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| archive-proof | EXTEND archive-proof | `EXTEND archive-proof` | The existing capability owns the proof pipeline; this change restates it per backend and adds the Fossil path. |
| adr-qualification-gate | EXTEND adr-qualification-gate | `NEW in fragments` | The capability lives under the support fragment, so the route's `EXTEND` form cannot resolve; the delta modifies the existing fragment scenario. |

## Out of Scope

- The managed upgrade on Fossil and backend-aware initialization, guidance,
  and attribution (later topic seeds).
- `givn status` naming the active backend and the revision in backend-neutral
  wording (later topic seed).
- A configuration key that forces a backend; remote and network operations.
- Installing or configuring Fossil inside givn; the test environment's
  `fossil` availability is a design decision for this change.
- Kiekste hosting.

## Open Questions

- None blocking. The test strategy for the Fossil scenarios (CI availability
  of the `fossil` executable) is a design decision.
