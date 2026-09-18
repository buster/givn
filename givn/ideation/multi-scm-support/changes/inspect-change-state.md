# Change seed: inspect-change-state

Status: implemented in `inspect-change-state`

- Use-case ID: `inspect-change-state` (existing, modified)
- Topic: `multi-scm-support`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`
- Suggested start: after `scm-backend`; could be folded into it
- Handoff: ready for `/givn-propose` (historical)

## Accepted decisions

### Backend in status output

- Context: the developer needs to know which backend owns the project
  without reading markers by hand.
- Options:
  1. **Report `SCM: git`, `SCM: fossil`, or `SCM: none`** — a project
     without a backend still exits 0 and names archive and managed upgrade
     as unavailable.
     In practice: `givn status` prints the active backend line; the unproven
     candidate report stays owned by `archive-change`.
     Gain: one place to see the selection.
     Cost: none.
  2. **Leave the backend implicit** — no status line.
     In practice: the developer inspects `.git`/`.fslckout` manually.
     Gain: no output change.
     Cost: hidden state.
- Chosen: **Option 1** because the selection is project state. Rejected:
  Option 2.

## Open questions

None.

## Scope note

Modified `givn/specs/inspect-change-state/givn-status-command.feature` with
one new scenario.
