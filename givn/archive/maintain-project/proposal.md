# Proposal: maintain-project

## Use-Case Context

- Use-case ID: `maintain-project` (existing, modified; core capability
  `release-0-3-0`)
- Ideation topic: multi-scm-support
- Confirmed Personas: developer--primary-user, givn-maintainer--owner

## Problem / Opportunity

The managed upgrade still refuses a Fossil project by name. The archive runs
on Fossil, but `givn upgrade` — the command that refreshes the managed
scaffold, backfills the run declaration, synchronizes generated guidance, and
records migration plans — cannot publish its revisions there. A Fossil project
therefore cannot cross a release migration without leaving givn's supported
path.

The publication contract already exists: the archive publishes
witness-bounded revisions through the SCM seam. The managed upgrade needs the
same treatment, plus its own rule: only the calculated managed allowlist is
ever staged, and ignored managed paths are force-added only when the allowlist
names them.

## Proposed Solution

A managed upgrade on a Fossil checkout behaves like one on Git:

- The preflight selects the backend, verifies the checkout is clean, and
  refuses a dirty project by name before writing anything.
- The managed scaffold changes are published as exactly one allowlist-only
  revision; ignored managed paths are recorded only through the calculated
  allowlist (`fossil add --force` for those paths, never a directory or the
  whole checkout).
- A migration-free upgrade publishes exactly one revision and no plan
  revision. A migration-bearing upgrade publishes the upgrade revision and
  then the plan revision, whose design records the upgrade revision
  identifier.
- The observable upgrade-revision content and wording become
  backend-neutral; a project without a usable backend is still refused by
  name before mutation.

The temporary "Fossil backend is not available for the managed upgrade"
refusal from the archive change is removed, and the fragment scenario that
asserted it ("A Fossil checkout is refused by name until the Fossil backend
is available") is retired because the behavior no longer holds.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| release-0-3-0 | EXTEND release-0-3-0 | `EXTEND release-0-3-0` | The existing capability owns the managed upgrade; this change makes its preflight, allowlist, and revision publication backend-neutral. |
| scm-backend-selection | EXTEND scm-backend-selection | `NEW in fragments` | The capability lives under the support fragment; the delta retires the scenario that asserted the upgrade's Fossil refusal. |

## Out of Scope

- The observable upgrade-revision content assertions in
  `givn/specs/operate-product/run-command.feature` (updated by a later topic
  seed).
- Backend-aware initialization, guidance, and commit attribution.
- Remote or network operations; a configuration key that forces a backend.
- Kiekste hosting.

## Open Questions

- None blocking. The migration-catalog content is unchanged; only the
  publication path becomes backend-neutral.
