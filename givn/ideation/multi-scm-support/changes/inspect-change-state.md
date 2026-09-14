# Change seed: inspect-change-state

- **Stable use-case ID:** `inspect-change-state` (existing, possibly modified)
- **Topic source:** `givn/ideation/multi-scm-support/`
  (`use-cases/inspect-change-state.md`, eventstorming A1, E2)
- **Confirmed Persona IDs:** `developer--primary-user`,
  `agent--technical-role`, `givn-maintainer--owner`
- **Accepted decisions:**
  - `givn status` reports the active backend as `SCM: git`, `SCM: fossil`, or
    `SCM: none`; a project without a backend still exits 0 and names archive
    and managed upgrade as unavailable. One new scenario; the unproven
    candidate report remains owned by `archive-change`.
- **Open questions:** none.
- **Handoff status:** ready for `/givn-propose`; may be folded into seed 1.
- **Scope note:** modifies `givn/specs/inspect-change-state/givn-status-command.feature`.
